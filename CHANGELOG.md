# Changelog

All notable changes to CBOR-Web (this read protocol) are tracked here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).
This repository uses [Semantic Versioning](https://semver.org/spec/v2.0.0.html)
for the specification version (e.g. `v3.0.0`, `v3.0.1`) and matches the
`tools/` reference implementation to the same versions.

## [Unreleased]

### Added
- **`cbor-web-canonicalize`** — re-encode a CBOR-Web file in canonical
  RFC 8949 §4.2 form (sorted map keys, definite lengths, minimal integers).
  Required before signing or computing `page.hash`.
- **`cbor-web-verify-signature`** — verify a `meta.signature` (Ed25519, 64
  bytes) against a public key (32-byte raw key, base64url). DNS TXT lookup
  is intentionally out of scope.
- Strict canonical-encoding check in `cbor-web-validate`: bytewise compare
  against a canonical re-encode (previously: length compare only).
- `page.hash` recompute and compare against SHA-256 of the canonical-encoded
  page with the `hash` field removed.
- `meta.total_pages` consistency check vs `pages.length`.
- `--max-size` flag on `cbor-web-validate`, default 5 MB per spec §6.2.
  Files larger than the cap are rejected before being read into memory.
- 8 unit tests and 2 integration tests (`cargo test`) covering canonical
  ordering, validation rejection paths, signature round-trip, and tamper
  detection.
- CI matrix expanded to ubuntu-latest, macos-latest, windows-latest for
  validate-examples and cargo-test jobs.
- `cargo audit` job added to CI.
- `proptest` declared as dev-dependency for future property-based tests.

### Changed
- `cbor-web-gen` and `cbor-web-migrate` now emit canonical CBOR through
  `canonical_bytes` (was: ciborium default order, which is insertion-order
  for maps).
- All 8 example `.cbor` files re-encoded canonically. Byte size unchanged
  (canonicalisation only reorders map keys).
- `CBOR-WEB-SPEC-v3.0.md` now specifies precisely what bytes feed into
  `page.hash` and `meta.signature` (see §6 and §8).

### Deprecated
- `cbor-web-migrate` will be removed in a future major release once the
  legacy 4-keys layout is no longer in circulation. A note has been added
  to the binary's output.

## [3.0.0] — 2026-05-21

Initial public release of the 7-keys structure and reference implementation.

### Added
- Specification (`CBOR-WEB-SPEC-v3.0.md`) with the 7 top-level keys:
  0 type, 1 version, 2 site, 3 security, 4 navigation, 5 pages, 6 meta.
- Reference implementation in Rust (`tools/`) with four binaries:
  `cbor-web-validate`, `cbor-web-gen`, `cbor-web-decode`, `cbor-web-migrate`.
- Optional commercial extension (`CBOR-WEB-MONETIZATION.md`) describing
  hosted generation services, visibility signals, and an ERC-20 token model.
  Not part of the read protocol.
- Eight worked examples (`examples/*.cbor`) and a human-readable JSON
  preview of one of them.
- Public-domain CC0 licence on the spec, the reference implementation, and
  the examples.

[Unreleased]: https://github.com/ploteddie-bit/cbor-web-read/compare/v3.0.0...HEAD
[3.0.0]: https://github.com/ploteddie-bit/cbor-web-read/releases/tag/v3.0.0
