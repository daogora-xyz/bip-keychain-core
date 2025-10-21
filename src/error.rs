//! Error types for BIP-Keychain operations

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BipKeychainError {
    #[error("Invalid entity JSON: {0}")]
    InvalidEntity(#[from] serde_json::Error),

    #[error("Hash function error: {0}")]
    HashError(String),

    #[error("BIP-32 derivation error: {0}")]
    Bip32Error(String),

    #[error("Invalid seed phrase: {0}")]
    InvalidSeedPhrase(String),

    #[error("Output format error: {0}")]
    OutputError(String),
}

pub type Result<T> = std::result::Result<T, BipKeychainError>;
