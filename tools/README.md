# tools/ — CBOR-Web reference implementation (Rust)

Four binaries that read, write and validate CBOR-Web files against the read
protocol spec. License: **CC0 1.0** (public domain).

## Build

Requires Rust 1.74+ (stable). From the repository root:

```sh
cargo build --release --manifest-path tools/Cargo.toml
```

Binaries land in `tools/target/release/`.

## Binaries

### `cbor-web-validate`

Verifies that a file conforms to the read protocol: self-described CBOR tag
(`55799`), required keys (0, 1, 2, 5), valid types, valid access tiers,
known block types, and canonical-length encoding (RFC 8949 §4.2).

```sh
cbor-web-validate examples/pacific-planet.cbor
cbor-web-validate --strict examples/*.cbor   # warnings become errors
```

Exit code is 0 if every file is valid, 1 otherwise.

### `cbor-web-decode`

Pretty-prints a CBOR-Web file as JSON. Integer top-level keys appear as their
string form (`"0"`, `"1"`, …). Use `--preview` to truncate page content to
the first five blocks per page (useful for human-readable previews next to
binary files).

```sh
cbor-web-decode examples/pacific-planet.cbor              # full JSON
cbor-web-decode examples/pacific-planet.cbor --preview    # truncated
```

### `cbor-web-gen`

Builds a CBOR-Web file from a YAML source. The generator fills in defaults
(security.default_access = `T2`, navigation.main = first 10 `T2` pages,
meta.total_pages, meta.generator).

```sh
cbor-web-gen tools/examples/site.yaml -o examples/site.cbor
```

See `tools/examples/site.yaml` for the source format.

### `cbor-web-migrate`

One-shot converter: legacy 4-keys file (pages under key 3) → v3.0 7-keys
structure. Output is canonical. Kept available for anyone holding legacy
files; will be removed in a future major release.

```sh
cbor-web-migrate old.cbor -o new.cbor
```

### `cbor-web-canonicalize`

Re-encodes a CBOR-Web file in canonical RFC 8949 §4.2 form: sorted map keys
(length-first then bytewise), definite lengths, minimal integers. Run this
on any file produced by a non-canonical encoder before signing or computing
`page.hash`.

```sh
cbor-web-canonicalize in.cbor -o out.cbor
```

### `cbor-web-verify-signature`

Verifies the Ed25519 signature in `meta.signature` against a public key the
operator passes in (32-byte raw key, base64url — same format published in
DNS TXT records as `pk=<base64url>`).

```sh
cbor-web-verify-signature site.cbor --pubkey "MCowBQYDK2VwAyEA…"
cbor-web-verify-signature site.cbor --pubkey-file ./operator.pub
```

DNS TXT lookup is **not** part of this binary. It is a network dependency
with its own caching, retry, and DNSSEC considerations; orchestration
tooling can call this binary after performing its own lookup.

## What the validator checks

| Check | Severity |
|-------|----------|
| First 3 bytes = `d9 d9 f7` (tag 55799) | error |
| Root is a CBOR map | error |
| Required keys present (0 type, 1 version, 2 site, 5 pages) | error |
| Key 0 equals `"cbor-web"` | error |
| Key 1 equals `3` | error |
| `site.domain` and `site.name` are text strings | error |
| `security.default_access` is one of `T0`/`T1`/`T2` | error |
| Each page has `path`, `title`, `lang`, `content` | error |
| `page.access` is one of `T0`/`T1`/`T2` | error |
| Each content block has a `t` (type) field | error |
| Block `t` is a known type (`h`/`p`/`ul`/...) | warning |
| Re-encoded bytes match original (canonical RFC 8949 §4.2 strict) | warning |
| `meta.total_pages` matches `pages.length` | error |
| `page.hash` matches SHA-256 of canonical-encoded page (without `hash`) | error |
| File size ≤ `--max-size` (default 5 MB per spec §6.2) | error |

Unknown top-level keys are accepted silently — RFC 8949 §4.2 requires that
agents tolerate forward-compatible extensions.

## Adding a new check

Open `src/lib.rs`, locate `validate_bytes`, and append to the `Report` via
`r.err` (hard failure) or `r.warn` (soft signal). Keep the checks ordered so
that earlier failures stop the validator from cascading misleading errors.
