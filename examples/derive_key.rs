//! Simple example demonstrating BIP-Keychain derivation
//!
//! This shows the complete flow:
//! 1. Parse entity JSON (schema.org)
//! 2. Create keychain from BIP-39 mnemonic
//! 3. Derive key using BIP-Keychain
//! 4. Extract Ed25519 seed and generate keypair
//!
//! Run with: cargo run --example derive_key

use bip_keychain::{Keychain, KeyDerivation, derive_key_from_entity};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== BIP-Keychain Derivation Example ===\n");

    // Example entity: GitHub repository signing key
    let entity_json = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/DAOgora-xyz/bip-keychain-core",
    "programmingLanguage": "Rust",
    "name": "BIP-Keychain Core"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Git commit signing key for bip-keychain-core repository"
}"#;

    println!("Entity JSON:");
    println!("{}\n", entity_json);

    // Parse the entity
    println!("Step 1: Parsing entity...");
    let key_derivation = KeyDerivation::from_json(entity_json)?;
    println!("✓ Parsed {} entity\n", key_derivation.schema_type);

    // Create keychain from mnemonic
    // NOTE: In production, NEVER hardcode mnemonics! Use environment variables.
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
    println!("Step 2: Creating keychain from mnemonic...");
    let keychain = Keychain::from_mnemonic(mnemonic)?;
    println!("✓ Keychain created\n");

    // Derive key
    // Parent entropy would typically come from a master seed or previous derivation
    let parent_entropy = b"example_parent_entropy_32bytes!!";
    println!("Step 3: Deriving key using BIP-Keychain...");
    let derived_key = derive_key_from_entity(&keychain, &key_derivation, parent_entropy)?;
    println!("✓ Key derived via BIP-32 path m/83696968'/67797668'/<index>'\n");

    // Extract Ed25519 seed
    println!("Step 4: Extracting Ed25519 seed...");
    let ed25519_seed = derived_key.to_seed();
    println!("✓ Ed25519 seed (32 bytes): {}\n", hex::encode(&ed25519_seed));

    // Show that derivation is deterministic
    println!("Step 5: Verifying determinism...");
    let derived_key_2 = derive_key_from_entity(&keychain, &key_derivation, parent_entropy)?;
    let ed25519_seed_2 = derived_key_2.to_seed();

    if ed25519_seed == ed25519_seed_2 {
        println!("✓ Derivation is deterministic (same input → same output)\n");
    } else {
        println!("✗ ERROR: Non-deterministic derivation!");
    }

    // Show how changing the entity changes the key
    println!("Step 6: Testing different entity...");
    let different_entity = r#"{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/DAOgora-xyz/different-repo",
    "name": "Different Repo"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  }
}"#;

    let key_derivation_diff = KeyDerivation::from_json(different_entity)?;
    let derived_key_diff = derive_key_from_entity(&keychain, &key_derivation_diff, parent_entropy)?;
    let ed25519_seed_diff = derived_key_diff.to_seed();

    if ed25519_seed != ed25519_seed_diff {
        println!("✓ Different entities produce different keys\n");
        println!("   Original: {}", hex::encode(&ed25519_seed[0..8]));
        println!("   Different: {}\n", hex::encode(&ed25519_seed_diff[0..8]));
    } else {
        println!("✗ ERROR: Same key for different entities!");
    }

    println!("=== BIP-Keychain Demo Complete ===");
    println!("\nKey properties verified:");
    println!("  ✓ Entity JSON → Derived key works");
    println!("  ✓ Derivation is deterministic");
    println!("  ✓ Different entities → Different keys");
    println!("  ✓ 32-byte Ed25519 seed ready for key generation");

    Ok(())
}
