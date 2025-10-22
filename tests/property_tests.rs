//! Property-based tests for BIP-Keychain
//!
//! Uses proptest to verify key properties of the system:
//! - Determinism: same input always produces same output
//! - Uniqueness: different inputs produce different outputs (with high probability)
//! - Stability: derived keys don't change across runs

use bip_keychain::{derive_key_from_entity, Keychain, KeyDerivation, HashFunction, hash_entity};
use proptest::prelude::*;

/// Test that identical entities produce identical keys (determinism)
#[test]
fn test_determinism_manual() {
    let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let parent_entropy = b"test_entropy";

    let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    // Derive the same key multiple times
    let derived1 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
    let derived2 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
    let derived3 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();

    // All should be identical
    assert_eq!(derived1.to_bytes(), derived2.to_bytes());
    assert_eq!(derived2.to_bytes(), derived3.to_bytes());
}

/// Test that different entity names produce different keys
#[test]
fn test_uniqueness_different_names() {
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let parent_entropy = b"test_entropy";
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    let names = ["Alice", "Bob", "Charlie", "David", "Eve"];
    let mut keys = Vec::new();

    for name in &names {
        let entity_json = format!(r#"{{
  "schema_type": "schema_org",
  "entity": {{"@type": "Person", "name": "{}"}},
  "derivation_config": {{"hash_function": "hmac_sha512", "hardened": true}}
}}"#, name);

        let key_deriv = KeyDerivation::from_json(&entity_json).unwrap();
        let derived = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
        keys.push(derived.to_bytes());
    }

    // All keys should be unique
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i], keys[j],
                "Keys for '{}' and '{}' should be different",
                names[i], names[j]
            );
        }
    }
}

/// Test that different hash functions produce different keys for the same entity
#[test]
fn test_different_hash_functions() {
    let entity_base = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test"},
  "derivation_config": {"hash_function": "PLACEHOLDER", "hardened": true}
}"#;

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    let parent_entropy = b"test_entropy";
    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    let hash_functions = ["hmac_sha512", "blake2b", "sha256"];
    let mut keys = Vec::new();

    for hash_fn in &hash_functions {
        let entity_json = entity_base.replace("PLACEHOLDER", hash_fn);
        let key_deriv = KeyDerivation::from_json(&entity_json).unwrap();
        let derived = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
        keys.push((hash_fn, derived.to_bytes()));
    }

    // All keys should be different
    for i in 0..keys.len() {
        for j in (i + 1)..keys.len() {
            assert_ne!(
                keys[i].1, keys[j].1,
                "Keys using {} and {} should be different",
                keys[i].0, keys[j].0
            );
        }
    }
}

/// Property test: hash determinism
/// For any input string, hashing it twice should give the same result
proptest! {
    #[test]
    fn prop_hash_determinism(input in "\\PC*") {
        let parent_entropy = b"test_entropy_32_bytes_long_here!";

        let hash1 = hash_entity(&input, parent_entropy, HashFunction::HmacSha512).unwrap();
        let hash2 = hash_entity(&input, parent_entropy, HashFunction::HmacSha512).unwrap();

        prop_assert_eq!(hash1, hash2, "Same input should produce same hash");
    }
}

/// Property test: different inputs produce different hashes (with high probability)
/// This tests collision resistance
proptest! {
    #[test]
    fn prop_hash_uniqueness(s1 in "\\PC{1,100}", s2 in "\\PC{1,100}") {
        prop_assume!(s1 != s2); // Only test when inputs are different

        let parent_entropy = b"test_entropy_32_bytes_long_here!";

        let hash1 = hash_entity(&s1, parent_entropy, HashFunction::HmacSha512).unwrap();
        let hash2 = hash_entity(&s2, parent_entropy, HashFunction::HmacSha512).unwrap();

        prop_assert_ne!(hash1, hash2, "Different inputs should produce different hashes");
    }
}

/// Property test: derivation determinism
/// Same mnemonic + same entity should always produce the same key
proptest! {
    #[test]
    fn prop_derivation_determinism(entity_name in "[a-zA-Z0-9]{1,50}") {
        let entity_json = format!(r#"{{
  "schema_type": "schema_org",
  "entity": {{"@type": "Thing", "name": "{}"}},
  "derivation_config": {{"hash_function": "hmac_sha512", "hardened": true}}
}}"#, entity_name);

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let parent_entropy = b"test";

        let key_deriv = KeyDerivation::from_json(&entity_json).unwrap();
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived1 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
        let derived2 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();

        prop_assert_eq!(derived1.to_bytes(), derived2.to_bytes());
    }
}

/// Property test: different entities produce different keys
proptest! {
    #[test]
    fn prop_entity_uniqueness(name1 in "[a-zA-Z0-9]{1,50}", name2 in "[a-zA-Z0-9]{1,50}") {
        prop_assume!(name1 != name2); // Only test when names are different

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let parent_entropy = b"test";
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let entity1 = format!(r#"{{
  "schema_type": "schema_org",
  "entity": {{"@type": "Thing", "name": "{}"}},
  "derivation_config": {{"hash_function": "hmac_sha512", "hardened": true}}
}}"#, name1);

        let entity2 = format!(r#"{{
  "schema_type": "schema_org",
  "entity": {{"@type": "Thing", "name": "{}"}},
  "derivation_config": {{"hash_function": "hmac_sha512", "hardened": true}}
}}"#, name2);

        let key_deriv1 = KeyDerivation::from_json(&entity1).unwrap();
        let key_deriv2 = KeyDerivation::from_json(&entity2).unwrap();

        let derived1 = derive_key_from_entity(&keychain, &key_deriv1, parent_entropy).unwrap();
        let derived2 = derive_key_from_entity(&keychain, &key_deriv2, parent_entropy).unwrap();

        prop_assert_ne!(derived1.to_bytes(), derived2.to_bytes());
    }
}

/// Property test: Ed25519 keypair generation is deterministic
proptest! {
    #[test]
    fn prop_ed25519_determinism(seed_byte in any::<u8>()) {
        use bip_keychain::Ed25519Keypair;

        let seed = [seed_byte; 32];

        let keypair1 = Ed25519Keypair::from_seed(seed);
        let keypair2 = Ed25519Keypair::from_seed(seed);

        prop_assert_eq!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
        prop_assert_eq!(keypair1.private_key_bytes(), keypair2.private_key_bytes());
    }
}

/// Property test: different seeds produce different Ed25519 keys
proptest! {
    #[test]
    fn prop_ed25519_uniqueness(byte1 in any::<u8>(), byte2 in any::<u8>()) {
        use bip_keychain::Ed25519Keypair;

        prop_assume!(byte1 != byte2);

        let seed1 = [byte1; 32];
        let seed2 = [byte2; 32];

        let keypair1 = Ed25519Keypair::from_seed(seed1);
        let keypair2 = Ed25519Keypair::from_seed(seed2);

        prop_assert_ne!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
    }
}
