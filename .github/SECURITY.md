# Security Policy

## Reporting a vulnerability

Do **not** open a public issue for a security vulnerability in the spec or
in the `tools/` reference implementation.

Use GitHub's private vulnerability reporting:

1. Go to the **Security** tab of this repository.
2. Click **Report a vulnerability**.
3. Provide a description, steps to reproduce, potential impact, and a
   suggested fix if you have one.

We aim to acknowledge reports within 48 hours.

## Supported versions

| Version | Supported |
|---|---|
| 3.0.x | yes |
| < 3.0 | no |

## Security considerations

CBOR-Web files contain only structured data — no executable content, no
scripts, no client-side code. That said, the following risks apply to any
parser handling untrusted CBOR:

- **Denial of service via malformed input.** A maliciously crafted file may
  trigger allocator pressure or pathological decode paths. Use decoders
  that enforce sane limits on map/array sizes and recursion depth. The
  reference Rust implementation in `tools/` relies on
  [`ciborium`](https://crates.io/crates/ciborium), which decodes in linear
  memory; large files should still be validated against a maximum size
  before decoding.
- **Untrusted signatures.** A signature in `meta.signature` is only
  meaningful if you verify it against the public key in the publisher's
  DNS TXT record. A file without a verified signature carries no
  authentication guarantees.
- **Self-described tag confusion.** Always verify the first three bytes are
  `D9 D9 F7` before treating the file as CBOR-Web.
- **Forward-compatible keys.** Unknown top-level keys MUST be ignored, not
  rejected. A parser that fails on unknown keys creates a fragility surface.

## Acknowledgements

Responsible disclosure is appreciated. Researchers who report valid issues
will be credited in the release notes (with permission).
