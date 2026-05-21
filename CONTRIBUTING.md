# Contributing to CBOR-Web

Thanks for considering a contribution. This repository contains the
specification (`CBOR-WEB-SPEC-v3.0.md`), the reference implementation in
`tools/` (Rust), and example `index.cbor` files generated from real sites.

The protocol and all code are released under **CC0 1.0** (public domain). By
opening a pull request, you confirm that your contribution can be released
under the same licence.

## Filing issues

Use the templates in `.github/ISSUE_TEMPLATE/`:

- **Bug report** — a parser, validator, or example behaves incorrectly.
- **Feature request** — proposed addition to the spec (new block type, new
  top-level key, …) or to the tooling.

Before filing, please run the validator on your file:

```sh
cargo run --release --manifest-path tools/Cargo.toml --bin cbor-web-validate -- your-file.cbor
```

and attach its output. Include the parser language and library if you hit a
decoding issue (e.g. `ciborium 0.2`, `fxamacker/cbor v2.5`).

## Pull requests

1. Fork and create a branch off `main`.
2. Make your change.
3. If you touch `examples/`, regenerate the affected files and ensure
   `cbor-web-validate examples/*.cbor` passes.
4. If you touch `tools/`, run `cargo fmt` and `cargo clippy -- -D warnings`.
5. Open the PR with a clear description of the change.

The CI workflow (`.github/workflows/validation.yml`) runs `cargo fmt --check`,
`cargo clippy -- -D warnings`, the validator on every example, and a
YAML → CBOR round-trip test. PRs must pass.

## What a good contribution looks like

| Type | Example |
|---|---|
| **Spec clarification** | A footnote explaining ambiguous wording, a precise example for an edge case |
| **New block type** | A complete entry: `t` code, required keys, example, encoding rule, tests in `tools/` |
| **New example** | A real-world `index.cbor` generated with `cbor-web-gen` and validating cleanly |
| **Tooling improvement** | Better error messages, additional validation rules, alternative implementations |
| **Implementation in another language** | Linked from the README's "Building your own implementation" table |

## Development setup

You need [Rust stable](https://rustup.rs/) (1.74+). No other runtime
dependency is required.

```sh
git clone https://github.com/YOUR-FORK/cbor-web-read.git
cd cbor-web-read
cargo build --release --manifest-path tools/Cargo.toml
./tools/target/release/cbor-web-validate examples/*.cbor
```

To generate a fresh example from scratch:

```sh
./tools/target/release/cbor-web-gen tools/examples/site.yaml -o /tmp/site.cbor
./tools/target/release/cbor-web-validate /tmp/site.cbor
```

## Spec changes

Spec changes go through one of two paths:

- **Clarifications and non-breaking additions** (typos, examples, new
  optional fields, new block types) — open a PR directly.
- **Breaking changes** (modifying the wire format of existing keys, removing
  block types, changing required fields) — open a discussion first under
  `Ideas`, gather feedback, and only then open a PR. Versioned bumps land in
  a new `CBOR-WEB-SPEC-v4.0.md` rather than overwriting v3.0.

## Questions

- General questions: [Discussions](../../discussions)
- Spec ambiguity: file an issue tagged `spec`
- Show your implementation: post in `Show and tell`

---

By contributing, you agree your work is released under CC0 1.0 (public
domain). No CLA, no attribution requirement.
