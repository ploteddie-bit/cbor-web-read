//! cbor-web-verify-signature — verifies the Ed25519 signature in a CBOR-Web
//! file against a public key provided by the operator.
//!
//! The signed bytes are the canonical CBOR encoding of the document with the
//! `meta.signature` entry removed.
//!
//! Usage:
//!   cbor-web-verify-signature <file.cbor> --pubkey <base64url>
//!   cbor-web-verify-signature <file.cbor> --pubkey-file <path>
//!
//! The public key MUST be a raw Ed25519 32-byte key encoded in base64url
//! (the same format published in DNS TXT records as `pk=<base64url>`).
//!
//! DNS TXT lookup is intentionally NOT part of this binary — it is a network
//! dependency with its own caching, retry, and DNSSEC considerations. The
//! operator passes the key explicitly; tooling that performs DNS lookup can
//! call this binary or `bytes_for_signature` directly.

use anyhow::{bail, Context, Result};
use base64::Engine;
use cbor_web_tools::bytes_for_signature;
use clap::Parser;
use ed25519_dalek::{Signature, Verifier, VerifyingKey};
use std::fs;
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Parser)]
#[command(version, about = "Verify Ed25519 signature in a CBOR-Web file")]
struct Args {
    /// Path to the .cbor file.
    file: PathBuf,
    /// Public key as base64url (Ed25519, 32 raw bytes).
    #[arg(long, conflicts_with = "pubkey_file")]
    pubkey: Option<String>,
    /// Path to a file containing the public key as base64url on the first line.
    #[arg(long)]
    pubkey_file: Option<PathBuf>,
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("verification failed: {e:#}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    let pubkey_b64 = match (args.pubkey, args.pubkey_file) {
        (Some(s), None) => s,
        (None, Some(p)) => fs::read_to_string(&p)
            .with_context(|| format!("reading {}", p.display()))?
            .lines()
            .next()
            .ok_or_else(|| anyhow::anyhow!("empty pubkey file"))?
            .trim()
            .to_string(),
        (Some(_), Some(_)) => bail!("pass exactly one of --pubkey or --pubkey-file"),
        (None, None) => bail!("pass --pubkey or --pubkey-file"),
    };

    let key_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(pubkey_b64.trim_end_matches('='))
        .context("decoding pubkey as base64url")?;
    if key_bytes.len() != 32 {
        bail!(
            "Ed25519 public key must be 32 bytes, got {}",
            key_bytes.len()
        );
    }
    let key_array: [u8; 32] = key_bytes
        .as_slice()
        .try_into()
        .expect("length checked above");
    let verifying_key =
        VerifyingKey::from_bytes(&key_array).context("invalid Ed25519 public key")?;

    let file_bytes =
        fs::read(&args.file).with_context(|| format!("reading {}", args.file.display()))?;
    let (signed_bytes, sig_bytes) = bytes_for_signature(&file_bytes)?;
    if sig_bytes.len() != 64 {
        bail!(
            "Ed25519 signature must be 64 bytes, meta.signature is {} bytes",
            sig_bytes.len()
        );
    }
    let sig_array: [u8; 64] = sig_bytes
        .as_slice()
        .try_into()
        .expect("length checked above");
    let signature = Signature::from_bytes(&sig_array);
    verifying_key
        .verify(&signed_bytes, &signature)
        .context("Ed25519 verification failed")?;

    println!(
        "[{}] signature OK ({} signed bytes)",
        args.file.display(),
        signed_bytes.len()
    );
    Ok(())
}
