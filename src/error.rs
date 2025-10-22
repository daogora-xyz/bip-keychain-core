//! Error types for BIP-Keychain operations
//!
//! Provides detailed, actionable error messages to help users debug issues.

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BipKeychainError {
    /// Entity JSON parsing failed
    ///
    /// This usually means:
    /// - Invalid JSON syntax
    /// - Missing required fields (schema_type, entity, derivation_config)
    /// - Incorrect field types
    #[error("Invalid entity JSON: {0}\n\nHelp: Ensure your JSON has:\n  - schema_type (string)\n  - entity (object)\n  - derivation_config (object with hash_function and hardened)")]
    InvalidEntity(#[from] serde_json::Error),

    /// Hash function error
    ///
    /// This indicates a problem during cryptographic hashing.
    #[error("Hash function error: {0}\n\nHelp: This is usually an internal error. Please report if it persists.")]
    HashError(String),

    /// BIP-32 key derivation error
    ///
    /// This means the HD key derivation failed, possibly due to:
    /// - Invalid seed phrase
    /// - Derivation index out of range
    /// - Internal BIP-32 library error
    #[error("BIP-32 derivation error: {0}\n\nHelp: Verify your seed phrase is a valid BIP-39 mnemonic (12-24 words).")]
    Bip32Error(String),

    /// Invalid or malformed BIP-39 seed phrase
    ///
    /// The seed phrase must be:
    /// - 12, 15, 18, 21, or 24 words
    /// - Words from the BIP-39 wordlist
    /// - Valid checksum
    #[error("Invalid seed phrase: {0}\n\nHelp: BIP-39 seed phrases must be:\n  - 12, 15, 18, 21, or 24 words\n  - Words from the official BIP-39 wordlist\n  - Have a valid checksum\n\nFor testing, use: abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about")]
    InvalidSeedPhrase(String),

    /// Key output formatting error
    ///
    /// This indicates a problem converting the derived key to the requested format.
    #[error("Output format error: {0}\n\nHelp: Supported formats:\n  - seed (raw 32-byte seed as hex)\n  - public-key (Ed25519 public key as hex)\n  - private-key (Ed25519 private key as hex)\n  - ssh (OpenSSH public key format)\n  - json (complete JSON with all keys)")]
    OutputError(String),

    /// General I/O error
    ///
    /// File system operations failed (reading entity JSON, etc.)
    #[error("I/O error: {0}\n\nHelp: Check that:\n  - The file exists\n  - You have read permissions\n  - The path is correct")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, BipKeychainError>;
