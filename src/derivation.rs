//! Core BIP-Keychain derivation algorithm
//!
//! Implements the entity → derived key pipeline:
//! 1. Canonicalize entity JSON
//! 2. Hash with configured function
//! 3. Extract first 4 bytes as u32 index
//! 4. Derive BIP-32 key at m/83696968'/67797668'/{index}'
//! 5. Return derived key

use crate::{
    entity::{KeyDerivation, HashFunctionConfig},
    hash::{hash_entity, HashFunction},
    bip32_wrapper::{Keychain, DerivedKey},
    error::{BipKeychainError, Result},
};

/// Derive a key from an entity using BIP-Keychain
///
/// This is the main entry point for BIP-Keychain derivation. It takes:
/// - A keychain (from BIP-39 mnemonic)
/// - A key derivation specification (entity + config)
/// - Parent entropy for HMAC-based hashing
///
/// And returns a derived key that can be used for Ed25519 key generation.
///
/// # Example
///
/// ```ignore
/// let keychain = Keychain::from_mnemonic("your mnemonic...")?;
/// let key_deriv = KeyDerivation::from_json(entity_json)?;
/// let parent_entropy = b"your_parent_entropy";
///
/// let derived_key = derive_key_from_entity(&keychain, &key_deriv, parent_entropy)?;
/// let ed25519_seed = derived_key.to_seed();  // 32 bytes
/// ```
pub fn derive_key_from_entity(
    keychain: &Keychain,
    key_derivation: &KeyDerivation,
    parent_entropy: &[u8],
) -> Result<DerivedKey> {
    // Step 1: Get entity as canonical JSON string
    let entity_json = key_derivation.entity_json()?;

    // Step 2: Select hash function based on config
    let hash_function = match key_derivation.derivation_config.hash_function {
        HashFunctionConfig::HmacSha512 => HashFunction::HmacSha512,
        HashFunctionConfig::Blake2b => HashFunction::Blake2b,
        HashFunctionConfig::Sha256 => HashFunction::Sha256,
    };

    // Step 3: Hash the entity JSON
    let hash_output = hash_entity(&entity_json, parent_entropy, hash_function)?;

    // Step 4: Extract first 4 bytes as big-endian u32 for BIP-32 child index
    let index = hash_to_index(&hash_output)?;

    // Step 5: Derive BIP-32 key at BIP-Keychain path with entity-specific index
    let derived_key = keychain.derive_bip_keychain_path(index)?;

    Ok(derived_key)
}

/// Convert hash output to BIP-32 child index
///
/// Extracts the first 4 bytes from the hash and interprets them as a
/// big-endian unsigned 32-bit integer. This index is used for BIP-32 derivation.
///
/// BIP-32 supports child indices from 0 to 2^32-1. We use the full range.
fn hash_to_index(hash: &[u8; 64]) -> Result<u32> {
    if hash.len() < 4 {
        return Err(BipKeychainError::HashError(
            "Hash output too short for index extraction".to_string()
        ));
    }

    // Take first 4 bytes and convert to u32 (big-endian)
    let index_bytes = [hash[0], hash[1], hash[2], hash[3]];
    let index = u32::from_be_bytes(index_bytes);

    Ok(index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_index() {
        let mut hash = [0u8; 64];

        // Test: [0x00, 0x00, 0x00, 0x01, ...] = 1
        hash[3] = 1;
        assert_eq!(hash_to_index(&hash).unwrap(), 1);

        // Test: [0x00, 0x00, 0x01, 0x00, ...] = 256
        hash = [0u8; 64];
        hash[2] = 1;
        assert_eq!(hash_to_index(&hash).unwrap(), 256);

        // Test: [0x00, 0x00, 0xff, 0xff, ...] = 65535
        hash = [0u8; 64];
        hash[2] = 0xff;
        hash[3] = 0xff;
        assert_eq!(hash_to_index(&hash).unwrap(), 65535);

        // Test: [0xff, 0xff, 0xff, 0xff, ...] = 2^32 - 1
        hash = [0xffu8; 64];
        assert_eq!(hash_to_index(&hash).unwrap(), 4_294_967_295);
    }

    #[test]
    fn test_end_to_end_derivation() {
        let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "name": "Test Entity"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  }
}"#;

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let parent_entropy = b"test_entropy_32_bytes_long_here!";

        let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived = derive_key_from_entity(&keychain, &key_deriv, parent_entropy)
            .expect("Should derive key from entity");

        // Should get 32 bytes for Ed25519
        let seed = derived.to_seed();
        assert_eq!(seed.len(), 32);
    }

    #[test]
    fn test_deterministic() {
        let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}"#;

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let parent_entropy = b"test_entropy";

        let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived1 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();
        let derived2 = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();

        assert_eq!(derived1.to_bytes(), derived2.to_bytes());
    }

    #[test]
    fn test_different_entities_different_keys() {
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
        let parent_entropy = b"test_entropy";

        let key_deriv1 = KeyDerivation::from_json(entity1).unwrap();
        let key_deriv2 = KeyDerivation::from_json(entity2).unwrap();
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived1 = derive_key_from_entity(&keychain, &key_deriv1, parent_entropy).unwrap();
        let derived2 = derive_key_from_entity(&keychain, &key_deriv2, parent_entropy).unwrap();

        assert_ne!(derived1.to_bytes(), derived2.to_bytes());
    }

    #[test]
    fn test_blake2b_derivation() {
        let entity_json = r#"{
  "schema_type": "gordian_envelope",
  "entity": {"envelope": "ur:envelope/example"},
  "derivation_config": {"hash_function": "blake2b", "hardened": true}
}"#;

        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let parent_entropy = b"dummy_entropy"; // BLAKE2b doesn't use this

        let key_deriv = KeyDerivation::from_json(entity_json).unwrap();
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived = derive_key_from_entity(&keychain, &key_deriv, parent_entropy).unwrap();

        assert_eq!(derived.to_seed().len(), 32);
    }
}
