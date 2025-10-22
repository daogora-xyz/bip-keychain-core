//! Output formatting for derived keys
//!
//! Converts BIP-Keychain derived seeds into usable key formats:
//! - Ed25519 keypairs (public + private keys)
//! - SSH public key format (OpenSSH)
//! - Raw hex encoding
//! - JSON with metadata

use crate::{bip32_wrapper::DerivedKey, entity::KeyDerivation, error::Result};
use ed25519_dalek::{SigningKey, VerifyingKey};

/// Output format options
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// Raw 32-byte seed as hex
    HexSeed,
    /// Ed25519 public key as hex
    Ed25519PublicHex,
    /// Ed25519 private key as hex (dangerous!)
    Ed25519PrivateHex,
    /// OpenSSH public key format
    SshPublicKey,
    /// GPG-compatible public key info (for manual import)
    GpgPublicKey,
    /// JSON with all key data
    Json,
}

/// A complete Ed25519 keypair derived from BIP-Keychain
pub struct Ed25519Keypair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
}

impl Ed25519Keypair {
    /// Generate Ed25519 keypair from a 32-byte seed
    pub fn from_seed(seed: [u8; 32]) -> Self {
        let signing_key = SigningKey::from_bytes(&seed);
        let verifying_key = signing_key.verifying_key();

        Self {
            signing_key,
            verifying_key,
        }
    }

    /// Generate keypair from a DerivedKey
    pub fn from_derived_key(derived: &DerivedKey) -> Self {
        Self::from_seed(derived.to_seed())
    }

    /// Get the public key bytes (32 bytes)
    pub fn public_key_bytes(&self) -> [u8; 32] {
        self.verifying_key.to_bytes()
    }

    /// Get the private key bytes (32 bytes)
    pub fn private_key_bytes(&self) -> [u8; 32] {
        self.signing_key.to_bytes()
    }

    /// Get the signing key reference (for creating signatures)
    pub fn signing_key(&self) -> &SigningKey {
        &self.signing_key
    }

    /// Get the verifying key reference (for verification)
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.verifying_key
    }

    /// Format as OpenSSH public key
    ///
    /// Format: `ssh-ed25519 <base64> <comment>`
    pub fn to_ssh_public_key(&self, comment: Option<&str>) -> String {
        // SSH wire format for Ed25519:
        // - 4 bytes: length of "ssh-ed25519" (11 bytes)
        // - 11 bytes: "ssh-ed25519"
        // - 4 bytes: length of public key (32 bytes)
        // - 32 bytes: public key

        let mut ssh_blob = Vec::new();

        // Algorithm name length and value
        let algo = b"ssh-ed25519";
        ssh_blob.extend_from_slice(&(algo.len() as u32).to_be_bytes());
        ssh_blob.extend_from_slice(algo);

        // Public key length and value
        let pubkey = self.public_key_bytes();
        ssh_blob.extend_from_slice(&(pubkey.len() as u32).to_be_bytes());
        ssh_blob.extend_from_slice(&pubkey);

        // Base64 encode the blob
        let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, ssh_blob);

        // Format with comment
        let comment_str = comment.unwrap_or("bip-keychain");
        format!("ssh-ed25519 {} {}", encoded, comment_str)
    }

    /// Format as OpenSSH private key
    ///
    /// Note: This is a simplified format. Real OpenSSH private keys have more structure.
    /// For production use, consider using `ssh-keygen` compatible libraries.
    pub fn to_ssh_private_key_warning(&self) -> String {
        format!(
            "Warning: Private key export not fully implemented.\n\
             Private key (raw hex): {}\n\
             \n\
             To use with SSH:\n\
             1. Use a proper SSH key generation library\n\
             2. Or convert this key using ssh-keygen tools\n\
             3. Never expose private keys in plain text!",
            hex::encode(self.private_key_bytes())
        )
    }

    /// Format as GPG-compatible public key information
    ///
    /// Provides the Ed25519 public key in a format that can be imported into GPG.
    /// Note: This provides key material for manual import, not a full OpenPGP packet.
    pub fn to_gpg_public_key(&self, comment: Option<&str>) -> String {
        let pubkey_hex = hex::encode(self.public_key_bytes());
        let comment_str = comment.unwrap_or("bip-keychain");

        format!(
            "GPG Ed25519 Public Key\n\
             =====================\n\
             Comment: {}\n\
             \n\
             Public Key (hex, 32 bytes):\n\
             {}\n\
             \n\
             To import into GPG:\n\
             1. Save this key material\n\
             2. Use: gpg --import (if in OpenPGP format)\n\
             3. Or use: gpg --expert --full-gen-key to create key from seed\n\
             \n\
             For Git signing:\n\
             1. Import/create GPG key\n\
             2. git config --global user.signingkey <KEY-ID>\n\
             3. git config --global commit.gpgsign true\n\
             \n\
             Note: Full OpenPGP packet format not yet implemented.\n\
             See GIT-SIGNING-GUIDE.md for detailed instructions.",
            comment_str, pubkey_hex
        )
    }
}

