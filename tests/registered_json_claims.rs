use json_syntax::{JsonParse, JsonValue};
use ssi_jws::JwsPayload;
use ssi_jwt::{RegisteredClaims, VerifiableCredential, VerifiablePresentation};

#[test]
fn credential_and_presentation_json_claims_preserve_nested_values() {
    let source = r#"{"id":"urn:example:credential","nested":{"member":true,"other":false},"score":9007199254740993,"name":"Amélie"}"#;
    let value = JsonValue::parse_str(source).unwrap().0;
    let mut claims = RegisteredClaims::new();
    claims.set(VerifiableCredential(value.clone()));
    claims.set(VerifiablePresentation(value.clone()));

    let encoded = claims.payload_bytes();
    let decoded: RegisteredClaims = serde_json::from_slice(&encoded).unwrap();
    assert_eq!(decoded.get::<VerifiableCredential>().unwrap().0, value);
    assert_eq!(decoded.get::<VerifiablePresentation>().unwrap().0, value);
    assert_eq!(decoded.len(), 2);
}
