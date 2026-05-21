# CBOR-Web — A Binary Read Protocol for Agents

[![License](https://img.shields.io/badge/license-CC0-lightgrey.svg)](LICENSE)
[![Built on](https://img.shields.io/badge/built%20on-RFC%208949-orange)](https://www.rfc-editor.org/rfc/rfc8949.html)
[![Reference impl](https://img.shields.io/badge/reference%20impl-Rust-dea584)](tools/)

> **One file. One request. The whole site as structured content.**

CBOR-Web defines a single binary file — `index.cbor` — placed at the root of
a domain, that contains the full structured content of the site (pages,
headings, paragraphs, lists, tables, links). It is meant to be served
alongside `index.html`, not to replace it: humans get HTML, machines get CBOR.

The protocol is built on [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html)
(Concise Binary Object Representation), and released into the public domain
under **CC0**. Anyone may implement it, fork it, or build commercial services
on top of it without permission.

---

## Why a binary protocol next to HTML?

The web is optimised for browsers: layout, fonts, ads, tracking pixels,
analytics. A typical page in 2025 weighs ~2.86 MB and triggers ~86 HTTP
requests to display a few paragraphs of useful text. For autonomous agents —
crawlers, AI assistants, scrapers — this means:

- **Heavy network traffic** for content that's mostly decoration.
- **Brittle parsing**: DOM extraction depends on CSS selectors that break.
- **No typing**: a price `"29.90"` and the string `"29.90"` are indistinguishable.

`index.cbor` strips that down to **structured content**: titles, paragraphs,
lists, tables, calls to action, and a few metadata blocks. No JavaScript, no
CSS, no DOM — just a self-described CBOR document that decodes in one pass.

### What CBOR-Web actually optimises

| Concern | Improvement |
|---|---|
| Bytes on the wire | Typically ~10–50× lighter than the equivalent HTML page (no markup, no styling, no scripts) |
| Requests per site | 1 (vs 50–100 for an HTML page with assets) |
| Parser determinism | Strict typing (RFC 8949), no DOM, no CSS-selector fragility |
| Crawler load on origin | One static file, easy to CDN and cache |
| Energy & carbon | Less data transferred, less compute on both ends |

### What CBOR-Web does *not* optimise

> ⚠️ A previous draft of this README claimed that CBOR-Web reduces LLM token
> costs by ~40%. That claim was wrong and has been removed.
>
> LLMs do not consume CBOR bytes — they consume the *decoded text* of titles,
> paragraphs and tables. The token count of a prompt depends on the semantic
> content, not on the wire encoding. CBOR-Web saves **bytes**, not tokens.
> The honest gain is bandwidth, parsing speed, determinism and crawler-load,
> not LLM inference cost.

---

## Structure

`index.cbor` is a self-described CBOR map with seven top-level keys:

```
0  type        "cbor-web"
1  version     3
2  site        { domain, name, description, languages, contact, geo }
3  security    { default_access, public_key? }                (optional)
4  navigation  { main: [...], footer: [...], hierarchy: ... } (optional)
5  pages       [ { path, title, lang, access, content: [...] } ]
6  meta        { generated_at, generator, total_pages, signature? }
```

Each page contains an ordered array of typed blocks:

```
{"t": "h",     "l": 1, "v": "Welcome"}        # heading, levels 1-6
{"t": "p",     "v": "…"}                       # paragraph
{"t": "ul",    "v": ["…", "…"]}                # bullet list
{"t": "ol",    "v": ["…", "…"]}                # numbered list
{"t": "table", "headers": [...], "rows": [...]}
{"t": "cta",   "v": "Read more", "href": "/x"} # call to action
{"t": "q",     "v": "…", "attr": "Source"}     # quote
{"t": "img",   "src": "…", "alt": "…"}         # image reference
{"t": "code",  "v": "…", "lang": "rust"}       # code block
{"t": "note",  "v": "…", "level": "warn"}      # editorial note
{"t": "dl",    "v": [{"term": "…", "def": "…"}]} # definitions
{"t": "embed", "src": "…", "description": "…"} # embedded content
{"t": "sep"}                                    # separator
```

Full structure and encoding rules: [CBOR-WEB-SPEC-v3.0.md](CBOR-WEB-SPEC-v3.0.md).

---

## Reference implementation

A Rust implementation lives in [`tools/`](tools/). It provides four binaries:

| Binary | Purpose |
|---|---|
| `cbor-web-validate` | Check that a file conforms to the spec (tag 55799, required keys, types, deterministic encoding) |
| `cbor-web-decode` | Pretty-print a `.cbor` file as JSON (full or truncated preview) |
| `cbor-web-gen` | Build a `.cbor` from a YAML source file |
| `cbor-web-migrate` | Convert legacy 4-keys files (pages under key 3) to the v3.0 structure |

Build:

```sh
cargo build --release --manifest-path tools/Cargo.toml
```

Quick check:

```sh
./tools/target/release/cbor-web-validate examples/pacific-planet.cbor
```

See [`tools/README.md`](tools/README.md) for details and the YAML source
format.

### Building your own implementation

Any RFC 8949 CBOR encoder/decoder can read or write CBOR-Web files. Known
compatible libraries:

| Language | Library |
|---|---|
| Rust | [`ciborium`](https://crates.io/crates/ciborium) (used by this repo) |
| Go | [`fxamacker/cbor`](https://github.com/fxamacker/cbor) |
| C | [`libcbor`](https://github.com/PJK/libcbor) |
| Java | [`jackson-dataformat-cbor`](https://github.com/FasterXML/jackson-dataformats-binary) |

These are **CBOR codecs**, not CBOR-Web libraries. They give you the
encoding/decoding primitives; the protocol-specific structure (the 7 keys,
the page schema, the block types) is up to you to implement on top.

---

## Examples

[`examples/`](examples/) contains real-world `index.cbor` files generated
from production sites:

```
deltopide-com.cbor            123 KB   Deltopide International (16 pages)
deltopide-es.cbor              95 KB   Deltopide España (12 pages)
deltopide-fr.cbor              68 KB   Deltopide FR (9 pages)
eloiseplot-dieteticienne.cbor  18 KB   Dietitian site (6 pages)
example-all-blocks.cbor         3 KB   Every block type, minimal site
laforetnousregale.cbor        709 KB   Edible-forest catalogue (55 pages)
pacific-planet.cbor            16 KB   Agroforestry programmes (6 pages)
verdetao.cbor                  30 KB   Mushroom shop (15 pages)
```

`example-readable.json` is a human-readable JSON preview generated by
`cbor-web-decode --preview`. The `_truncated` markers exist only for the
preview — the actual `.cbor` files contain every block.

---

## CBOR-Web vs `llms.txt`

[`llms.txt`](https://llmstxt.org/) is a Markdown summary of a site's content,
intended for LLMs. It serves a different need:

| Aspect | `llms.txt` | `index.cbor` |
|---|---|---|
| Format | Markdown (text) | CBOR (binary) |
| Audience | LLMs reading a prompt | Crawlers, agents, parsers |
| Granularity | Curated digest (the author chooses what's important) | Full structured content (every page, every block) |
| Typing | Implicit | Strict types via RFC 8949 |
| Size | Small (~KB) | Compact but larger (full content) |
| Adoption | Growing rapidly (early 2026) | Early stage |
| Parsing | LLM reads it as text | Binary one-shot decode |

The two are **complementary**, not competitive. A site can serve both: an
`llms.txt` for narrative LLM consumption, and an `index.cbor` for machine
crawlers and data pipelines that need typed, exhaustive content. Most sites
do not need `index.cbor` — `llms.txt` is sufficient. The cases where CBOR-Web
adds value:

- Catalogues and listings (tables, structured products)
- Sites with many pages where a digest is too lossy
- Crawlers that need deterministic parsing for indexing or feature extraction
- Constrained clients (IoT, embedded) that prefer binary

If `llms.txt` covers your need, use `llms.txt`.

---

## Positioning

```
Format         Audience                          Status
─────────────────────────────────────────────────────────────────────
HTML           Humans (browsers)                 Universal
JSON / REST    APIs                              Universal
llms.txt       LLM context (Markdown summary)    Growing
robots.txt     Crawlers (rules)                  Universal
sitemap.xml    Crawlers (URL list)               Universal
index.cbor     Agents (full binary content)      Early stage
```

CBOR-Web is one of several formats designed for non-human consumers. None of
them replaces HTML; each one optimises for a different reader.

---

## Roadmap

| Phase | Status |
|---|---|
| **Foundation** — spec, examples, Rust reference implementation | Done |
| **Ecosystem** — CMS plugins, Go/JS implementations, validator CI | In progress |
| **Standardisation** — IETF draft, browser/crawler integration | Not started |

Contributions to any of these are welcome — see
[CONTRIBUTING.md](CONTRIBUTING.md).

---

## Languages

Short overview in three languages. The full specification is currently in
English only; translations are welcome via pull request.

### Français

`index.cbor` est un fichier binaire placé à la racine d'un site, à côté
d'`index.html`. Il contient le contenu structuré complet du site (titres,
paragraphes, listes, tableaux, liens) dans le format binaire CBOR (RFC 8949).
Les humains lisent l'HTML, les machines lisent le CBOR. Un seul fichier, une
seule requête, un site entier. Licence : domaine public (CC0).

### Español

`index.cbor` es un archivo binario colocado en la raíz de un sitio, junto a
`index.html`. Contiene el contenido estructurado completo del sitio (títulos,
párrafos, listas, tablas, enlaces) en formato binario CBOR (RFC 8949). Los
humanos leen HTML, las máquinas leen CBOR. Un archivo, una solicitud, un sitio
entero. Licencia: dominio público (CC0).

---

## Related projects

- [`cbor-web.com`](https://cbor-web.com) — one possible hosted service for
  generating and signing `index.cbor` files. See
  [CBOR-WEB-MONETIZATION.md](CBOR-WEB-MONETIZATION.md) for the model it
  implements. The service is independent of this spec; alternative
  generators (self-hosted, OSS, vendor-specific) are equally valid.
- [`llms.txt`](https://llmstxt.org/) — Markdown-based site summary for LLMs.
  Complementary to CBOR-Web (see above).

---

## License

This repository — specification, examples, and reference implementation — is
released under [CC0 1.0 Universal](LICENSE) (public domain).

The read protocol belongs to everyone.

---

## Contact

- Bugs and spec issues: [open an issue](../../issues)
- Discussion: [open a discussion](../../discussions)
- Security: see [`.github/SECURITY.md`](.github/SECURITY.md)
