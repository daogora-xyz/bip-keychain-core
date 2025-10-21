//! Hash functions for entity-to-index conversion
//!
//! Supports multiple hash functions for semantic entity derivation:
//! - HMAC-SHA-512 (BIP-85 standard)
//! - BLAKE2b (Blockchain Commons compatibility)
//! - SHA-256

use crate::error::{BipKeychainError, Result};
use serde_json::Value;

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

    // Canonicalize JSON for deterministic hashing
    let canonical = canonicalize_json(entity_json)?;

    // Create HMAC instance with parent entropy as key
    let mut mac = HmacSha512::new_from_slice(parent_entropy)
        .map_err(|e| BipKeychainError::HashError(format!("HMAC key error: {}", e)))?;

    // Hash the canonical JSON string
    mac.update(canonical.as_bytes());

    // Finalize and get the result
    let result = mac.finalize();
    let bytes = result.into_bytes();

    // Convert to fixed-size array
    let mut output = [0u8; 64];
    output.copy_from_slice(&bytes);

    Ok(output)
}

/// BLAKE2b implementation (Blockchain Commons)
fn blake2b_hash(entity_json: &str) -> Result<[u8; 64]> {
    use alkali::hash::generic;

    // Canonicalize JSON for deterministic hashing
    let canonical = canonicalize_json(entity_json)?;

    // BLAKE2b-512 hash (64 bytes) using libsodium via alkali
    // Blockchain Commons uses libsodium's implementation
    let hash = generic::hash(canonical.as_bytes())
        .map_err(|e| BipKeychainError::HashError(format!("BLAKE2b error: {:?}", e)))?;

    // Convert to fixed-size array
    let mut output = [0u8; 64];
    output.copy_from_slice(hash.as_ref());

    Ok(output)
}

/// SHA-256 implementation (padded to 64 bytes)
fn sha256_padded(_entity_json: &str, _parent_entropy: &[u8]) -> Result<[u8; 64]> {
    // Stub: will be implemented later
    unimplemented!("SHA-256 not yet implemented")
}

/// Canonicalize JSON string for deterministic hashing
///
/// If the input is valid JSON, re-serialize it in canonical form:
/// - Keys sorted alphabetically
/// - No whitespace
/// - UTF-8 encoding
///
/// If the input is not JSON (e.g., plain text test vectors), return as-is.
fn canonicalize_json(input: &str) -> Result<String> {
    // Try to parse as JSON
    match serde_json::from_str::<Value>(input) {
        Ok(value) => {
            // Re-serialize in canonical form (serde_json sorts keys by default)
            serde_json::to_string(&value)
                .map_err(|e| BipKeychainError::HashError(format!("JSON serialization error: {}", e)))
        }
        Err(_) => {
            // Not JSON, use input as-is (for test vectors)
            Ok(input.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canonicalize_json() {
        // Test with pretty-printed JSON
        let pretty = r#"{
  "name": "test",
  "age": 30,
  "city": "NYC"
}"#;
        let canonical = canonicalize_json(pretty).unwrap();
        assert_eq!(canonical, r#"{"age":30,"city":"NYC","name":"test"}"#);

        // Test with plain text (non-JSON)
        let plain = "Hi There";
        let result = canonicalize_json(plain).unwrap();
        assert_eq!(result, plain);
    }
}
