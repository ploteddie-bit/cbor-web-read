# CBOR-Web Specification v3.0

**Machine-Readable Binary Web Content for Autonomous Agents**

```
Status:  Draft
Date:    2026-05-21
License: CC0 1.0 (Public Domain)
Format:  CBOR (RFC 8949)
```

This document defines the **CBOR-Web read protocol**: a binary representation
of a website's public content, served as a single file at the root of the
domain, designed to be parsed by autonomous agents without HTML, CSS or
JavaScript.

The protocol is licensed under CC0 — any party may implement, fork, extend,
or build commercial services on top of it without permission.

---

## 1. One file, one site

A site serves `index.html` to browsers. It MAY also serve **`index.cbor`** to
machines:

```
example.com/index.html   →  home page for humans
example.com/index.cbor   →  full site as a single binary file
```

`index.cbor` is a self-described CBOR document (RFC 8949) containing:

- Site identity (domain, name, languages, contact, geo)
- Optional security metadata (access tiers, public key)
- Optional navigation (menus, hierarchy)
- All public pages and their structured content
- Generation metadata (timestamp, generator, optional signature)

An agent issues `GET /index.cbor` and receives the entire site in one
response.

---

## 2. Discovery

An agent discovers CBOR-Web by requesting `index.cbor` at the domain root:

```
GET /index.cbor HTTP/1.1
Host: example.com
Accept: application/cbor
```

| Response | Meaning |
|---|---|
| `200 OK` + `Content-Type: application/cbor` | Supported. Body is the site. |
| `404 Not Found` | Not supported. Fall back to HTML. |

The first three bytes of the body MUST be `D9 D9 F7` (tag 55799,
self-described CBOR).

### Optional discovery aids

| Method | Use |
|---|---|
| DNS TXT `_cbor-web.example.com` | Discovery at scale without an HTTP request |
| HTTP `Link: <…>; rel="alternate"; type="application/cbor"` | Discovery during HTML browsing |
| `robots.txt` directive `CBOR-Web: /index.cbor` | Compatible with existing crawlers |

---

## 3. Structure of `index.cbor`

```cbor-diag
55799({
  0: "cbor-web",
  1: 3,
  2: {                                  / site metadata /
    "domain": "example.com",
    "name": "Example",
    "description": "…",
    "languages": ["en", "fr"],
    "default_language": "en",
    "contact": {"email": "…", "phone": "…"},
    "geo": {"country": "US", "region": "California"}
  },
  3: {                                  / security (optional) /
    "default_access": "T2",
    "public_key": h'MCowBQYDK2VwAyEA…'
  },
  4: {                                  / navigation (optional) /
    "main": ["/", "/catalogue", "/about"],
    "footer": ["/legal", "/contact"]
  },
  5: [                                  / pages /
    {
      "path": "/",
      "title": "Welcome",
      "lang": "en",
      "access": "T2",
      "updated": 1(1748000000),
      "content": [
        {"t": "h", "l": 1, "v": "Welcome"},
        {"t": "p", "v": "…"}
      ]
    }
  ],
  6: {                                  / meta /
    "generated_at": 1(1748000000),
    "generator": "cbor-web-gen/0.1",
    "total_pages": 1,
    "signature": h'…'                   / optional /
  }
})
```

### 3.1 Top-level keys

| Key | Name | Type | Required | Description |
|---|---|---|---|---|
| 0 | type | text | YES | `"cbor-web"` |
| 1 | version | uint | YES | `3` for this version |
| 2 | site | map | YES | Domain, name, languages, contact, geo |
| 3 | security | map | NO | Default access tier and public key |
| 4 | navigation | map | NO | Menus and hierarchy |
| 5 | pages | array | YES | Pages with their content |
| 6 | meta | map | NO | Generation timestamp, generator, optional signature |

Agents MUST ignore unknown keys (forward compatibility).

### 3.2 Page structure (key 5 elements)

| Field | Type | Required | Description |
|---|---|---|---|
| `path` | text | YES | URL path (`"/"`, `"/catalogue/item"`) |
| `title` | text | YES | Page title |
| `lang` | text | YES | BCP 47 language tag |
| `access` | text | YES | `"T0"`, `"T1"`, or `"T2"` (see §4) |
| `content` | array | YES | Ordered array of content blocks (§5) |
| `description` | text | NO | Short description |
| `updated` | tag 1 | NO | Last-modified timestamp (epoch seconds) |
| `hash` | bstr | NO | SHA-256 of serialised content (32 bytes) |
| `alternates` | map | NO | Language alternates `{"fr": "/fr/page"}` |
| `structured_data` | map | NO | Schema.org typed object |

---

## 4. Access tiers

A page declares an access tier. The read protocol defines three values:

| Tier | Meaning | Authentication |
|---|---|---|
| `T2` | Open access (default) | None |
| `T1` | Authenticated access | Implementation-defined |
| `T0` | Restricted access | Implementation-defined |

For `T1` and `T0` pages, the publisher MAY omit the `content` array or replace
it with an opaque byte string. Authentication mechanisms are intentionally
**out of scope** for the read protocol — see
[CBOR-WEB-MONETIZATION.md](CBOR-WEB-MONETIZATION.md) for one possible
authentication and monetisation model.

The default tier for any page that does not declare `access` is `T2`.

---

## 5. Content blocks

Each block is a CBOR map. The `t` (type) field is required.

### 5.1 Editorial blocks

