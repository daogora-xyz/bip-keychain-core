//! Hash functions for entity-to-index conversion
//!
//! Supports multiple hash functions for semantic entity derivation:
//! - HMAC-SHA-512 (BIP-85 standard)
//! - BLAKE2b (Blockchain Commons compatibility)
//! - SHA-256

use crate::error::{BipKeychainError, Result};

/// Hash function selection for entity derivation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashFunction {
    /// HMAC-SHA-512 (BIP-85 standard)
    HmacSha512,
    /// BLAKE2b (Blockchain Commons)
    Blake2b,
    /// SHA-256
    Sha256,
}

/// Hash an entity JSON string with parent entropy
///
/// Returns a 64-byte digest for all hash functions (padded if needed).
pub fn hash_entity(
    entity_json: &str,
    parent_entropy: &[u8],
    hash_fn: HashFunction,
) -> Result<[u8; 64]> {
    match hash_fn {
        HashFunction::HmacSha512 => hmac_sha512(entity_json, parent_entropy),
        HashFunction::Blake2b => blake2b_hash(entity_json),
        HashFunction::Sha256 => sha256_padded(entity_json, parent_entropy),
    }
}

/// HMAC-SHA-512 implementation (BIP-85 standard)
fn hmac_sha512(entity_json: &str, parent_entropy: &[u8]) -> Result<[u8; 64]> {
    use hmac::{Hmac, Mac};
    use sha2::Sha512;

    type HmacSha512 = Hmac<Sha512>;

    // Create HMAC instance with parent entropy as key
    let mut mac = HmacSha512::new_from_slice(parent_entropy)
        .map_err(|e| BipKeychainError::HashError(format!("HMAC key error: {}", e)))?;

    // Hash the entity JSON string
    mac.update(entity_json.as_bytes());

    // Finalize and get the result
    let result = mac.finalize();
    let bytes = result.into_bytes();

    // Convert to fixed-size array
    let mut output = [0u8; 64];
    output.copy_from_slice(&bytes);

    Ok(output)
}

/// BLAKE2b implementation (Blockchain Commons)
fn blake2b_hash(_entity_json: &str) -> Result<[u8; 64]> {
    // Stub: will be implemented later
    unimplemented!("BLAKE2b not yet implemented")
}

/// SHA-256 implementation (padded to 64 bytes)
fn sha256_padded(_entity_json: &str, _parent_entropy: &[u8]) -> Result<[u8; 64]> {
    // Stub: will be implemented later
    unimplemented!("SHA-256 not yet implemented")
}
