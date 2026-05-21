//! cbor-web-decode — pretty-prints a CBOR-Web file as JSON.
//!
//! Usage:
//!   cbor-web-decode <file.cbor>            # full JSON to stdout
//!   cbor-web-decode <file.cbor> --preview  # truncates page content blocks

use anyhow::{Context, Result};
use cbor_web_tools::decode_to_json;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Decode a CBOR-Web file to JSON")]
struct Args {
    file: PathBuf,
    /// Show only the first 5 content blocks per page.
    #[arg(long)]
    preview: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bytes = fs::read(&args.file).with_context(|| format!("reading {}", args.file.display()))?;
    let mut json = decode_to_json(&bytes)?;
    if args.preview {
        truncate(&mut json);
    }
    println!("{}", serde_json::to_string_pretty(&json)?);
    Ok(())
}

fn truncate(v: &mut serde_json::Value) {
    let Some(obj) = v.as_object_mut() else { return };
    let Some(pages) = obj.get_mut("5") else {
        return;
    };
    let Some(arr) = pages.as_array_mut() else {
        return;
    };
    for page in arr {
        let Some(p) = page.as_object_mut() else {
            continue;
        };
        let Some(content) = p.get_mut("content") else {
            continue;
        };
        let Some(blocks) = content.as_array_mut() else {
            continue;
        };
        let total = blocks.len();
        if total > 5 {
            blocks.truncate(5);
            blocks.push(
                serde_json::json!({"_truncated": format!("{} more blocks omitted", total - 5)}),
            );
        }
    }
}
