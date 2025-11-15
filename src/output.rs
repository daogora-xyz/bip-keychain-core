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
    /// UR-encoded entity definition (for airgapped transfer)
    #[cfg(feature = "bc")]
    UrEntity,
    /// UR-encoded public key only (for returning from airgapped)
    #[cfg(feature = "bc")]
    UrPubkey,
    /// QR code containing UR-encoded entity
    #[cfg(feature = "bc")]
    QrEntity,
    /// QR code containing UR-encoded public key
    #[cfg(feature = "bc")]
    QrPubkey,
    /// Animated QR code sequence for large entities (fountain codes)
    #[cfg(feature = "bc")]
    QrEntityAnimated,
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
            let comment = key_derivation.purpose.as_deref().unwrap_or("bip-keychain");
            Ok(keypair.to_ssh_public_key(Some(comment)))
        }

        OutputFormat::GpgPublicKey => {
            // GPG public key information
            let keypair = Ed25519Keypair::from_derived_key(derived);
            let comment = key_derivation.purpose.as_deref().unwrap_or("bip-keychain");
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
                    key_derivation.purpose.as_deref()
                ),
                "schema_type": key_derivation.schema_type,
                "hash_function": format!("{:?}", key_derivation.derivation_config.hash_function),
                "purpose": key_derivation.purpose,
            });

            Ok(serde_json::to_string_pretty(&json)?)
        }

        #[cfg(feature = "bc")]
        OutputFormat::UrEntity => {
            // UR-encoded entity definition (for sending to airgapped machine)
            ur::encode_entity(key_derivation)
        }

        #[cfg(feature = "bc")]
        OutputFormat::UrPubkey => {
            // UR-encoded public key only (for returning from airgapped)
            let keypair = Ed25519Keypair::from_derived_key(derived);
            ur::encode_pubkey(&keypair.public_key_bytes())
        }

        #[cfg(feature = "bc")]
        OutputFormat::QrEntity => {
            // QR code with UR-encoded entity
            let ur_string = ur::encode_entity(key_derivation)?;
            ur::generate_qr(&ur_string)
        }

        #[cfg(feature = "bc")]
        OutputFormat::QrPubkey => {
            // QR code with UR-encoded public key
            let keypair = Ed25519Keypair::from_derived_key(derived);
            let ur_string = ur::encode_pubkey(&keypair.public_key_bytes())?;
            ur::generate_qr(&ur_string)
        }

        #[cfg(feature = "bc")]
        OutputFormat::QrEntityAnimated => {
            // Animated QR code sequence using fountain codes
            // This generates multiple QR frames for larger entities
            let ur_parts = ur::encode_entity_animated(key_derivation, 200, 0)?;
            let qr_frames = ur::generate_animated_qr(&ur_parts)?;

            // Display the animated sequence
            // Note: This blocks and loops forever until Ctrl+C
            ur::display_animated_qr(&qr_frames, 500)?;

            // Never reached due to infinite loop, but needed for type
            Ok(String::new())
        }
    }
}

/// Blockchain Commons UR encoding support
#[cfg(feature = "bc")]
pub mod ur {
    use crate::{
        entity::KeyDerivation,
        error::{BipKeychainError, Result},
    };
    use bc_ur::prelude::*;

