//! Tests for entity parsing
//!
//! Tests parsing of Nickel-exported JSON entities into Rust structs.

use bip_keychain::{HashFunctionConfig, KeyDerivation};

#[test]
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

    let key_derivation =
        KeyDerivation::from_json(json).expect("Should parse schema.org entity JSON");

    assert_eq!(key_derivation.schema_type, "schema_org");
    assert_eq!(
        key_derivation.derivation_config.hash_function,
        HashFunctionConfig::HmacSha512
    );
    assert!(key_derivation.derivation_config.hardened);
    assert_eq!(
        key_derivation.purpose.unwrap(),
        "Git commit signing key for bip-keychain-core repository"
    );

    // Verify entity fields
    assert_eq!(key_derivation.entity["@type"], "SoftwareSourceCode");
    assert_eq!(key_derivation.entity["name"], "BIP-Keychain Core");
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

    let key_derivation = KeyDerivation::from_json(json).expect("Should parse minimal entity");

    assert_eq!(key_derivation.schema_type, "schema_org");
    assert!(key_derivation.purpose.is_none());
    assert!(key_derivation.metadata.is_none());
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

    let key_derivation = KeyDerivation::from_json(json).expect("Should parse BLAKE2b entity");

    assert_eq!(
        key_derivation.derivation_config.hash_function,
        HashFunctionConfig::Blake2b
    );
    assert_eq!(key_derivation.schema_type, "gordian_envelope");
    assert_eq!(
        key_derivation.purpose.unwrap(),
        "Selective disclosure credentials"
    );
}
