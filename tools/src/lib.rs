//! Reference data model and helpers for the CBOR-Web read protocol.
//!
//! Implements the 7-keys structure defined in CBOR-WEB-SPEC-v3.0.md.
//! No I/O here — binaries in `src/bin/` are responsible for reading/writing files.

use anyhow::{anyhow, bail, Context, Result};
use ciborium::value::{Integer, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const SELF_DESCRIBED_TAG: u64 = 55799;
pub const PROTOCOL_TYPE: &str = "cbor-web";
pub const PROTOCOL_VERSION: u64 = 3;

pub mod keys {
    pub const TYPE: i128 = 0;
    pub const VERSION: i128 = 1;
    pub const SITE: i128 = 2;
    pub const SECURITY: i128 = 3;
    pub const NAVIGATION: i128 = 4;
    pub const PAGES: i128 = 5;
    pub const META: i128 = 6;
}

pub const VALID_BLOCK_TYPES: &[&str] = &[
    "h", "p", "ul", "ol", "table", "cta", "q", "img", "code", "dl", "note", "sep", "embed",
];

pub const VALID_ACCESS_TIERS: &[&str] = &["T0", "T1", "T2"];

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteDoc {
    pub site: SiteMeta,
    #[serde(default)]
    pub security: SecurityMeta,
    #[serde(default)]
    pub navigation: NavigationMeta,
    pub pages: Vec<Page>,
    #[serde(default)]
    pub meta: MetaInfo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SiteMeta {
    pub domain: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub languages: Vec<String>,
    #[serde(default)]
    pub default_language: Option<String>,
    #[serde(default)]
    pub contact: BTreeMap<String, String>,
    #[serde(default)]
    pub geo: BTreeMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityMeta {
    #[serde(default = "default_access_t2")]
    pub default_access: String,
    #[serde(default)]
    pub public_key: Option<String>,
}

impl Default for SecurityMeta {
    fn default() -> Self {
        Self {
            default_access: default_access_t2(),
            public_key: None,
        }
    }
}

fn default_access_t2() -> String {
    "T2".to_string()
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct NavigationMeta {
    #[serde(default)]
    pub main: Vec<String>,
    #[serde(default)]
    pub footer: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page {
    pub path: String,
    pub title: String,
    pub lang: String,
    #[serde(default = "default_access_t2")]
    pub access: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub updated: Option<i64>,
    pub content: Vec<Block>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum Block {
    Map(BTreeMap<String, serde_yaml::Value>),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MetaInfo {
    #[serde(default)]
    pub generated_at: Option<i64>,
    #[serde(default)]
    pub generator: Option<String>,
}

/// Validation report — collects every issue rather than failing on the first.
#[derive(Debug, Default)]
pub struct Report {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl Report {
    pub fn err(&mut self, msg: impl Into<String>) {
        self.errors.push(msg.into());
    }
    pub fn warn(&mut self, msg: impl Into<String>) {
        self.warnings.push(msg.into());
    }
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }
}

/// Validate raw bytes of a CBOR-Web index file. Returns a report.
pub fn validate_bytes(bytes: &[u8]) -> Result<Report> {
    let mut r = Report::default();

    if bytes.len() < 3 {
        r.err("file too short to contain CBOR self-described tag");
        return Ok(r);
    }
    if bytes[0..3] != [0xd9, 0xd9, 0xf7] {
        r.err(format!(
            "missing self-described CBOR tag 55799; first 3 bytes are {:02x} {:02x} {:02x}, expected d9 d9 f7",
            bytes[0], bytes[1], bytes[2]
        ));
    }

    let value: Value = ciborium::de::from_reader(bytes).context("failed to decode CBOR")?;
    let inner = match &value {
        Value::Tag(tag, inner) if *tag == SELF_DESCRIBED_TAG => inner.as_ref(),
        Value::Tag(tag, _) => {
            r.err(format!("expected tag {SELF_DESCRIBED_TAG}, got tag {tag}"));
            &value
        }
        other => other,
    };

    let map = match inner {
        Value::Map(m) => m,
        _ => {
            r.err("root value is not a CBOR map");
            return Ok(r);
        }
    };

    check_keys(map, &mut r);
    check_total_pages(map, &mut r);
    check_page_hashes(map, &mut r);
    check_canonical(bytes, &value, &mut r);
    Ok(r)
}

/// Re-encode a CBOR value into bytes that strictly follow RFC 8949 §4.2:
/// definite-length items, deterministic integer encoding, and map keys sorted
/// by length-first then bytewise. ciborium gives us most of this for free; we
/// enforce the map-key sort manually because ciborium emits keys in insertion
/// order.
pub fn canonical_bytes(v: &Value) -> Result<Vec<u8>> {
    let canon = canonicalise(v);
    let mut out = Vec::new();
    ciborium::ser::into_writer(&canon, &mut out).map_err(|e| anyhow!("encode error: {e}"))?;
    Ok(out)
}

fn canonicalise(v: &Value) -> Value {
    match v {
        Value::Map(entries) => {
            let mut canon_entries: Vec<(Vec<u8>, Value, Value)> = entries
                .iter()
                .map(|(k, val)| {
                    let mut k_bytes = Vec::new();
                    let _ = ciborium::ser::into_writer(k, &mut k_bytes);
                    (k_bytes, canonicalise(k), canonicalise(val))
                })
                .collect();
            canon_entries.sort_by(|a, b| match a.0.len().cmp(&b.0.len()) {
                std::cmp::Ordering::Equal => a.0.cmp(&b.0),
                other => other,
            });
            Value::Map(canon_entries.into_iter().map(|(_, k, v)| (k, v)).collect())
        }
        Value::Array(items) => Value::Array(items.iter().map(canonicalise).collect()),
        Value::Tag(t, inner) => Value::Tag(*t, Box::new(canonicalise(inner))),
        other => other.clone(),
    }
}

fn as_int(v: &Value) -> Option<i128> {
    match v {
        Value::Integer(i) => Some((*i).into()),
        _ => None,
    }
}

fn check_keys(map: &[(Value, Value)], r: &mut Report) {
    let mut seen: BTreeMap<i128, &Value> = BTreeMap::new();
    for (k, v) in map {
        match as_int(k) {
            Some(n) => {
                if seen.insert(n, v).is_some() {
                    r.err(format!("duplicate key {n}"));
                }
            }
            None => r.err(format!("non-integer top-level key: {k:?}")),
        }
    }

    for &(expected, name) in &[
        (keys::TYPE, "type"),
        (keys::VERSION, "version"),
        (keys::SITE, "site"),
        (keys::PAGES, "pages"),
    ] {
        if !seen.contains_key(&expected) {
            r.err(format!("missing required key {expected} ({name})"));
        }
    }

    if let Some(Value::Text(t)) = seen.get(&keys::TYPE) {
        if t != PROTOCOL_TYPE {
            r.err(format!(
                "key 0 (type) must be \"{PROTOCOL_TYPE}\", got \"{t}\""
            ));
        }
    } else if seen.contains_key(&keys::TYPE) {
        r.err("key 0 (type) must be a text string");
    }

    if let Some(v) = seen.get(&keys::VERSION) {
        match as_int(v) {
            Some(n) if n == PROTOCOL_VERSION as i128 => {}
            Some(n) => r.err(format!(
                "key 1 (version) must be {PROTOCOL_VERSION}, got {n}"
            )),
            None => r.err("key 1 (version) must be an integer"),
        }
    }

    if let Some(v) = seen.get(&keys::SITE) {
        check_site(v, r);
    }
    if let Some(v) = seen.get(&keys::SECURITY) {
        check_security(v, r);
    }
    if let Some(v) = seen.get(&keys::PAGES) {
        check_pages(v, r);
    }
}

fn check_site(v: &Value, r: &mut Report) {
    let Value::Map(m) = v else {
        r.err("key 2 (site) must be a map");
        return;
    };
    let mut has_domain = false;
    let mut has_name = false;
    for (k, val) in m {
        if let Value::Text(key) = k {
            if key == "domain" && matches!(val, Value::Text(_)) {
                has_domain = true;
            }
            if key == "name" && matches!(val, Value::Text(_)) {
                has_name = true;
            }
        }
    }
    if !has_domain {
        r.err("site.domain (text) is required");
    }
    if !has_name {
        r.err("site.name (text) is required");
    }
}

fn check_security(v: &Value, r: &mut Report) {
    let Value::Map(m) = v else {
        r.err("key 3 (security) must be a map");
        return;
    };
    for (k, val) in m {
        if let (Value::Text(key), Value::Text(tier)) = (k, val) {
            if key == "default_access" && !VALID_ACCESS_TIERS.contains(&tier.as_str()) {
                r.err(format!(
                    "security.default_access must be one of {VALID_ACCESS_TIERS:?}, got \"{tier}\""
                ));
            }
        }
    }
}

fn check_pages(v: &Value, r: &mut Report) {
    let Value::Array(pages) = v else {
        r.err("key 5 (pages) must be an array");
        return;
    };
    if pages.is_empty() {
        r.warn("pages array is empty");
    }
    for (i, p) in pages.iter().enumerate() {
        check_page(p, i, r);
    }
}

fn check_page(v: &Value, idx: usize, r: &mut Report) {
    let Value::Map(m) = v else {
        r.err(format!("page[{idx}] is not a map"));
        return;
    };
    let mut path = None;
    let mut has_title = false;
    let mut has_lang = false;
    let mut content: Option<&Vec<Value>> = None;
    for (k, val) in m {
        let Value::Text(key) = k else { continue };
        match key.as_str() {
            "path" => {
                if let Value::Text(p) = val {
                    path = Some(p.as_str());
                } else {
                    r.err(format!("page[{idx}].path must be text"));
                }
            }
            "title" => {
                if matches!(val, Value::Text(_)) {
                    has_title = true;
                } else {
                    r.err(format!("page[{idx}].title must be text"));
                }
            }
            "lang" => {
                if matches!(val, Value::Text(_)) {
                    has_lang = true;
                } else {
                    r.err(format!("page[{idx}].lang must be text"));
                }
            }
            "access" => {
                if let Value::Text(t) = val {
                    if !VALID_ACCESS_TIERS.contains(&t.as_str()) {
                        r.err(format!(
                            "page[{idx}].access must be one of {VALID_ACCESS_TIERS:?}, got \"{t}\""
                        ));
                    }
                }
            }
            "content" => {
                if let Value::Array(a) = val {
                    content = Some(a);
                } else {
                    r.err(format!("page[{idx}].content must be an array"));
                }
            }
            _ => {}
        }
    }
    let label = path.unwrap_or("?");
    if path.is_none() {
        r.err(format!("page[{idx}] missing path"));
    }
    if !has_title {
        r.err(format!("page[{idx}] ({label}) missing title"));
    }
    if !has_lang {
        r.err(format!("page[{idx}] ({label}) missing lang"));
    }
    if let Some(blocks) = content {
        for (j, b) in blocks.iter().enumerate() {
            check_block(b, label, j, r);
        }
    } else {
        r.err(format!("page[{idx}] ({label}) missing content"));
    }
}

/// `meta.total_pages` SHOULD equal `pages.len()`. Mismatches are usually
/// generator bugs.
fn check_total_pages(map: &[(Value, Value)], r: &mut Report) {
    let meta = map.iter().find_map(|(k, v)| {
        if as_int(k) == Some(keys::META) {
            Some(v)
        } else {
            None
        }
    });
    let pages = map.iter().find_map(|(k, v)| {
        if as_int(k) == Some(keys::PAGES) {
            Some(v)
        } else {
            None
        }
    });
    let (Some(Value::Map(meta_map)), Some(Value::Array(pages_arr))) = (meta, pages) else {
        return;
    };
    let declared = meta_map.iter().find_map(|(k, v)| match (k, v) {
        (Value::Text(t), val) if t == "total_pages" => as_int(val),
        _ => None,
    });
    if let Some(n) = declared {
        let actual = pages_arr.len() as i128;
        if n != actual {
            r.err(format!(
                "meta.total_pages ({n}) does not match pages.len() ({actual})"
            ));
        }
    }
}

/// For each page that declares a `hash`, recompute SHA-256 over the canonical
/// encoding of the page map with the `hash` field removed and compare.
fn check_page_hashes(map: &[(Value, Value)], r: &mut Report) {
    let pages = map.iter().find_map(|(k, v)| {
        if as_int(k) == Some(keys::PAGES) {
            Some(v)
        } else {
            None
        }
    });
    let Some(Value::Array(pages_arr)) = pages else {
        return;
    };
    for (idx, page) in pages_arr.iter().enumerate() {
        let Value::Map(fields) = page else { continue };
        let declared = fields.iter().find_map(|(k, v)| match (k, v) {
            (Value::Text(t), Value::Bytes(b)) if t == "hash" => Some(b.clone()),
            _ => None,
        });
        let Some(expected) = declared else { continue };
        if expected.len() != 32 {
            r.err(format!(
                "page[{idx}].hash must be 32 bytes (SHA-256), got {} bytes",
                expected.len()
            ));
            continue;
        }
        let stripped: Vec<(Value, Value)> = fields
            .iter()
            .filter(|(k, _)| !matches!(k, Value::Text(t) if t == "hash"))
            .cloned()
            .collect();
        let canon = match canonical_bytes(&Value::Map(stripped)) {
            Ok(b) => b,
            Err(_) => continue,
        };
        use sha2::Digest;
        let actual = sha2::Sha256::digest(&canon);
        if actual.as_slice() != expected.as_slice() {
            r.err(format!(
                "page[{idx}].hash does not match SHA-256 of the page (with `hash` field removed) in canonical CBOR encoding"
            ));
        }
    }
}

fn check_block(v: &Value, page_path: &str, j: usize, r: &mut Report) {
    let Value::Map(m) = v else {
        r.err(format!("page {page_path} block[{j}] not a map"));
        return;
    };
    let mut t = None;
    for (k, val) in m {
        if let (Value::Text(key), Value::Text(s)) = (k, val) {
            if key == "t" {
                t = Some(s.as_str());
            }
        }
    }
    match t {
        None => r.err(format!("page {page_path} block[{j}] missing \"t\" (type)")),
        Some(bt) if !VALID_BLOCK_TYPES.contains(&bt) => {
            r.warn(format!(
                "page {page_path} block[{j}] unknown block type \"{bt}\" (agents must ignore unknown types)"
            ));
        }
        _ => {}
    }
}

/// Check canonical encoding (RFC 8949 §4.2) by re-encoding the document
/// canonically (sorted map keys, definite lengths, minimal integers) and
/// comparing bytes. Any difference is a warning — the file is still a valid
/// CBOR-Web document, but it does not match the canonical form a verifier
/// would compute, which means signatures and `page.hash` values produced by
/// strict encoders will not match.
fn check_canonical(original: &[u8], value: &Value, r: &mut Report) {
    let re_encoded = match canonical_bytes(value) {
        Ok(b) => b,
        Err(_) => {
            r.warn("could not re-encode for canonical check");
            return;
        }
    };
    if re_encoded != original {
        r.warn(format!(
            "file is not canonically encoded per RFC 8949 §4.2 ({} bytes original, {} bytes canonical) — signatures and hashes computed by strict encoders will not match",
            original.len(),
            re_encoded.len()
        ));
    }
}

/// Encode a SiteDoc to CBOR-Web bytes with the self-described tag prepended.
pub fn encode_site(doc: &SiteDoc) -> Result<Vec<u8>> {
    let entries: Vec<(Value, Value)> = vec![
        (int(keys::TYPE), Value::Text(PROTOCOL_TYPE.to_string())),
        (int(keys::VERSION), int(PROTOCOL_VERSION as i128)),
        (int(keys::SITE), site_to_value(&doc.site)),
        (int(keys::SECURITY), security_to_value(&doc.security)),
        (
            int(keys::NAVIGATION),
            navigation_to_value(&doc.navigation, &doc.pages),
        ),
        (int(keys::PAGES), pages_to_value(&doc.pages)?),
        (int(keys::META), meta_to_value(&doc.meta, doc.pages.len())),
    ];

    let root = Value::Tag(SELF_DESCRIBED_TAG, Box::new(Value::Map(entries)));
    canonical_bytes(&root)
}

fn int(n: i128) -> Value {
    Value::Integer(Integer::try_from(n).expect("integer in range"))
}

fn text(s: impl Into<String>) -> Value {
    Value::Text(s.into())
}

fn site_to_value(s: &SiteMeta) -> Value {
    let mut m: Vec<(Value, Value)> = Vec::new();
    m.push((text("domain"), text(&s.domain)));
    m.push((text("name"), text(&s.name)));
    if let Some(d) = &s.description {
        m.push((text("description"), text(d)));
    }
    if !s.languages.is_empty() {
        m.push((
            text("languages"),
            Value::Array(s.languages.iter().cloned().map(Value::Text).collect()),
        ));
    }
    if let Some(dl) = &s.default_language {
        m.push((text("default_language"), text(dl)));
    }
    if !s.contact.is_empty() {
        m.push((
            text("contact"),
            Value::Map(s.contact.iter().map(|(k, v)| (text(k), text(v))).collect()),
        ));
    }
    if !s.geo.is_empty() {
        m.push((
            text("geo"),
            Value::Map(s.geo.iter().map(|(k, v)| (text(k), text(v))).collect()),
        ));
    }
    Value::Map(m)
}

fn security_to_value(s: &SecurityMeta) -> Value {
    let mut m: Vec<(Value, Value)> = Vec::new();
    m.push((text("default_access"), text(&s.default_access)));
    if let Some(pk) = &s.public_key {
        m.push((text("public_key"), text(pk)));
    }
    Value::Map(m)
}

fn navigation_to_value(nav: &NavigationMeta, pages: &[Page]) -> Value {
    let mut m: Vec<(Value, Value)> = Vec::new();
    let main: Vec<Value> = if nav.main.is_empty() {
        pages
            .iter()
            .filter(|p| p.access == "T2")
            .take(10)
            .map(|p| text(&p.path))
            .collect()
    } else {
        nav.main.iter().cloned().map(Value::Text).collect()
    };
    m.push((text("main"), Value::Array(main)));
    if !nav.footer.is_empty() {
        m.push((
            text("footer"),
            Value::Array(nav.footer.iter().cloned().map(Value::Text).collect()),
        ));
    }
    Value::Map(m)
}

fn pages_to_value(pages: &[Page]) -> Result<Value> {
    let mut out = Vec::with_capacity(pages.len());
    for p in pages {
        out.push(page_to_value(p)?);
    }
    Ok(Value::Array(out))
}

fn page_to_value(p: &Page) -> Result<Value> {
    let mut m: Vec<(Value, Value)> = vec![
        (text("path"), text(&p.path)),
        (text("title"), text(&p.title)),
        (text("lang"), text(&p.lang)),
        (text("access"), text(&p.access)),
    ];
    if let Some(d) = &p.description {
        m.push((text("description"), text(d)));
    }
    if let Some(ts) = p.updated {
        m.push((text("updated"), Value::Tag(1, Box::new(int(ts as i128)))));
    }
    let mut blocks = Vec::with_capacity(p.content.len());
    for b in &p.content {
        blocks.push(block_to_value(b)?);
    }
    m.push((text("content"), Value::Array(blocks)));
    Ok(Value::Map(m))
}

fn block_to_value(b: &Block) -> Result<Value> {
    let Block::Map(m) = b;
    let mut out: Vec<(Value, Value)> = Vec::new();
    for (k, v) in m {
        out.push((text(k), yaml_to_cbor(v)?));
    }
    Ok(Value::Map(out))
}

fn yaml_to_cbor(v: &serde_yaml::Value) -> Result<Value> {
    use serde_yaml::Value as Y;
    Ok(match v {
        Y::Null => Value::Null,
        Y::Bool(b) => Value::Bool(*b),
        Y::Number(n) => {
            if let Some(i) = n.as_i64() {
                int(i as i128)
            } else if let Some(u) = n.as_u64() {
                int(u as i128)
            } else if let Some(f) = n.as_f64() {
                Value::Float(f)
            } else {
                bail!("unsupported numeric: {n:?}")
            }
        }
        Y::String(s) => text(s),
        Y::Sequence(seq) => {
            let mut out = Vec::with_capacity(seq.len());
            for item in seq {
                out.push(yaml_to_cbor(item)?);
            }
            Value::Array(out)
        }
        Y::Mapping(map) => {
            let mut out: Vec<(Value, Value)> = Vec::new();
            for (k, v) in map {
                let Y::String(ks) = k else {
                    bail!("map keys must be strings, got {k:?}");
                };
                out.push((text(ks), yaml_to_cbor(v)?));
            }
            Value::Map(out)
        }
        Y::Tagged(_) => bail!("YAML tagged values are not supported in block content"),
    })
}

fn meta_to_value(m: &MetaInfo, total_pages: usize) -> Value {
    let mut out: Vec<(Value, Value)> = Vec::new();
    let ts = m.generated_at.unwrap_or(0);
    out.push((
        text("generated_at"),
        Value::Tag(1, Box::new(int(ts as i128))),
    ));
    let gen = m
        .generator
        .clone()
        .unwrap_or_else(|| "cbor-web-gen/0.1".into());
    out.push((text("generator"), text(gen)));
    out.push((text("total_pages"), int(total_pages as i128)));
    Value::Map(out)
}

/// Extract from a CBOR-Web file the exact bytes that a strict verifier must
/// sign-check against `meta.signature`: the document, in canonical form,
/// with the `signature` entry of the `meta` map removed.
pub fn bytes_for_signature(file_bytes: &[u8]) -> Result<(Vec<u8>, Vec<u8>)> {
    let value: Value = ciborium::de::from_reader(file_bytes).context("decoding source CBOR")?;
    let (tag, mut inner) = match value {
        Value::Tag(t, b) => (Some(t), *b),
        other => (None, other),
    };
    let Value::Map(root_entries) = &mut inner else {
        bail!("root is not a map");
    };
    let mut signature: Option<Vec<u8>> = None;
    for (k, v) in root_entries.iter_mut() {
        if as_int(k) != Some(keys::META) {
            continue;
        }
        let Value::Map(meta_entries) = v else {
            continue;
        };
        meta_entries.retain(|(mk, mv)| {
            if matches!(mk, Value::Text(t) if t == "signature") {
                if let Value::Bytes(b) = mv {
                    signature = Some(b.clone());
                }
                false
            } else {
                true
            }
        });
    }
    let Some(sig) = signature else {
        bail!("no meta.signature found in file");
    };
    let stripped = match tag {
        Some(t) => Value::Tag(t, Box::new(inner)),
        None => inner,
    };
    let signed_bytes = canonical_bytes(&stripped)?;
    Ok((signed_bytes, sig))
}

/// Decode a CBOR-Web file to a JSON-friendly serde_json::Value for inspection.
pub fn decode_to_json(bytes: &[u8]) -> Result<serde_json::Value> {
    let v: Value = ciborium::de::from_reader(bytes).context("CBOR decode failed")?;
    Ok(value_to_json(&v))
}

fn value_to_json(v: &Value) -> serde_json::Value {
    use serde_json::Value as J;
    match v {
        Value::Null => J::Null,
        Value::Bool(b) => J::Bool(*b),
        Value::Integer(i) => {
            let n: i128 = (*i).into();
            if let Ok(n64) = i64::try_from(n) {
                J::Number(n64.into())
            } else {
                J::String(n.to_string())
            }
        }
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(J::Number)
            .unwrap_or(J::Null),
        Value::Bytes(b) => J::String(format!("base16:{}", hex(b))),
        Value::Text(s) => J::String(s.clone()),
        Value::Array(a) => J::Array(a.iter().map(value_to_json).collect()),
        Value::Map(m) => {
            let mut o = serde_json::Map::new();
            for (k, v) in m {
                let key = match k {
                    Value::Text(s) => s.clone(),
                    Value::Integer(i) => {
                        let n: i128 = (*i).into();
                        n.to_string()
                    }
                    other => format!("{other:?}"),
                };
                o.insert(key, value_to_json(v));
            }
            J::Object(o)
        }
        Value::Tag(_, inner) => value_to_json(inner),
        _ => J::Null,
    }
}

fn hex(b: &[u8]) -> String {
    let mut out = String::with_capacity(b.len() * 2);
    for byte in b {
        out.push_str(&format!("{:02x}", byte));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn minimal_doc() -> Value {
        Value::Tag(
            SELF_DESCRIBED_TAG,
            Box::new(Value::Map(vec![
                (int(0), Value::Text("cbor-web".into())),
                (int(1), int(3)),
                (
                    int(2),
                    Value::Map(vec![
                        (Value::Text("domain".into()), Value::Text("ex.com".into())),
                        (Value::Text("name".into()), Value::Text("Ex".into())),
                    ]),
                ),
                (
                    int(5),
                    Value::Array(vec![Value::Map(vec![
                        (Value::Text("path".into()), Value::Text("/".into())),
                        (Value::Text("title".into()), Value::Text("Home".into())),
                        (Value::Text("lang".into()), Value::Text("en".into())),
                        (Value::Text("access".into()), Value::Text("T2".into())),
                        (
                            Value::Text("content".into()),
                            Value::Array(vec![Value::Map(vec![
                                (Value::Text("t".into()), Value::Text("p".into())),
                                (Value::Text("v".into()), Value::Text("hello".into())),
                            ])]),
                        ),
                    ])]),
                ),
            ])),
        )
    }

    #[test]
    fn canonical_sorts_map_keys_length_first_then_bytewise() {
        // {"bb": 1, "a": 2} → canonical orders shorter key first
        let v = Value::Map(vec![
            (Value::Text("bb".into()), int(1)),
            (Value::Text("a".into()), int(2)),
        ]);
        let bytes = canonical_bytes(&v).unwrap();
        let decoded: Value = ciborium::de::from_reader(&bytes[..]).unwrap();
        let Value::Map(entries) = decoded else {
            panic!("not a map");
        };
        let first_key = match &entries[0].0 {
            Value::Text(s) => s.clone(),
            _ => panic!(),
        };
        assert_eq!(first_key, "a", "shorter key must come first");
    }

    #[test]
    fn canonical_is_idempotent() {
        let v = minimal_doc();
        let once = canonical_bytes(&v).unwrap();
        let decoded: Value = ciborium::de::from_reader(&once[..]).unwrap();
        let twice = canonical_bytes(&decoded).unwrap();
        assert_eq!(once, twice, "canonical encoding must be a fixed point");
    }

    #[test]
    fn validate_rejects_missing_tag() {
        let mut bytes = canonical_bytes(&minimal_doc()).unwrap();
        // Strip the 3-byte self-described tag header (d9 d9 f7).
        bytes.drain(0..3);
        let r = validate_bytes(&bytes).unwrap();
        assert!(
            r.errors
                .iter()
                .any(|e| e.contains("missing self-described")),
            "errors: {:?}",
            r.errors
        );
    }

    #[test]
    fn validate_accepts_minimal_canonical_doc() {
        let bytes = canonical_bytes(&minimal_doc()).unwrap();
        let r = validate_bytes(&bytes).unwrap();
        assert!(r.ok(), "errors: {:?}", r.errors);
        assert!(
            r.warnings.is_empty(),
            "unexpected warnings: {:?}",
            r.warnings
        );
    }

    #[test]
    fn validate_detects_total_pages_mismatch() {
        // Add a meta with total_pages = 99 while pages.len() == 1.
        let mut doc = minimal_doc();
        if let Value::Tag(_, inner) = &mut doc {
            if let Value::Map(entries) = inner.as_mut() {
                entries.push((
                    int(6),
                    Value::Map(vec![(Value::Text("total_pages".into()), int(99))]),
                ));
            }
        }
        let bytes = canonical_bytes(&doc).unwrap();
        let r = validate_bytes(&bytes).unwrap();
        assert!(
            r.errors.iter().any(|e| e.contains("total_pages")),
            "errors: {:?}",
            r.errors
        );
    }

    #[test]
    fn validate_detects_wrong_page_hash() {
        // Inject a 32-byte all-zeros hash on the page — won't match SHA-256.
        let mut doc = minimal_doc();
        if let Value::Tag(_, inner) = &mut doc {
            if let Value::Map(entries) = inner.as_mut() {
                for (k, v) in entries.iter_mut() {
                    if as_int(k) == Some(keys::PAGES) {
                        if let Value::Array(pages) = v {
                            if let Value::Map(p) = &mut pages[0] {
                                p.push((Value::Text("hash".into()), Value::Bytes(vec![0u8; 32])));
                            }
                        }
                    }
                }
            }
        }
        let bytes = canonical_bytes(&doc).unwrap();
        let r = validate_bytes(&bytes).unwrap();
        assert!(
            r.errors.iter().any(|e| e.contains("hash does not match")),
            "errors: {:?}",
            r.errors
        );
    }

    #[test]
    fn validate_warns_on_unknown_block_type() {
        let mut doc = minimal_doc();
        if let Value::Tag(_, inner) = &mut doc {
            if let Value::Map(entries) = inner.as_mut() {
                for (k, v) in entries.iter_mut() {
                    if as_int(k) == Some(keys::PAGES) {
                        if let Value::Array(pages) = v {
                            if let Value::Map(p) = &mut pages[0] {
                                for (pk, pv) in p.iter_mut() {
                                    if matches!(pk, Value::Text(s) if s == "content") {
                                        *pv = Value::Array(vec![Value::Map(vec![(
                                            Value::Text("t".into()),
                                            Value::Text("never-heard-of-this".into()),
                                        )])]);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        let bytes = canonical_bytes(&doc).unwrap();
        let r = validate_bytes(&bytes).unwrap();
        assert!(r.ok(), "should not be a hard error");
        assert!(
            r.warnings.iter().any(|w| w.contains("unknown block type")),
            "warnings: {:?}",
            r.warnings
        );
    }

    #[test]
    fn bytes_for_signature_round_trip() {
        // Build a doc with a fake signature, then extract → re-canonicalise.
        let mut doc = minimal_doc();
        if let Value::Tag(_, inner) = &mut doc {
            if let Value::Map(entries) = inner.as_mut() {
                entries.push((
                    int(6),
                    Value::Map(vec![
                        (Value::Text("generator".into()), Value::Text("test".into())),
                        (Value::Text("signature".into()), Value::Bytes(vec![7u8; 64])),
                    ]),
                ));
            }
        }
        let file_bytes = canonical_bytes(&doc).unwrap();
        let (signed, sig) = bytes_for_signature(&file_bytes).unwrap();
        assert_eq!(sig, vec![7u8; 64]);
        // signed bytes must round-trip and contain no `signature` key.
        let decoded: Value = ciborium::de::from_reader(&signed[..]).unwrap();
        let json = decode_to_json(&signed).unwrap();
        assert!(
            !serde_json::to_string(&json).unwrap().contains("signature"),
            "signature must be stripped before signing: {:?}",
            decoded
        );
    }
}
