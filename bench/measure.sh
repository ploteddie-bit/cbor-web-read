#!/usr/bin/env bash
# Measure HTML vs CBOR byte ratio for one or more domains that serve both
# /index.html and /index.cbor. Output is appended to results.tsv as
# tab-separated columns: timestamp, domain, html_bytes, html_gz_bytes,
# cbor_bytes, pages, ratio_raw, ratio_gz.
#
# Usage: ./measure.sh deltopide.com pacific-planet.com

set -euo pipefail

OUT="$(dirname "$0")/results.tsv"
TS="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

if [ ! -s "$OUT" ]; then
  printf 'timestamp\tdomain\thtml_bytes\thtml_gz_bytes\tcbor_bytes\tpages\tratio_raw\tratio_gz\n' > "$OUT"
fi

# Locate cbor-web-decode; build it if it isn't already present.
DECODE="$(dirname "$0")/../tools/target/release/cbor-web-decode"
if [ ! -x "$DECODE" ]; then
  echo "Building cbor-web-decode (release)..." >&2
  cargo build --release --manifest-path "$(dirname "$0")/../tools/Cargo.toml" --bin cbor-web-decode >&2
fi

bytes_of() {
  curl -sSL -A "cbor-web-bench/1.0" "$1" -o "$2"
  wc -c < "$2"
}

bytes_of_gz() {
  curl -sSL -A "cbor-web-bench/1.0" -H "Accept-Encoding: gzip" --compressed "$1" -o /dev/null -w '%{size_download}\n'
}

for DOMAIN in "$@"; do
  HTML_TMP="$(mktemp)"
  CBOR_TMP="$(mktemp)"
  HTML_BYTES="$(bytes_of "https://$DOMAIN/" "$HTML_TMP")"
  HTML_GZ_BYTES="$(bytes_of_gz "https://$DOMAIN/")"
  CBOR_BYTES="$(bytes_of "https://$DOMAIN/index.cbor" "$CBOR_TMP")"
  if [ "$CBOR_BYTES" -lt 10 ]; then
    echo "$DOMAIN: no /index.cbor served (size $CBOR_BYTES)" >&2
    rm -f "$HTML_TMP" "$CBOR_TMP"
    continue
  fi
  PAGES="$("$DECODE" "$CBOR_TMP" 2>/dev/null | jq -r '."5" | length' 2>/dev/null || echo "?")"
  RATIO_RAW="$(awk "BEGIN { printf \"%.1f\", $HTML_BYTES / $CBOR_BYTES }")"
  RATIO_GZ="$(awk "BEGIN { printf \"%.1f\", $HTML_GZ_BYTES / $CBOR_BYTES }")"
  printf '%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' \
    "$TS" "$DOMAIN" "$HTML_BYTES" "$HTML_GZ_BYTES" "$CBOR_BYTES" "$PAGES" "$RATIO_RAW" "$RATIO_GZ" \
    | tee -a "$OUT"
  rm -f "$HTML_TMP" "$CBOR_TMP"
done
