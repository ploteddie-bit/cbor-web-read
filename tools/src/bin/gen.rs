//! cbor-web-gen — turns a YAML source file into a signed-ready CBOR-Web index.
//!
//! Usage:
//!   cbor-web-gen <input.yaml> -o <output.cbor>
//!
//! Source format: see tools/examples/*.yaml. The generator fills in default
//! values for security (default_access=T2), navigation (main = T2 paths),
//! and meta (generated_at, generator, total_pages).

use anyhow::{Context, Result};
use cbor_web_tools::{encode_site, SiteDoc};
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Generate a CBOR-Web index from a YAML source file")]
struct Args {
    /// YAML source describing the site.
    input: PathBuf,
    /// Output .cbor path.
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let yaml = fs::read_to_string(&args.input)
        .with_context(|| format!("reading {}", args.input.display()))?;
    let doc: SiteDoc = serde_yaml::from_str(&yaml).context("parsing YAML source")?;
    let bytes = encode_site(&doc).context("encoding to CBOR")?;
    fs::write(&args.output, &bytes)
        .with_context(|| format!("writing {}", args.output.display()))?;
    println!(
        "wrote {} ({} bytes, {} pages)",
        args.output.display(),
        bytes.len(),
        doc.pages.len()
    );
    Ok(())
}
