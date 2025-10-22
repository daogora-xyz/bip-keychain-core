//! Integration tests for end-to-end BIP-Keychain derivation
//!
//! Tests the complete flow: Entity JSON → Hash → Index → BIP-32 Derive → Key

use bip_keychain::{Keychain, KeyDerivation, HashFunction};

// This will fail until we implement derivation.rs
// use bip_keychain::derive_key_from_entity;

#[test]
#[ignore] // Ignore until derivation module is implemented
fn test_end_to_end_derivation() {
    // Complete workflow: entity → derived key

    let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/DAOgora-xyz/bip-keychain-core",
    "name": "BIP-Keychain Core"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Git commit signing key"
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    // Parse entity
    let key_deriv = KeyDerivation::from_json(entity_json)
        .expect("Should parse entity JSON");

    // Create keychain
    let keychain = Keychain::from_mnemonic(mnemonic)
        .expect("Should create keychain");

    // Derive key (this will work once we implement derivation.rs)
    // let derived_key = derive_key_from_entity(&keychain, &key_deriv, b"parent_entropy")
    //     .expect("Should derive key from entity");

    // Should get 32 bytes for Ed25519
    // let seed = derived_key.to_seed();
    // assert_eq!(seed.len(), 32);
}

#[test]
#[ignore]
fn test_deterministic_entity_derivation() {
    // Same entity should always produce same key

    let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "Thing",
    "name": "Test Entity"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  }
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    // Derive twice
    // let derived1 = derive_key_from_entity(&keychain, &key_deriv, b"parent_entropy").unwrap();
    // let derived2 = derive_key_from_entity(&keychain, &key_deriv, b"parent_entropy").unwrap();

    // Should be identical
    // assert_eq!(derived1.to_bytes(), derived2.to_bytes());
}

#[test]
#[ignore]
fn test_different_entities_different_keys() {
    // Different entities should produce different keys

    let entity1 = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Entity 1"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}"#;

    let entity2 = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Entity 2"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let key_deriv1 = KeyDerivation::from_json(entity1).unwrap();
    let key_deriv2 = KeyDerivation::from_json(entity2).unwrap();
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    // Derive from different entities
    // let derived1 = derive_key_from_entity(&keychain, &key_deriv1, b"parent_entropy").unwrap();
    // let derived2 = derive_key_from_entity(&keychain, &key_deriv2, b"parent_entropy").unwrap();

    // Should be different
    // assert_ne!(derived1.to_bytes(), derived2.to_bytes());
}

#[test]
#[ignore]
fn test_blake2b_derivation() {
    // Test using BLAKE2b hash function

    let entity_json = r#"{
  "schema_type": "gordian_envelope",
  "entity": {
    "envelope": "ur:envelope/example",
    "format": "ur:envelope"
  },
  "derivation_config": {
    "hash_function": "blake2b",
    "hardened": true
  }
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    // Derive using BLAKE2b
    // let derived = derive_key_from_entity(&keychain, &key_deriv, b"parent_entropy").unwrap();

    // Should still get valid key
    // assert_eq!(derived.to_seed().len(), 32);
}

// Helper test: manually test the derivation flow step by step
#[test]
fn test_manual_derivation_flow() {
    use bip_keychain::hash::hash_entity;

    // Step 1: Parse entity
    let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}"#;

    let key_deriv = KeyDerivation::from_json(entity_json)
        .expect("Should parse entity");

    // Step 2: Hash entity
    let entity_str = key_deriv.entity_json().unwrap();
    let parent_entropy = b"test_entropy_32_bytes_long_here!";

    let hash = hash_entity(&entity_str, parent_entropy, HashFunction::HmacSha512)
        .expect("Should hash entity");

    assert_eq!(hash.len(), 64);

    // Step 3: Extract first 4 bytes as u32 index
    let index_bytes = &hash[0..4];
    let index = u32::from_be_bytes([index_bytes[0], index_bytes[1], index_bytes[2], index_bytes[3]]);

    // Step 4: Derive at BIP-Keychain path
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    let derived = keychain.derive_bip_keychain_path(index).unwrap();

    // Step 5: Get seed for Ed25519
    let seed = derived.to_seed();
    assert_eq!(seed.len(), 32);

    // This proves the manual flow works!
    // Now we just need to wrap it in a nice API (derivation.rs)
}
