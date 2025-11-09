//! BIP-Keychain CLI tool
//!
//! Command-line interface for deriving cryptographic keys from semantic entities.

use anyhow::{Context, Result};
use bip_keychain::{derive_key_from_entity, format_key, KeyDerivation, Keychain, OutputFormat};
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
        #[arg(long, value_enum, default_value = "ssh")]
        format: CliOutputFormat,
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
enum CliOutputFormat {
    /// Raw 32-byte seed as hex
    Seed,
    /// Ed25519 public key as hex
    PublicKey,
    /// Ed25519 private key as hex (use with caution!)
    PrivateKey,
    /// OpenSSH public key format (default, most useful)
    Ssh,
    /// GPG-compatible public key info (for Git signing)
    Gpg,
    /// JSON with all key data and metadata
    Json,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(cli_format: CliOutputFormat) -> Self {
        match cli_format {
            CliOutputFormat::Seed => OutputFormat::HexSeed,
            CliOutputFormat::PublicKey => OutputFormat::Ed25519PublicHex,
            CliOutputFormat::PrivateKey => OutputFormat::Ed25519PrivateHex,
            CliOutputFormat::Ssh => OutputFormat::SshPublicKey,
            CliOutputFormat::Gpg => OutputFormat::GpgPublicKey,
            CliOutputFormat::Json => OutputFormat::Json,
        }
    }
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
    format: CliOutputFormat,
) -> Result<()> {
    // Read entity JSON file
    let entity_json = fs::read_to_string(&entity_file)
        .with_context(|| format!("Failed to read entity file: {}", entity_file.display()))?;

    // Parse entity
    let key_derivation =
        KeyDerivation::from_json(&entity_json).context("Failed to parse entity JSON")?;

    // Get seed phrase from environment variable
    let seed_phrase = env::var("BIP_KEYCHAIN_SEED").context(
        "BIP_KEYCHAIN_SEED environment variable not set.\n\
         Set your BIP-39 seed phrase: export BIP_KEYCHAIN_SEED=\"your twelve word phrase...\"\n\
         \n\
         For security reasons, we require the seed phrase to be passed via environment variable\n\
         rather than command-line arguments (which would be visible in process listings).",
    )?;

    // Create keychain from seed phrase
    let keychain = Keychain::from_mnemonic(&seed_phrase).context(
        "Failed to create keychain from seed phrase.\n\
                  Ensure BIP_KEYCHAIN_SEED contains a valid BIP-39 mnemonic (12-24 words).",
    )?;

    // Parse parent entropy (or use default)
    let parent_entropy = if let Some(hex_str) = parent_entropy_hex {
        hex::decode(&hex_str).context("Failed to decode parent entropy hex string")?
    } else {
        // Default parent entropy (in production, this should be derived from the master seed)
        b"bip-keychain-default-entropy-32!".to_vec()
    };

    // Derive key
    let derived_key = derive_key_from_entity(&keychain, &key_derivation, &parent_entropy)
        .context("Failed to derive key from entity")?;

    // Format and output
    let output_format: OutputFormat = format.into();
    let output = format_key(&derived_key, &key_derivation, output_format)
        .context("Failed to format key output")?;

    println!("{}", output);

    Ok(())
}

fn generate_seed_command(words: usize) -> Result<()> {
    use bip39::Mnemonic;

    // Validate word count and calculate entropy size
    // BIP-39 spec: each word encodes 11 bits
    // Total bits = words * 11 = entropy bits + checksum bits
    let entropy_bytes = match words {
        12 => 16, // 128 bits entropy + 4 bits checksum = 132 bits / 11 = 12 words
        15 => 20, // 160 bits entropy + 5 bits checksum = 165 bits / 11 = 15 words
        18 => 24, // 192 bits entropy + 6 bits checksum = 198 bits / 11 = 18 words
        21 => 28, // 224 bits entropy + 7 bits checksum = 231 bits / 11 = 21 words
        24 => 32, // 256 bits entropy + 8 bits checksum = 264 bits / 11 = 24 words
        _ => anyhow::bail!(
            "Invalid word count: {}\n\
             \n\
             Word count must be one of: 12, 15, 18, 21, or 24\n\
             \n\
             Recommended:\n\
             - 24 words: Maximum security (256 bits entropy)\n\
             - 12 words: Good security, easier to write down (128 bits entropy)",
            words
        ),
    };

    // Generate cryptographically secure random entropy
    // Uses getrandom crate which uses OS-provided CSPRNG (ChaCha20, /dev/urandom, etc.)
    let mut entropy = vec![0u8; entropy_bytes];
    getrandom::getrandom(&mut entropy).context(
        "Failed to generate secure random entropy.\n\
                  This usually indicates a problem with the system's random number generator.",
    )?;

    // Create mnemonic from entropy
    let mnemonic =
        Mnemonic::from_entropy(&entropy).context("Failed to generate mnemonic from entropy")?;

    // Display the mnemonic
    println!("{}", mnemonic);

    // Print security warnings to stderr so they don't interfere with piping the mnemonic
    eprintln!();
    eprintln!("⚠️  SECURITY WARNING - READ CAREFULLY:");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    eprintln!("This seed phrase is the MASTER KEY to all derived keys.");
    eprintln!();
    eprintln!("DO:");
    eprintln!("  ✓ Write it down on paper immediately");
    eprintln!("  ✓ Store in a secure location (fireproof safe, safety deposit box)");
    eprintln!("  ✓ Consider making multiple copies in different secure locations");
    eprintln!("  ✓ Verify you wrote it correctly by re-reading");
    eprintln!();
    eprintln!("DO NOT:");
    eprintln!("  ✗ Store digitally (no screenshots, photos, or files)");
    eprintln!("  ✗ Share with anyone");
    eprintln!("  ✗ Send via email, messaging, or cloud storage");
    eprintln!("  ✗ Enter into any website or application (except wallet recovery)");
    eprintln!();
    eprintln!("LOSS = PERMANENT:");
    eprintln!("  • If you lose this seed phrase, you CANNOT recover your keys");
    eprintln!("  • If someone else gets this phrase, they can steal ALL your keys");
    eprintln!("  • There is NO password reset or customer support");
    eprintln!();
    eprintln!("For advanced backup, consider Shamir's Secret Sharing (SSKR):");
    eprintln!("  https://github.com/BlockchainCommons/bc-sskr");
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}