| `t` | Type | Keys | Example |
|---|---|---|---|
| `h` | Heading | `l` (1-6), `v` | `{"l": 1, "t": "h", "v": "Title"}` |
| `p` | Paragraph | `v` | `{"t": "p", "v": "…"}` |
| `ul` | Bullet list | `v` (array) | `{"t": "ul", "v": ["A", "B"]}` |
| `ol` | Numbered list | `v` (array) | `{"t": "ol", "v": ["1", "2"]}` |
| `q` | Quote | `v`, `attr` | `{"t": "q", "v": "…", "attr": "Source"}` |
| `code` | Code block | `v`, `lang` | `{"t": "code", "v": "…", "lang": "rust"}` |
| `table` | Data table | `headers`, `rows` | `{"t": "table", "headers": ["A"], "rows": [["1"]]}` |
| `dl` | Definitions | `v` (array of `{term, def}` maps) | … |
| `note` | Note | `v`, `level` | `{"t": "note", "v": "…", "level": "warn"}` |
| `sep` | Separator | — | `{"t": "sep"}` |

### 5.2 Action and media blocks

| `t` | Type | Keys |
|---|---|---|
| `cta` | Call to action | `v`, `href` |
| `img` | Image | `src`, `alt`, optional `caption` |
| `embed` | Embedded content | `src`, `description` |

An agent encountering an unknown `t` value MUST skip the block silently. This
permits forward-compatible extensions.

---

## 6. Encoding rules

### 6.1 Required

| Rule | Requirement |
|---|---|
| Deterministic encoding | RFC 8949 §4.2 — sorted keys, minimal integers |
| Self-described tag | `D9 D9 F7` (tag 55799) at the start of the file |
| Text encoding | UTF-8 (CBOR major type 3), NFC normalised, LF line breaks |
| Hashes | SHA-256 as 32-byte byte string (CBOR major type 2) |
| Timestamps | Tag 1 + integer (epoch seconds, UTC) |
| Definite lengths | No indefinite-length arrays or maps |
| Map key order | Shorter encoded keys first, then bytewise |
| Forward compatibility | Unknown keys MUST be ignored, not rejected |

### 6.2 Size limits

| File | Max |
|---|---|
| `index.cbor` (≤500 pages) | 5 MB |
| `index.cbor` (>500 pages) | Paginated (§7) |
| Per-page content | 1 MB |

---

## 7. Large sites (>500 pages)

For sites exceeding 500 pages, `index.cbor` contains the manifest plus the
first 500 pages and a `"next"` pointer in the meta map:

```cbor-diag
6: {
  "generated_at": 1(1748000000),
  "total_pages": 12000,
  "next": "/cbor-web/pages-501-1000.cbor"
}
```

The agent follows `next` to load subsequent batches. The first file always
holds site metadata (key 2), navigation (key 4), and security (key 3).

---

## 8. Identity and signature

A publisher MAY prove ownership by publishing a DNS TXT record:

```
_cbor-web.example.com. 3600 IN TXT "v=3; pk=MCowBQYDK2VwAyEA…"
```

| Field | Description |
|---|---|
| `v` | Protocol version |
| `pk` | Public key in base64url (Ed25519 or P-256) |

If a public key is published, the publisher SHOULD sign `index.cbor` with the
matching private key and place the signature in `6.signature`. Verifiers MAY
compare the signed bytes against the DNS public key to authenticate the
document.

DNS-based identity is optional. A site without DNS TXT or signature is still
a valid CBOR-Web document — it simply offers no authentication guarantees.

---

## 9. Verification

A CBOR-Web verifier crawls declared sites and checks:

1. The first three bytes equal `D9 D9 F7`
2. The CBOR document decodes without error
3. Required keys (0, 1, 2, 5) are present
4. Encoding is deterministic per RFC 8949 §4.2
5. If a signature is present, it verifies against the DNS TXT public key
6. Each page declares `path`, `title`, `lang`, `content`

A reference Rust implementation lives in [`tools/`](tools/) of this
repository.

---

## 10. Positioning

| Standard | Role | Relationship |
|---|---|---|
| `index.html` | Home page for humans | Parallel — `index.cbor` is the machine equivalent |
| `sitemap.xml` | List of URLs | Superseded by `index.cbor`'s pages array |
| `robots.txt` | Crawl rules | Complementary |
| `llms.txt` | Markdown summary for LLMs | Complementary — `llms.txt` is a curated digest, `index.cbor` is the full structured content |
| **`index.cbor`** | **Full site as binary structured content** | **This document** |

---

## 11. References

- **[RFC 8949]** CBOR — Concise Binary Object Representation
- **[RFC 8610]** CDDL — Concise Data Definition Language
- **[RFC 9052]** COSE — CBOR Object Signing and Encryption
- **[RFC 8615]** Well-Known URIs

---

## Authors and acknowledgements

Authored by Eddie Plot (2026). The early drafts were developed in dialogue
with the Claude assistant from Anthropic, which contributed to the
formalisation of the structure and the encoding rules.

Implementations, services and ecosystem tooling are tracked in
[`CBOR-WEB-MONETIZATION.md`](CBOR-WEB-MONETIZATION.md) (optional commercial
extensions) and in the `Related projects` section of the [README](README.md).

## License

This specification is released into the **public domain** under
[CC0 1.0 Universal](https://creativecommons.org/publicdomain/zero/1.0/).
You may read, implement, copy, modify, distribute, and build commercial
services on top of it without permission or attribution.

The reading protocol belongs to everyone.
