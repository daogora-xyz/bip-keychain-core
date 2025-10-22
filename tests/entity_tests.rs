//! Tests for entity parsing
//!
//! Tests parsing of Nickel-exported JSON entities into Rust structs.

// This will fail until we implement entity.rs
// use bip_keychain::entity::{KeyDerivation, DerivationConfig, HashFunctionConfig};

#[test]
#[ignore] // Ignore until entity module is implemented
fn test_parse_schema_org_entity() {
    // Example from nickel/examples/github-repo-schema-org.ncl
    let json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/DAOgora-xyz/bip-keychain-core",
    "programmingLanguage": "Nickel",
    "name": "BIP-Keychain Core"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Git commit signing key for bip-keychain-core repository",
  "metadata": {
    "created": "2025-10-21",
    "owner": "DAOgora-xyz"
  }
}"#;

    // This will work once we implement entity.rs
    // let key_derivation: KeyDerivation = serde_json::from_str(json)
    //     .expect("Should parse schema.org entity JSON");

    // assert_eq!(key_derivation.schema_type, "schema_org");
    // assert_eq!(key_derivation.derivation_config.hash_function, "hmac_sha512");
    // assert_eq!(key_derivation.derivation_config.hardened, true);
    // assert_eq!(key_derivation.purpose.unwrap(), "Git commit signing key for bip-keychain-core repository");

    // For now, just verify it's valid JSON
    let value: serde_json::Value = serde_json::from_str(json)
        .expect("Should be valid JSON");

    assert_eq!(value["schema_type"], "schema_org");
    assert_eq!(value["entity"]["@type"], "SoftwareSourceCode");
}

#[test]
fn test_parse_entity_with_minimal_fields() {
    // Minimal valid entity with just required fields
    let json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "Thing",
    "name": "Test"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  }
}"#;

    let value: serde_json::Value = serde_json::from_str(json)
        .expect("Should parse minimal entity");

    assert_eq!(value["schema_type"], "schema_org");
}

#[test]
fn test_parse_entity_with_blake2b() {
    // Entity using BLAKE2b hash function
    let json = r#"{
  "schema_type": "gordian_envelope",
  "entity": {
    "envelope": "ur:envelope/example123",
    "format": "ur:envelope"
  },
  "derivation_config": {
    "hash_function": "blake2b",
    "hardened": true
  },
  "purpose": "Selective disclosure credentials"
}"#;

    let value: serde_json::Value = serde_json::from_str(json)
        .expect("Should parse BLAKE2b entity");

    assert_eq!(value["derivation_config"]["hash_function"], "blake2b");
}