    /// Encode entity definition as UR string
    ///
    /// This creates a UR that can be transferred to an airgapped machine
    /// via QR code for secure key derivation.
    pub fn encode_entity(key_derivation: &KeyDerivation) -> Result<String> {
        use dcbor::prelude::*;

        // Serialize entire KeyDerivation struct as JSON bytes
        let json_bytes = serde_json::to_vec(key_derivation).map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to serialize entity: {}", e))
        })?;

        // Create CBOR byte string (prevents auto-conversion to CBOR array)
        let cbor = CBOR::to_byte_string(json_bytes);

        // Create UR with CBOR byte string
        let ur = UR::new("crypto-entity", cbor)
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to create UR: {:?}", e)))?;

        Ok(ur.string())
    }

    /// Encode Ed25519 public key as UR string
    ///
    /// This creates a UR for returning the public key from an airgapped machine.
    pub fn encode_pubkey(pubkey: &[u8; 32]) -> Result<String> {
        use dcbor::prelude::*;

        // Create CBOR byte string (prevents auto-conversion to CBOR array)
        let cbor = CBOR::to_byte_string(pubkey.to_vec());

        // Create UR with CBOR byte string
        let ur = UR::new("crypto-pubkey", cbor)
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to create UR: {:?}", e)))?;

        Ok(ur.string())
    }

    /// Generate ASCII QR code from UR string
    ///
    /// Returns a terminal-printable QR code that can be scanned with a camera.
    pub fn generate_qr(ur_string: &str) -> Result<String> {
        use qrcode::{render::unicode, QrCode};

        let code = QrCode::new(ur_string.as_bytes()).map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to generate QR code: {:?}", e))
        })?;

        let qr_string = code
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();

        Ok(format!(
            "Scan this QR code:\n\n{}\n\nUR: {}",
            qr_string, ur_string
        ))
    }

    /// Decode entity from UR string
    ///
    /// This parses a UR-encoded entity definition.
    pub fn decode_entity(ur_string: &str) -> Result<KeyDerivation> {
        use dcbor::prelude::*;

        let ur = UR::from_ur_string(ur_string)
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to parse UR: {:?}", e)))?;

        // Verify UR type
        if ur.ur_type_str() != "crypto-entity" {
            return Err(BipKeychainError::OutputError(format!(
                "Invalid UR type: expected crypto-entity, got {}",
                ur.ur_type_str()
            )));
        }

        // Extract CBOR byte string from UR
        use dcbor::prelude::*;
        let cbor = ur.cbor();
        let json_bytes = cbor.try_into_byte_string().map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to extract byte string from CBOR: {:?}", e))
        })?;

        // Parse JSON directly to KeyDerivation struct
        let key_derivation: KeyDerivation = serde_json::from_slice(&json_bytes).map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to decode entity JSON: {}", e))
        })?;

        Ok(key_derivation)
    }

    /// Decode Ed25519 public key from UR string
    pub fn decode_pubkey(ur_string: &str) -> Result<[u8; 32]> {
        let ur = UR::from_ur_string(ur_string)
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to parse UR: {:?}", e)))?;

        // Verify UR type
        if ur.ur_type_str() != "crypto-pubkey" {
            return Err(BipKeychainError::OutputError(format!(
                "Invalid UR type: expected crypto-pubkey, got {}",
                ur.ur_type_str()
            )));
        }

        // Extract CBOR byte string from UR
        use dcbor::prelude::*;
        let cbor = ur.cbor();
        let pubkey_bytes = cbor.try_into_byte_string().map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to extract byte string from CBOR: {:?}", e))
        })?;

        if pubkey_bytes.len() != 32 {
            return Err(BipKeychainError::OutputError(format!(
                "Invalid public key length: expected 32 bytes, got {}",
                pubkey_bytes.len()
            )));
        }

        let mut pubkey = [0u8; 32];
        pubkey.copy_from_slice(&pubkey_bytes);
        Ok(pubkey)
    }

    /// Encode entity as multi-part animated UR using fountain codes
    ///
    /// For larger entities that don't fit in a single QR code, this generates
    /// a sequence of UR parts using fountain codes. The receiver can reconstruct
    /// the original entity from any subset of parts (typically 1.5x the number of
    /// original fragments).
    ///
    /// # Arguments
    /// * `key_derivation` - Entity to encode
    /// * `max_fragment_len` - Maximum bytes per QR code fragment (default: 200)
    /// * `num_parts` - Number of QR code parts to generate (0 = infinite)
    ///
    /// # Returns
    /// Vector of UR strings, one per QR code frame
    #[cfg(feature = "bc")]
    pub fn encode_entity_animated(
        key_derivation: &KeyDerivation,
        max_fragment_len: usize,
        num_parts: usize,
    ) -> Result<Vec<String>> {
        use ur::Encoder;

        // Serialize entire KeyDerivation struct as JSON bytes
        let json_bytes = serde_json::to_vec(key_derivation).map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to serialize entity: {}", e))
        })?;

        // Create fountain encoder with raw JSON bytes
        let mut encoder = Encoder::bytes(&json_bytes, max_fragment_len)
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to create encoder: {:?}", e)))?;

        // Generate parts
        let mut parts = Vec::new();

        // Calculate recommended parts based on data size
        // Fountain codes need ~1.5x the minimum fragments for reliable decoding
        let min_fragments = (json_bytes.len() + max_fragment_len - 1) / max_fragment_len;
        let recommended_parts = if num_parts == 0 {
            (min_fragments as f32 * 1.5).ceil() as usize
        } else {
            num_parts
        };

        for _ in 0..recommended_parts {
            let part = encoder.next_part()
                .map_err(|e| BipKeychainError::OutputError(format!("Failed to generate part: {:?}", e)))?;
            parts.push(part);
        }

        Ok(parts)
    }

    /// Generate animated QR code sequence from multi-part UR
    ///
    /// Creates ASCII QR codes for each UR part, suitable for terminal animation.
    ///
    /// # Arguments
    /// * `ur_parts` - Vector of UR strings from encode_entity_animated()
    ///
    /// # Returns
    /// Vector of QR code strings, one per frame
    #[cfg(feature = "bc")]
    pub fn generate_animated_qr(ur_parts: &[String]) -> Result<Vec<String>> {
        use qrcode::{render::unicode, QrCode};

        let mut qr_frames = Vec::new();

        for (idx, ur_string) in ur_parts.iter().enumerate() {
            let code = QrCode::new(ur_string.as_bytes()).map_err(|e| {
                BipKeychainError::OutputError(format!("Failed to generate QR code: {:?}", e))
            })?;

            let qr_string = code
                .render::<unicode::Dense1x2>()
                .dark_color(unicode::Dense1x2::Light)
                .light_color(unicode::Dense1x2::Dark)
                .build();

            let frame = format!(
                "Frame {}/{}\n\n{}\n\nUR: {}",
                idx + 1,
                ur_parts.len(),
                qr_string,
                ur_string
            );
            qr_frames.push(frame);
        }

        Ok(qr_frames)
    }

    /// Display animated QR codes in terminal
    ///
    /// Cycles through QR code frames with configurable delay.
    /// Press Ctrl+C to stop.
    ///
    /// # Arguments
    /// * `qr_frames` - Vector of QR code strings from generate_animated_qr()
    /// * `frame_delay_ms` - Milliseconds to display each frame (default: 500)
    #[cfg(feature = "bc")]
    pub fn display_animated_qr(qr_frames: &[String], frame_delay_ms: u64) -> Result<()> {
        use std::io::Write;
        use std::thread;
        use std::time::Duration;

        if qr_frames.is_empty() {
            return Err(BipKeychainError::OutputError(
                "No QR frames to display".to_string(),
            ));
        }

        eprintln!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        eprintln!("  Animated QR Code - {} frames", qr_frames.len());
        eprintln!("  Press Ctrl+C to stop");
        eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

        loop {
            for frame in qr_frames {
                // Clear screen (ANSI escape codes)
                print!("\x1B[2J\x1B[1;1H");
                println!("{}", frame);
                std::io::stdout().flush().ok();

                thread::sleep(Duration::from_millis(frame_delay_ms));
            }
        }
    }

    /// Decode entity from multi-part UR sequence (fountain codes)
    ///
    /// Collects UR parts until enough fragments are received to reconstruct
    /// the original entity. Uses fountain decoding - parts can arrive in any
    /// order and some can be missed.
    ///
    /// # Arguments
    /// * `ur_parts` - Vector of UR strings from scanning QR codes
    ///
    /// # Returns
    /// Decoded entity once enough parts collected
    #[cfg(feature = "bc")]
    pub fn decode_entity_animated(ur_parts: &[String]) -> Result<KeyDerivation> {
        use ur::Decoder;

        if ur_parts.is_empty() {
            return Err(BipKeychainError::OutputError(
                "No UR parts provided for decoding".to_string(),
            ));
        }

        // Create decoder
        let mut decoder = Decoder::default();

        // Feed parts to decoder
        for (idx, part) in ur_parts.iter().enumerate() {
            decoder
                .receive(part)
                .map_err(|e| BipKeychainError::OutputError(format!("Failed to receive part {}: {:?}", idx + 1, e)))?;

            // Check if we have enough to decode
            if decoder.complete() {
                eprintln!("✓ Decoded successfully after {} parts", idx + 1);
                break;
            }
        }

        // Check if decoding completed
        if !decoder.complete() {
            return Err(BipKeychainError::OutputError(format!(
                "Insufficient parts to decode: received {}, need more fragments",
                ur_parts.len()
            )));
        }

        // Extract decoded message (raw JSON bytes from fountain decoder)
        let json_bytes = decoder.message()
            .map_err(|e| BipKeychainError::OutputError(format!("Failed to extract message: {:?}", e)))?
            .ok_or_else(|| BipKeychainError::OutputError("No message available from decoder".to_string()))?;

        // Parse JSON directly to KeyDerivation struct
        let key_derivation: KeyDerivation = serde_json::from_slice(&json_bytes).map_err(|e| {
            BipKeychainError::OutputError(format!("Failed to decode entity JSON: {}", e))
        })?;

        Ok(key_derivation)
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

    #[cfg(feature = "bc")]
    #[test]
    fn test_ur_encode_pubkey() {
        let pubkey = [42u8; 32];
        let ur_string = ur::encode_pubkey(&pubkey).expect("Should encode pubkey");

        // Should start with ur:crypto-pubkey
        assert!(ur_string.starts_with("ur:crypto-pubkey/"));

        // Should be decodable
        let decoded = ur::decode_pubkey(&ur_string).expect("Should decode pubkey");
        assert_eq!(decoded, pubkey);
    }

    #[cfg(feature = "bc")]
    #[test]
    fn test_ur_encode_entity() {
        use crate::entity::{DerivationConfig, HashFunctionConfig, KeyDerivation};

        let entity_json = r#"{
            "schema_type": "test",
            "entity": {"name": "test"},
            "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
        }"#;

        let key_derivation = KeyDerivation::from_json(entity_json).expect("Should parse entity");

        let ur_string = ur::encode_entity(&key_derivation).expect("Should encode entity");

        // Should start with ur:crypto-entity
        assert!(ur_string.starts_with("ur:crypto-entity/"));

        // Should be decodable
        let decoded = ur::decode_entity(&ur_string).expect("Should decode entity");

        assert_eq!(decoded.schema_type, key_derivation.schema_type);
        assert_eq!(
            decoded.derivation_config.hash_function,
            key_derivation.derivation_config.hash_function
        );
        assert_eq!(
            decoded.derivation_config.hardened,
            key_derivation.derivation_config.hardened
        );
    }

    #[cfg(feature = "bc")]
    #[test]
    fn test_qr_generation() {
        let pubkey = [123u8; 32];
        let ur_string = ur::encode_pubkey(&pubkey).expect("Should encode pubkey");
        let qr_output = ur::generate_qr(&ur_string).expect("Should generate QR");

        // Should contain the UR string
        assert!(qr_output.contains(&ur_string));
        // Should have QR code blocks
        assert!(qr_output.contains("█"));
    }
}
