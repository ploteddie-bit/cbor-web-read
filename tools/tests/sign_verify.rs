//! Round-trip test: sign a CBOR-Web document with Ed25519 and verify it back
//! using the public bytes_for_signature helper. Mirrors the path that
//! `cbor-web-verify-signature` takes in production.

use cbor_web_tools::{bytes_for_signature, canonical_bytes};
use ciborium::value::{Integer, Value};
use ed25519_dalek::{Signer, SigningKey, Verifier, VerifyingKey};

fn int(n: i128) -> Value {
    Value::Integer(Integer::try_from(n).unwrap())
}

fn text(s: &str) -> Value {
    Value::Text(s.into())
}

fn doc_with_placeholder_signature() -> Value {
    Value::Tag(
        55799,
        Box::new(Value::Map(vec![
            (int(0), text("cbor-web")),
            (int(1), int(3)),
            (
                int(2),
                Value::Map(vec![
                    (text("domain"), text("ex.com")),
                    (text("name"), text("Ex")),
                ]),
            ),
            (
                int(5),
                Value::Array(vec![Value::Map(vec![
                    (text("path"), text("/")),
                    (text("title"), text("Home")),
                    (text("lang"), text("en")),
                    (text("access"), text("T2")),
                    (
                        text("content"),
                        Value::Array(vec![Value::Map(vec![
                            (text("t"), text("p")),
                            (text("v"), text("hello")),
                        ])]),
                    ),
                ])]),
            ),
            (
                int(6),
                Value::Map(vec![
                    (text("generator"), text("test")),
                    // 64 zero bytes — placeholder, will be replaced after signing.
                    (text("signature"), Value::Bytes(vec![0u8; 64])),
                ]),
            ),
        ])),
    )
}

fn replace_signature(doc: &mut Value, new_sig: Vec<u8>) {
    let Value::Tag(_, inner) = doc else { return };
    let Value::Map(entries) = inner.as_mut() else {
        return;
    };
    for (k, v) in entries.iter_mut() {
        if !matches!(k, Value::Integer(i) if i128::from(*i) == 6) {
            continue;
        }
        let Value::Map(meta) = v else { continue };
        for (mk, mv) in meta.iter_mut() {
            if matches!(mk, Value::Text(t) if t == "signature") {
                *mv = Value::Bytes(new_sig.clone());
            }
        }
    }
}

#[test]
fn ed25519_sign_then_verify_round_trip() {
    // Use a deterministic key so the test does not depend on system RNG.
    let signing_key = SigningKey::from_bytes(&[42u8; 32]);
    let verifying_key: VerifyingKey = signing_key.verifying_key();

    let mut doc = doc_with_placeholder_signature();

    // 1. Encode the doc once with a placeholder signature.
    let placeholder_bytes = canonical_bytes(&doc).unwrap();

    // 2. Extract the bytes-to-sign (signature stripped from meta).
    let (signed_bytes, _) = bytes_for_signature(&placeholder_bytes).unwrap();

    // 3. Sign those bytes.
    let signature = signing_key.sign(&signed_bytes);
    let sig_bytes = signature.to_bytes().to_vec();
    assert_eq!(sig_bytes.len(), 64);

    // 4. Embed the real signature, re-encode canonically.
    replace_signature(&mut doc, sig_bytes.clone());
    let final_bytes = canonical_bytes(&doc).unwrap();

    // 5. Verifier path: extract bytes-to-sign + signature again, then check.
    let (verify_bytes, embedded_sig) = bytes_for_signature(&final_bytes).unwrap();
    assert_eq!(verify_bytes, signed_bytes);
    assert_eq!(embedded_sig, sig_bytes);

    let sig_array: [u8; 64] = embedded_sig.as_slice().try_into().unwrap();
    let parsed = ed25519_dalek::Signature::from_bytes(&sig_array);
    verifying_key
        .verify(&verify_bytes, &parsed)
        .expect("signature must verify");
}

#[test]
fn ed25519_verify_rejects_tampered_content() {
    let signing_key = SigningKey::from_bytes(&[7u8; 32]);
    let verifying_key = signing_key.verifying_key();

    let mut doc = doc_with_placeholder_signature();
    let placeholder = canonical_bytes(&doc).unwrap();
    let (signed_bytes, _) = bytes_for_signature(&placeholder).unwrap();
    let sig = signing_key.sign(&signed_bytes).to_bytes().to_vec();
    replace_signature(&mut doc, sig);

    // Tamper: change the page title in the signed document.
    if let Value::Tag(_, inner) = &mut doc {
        if let Value::Map(entries) = inner.as_mut() {
            for (k, v) in entries.iter_mut() {
                if !matches!(k, Value::Integer(i) if i128::from(*i) == 5) {
                    continue;
                }
                let Value::Array(pages) = v else { continue };
                let Value::Map(p) = &mut pages[0] else {
                    continue;
                };
                for (pk, pv) in p.iter_mut() {
                    if matches!(pk, Value::Text(s) if s == "title") {
                        *pv = Value::Text("TAMPERED".into());
                    }
                }
            }
        }
    }

    let tampered = canonical_bytes(&doc).unwrap();
    let (verify_bytes, embedded_sig) = bytes_for_signature(&tampered).unwrap();
    let sig_array: [u8; 64] = embedded_sig.as_slice().try_into().unwrap();
    let parsed = ed25519_dalek::Signature::from_bytes(&sig_array);
    assert!(
        verifying_key.verify(&verify_bytes, &parsed).is_err(),
        "tampered document must NOT verify"
    );
}
