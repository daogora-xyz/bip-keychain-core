//! BIP-32 wrapper for BIP-Keychain derivation
//!
//! Simplifies BIP-32 operations with sensible defaults for BIP-Keychain.
//! Derives keys at the path: m/83696968'/67797668'/{index}'

use crate::error::{BipKeychainError, Result};
use bip32::{DerivationPath, ExtendedKey, XPrv};
use bip39::Mnemonic;

/// BIP-Keychain path constants
///
/// BIP-Keychain uses the BIP-85 application number (83696968) as the first level,
/// then a BIP-Keychain specific application code (67797668) as the second level.
///
/// Path format: m/83696968'/67797668'/{entity_index}'
pub const BIP85_APP: u32 = 83696968;
pub const BIPKEYCHAIN_APP: u32 = 67797668;

/// Keychain wrapper for BIP-32 hierarchical deterministic key derivation
pub struct Keychain {
    /// Master extended private key derived from seed
    master_key: XPrv,
}

impl Keychain {
    /// Create a keychain from a BIP-39 mnemonic phrase
    ///
    /// # Example
    ///
    /// ```ignore
    /// let keychain = Keychain::from_mnemonic("your twelve word seed phrase...")?;
    /// ```
    pub fn from_mnemonic(phrase: &str) -> Result<Self> {
        // Parse the mnemonic phrase
        let mnemonic = Mnemonic::parse(phrase)
            .map_err(|e| BipKeychainError::InvalidSeedPhrase(format!("Invalid mnemonic: {}", e)))?;

        // Convert mnemonic to seed (no password)
        let seed = mnemonic.to_seed("");

        // Derive master key from seed
        let master_key = XPrv::new(&seed)
            .map_err(|e| BipKeychainError::Bip32Error(format!("Failed to derive master key: {}", e)))?;

        Ok(Self { master_key })
    }

    /// Derive a key at the BIP-Keychain path for a given entity index
    ///
    /// Derives at: m/83696968'/67797668'/{index}'
    ///
    /// All derivations are hardened (indicated by ') for security.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let keychain = Keychain::from_mnemonic("...")?;
    /// let derived = keychain.derive_bip_keychain_path(42)?;
    /// let seed = derived.to_seed();  // 32 bytes for Ed25519
    /// ```
    pub fn derive_bip_keychain_path(&self, entity_index: u32) -> Result<DerivedKey> {
        // Build derivation path: m/83696968'/67797668'/{entity_index}'
        // Note: bip32 crate uses hardened indices by adding 2^31
        let hardened_bip85 = BIP85_APP + (1 << 31);
        let hardened_bipkeychain = BIPKEYCHAIN_APP + (1 << 31);
        let hardened_index = entity_index + (1 << 31);

        // Derive step by step
        // m/83696968'
        let key_bip85 = self.master_key
            .derive_child(hardened_bip85.into())
            .map_err(|e| BipKeychainError::Bip32Error(format!("Failed to derive BIP-85 level: {}", e)))?;

        // m/83696968'/67797668'
        let key_bipkeychain = key_bip85
            .derive_child(hardened_bipkeychain.into())
            .map_err(|e| BipKeychainError::Bip32Error(format!("Failed to derive BIP-Keychain level: {}", e)))?;

        // m/83696968'/67797668'/{entity_index}'
        let derived_key = key_bipkeychain
            .derive_child(hardened_index.into())
            .map_err(|e| BipKeychainError::Bip32Error(format!("Failed to derive entity level: {}", e)))?;

        Ok(DerivedKey { key: derived_key })
    }

    /// Get a reference to the master extended key
    pub fn master_key(&self) -> &XPrv {
        &self.master_key
    }
}

/// A derived key at a specific BIP-Keychain path
pub struct DerivedKey {
    key: XPrv,
}

impl DerivedKey {
    /// Extract 32 bytes from the derived key as a seed for Ed25519
    ///
    /// This follows the BIP-85 pattern: use BIP-32 derivation to generate
    /// entropy, then use that entropy as a seed for other cryptographic operations.
    pub fn to_seed(&self) -> [u8; 32] {
        // Extract private key bytes (32 bytes)
        let private_key_bytes = self.key.private_key().to_bytes();

        let mut seed = [0u8; 32];
        seed.copy_from_slice(&private_key_bytes);
        seed
    }

    /// Get the raw bytes of the derived private key
    pub fn to_bytes(&self) -> Vec<u8> {
        self.key.to_bytes().to_vec()
    }

    /// Get the extended private key
    pub fn xprv(&self) -> &XPrv {
        &self.key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants() {
        assert_eq!(BIP85_APP, 83_696_968);
        assert_eq!(BIPKEYCHAIN_APP, 67_797_668);
    }

    #[test]
    fn test_from_mnemonic() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let keychain = Keychain::from_mnemonic(mnemonic)
            .expect("Should create keychain from valid mnemonic");

        // Should have master key
        assert!(keychain.master_key().private_key().to_bytes().len() == 32);
    }

    #[test]
    fn test_deterministic() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

        let keychain1 = Keychain::from_mnemonic(mnemonic).unwrap();
        let keychain2 = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived1 = keychain1.derive_bip_keychain_path(42).unwrap();
        let derived2 = keychain2.derive_bip_keychain_path(42).unwrap();

        assert_eq!(derived1.to_bytes(), derived2.to_bytes());
    }

    #[test]
    fn test_different_indices() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived_0 = keychain.derive_bip_keychain_path(0).unwrap();
        let derived_1 = keychain.derive_bip_keychain_path(1).unwrap();

        assert_ne!(derived_0.to_bytes(), derived_1.to_bytes());
    }

    #[test]
    fn test_seed_extraction() {
        let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
        let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

        let derived = keychain.derive_bip_keychain_path(0).unwrap();
        let seed = derived.to_seed();

        // Should be exactly 32 bytes for Ed25519
        assert_eq!(seed.len(), 32);
    }
}
