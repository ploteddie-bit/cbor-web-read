//! cbor-web-migrate — converts a legacy 4-keys CBOR-Web file (pages in key 3)
//! to the v3.0 structure with 7 keys (pages in key 5).
//!
//! Usage:
//!   cbor-web-migrate <old.cbor> -o <new.cbor>

use anyhow::{anyhow, bail, Context, Result};
use ciborium::value::{Integer, Value};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about = "Migrate a legacy CBOR-Web file (4 keys) to v3.0 (7 keys)"
)]
struct Args {
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    eprintln!(
        "cbor-web-migrate is a one-shot tool for the legacy 4-keys layout. It will be removed in a future major release once such files are no longer in circulation."
    );
    let bytes =
        fs::read(&args.input).with_context(|| format!("reading {}", args.input.display()))?;
    let value: Value = ciborium::de::from_reader(&bytes[..])
        .with_context(|| format!("decoding {}", args.input.display()))?;
    let inner = match &value {
        Value::Tag(55799, inner) => inner.as_ref().clone(),
        v => v.clone(),
    };
    let Value::Map(legacy) = inner else {
        bail!("root is not a map");
    };

    let mut site_meta: Option<Value> = None;
    let mut pages_map: Option<Vec<(Value, Value)>> = None;
    let mut version: Option<Value> = None;
    let mut type_v: Option<Value> = None;
    for (k, v) in legacy {
        match key_as_int(&k) {
            Some(0) => type_v = Some(v),
            Some(1) => version = Some(v),
            Some(2) => site_meta = Some(v),
            Some(3) => match v {
                Value::Map(m) => pages_map = Some(m),
                _ => bail!("legacy key 3 expected to be a pages map"),
            },
            _ => {}
        }
    }

    let site_meta = site_meta.ok_or_else(|| anyhow!("missing key 2 (site)"))?;
    let pages_map = pages_map.ok_or_else(|| anyhow!("missing key 3 (pages map)"))?;

    let (site_v2, default_lang) = transform_site(site_meta);
    let pages_v5 = transform_pages(pages_map, default_lang.as_deref());
    let nav_v4 = derive_navigation(&pages_v5);
    let total_pages = match &pages_v5 {
        Value::Array(a) => a.len(),
        _ => 0,
    };

    let entries: Vec<(Value, Value)> = vec![
        (
            int(0),
            type_v.unwrap_or_else(|| Value::Text("cbor-web".into())),
        ),
        (int(1), version.unwrap_or_else(|| int(3))),
        (int(2), site_v2),
        (
            int(3),
            Value::Map(vec![(
                Value::Text("default_access".into()),
                Value::Text("T2".into()),
            )]),
        ),
        (int(4), nav_v4),
        (int(5), pages_v5),
        (
            int(6),
            Value::Map(vec![
                (
                    Value::Text("generator".into()),
                    Value::Text("cbor-web-migrate/0.1".into()),
                ),
                (Value::Text("total_pages".into()), int(total_pages as i128)),
            ]),
        ),
    ];

    let root = Value::Tag(55799, Box::new(Value::Map(entries)));
    let out = cbor_web_tools::canonical_bytes(&root)?;
    fs::write(&args.output, &out).with_context(|| format!("writing {}", args.output.display()))?;
    println!(
        "migrated {} ({} bytes) -> {} ({} bytes, {} pages)",
        args.input.display(),
        bytes.len(),
        args.output.display(),
        out.len(),
        total_pages
    );
    Ok(())
}

fn key_as_int(v: &Value) -> Option<i128> {
    if let Value::Integer(i) = v {
        Some((*i).into())
    } else {
        None
    }
}

fn int(n: i128) -> Value {
    Value::Integer(Integer::try_from(n).expect("integer in range"))
}

fn text(s: impl Into<String>) -> Value {
    Value::Text(s.into())
}

/// Legacy site (key 2) used "lang" (single) and "legal"/"network" extras.
/// New site keeps everything but normalises to "languages" + "default_language".
fn transform_site(v: Value) -> (Value, Option<String>) {
    let Value::Map(m) = v else {
        return (v, None);
    };
    let mut default_lang = None;
    let mut entries: Vec<(Value, Value)> = Vec::new();
    for (k, val) in m {
        if let Value::Text(key) = &k {
            if key == "lang" {
                if let Value::Text(l) = &val {
                    default_lang = Some(l.clone());
                    entries.push((text("languages"), Value::Array(vec![text(l)])));
                    entries.push((text("default_language"), text(l)));
                    continue;
                }
            }
        }
        entries.push((k, val));
    }
    (Value::Map(entries), default_lang)
}

fn transform_pages(legacy: Vec<(Value, Value)>, default_lang: Option<&str>) -> Value {
    let mut out: Vec<Value> = Vec::with_capacity(legacy.len());
    for (path_k, page_v) in legacy {
        let Value::Text(path) = path_k else { continue };
        let Value::Map(fields) = page_v else { continue };
        let mut entries: Vec<(Value, Value)> = Vec::new();
        entries.push((text("path"), text(&path)));
        let mut has_lang = false;
        let mut has_access = false;
        for (k, v) in fields {
            if let Value::Text(name) = &k {
                if name == "lang" {
                    has_lang = true;
                }
                if name == "access" {
                    has_access = true;
                }
            }
            entries.push((k, v));
        }
        if !has_lang {
            entries.push((text("lang"), text(default_lang.unwrap_or("en"))));
        }
        if !has_access {
            entries.push((text("access"), text("T2")));
        }
        out.push(Value::Map(entries));
    }
    Value::Array(out)
}

fn derive_navigation(pages: &Value) -> Value {
    let Value::Array(arr) = pages else {
        return Value::Map(vec![(text("main"), Value::Array(vec![]))]);
    };
    let mut main: Vec<Value> = Vec::new();
    for p in arr {
        if let Value::Map(m) = p {
            for (k, v) in m {
                if let (Value::Text(name), Value::Text(path)) = (k, v) {
                    if name == "path" {
                        main.push(text(path));
                    }
                }
            }
        }
        if main.len() >= 10 {
            break;
        }
    }
    Value::Map(vec![(text("main"), Value::Array(main))])
}
