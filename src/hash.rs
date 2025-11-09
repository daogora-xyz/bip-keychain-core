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
///
/// Uses libsodium's BLAKE2b-512 implementation via the alkali crate for
/// compatibility with Blockchain Commons tooling. BLAKE2b is faster than
/// SHA-512 while providing equivalent security (512-bit output).
///
/// Note: This implementation does NOT use parent entropy as BLAKE2b is used
/// as a pure hash function (not keyed hash like HMAC-SHA-512).
fn blake2b_hash(entity_json: &str) -> Result<[u8; 64]> {
    use alkali::hash::generic;

    // Canonicalize JSON for deterministic hashing
    // For large entities, this allocates a new string. For pre-canonicalized
    // inputs, this is a small overhead but ensures correctness.
    let canonical = canonicalize_json(entity_json)?;

    // BLAKE2b-512 hash (64 bytes) using libsodium via alkali
    // Blockchain Commons uses libsodium's implementation for consistency
    // across their ecosystem (Gordian Envelope, etc.)
    // Use hash_custom to specify 64-byte output (default is 32 bytes)
    let mut output = [0u8; 64];
    generic::hash_custom(canonical.as_bytes(), None, &mut output)
        .map_err(|e| BipKeychainError::HashError(format!("BLAKE2b hashing failed: {:?}", e)))?;

    Ok(output)
}

/// SHA-256 implementation (padded to 64 bytes)
///
/// Uses SHA-256 which produces 32 bytes, then pads with zeros to 64 bytes
/// for consistency with other hash functions. This is an alternative hash
/// function for compatibility with systems that don't support BLAKE2b.
///
/// Note: For security-critical applications, prefer HMAC-SHA-512 or BLAKE2b
/// which natively produce 512-bit (64-byte) outputs.
fn sha256_padded(entity_json: &str, _parent_entropy: &[u8]) -> Result<[u8; 64]> {
    use sha2::{Digest, Sha256};

    // Canonicalize JSON for deterministic hashing
    let canonical = canonicalize_json(entity_json)?;

    // SHA-256 hash (32 bytes)
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    let hash_32 = hasher.finalize();

    // Pad to 64 bytes with zeros
    let mut output = [0u8; 64];
    output[..32].copy_from_slice(&hash_32);
    // Remaining 32 bytes stay as zeros

    Ok(output)
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
            serde_json::to_string(&value).map_err(|e| {
                BipKeychainError::HashError(format!("JSON serialization error: {}", e))
            })
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
