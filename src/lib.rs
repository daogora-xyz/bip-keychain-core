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
// pub mod entity;          // TODO: Implement in Phase 2
pub mod hash;
// pub mod derivation;      // TODO: Implement in Phase 2
// pub mod bip32_wrapper;   // TODO: Implement in Phase 2
// pub mod output;          // TODO: Implement in Phase 2
pub mod error;

// Re-exports for convenience
// pub use entity::{Entity, KeyDerivation};
pub use hash::{HashFunction, hash_entity};
// pub use derivation::derive_key_from_entity;
// pub use bip32_wrapper::Keychain;
// pub use output::{OutputFormat, format_key};
pub use error::BipKeychainError;

// Also expose hash module for tests
pub use hash;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
