//! Entity type definitions for BIP-Keychain
//!
//! Parses Nickel-exported JSON into type-safe Rust structs.
//! For MVP, we store the entity as a generic JSON value and will add
//! type-safe parsing for specific schema types later.

use crate::error::{BipKeychainError, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Hash function configuration for entity derivation
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum HashFunctionConfig {
    #[serde(rename = "hmac_sha512")]
    HmacSha512,
    #[serde(rename = "blake2b")]
    Blake2b,
    #[serde(rename = "sha256")]
    Sha256,
}

/// Derivation configuration
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct DerivationConfig {
    /// Hash function to use for entity→index conversion
    pub hash_function: HashFunctionConfig,

    /// Whether to use hardened derivation (default: true)
    pub hardened: bool,
}

/// A complete key derivation specification
///
/// This is the top-level struct that represents a Nickel-exported entity
/// ready for BIP-Keychain derivation.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct KeyDerivation {
    /// Schema type identifier (e.g., "schema_org", "did", "gordian_envelope")
    pub schema_type: String,

    /// The actual entity data (stored as generic JSON for now)
    /// In the future, this could be an enum with type-safe variants
    pub entity: Value,

    /// Derivation configuration
    pub derivation_config: DerivationConfig,

    /// Optional human-readable purpose/description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<String>,

    /// Optional additional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

impl KeyDerivation {
    /// Parse a KeyDerivation from JSON string
    pub fn from_json(json: &str) -> Result<Self> {
        serde_json::from_str(json).map_err(BipKeychainError::InvalidEntity)
    }

    /// Get the entity as a canonical JSON string for hashing
    pub fn entity_json(&self) -> Result<String> {
        serde_json::to_string(&self.entity).map_err(|e| {
            BipKeychainError::HashError(format!("Failed to serialize entity: {}", e))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_key_derivation() {
        let json = r#"{
            "schema_type": "schema_org",
            "entity": {"@type": "Thing"},
            "derivation_config": {
                "hash_function": "hmac_sha512",
                "hardened": true
            }
        }"#;

        let kd = KeyDerivation::from_json(json).unwrap();
        assert_eq!(kd.schema_type, "schema_org");
        assert_eq!(kd.derivation_config.hash_function, HashFunctionConfig::HmacSha512);
        assert_eq!(kd.derivation_config.hardened, true);
    }

    #[test]
    fn test_hash_function_config_deserialize() {
        let json = r#"{"hash_function": "blake2b", "hardened": false}"#;
        let config: DerivationConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.hash_function, HashFunctionConfig::Blake2b);
        assert_eq!(config.hardened, false);
    }
}
