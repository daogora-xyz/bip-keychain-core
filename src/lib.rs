//! BIP-Keychain: Multi-schema semantic hierarchical key derivation
//!
//! This library implements the BIP-Keychain specification with extensions
//! for multiple schema types beyond schema.org (Blockchain Commons, DIDs, etc.).
//!
//! # Example
//!
//! ```ignore
//! use bip_keychain::{Keychain, Entity, HashFunction};
//!
//! let keychain = Keychain::from_mnemonic("your twelve word seed phrase here...")?;
//! let entity = Entity::from_json(entity_json)?;
//! let key = keychain.derive_from_entity(&entity, HashFunction::HmacSha512)?;
//! ```

// Module declarations
pub mod bip32_wrapper;
pub mod derivation;
pub mod entity;
pub mod error;
pub mod hash;
pub mod output;

// Re-exports for convenience
pub use bip32_wrapper::{DerivedKey, Keychain};
pub use derivation::derive_key_from_entity;
pub use entity::{DerivationConfig, HashFunctionConfig, KeyDerivation};
pub use error::BipKeychainError;
pub use hash::{hash_entity, HashFunction};
pub use output::{format_key, Ed25519Keypair, OutputFormat};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        // Verify VERSION constant is correctly set from Cargo.toml
        assert_eq!(VERSION, "0.1.0");
    }
}
