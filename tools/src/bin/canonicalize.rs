//! cbor-web-canonicalize — re-encodes a CBOR-Web file in canonical form
//! (RFC 8949 §4.2: sorted map keys, definite lengths, minimal integers).
//!
//! Use this on files produced by encoders that do not enforce canonical map
//! key ordering — without it, signatures and `page.hash` values will not
//! match what a strict verifier computes.
//!
//! Usage:
//!   cbor-web-canonicalize <in.cbor> -o <out.cbor>

use anyhow::{anyhow, Context, Result};
use cbor_web_tools::canonical_bytes;
use ciborium::value::Value;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about = "Re-encode a CBOR-Web file in canonical form")]
struct Args {
    input: PathBuf,
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let bytes =
        fs::read(&args.input).with_context(|| format!("reading {}", args.input.display()))?;
    let value: Value = ciborium::de::from_reader(&bytes[..]).context("decoding source CBOR")?;
    let out = canonical_bytes(&value).map_err(|e| anyhow!("encode error: {e}"))?;
    fs::write(&args.output, &out).with_context(|| format!("writing {}", args.output.display()))?;
    println!(
        "canonicalised {} ({} bytes) -> {} ({} bytes)",
        args.input.display(),
        bytes.len(),
        args.output.display(),
        out.len()
    );
    Ok(())
}
