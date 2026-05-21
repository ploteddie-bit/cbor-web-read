//! cbor-web-validate — checks a CBOR-Web file against the read protocol spec.
//!
//! Usage:
//!   cbor-web-validate <file.cbor> [more.cbor ...]
//!
//! Exits with status 0 if every file is valid, 1 otherwise.

use anyhow::{bail, Context, Result};
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_MAX_BYTES: u64 = 5 * 1024 * 1024;

#[derive(Parser)]
#[command(
    version,
    about = "Validate CBOR-Web index files against the read protocol"
)]
struct Args {
    /// Files to validate.
    #[arg(required = true)]
    files: Vec<PathBuf>,
    /// Treat warnings as errors.
    #[arg(long)]
    strict: bool,
    /// Reject files larger than this many bytes before decoding.
    /// Default is the spec's 5 MB cap.
    #[arg(long, default_value_t = DEFAULT_MAX_BYTES)]
    max_size: u64,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let mut had_error = false;
    for path in &args.files {
        match check(path, args.strict, args.max_size) {
            Ok(failed) => {
                if failed {
                    had_error = true;
                }
            }
            Err(e) => {
                eprintln!("[{}] error: {e:#}", path.display());
                had_error = true;
            }
        }
    }
    if had_error {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}

fn check(path: &PathBuf, strict: bool, max_size: u64) -> Result<bool> {
    let meta = fs::metadata(path).with_context(|| format!("stat {}", path.display()))?;
    if meta.len() > max_size {
        bail!(
            "file is {} bytes, exceeds --max-size {} (spec §6.2 caps index.cbor at 5 MB; paginate with `next` for larger sites)",
            meta.len(),
            max_size
        );
    }
    let bytes = fs::read(path).with_context(|| format!("reading {}", path.display()))?;
    let report = cbor_web_tools::validate_bytes(&bytes)?;
    let label = path.display();
    if report.ok() && report.warnings.is_empty() {
        println!("[{label}] OK ({} bytes)", bytes.len());
        return Ok(false);
    }
    for w in &report.warnings {
        println!("[{label}] warning: {w}");
    }
    for e in &report.errors {
        println!("[{label}] error: {e}");
    }
    let failed = !report.ok() || (strict && !report.warnings.is_empty());
    if failed {
        println!("[{label}] FAILED");
    } else {
        println!("[{label}] OK with warnings ({} bytes)", bytes.len());
    }
    Ok(failed)
}
