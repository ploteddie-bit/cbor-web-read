# CBOR-Web — Monetisation Extension (Optional)

```
Status:   Draft
Date:     2026-05-21
License:  CC0 1.0 (Public Domain)
Scope:    Optional commercial extension to the CBOR-Web read protocol
```

This document is **not** part of the read protocol. It describes one possible
commercial model that publishers and tool vendors MAY adopt. Implementations
of CBOR-Web are not required to read, understand or support anything in this
file. Agents that ignore this extension still consume `index.cbor` files
correctly.

The read protocol itself is defined in
[CBOR-WEB-SPEC-v3.0.md](CBOR-WEB-SPEC-v3.0.md). Everything below builds on
top of it.

---

## 1. Why an optional extension

The base protocol intentionally leaves three things unspecified:

1. How publishers generate their `index.cbor` (file, CMS plugin, hosted
   service, …).
2. How agents authenticate when requesting `T1` or `T0` content.
3. How publishers monetise the visibility of their content to AI agents.

This document proposes one coherent answer to all three. Any other answer is
equally valid as long as it produces files that conform to the base spec.

---

## 2. Access tiers — detailed semantics

The base spec defines three tier labels (`T0`, `T1`, `T2`). This extension
proposes one possible interpretation:

| Tier | Audience | Authentication |
|---|---|---|
| `T0` | Institutional consumers (governments, verified entities) | eIDAS 2.0 / X.509 EV certificate |
| `T1` | Authenticated agents | API key, OAuth 2.0 client credentials, or on-chain token |
| `T2` | Open access | None |

When an unauthenticated agent fetches `index.cbor`:

- `T2` pages are served in clear.
- `T1` and `T0` pages have their `content` field either omitted or replaced
  with an opaque byte string. Path and title remain visible so the agent
  knows what exists.

After authenticating, the agent receives a copy of `index.cbor` with the
restricted pages' `content` fields filled in.

A site that ignores tiers entirely (everything `T2`) is fully conformant to
the read protocol.

---

## 3. Generation service (one possible implementation)

`cbor-web.com` is one hosted implementation of an `index.cbor` generator. The
flow below describes that specific implementation; any equivalent service or
self-hosted tool may follow a similar pattern.

```
1. Publisher creates an account → receives an API token (valid 365 days).
2. Publisher publishes a DNS TXT record (_cbor-web.example.com) with their
   public key.
3. Publisher calls POST /generate with the page list and tier assignments.
4. Service crawls, converts to CBOR, signs with publisher key.
5. Download window opens for 48–72 hours.
6. Publisher downloads index.cbor and places it at the domain root.
7. Window closes; the publisher can call /regenerate to reopen one.
```

### 3.1 API surface

```
POST https://api.cbor-web.com/register
POST https://api.cbor-web.com/generate
GET  https://api.cbor-web.com/download/{job_id}
POST https://api.cbor-web.com/regenerate
```

See the `cbor-web.com` service documentation for current request and response
schemas. This document records the model, not the on-the-wire details, which
may evolve outside this repository.

### 3.2 Self-hosting

Publishers who prefer not to use a hosted service can produce `index.cbor`
files with the reference implementation in [`tools/`](tools/) of this
repository, or with any CBOR encoder that respects RFC 8949 §4.2 deterministic
encoding rules.

---

## 4. Visibility signals

This extension adds three optional per-page fields that crawlers MAY respect:

| Field | Range | Effect |
|---|---|---|
| `priority` | 0.0 – 1.0 (default 0.5) | Crawl order. Higher first. |
| `freshness` | `realtime` / `hourly` / `daily` / `weekly` / `monthly` | Recrawl cadence. |
| `boost` | `{"until": tag1(timestamp), "label": "…"}` | Temporary `priority = 1.0` until the deadline. |

Crawlers that participate in a publisher-pays model SHOULD honour these
signals. Crawlers that do not participate (independent open-source crawlers,
search engines, archives) MAY ignore them entirely — the base read protocol
remains unaffected.

---

## 5. ERC-20 access token (optional)

This extension proposes a token called **CBORW** as one possible authentication
material for `T1` access:

| Property | Value |
|---|---|
| Standard | ERC-20 |
| Chain | (chain identifier, to be specified by issuer) |
| Use | Holding ≥ N tokens grants T1 read access to participating sites |

When a publisher declares `auth.mechanisms` containing `erc20`, an agent can
authenticate by signing a challenge with an address that holds the required
balance. This is **one** authentication mechanism — API key and OAuth M2M
are equally valid.

Sites that do not opt into this mechanism (the overwhelming majority,
expected) are not affected and remain interoperable with all agents.

---

## 6. Two-tier economic model

| Tier | Cost | What the publisher gets |
|---|---|---|
| Free | 0 | `index.cbor` with `priority: 0.5`, `freshness: monthly`, no boost. Standard indexing. |
| Publisher (annual token) | Paid | Configurable `priority`, `freshness` up to `realtime`, temporary `boost`, priority recrawl. |

### Analogy

```
Google      :  free indexing  +  Ads (pay to surface first to humans)
CBOR-Web    :  free index.cbor +  visibility options (pay to surface first to agents)
```

The substance of the offer is *machine readability prioritisation*, not
advertising.

---

## 7. Why this is an extension and not part of the spec

A read protocol stays open by separating wire format from business model. By
moving monetisation, blockchain authentication, and hosted-service flows out
of the base spec:

- Independent implementers can support `index.cbor` without engaging with
  any service or token.
- Standards bodies (IETF, W3C) can evaluate the base protocol on its
  technical merits.
- The reading protocol remains in the public domain (CC0).
- Commercial vendors compete on tooling quality, not on protocol control.

If a future revision of the base spec absorbs any part of this extension,
that decision will be made by the broader community via the relevant
standards process — not by a single vendor.

---

## License

This extension is released under [CC0 1.0](https://creativecommons.org/publicdomain/zero/1.0/).
