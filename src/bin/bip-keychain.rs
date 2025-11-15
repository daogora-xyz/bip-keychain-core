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

    /// Backup seed using SSKR (Shamir's Secret Sharing)
    ///
    /// Splits a BIP-39 seed into N shares where M are required to recover.
    /// Outputs shares as hex-encoded files for distribution to trusted parties.
    ///
    /// Example: 2-of-3 backup (distribute 3 shares, any 2 can recover)
    ///   bip-keychain backup-seed --groups 3 --threshold 2 --output-dir ./shares
    #[cfg(feature = "bc")]
    BackupSeed {
        /// Total number of shares to generate (2-16)
        #[arg(short = 'n', long, default_value = "3")]
        groups: u8,

        /// Number of shares required to recover (1-groups)
        #[arg(short = 't', long, default_value = "2")]
        threshold: u8,

        /// Output directory for share files
        #[arg(short = 'o', long, default_value = "./sskr-shares")]
        output_dir: PathBuf,
    },

    /// Recover seed from SSKR shares
    ///
    /// Combines M-of-N SSKR shares to recover the original seed phrase.
    ///
    /// Example:
    ///   bip-keychain recover-seed share-1.hex share-2.hex
    #[cfg(feature = "bc")]
    RecoverSeed {
        /// Paths to share files (hex-encoded)
        #[arg(value_name = "SHARE_FILES", required = true)]
        share_files: Vec<PathBuf>,
    },

    /// Decode single-part UR string
    ///
    /// Decodes a UR-encoded entity or public key from airgapped transfer.
    ///
    /// Example:
    ///   bip-keychain decode-ur "ur:crypto-entity/..."
    #[cfg(feature = "bc")]
    DecodeUr {
        /// UR string to decode
        #[arg(value_name = "UR_STRING")]
        ur_string: String,

        /// Output file for decoded entity JSON (stdout if not specified)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
    },

    /// Decode multi-part UR sequence (fountain codes)
    ///
    /// Decodes an animated QR sequence by collecting UR parts from files.
    /// Parts can be in any order and some can be missing - fountain codes
    /// will reconstruct the original data.
    ///
    /// Example:
    ///   bip-keychain decode-ur-animated part-*.txt
    #[cfg(feature = "bc")]
    DecodeUrAnimated {
        /// Files containing UR parts (one UR string per file)
        #[arg(value_name = "UR_PART_FILES", required = true)]
        part_files: Vec<PathBuf>,

        /// Output file for decoded entity JSON (stdout if not specified)
        #[arg(short = 'o', long)]
        output: Option<PathBuf>,
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
    /// UR-encoded entity (for airgapped transfer)
    #[cfg(feature = "bc")]
    UrEntity,
    /// UR-encoded public key (for returning from airgapped)
    #[cfg(feature = "bc")]
    UrPubkey,
    /// QR code with UR-encoded entity
    #[cfg(feature = "bc")]
    QrEntity,
    /// QR code with UR-encoded public key
    #[cfg(feature = "bc")]
    QrPubkey,
    /// Animated QR code sequence (fountain codes for large entities)
    #[cfg(feature = "bc")]
    QrAnimated,
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
            #[cfg(feature = "bc")]
            CliOutputFormat::UrEntity => OutputFormat::UrEntity,
            #[cfg(feature = "bc")]
            CliOutputFormat::UrPubkey => OutputFormat::UrPubkey,
            #[cfg(feature = "bc")]
            CliOutputFormat::QrEntity => OutputFormat::QrEntity,
            #[cfg(feature = "bc")]
            CliOutputFormat::QrPubkey => OutputFormat::QrPubkey,
            #[cfg(feature = "bc")]
            CliOutputFormat::QrAnimated => OutputFormat::QrEntityAnimated,
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
        #[cfg(feature = "bc")]
        Commands::BackupSeed {
            groups,
            threshold,
            output_dir,
        } => backup_seed_command(groups, threshold, output_dir),
        #[cfg(feature = "bc")]
        Commands::RecoverSeed { share_files } => recover_seed_command(share_files),
        #[cfg(feature = "bc")]
        Commands::DecodeUr { ur_string, output } => decode_ur_command(ur_string, output),
        #[cfg(feature = "bc")]
        Commands::DecodeUrAnimated { part_files, output } => {
            decode_ur_animated_command(part_files, output)
        }
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
    eprintln!("  bip-keychain backup-seed --help");
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

#[cfg(feature = "bc")]
fn backup_seed_command(groups: u8, threshold: u8, output_dir: PathBuf) -> Result<()> {
    use bip39::Mnemonic;
    use bip_keychain::sskr::{shard_seed, SskrPolicy};

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  SSKR Seed Backup - Shamir's Secret Sharing");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();

    // Create SSKR policy
    let policy = SskrPolicy::new(groups, threshold)
        .context("Failed to create SSKR policy")?;

    eprintln!("Policy: {}-of-{} shares", threshold, groups);
    eprintln!("  • Total shares: {}", groups);
    eprintln!("  • Required to recover: {}", threshold);
    eprintln!();

    // Get seed phrase from environment variable
    let seed_phrase = env::var("BIP_KEYCHAIN_SEED").context(
        "BIP_KEYCHAIN_SEED environment variable not set.\n\
         Set your BIP-39 seed phrase: export BIP_KEYCHAIN_SEED=\"your twelve word phrase...\"\n\
         \n\
         For security reasons, seed phrases must be passed via environment variable.",
    )?;

    // Parse mnemonic to get seed entropy
    let mnemonic = Mnemonic::parse(&seed_phrase)
        .context("Failed to parse seed phrase. Ensure it's a valid BIP-39 mnemonic.")?;

    let seed_entropy = mnemonic.to_entropy();

    eprintln!("Seed entropy: {} bytes ({} bits)", seed_entropy.len(), seed_entropy.len() * 8);
    eprintln!();

    // Shard the seed
    eprintln!("Generating SSKR shares...");
    let shares = shard_seed(&seed_entropy, &policy)
        .context("Failed to shard seed")?;

    eprintln!("✓ Generated {} shares", shares.len());
    eprintln!();

    // Create output directory
    std::fs::create_dir_all(&output_dir)
        .with_context(|| format!("Failed to create output directory: {}", output_dir.display()))?;

    eprintln!("Writing shares to: {}", output_dir.display());
    eprintln!();

    // Write shares to files
    for (idx, share) in shares.iter().enumerate() {
        let share_num = idx + 1;
        let filename = format!("share-{:02}-of-{:02}.hex", share_num, groups);
        let filepath = output_dir.join(&filename);

        let hex_share = hex::encode(share);
        std::fs::write(&filepath, &hex_share)
            .with_context(|| format!("Failed to write share file: {}", filepath.display()))?;

        eprintln!("  ✓ {} ({} bytes)", filename, share.len());
    }

    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  DISTRIBUTION INSTRUCTIONS");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    eprintln!("1. DISTRIBUTE shares to {} trusted parties/locations", groups);
    eprintln!("2. Any {} shares can RECOVER the seed", threshold);
    eprintln!("3. Store shares SEPARATELY (different physical locations)");
    eprintln!("4. NEVER store all shares together or in one location");
    eprintln!();
    eprintln!("Use cases:");
    eprintln!("  • 2-of-3: Personal backup (family, trusted friends, safe deposit box)");
    eprintln!("  • 3-of-5: Enterprise backup (require 3 of 5 executives)");
    eprintln!("  • 2-of-2: Couples/partners (both required)");
    eprintln!();
    eprintln!("To recover:");
    eprintln!("  bip-keychain recover-seed share-*.hex");
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

#[cfg(feature = "bc")]
fn recover_seed_command(share_files: Vec<PathBuf>) -> Result<()> {
    use bip39::Mnemonic;
    use bip_keychain::sskr::recover_seed;

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  SSKR Seed Recovery");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();

    eprintln!("Loading {} share files...", share_files.len());
    eprintln!();

    // Read all share files
    let mut shares: Vec<Vec<u8>> = Vec::new();
    for share_file in share_files.iter() {
        let hex_share = std::fs::read_to_string(share_file)
            .with_context(|| format!("Failed to read share file: {}", share_file.display()))?;

        let share_bytes = hex::decode(hex_share.trim())
            .with_context(|| format!("Failed to decode hex from: {}", share_file.display()))?;

        eprintln!("  ✓ {} ({} bytes)", share_file.display(), share_bytes.len());
        shares.push(share_bytes);
    }

    eprintln!();
    eprintln!("Recovering seed from shares...");

    // Recover the seed
    let recovered_entropy = recover_seed(&shares)
        .context("Failed to recover seed from shares. Ensure you have enough shares (threshold).")?;

    eprintln!("✓ Seed recovered successfully");
    eprintln!();

    // Convert entropy back to mnemonic
    let mnemonic = Mnemonic::from_entropy(&recovered_entropy)
        .context("Failed to create mnemonic from recovered entropy")?;

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  RECOVERED SEED PHRASE");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    println!("{}", mnemonic);
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  SECURITY REMINDER");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    eprintln!("WRITE DOWN this seed phrase on paper immediately.");
    eprintln!("NEVER store digitally or share with anyone.");
    eprintln!();
    eprintln!("To use:");
    eprintln!("  export BIP_KEYCHAIN_SEED=\"<your recovered phrase>\"");
    eprintln!("  bip-keychain derive entity.json");
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

#[cfg(feature = "bc")]
fn decode_ur_command(ur_string: String, output: Option<PathBuf>) -> Result<()> {
    use bip_keychain::output::ur;

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  UR Decoder - Single-part");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();

    // Try to decode as entity
    match ur::decode_entity(&ur_string) {
        Ok(entity) => {
            let json = entity.entity_json()?;
            let json_str = serde_json::to_string_pretty(&json)?;

            if let Some(output_path) = output {
                std::fs::write(&output_path, &json_str)
                    .with_context(|| format!("Failed to write to {}", output_path.display()))?;
                eprintln!("✓ Decoded entity written to: {}", output_path.display());
            } else {
                println!("{}", json_str);
            }

            eprintln!();
            eprintln!("Decoded entity:");
            eprintln!("  Schema type: {:?}", entity.schema_type);
            eprintln!("  Hash function: {:?}", entity.derivation_config.hash_function);
            if let Some(purpose) = &entity.purpose {
                eprintln!("  Purpose: {}", purpose);
            }

            Ok(())
        }
        Err(_) => {
            // Try to decode as public key
            match ur::decode_pubkey(&ur_string) {
                Ok(pubkey) => {
                    let hex_pubkey = hex::encode(pubkey);

                    if let Some(output_path) = output {
                        std::fs::write(&output_path, &hex_pubkey)
                            .with_context(|| format!("Failed to write to {}", output_path.display()))?;
                        eprintln!("✓ Decoded public key written to: {}", output_path.display());
                    } else {
                        println!("{}", hex_pubkey);
                    }

                    eprintln!();
                    eprintln!("Decoded public key:");
                    eprintln!("  {}", hex_pubkey);

                    Ok(())
                }
                Err(e) => {
                    anyhow::bail!("Failed to decode UR as entity or public key: {}", e)
                }
            }
        }
    }
}

#[cfg(feature = "bc")]
fn decode_ur_animated_command(part_files: Vec<PathBuf>, output: Option<PathBuf>) -> Result<()> {
    use bip_keychain::output::ur;

    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  UR Decoder - Multi-part (Fountain Codes)");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();

    eprintln!("Loading {} UR part files...", part_files.len());
    eprintln!();

    // Read all UR parts from files
    let mut ur_parts = Vec::new();
    for part_file in &part_files {
        let part_content = std::fs::read_to_string(part_file)
            .with_context(|| format!("Failed to read {}", part_file.display()))?;

        // Trim whitespace and add to parts
        let part = part_content.trim().to_string();
        ur_parts.push(part);
        eprintln!("  ✓ {} ({} bytes)", part_file.display(), part_content.len());
    }

    eprintln!();
    eprintln!("Decoding with fountain codes...");

    // Decode using fountain codes
    let entity = ur::decode_entity_animated(&ur_parts)
        .context("Failed to decode animated UR sequence")?;

    // Output the decoded entity
    let json = entity.entity_json()?;
    let json_str = serde_json::to_string_pretty(&json)?;

    if let Some(output_path) = output {
        std::fs::write(&output_path, &json_str)
            .with_context(|| format!("Failed to write to {}", output_path.display()))?;
        eprintln!();
        eprintln!("✓ Decoded entity written to: {}", output_path.display());
    } else {
        println!("{}", json_str);
    }

    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!("  Decode Summary");
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    eprintln!();
    eprintln!("Schema type: {:?}", entity.schema_type);
    eprintln!("Hash function: {:?}", entity.derivation_config.hash_function);
    if let Some(purpose) = &entity.purpose {
        eprintln!("Purpose: {}", purpose);
    }
    eprintln!();
    eprintln!("✓ Successfully decoded from {} parts", part_files.len());
    eprintln!();
    eprintln!("Fountain code efficiency:");
    eprintln!("  - Theoretical minimum: ~{} parts", (part_files.len() as f32 / 1.5) as usize);
    eprintln!("  - Parts provided: {}", part_files.len());
    eprintln!("  - Overhead: ~{:.1}%", ((part_files.len() as f32 / (part_files.len() as f32 / 1.5)) - 1.0) * 100.0);
    eprintln!();
    eprintln!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    Ok(())
}

