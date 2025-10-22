//! BIP-Keychain CLI tool
//!
//! Command-line interface for deriving cryptographic keys from semantic entities.

use anyhow::{Context, Result};
use bip_keychain::{derive_key_from_entity, Keychain, KeyDerivation};
use clap::{Parser, Subcommand};
use std::env;
use std::fs;
use std::path::PathBuf;

/// BIP-Keychain: Semantic hierarchical key derivation
///
/// Derives cryptographic keys from human-readable JSON entities
/// using BIP-32 hierarchical deterministic key derivation.
#[derive(Parser)]
#[command(name = "bip-keychain")]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Derive a key from an entity JSON file
    ///
    /// Reads a Nickel-exported JSON entity and derives a cryptographic key
    /// using BIP-Keychain. The seed phrase must be provided via the
    /// BIP_KEYCHAIN_SEED environment variable for security.
    ///
    /// Example:
    ///   export BIP_KEYCHAIN_SEED="your twelve word seed phrase here..."
    ///   bip-keychain derive entity.json
    Derive {
        /// Path to entity JSON file (Nickel-exported)
        #[arg(value_name = "ENTITY_JSON")]
        entity_file: PathBuf,

        /// Parent entropy (hex encoded, optional)
        ///
        /// Used as HMAC key for HMAC-based hash functions.
        /// If not provided, uses a default value.
        #[arg(long, value_name = "HEX")]
        parent_entropy: Option<String>,

        /// Output format
        #[arg(long, value_enum, default_value = "hex")]
        format: OutputFormat,
    },

    /// Generate a new BIP-39 seed phrase
    ///
    /// Creates a cryptographically secure random mnemonic seed phrase.
    ///
    /// WARNING: Store this securely! Anyone with this phrase can derive all your keys.
    GenerateSeed {
        /// Number of words (12, 15, 18, 21, or 24)
        #[arg(short = 'w', long, default_value = "24")]
        words: usize,
    },
}

#[derive(Debug, Clone, clap::ValueEnum)]
enum OutputFormat {
    /// Hexadecimal encoding of Ed25519 seed
    Hex,
    /// JSON with metadata
    Json,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Derive {
            entity_file,
            parent_entropy,
            format,
        } => derive_command(entity_file, parent_entropy, format),
        Commands::GenerateSeed { words } => generate_seed_command(words),
    }
}

fn derive_command(
    entity_file: PathBuf,
    parent_entropy_hex: Option<String>,
    format: OutputFormat,
) -> Result<()> {
    // Read entity JSON file
    let entity_json = fs::read_to_string(&entity_file)
        .with_context(|| format!("Failed to read entity file: {}", entity_file.display()))?;

    // Parse entity
    let key_derivation = KeyDerivation::from_json(&entity_json)
        .context("Failed to parse entity JSON")?;

    // Get seed phrase from environment variable
    let seed_phrase = env::var("BIP_KEYCHAIN_SEED").context(
        "BIP_KEYCHAIN_SEED environment variable not set.\n\
         Set your BIP-39 seed phrase: export BIP_KEYCHAIN_SEED=\"your twelve word phrase...\"\n\
         \n\
         For security reasons, we require the seed phrase to be passed via environment variable\n\
         rather than command-line arguments (which would be visible in process listings)."
    )?;

    // Create keychain from seed phrase
    let keychain = Keychain::from_mnemonic(&seed_phrase)
        .context("Failed to create keychain from seed phrase.\n\
                  Ensure BIP_KEYCHAIN_SEED contains a valid BIP-39 mnemonic (12-24 words).")?;

    // Parse parent entropy (or use default)
    let parent_entropy = if let Some(hex_str) = parent_entropy_hex {
        hex::decode(&hex_str)
            .context("Failed to decode parent entropy hex string")?
    } else {
        // Default parent entropy (in production, this should be derived from the master seed)
        b"bip-keychain-default-entropy-32!".to_vec()
    };

    // Derive key
    let derived_key = derive_key_from_entity(&keychain, &key_derivation, &parent_entropy)
        .context("Failed to derive key from entity")?;

    // Extract Ed25519 seed
    let ed25519_seed = derived_key.to_seed();

    // Output based on format
    match format {
        OutputFormat::Hex => {
            println!("{}", hex::encode(ed25519_seed));
        }
        OutputFormat::Json => {
            let output = serde_json::json!({
                "ed25519_seed": hex::encode(ed25519_seed),
                "schema_type": key_derivation.schema_type,
                "hash_function": format!("{:?}", key_derivation.derivation_config.hash_function),
                "purpose": key_derivation.purpose,
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
    }

    Ok(())
}

fn generate_seed_command(words: usize) -> Result<()> {
    // For now, we'll skip the generate-seed command and focus on derive
    // The bip39 crate API varies by version, and we want to focus on the core functionality
    anyhow::bail!(
        "generate-seed command not yet implemented.\n\
         \n\
         For now, you can generate a seed phrase using any BIP-39 compatible tool:\n\
         - https://iancoleman.io/bip39/ (offline use recommended)\n\
         - `bitcoin-cli` with `-named createwallet`\n\
         - Hardware wallets (Ledger, Trezor, etc.)\n\
         \n\
         For testing, you can use the standard test mnemonic:\n\
         abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
    )
}