/// Format a derived key according to the specified output format
pub fn format_key(
    derived: &DerivedKey,
    key_derivation: &KeyDerivation,
    format: OutputFormat,
) -> Result<String> {
    match format {
        OutputFormat::HexSeed => {
            // Just the raw 32-byte seed
            Ok(hex::encode(derived.to_seed()))
        }

        OutputFormat::Ed25519PublicHex => {
            // Ed25519 public key as hex
            let keypair = Ed25519Keypair::from_derived_key(derived);
            Ok(hex::encode(keypair.public_key_bytes()))
        }

        OutputFormat::Ed25519PrivateHex => {
            // Ed25519 private key as hex (dangerous!)
            let keypair = Ed25519Keypair::from_derived_key(derived);
            Ok(hex::encode(keypair.private_key_bytes()))
        }

        OutputFormat::SshPublicKey => {
            // OpenSSH public key format
            let keypair = Ed25519Keypair::from_derived_key(derived);
            let comment = key_derivation
                .purpose
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("bip-keychain");
            Ok(keypair.to_ssh_public_key(Some(comment)))
        }

        OutputFormat::GpgPublicKey => {
            // GPG public key information
            let keypair = Ed25519Keypair::from_derived_key(derived);
            let comment = key_derivation
                .purpose
                .as_ref()
                .map(|s| s.as_str())
                .unwrap_or("bip-keychain");
            Ok(keypair.to_gpg_public_key(Some(comment)))
        }

        OutputFormat::Json => {
            // JSON with all metadata
            let keypair = Ed25519Keypair::from_derived_key(derived);
            let seed = derived.to_seed();

            let json = serde_json::json!({
                "seed_hex": hex::encode(seed),
                "ed25519_public_key": hex::encode(keypair.public_key_bytes()),
                "ed25519_private_key": hex::encode(keypair.private_key_bytes()),
                "ssh_public_key": keypair.to_ssh_public_key(
                    key_derivation.purpose.as_ref().map(|s| s.as_str())
                ),
                "schema_type": key_derivation.schema_type,
                "hash_function": format!("{:?}", key_derivation.derivation_config.hash_function),
                "purpose": key_derivation.purpose,
            });

            Ok(serde_json::to_string_pretty(&json)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_keypair_generation() {
        let seed = [0u8; 32]; // All zeros for test
        let keypair = Ed25519Keypair::from_seed(seed);

        // Should have valid 32-byte keys
        assert_eq!(keypair.public_key_bytes().len(), 32);
        assert_eq!(keypair.private_key_bytes().len(), 32);
    }

    #[test]
    fn test_ed25519_deterministic() {
        let seed = [42u8; 32];

        let keypair1 = Ed25519Keypair::from_seed(seed);
        let keypair2 = Ed25519Keypair::from_seed(seed);

        assert_eq!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
        assert_eq!(keypair1.private_key_bytes(), keypair2.private_key_bytes());
    }

    #[test]
    fn test_ssh_public_key_format() {
        let seed = [1u8; 32];
        let keypair = Ed25519Keypair::from_seed(seed);

        let ssh_key = keypair.to_ssh_public_key(Some("test-key"));

        // Should start with ssh-ed25519
        assert!(ssh_key.starts_with("ssh-ed25519 "));

        // Should end with comment
        assert!(ssh_key.ends_with(" test-key"));

        // Should have base64 in the middle
        let parts: Vec<&str> = ssh_key.split_whitespace().collect();
        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "ssh-ed25519");
        assert_eq!(parts[2], "test-key");
    }

    #[test]
    fn test_different_seeds_different_keys() {
        let seed1 = [1u8; 32];
        let seed2 = [2u8; 32];

        let keypair1 = Ed25519Keypair::from_seed(seed1);
        let keypair2 = Ed25519Keypair::from_seed(seed2);

        assert_ne!(keypair1.public_key_bytes(), keypair2.public_key_bytes());
        assert_ne!(keypair1.private_key_bytes(), keypair2.private_key_bytes());
    }
}
