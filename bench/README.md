# Benchmarks — CBOR-Web vs HTML

This directory hosts the methodology and a runnable script for measuring
the byte-size ratio between the `index.cbor` file a site serves and the
HTML version of the same content.

The README claims a "~10–50× lighter than HTML" range — this is the
folder where that claim is backed up (or refuted) with real numbers.

## Methodology

For each domain that has both `index.html` and `index.cbor` at the root,
we record:

- `html_bytes` — Content-Length of the home page over HTTPS, no `Accept-Encoding`
- `html_bytes_gzip` — same with `Accept-Encoding: gzip`
- `cbor_bytes` — Content-Length of `/index.cbor`
- `pages` — number of pages in the CBOR file (from `cbor-web-decode | jq '."5" | length'`)
- `ratio` — `html_bytes / cbor_bytes`

The HTML measurement is the **uncompressed** size, since CBOR-Web files
are usually served without gzip (binary content already dense). We also
report the gzipped size for fairness — gzip narrows the gap.

## Running the bench

```sh
./measure.sh deltopide.com pacific-planet.com laforetnousregale.fr
```

Output is appended to `results.tsv` with one row per domain. Re-run
periodically to track drift.

## Caveats

- Single-page comparison: the HTML home page often doesn't contain every
  page that's bundled in `index.cbor`. A fairer "site total" would crawl
  all linked pages and sum HTML bytes — that's the next step, but it
  conflates with crawl politeness. For now we measure home page vs full
  site, which gives an **upper bound** on the ratio (CBOR-Web is favoured).
- Gzip vs no-gzip is the biggest swing factor; report both.
- DNS / TLS handshake time and request count are not measured here. See
  `tools/` for a future `cbor-web-bench --network` mode if needed.

## Current snapshot

| Domain | HTML (B) | HTML gz (B) | CBOR (B) | Pages | Ratio (uncomp.) | Ratio (gz) |
|---|---|---|---|---|---|---|
| _to be measured_ | | | | | | |

Re-run `./measure.sh` to fill the table. Do **not** paste rough estimates
in the meantime — the point of this folder is to keep the numbers honest.
