# BIP-Keychain CLI Usage Guide

## Building

```bash
# Build in debug mode
cargo build

# Build in release mode (optimized)
cargo build --release
```

## Quick Start

### 1. Set Your Seed Phrase

For security, the seed phrase must be provided via environment variable:

```bash
export BIP_KEYCHAIN_SEED="your twelve or twenty-four word BIP-39 seed phrase here"
```

**WARNING**: Never commit your real seed phrase to version control!

For testing, you can use the standard BIP-39 test mnemonic:
```bash
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
```

### 2. Derive a Key from an Entity

```bash
# Using the debug build
cargo run --bin bip-keychain -- derive examples/test-entity.json

# Using the release build (after cargo build --release)
./target/release/bip-keychain derive examples/test-entity.json
```

This will output a 64-character hex string representing the 32-byte Ed25519 seed.

## Command Reference

### `derive` - Derive key from entity

Derives a cryptographic key from a JSON entity file.

**Syntax:**
```bash
bip-keychain derive <ENTITY_JSON> [OPTIONS]
```

**Arguments:**
- `<ENTITY_JSON>` - Path to the entity JSON file (Nickel-exported)

**Options:**
- `--parent-entropy <HEX>` - Parent entropy in hex format (optional)
  - Used as HMAC key for HMAC-based hash functions
  - Default: `bip-keychain-default-entropy-32!` (for testing)

- `--format <FORMAT>` - Output format (default: `hex`)
  - `hex` - Hexadecimal encoding of Ed25519 seed
  - `json` - JSON with metadata

**Environment Variables:**
- `BIP_KEYCHAIN_SEED` - (Required) BIP-39 mnemonic seed phrase

**Examples:**

```bash
# Basic usage with hex output
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
cargo run --bin bip-keychain -- derive examples/test-entity.json

# JSON output with metadata
cargo run --bin bip-keychain -- derive examples/test-entity.json --format json

# With custom parent entropy
cargo run --bin bip-keychain -- derive examples/test-entity.json \
  --parent-entropy $(echo -n "my-custom-entropy" | xxd -p)
```

**Output (hex format):**
```
a1b2c3d4e5f6...  (64 hex characters = 32 bytes)
```

**Output (json format):**
```json
{
  "ed25519_seed": "a1b2c3d4e5f6...",
  "schema_type": "schema_org",
  "hash_function": "HmacSha512",
  "purpose": "Git commit signing key for bip-keychain-core repository"
}
```

### `generate-seed` - Generate BIP-39 seed phrase

**Status:** Not yet implemented

For now, use external tools:
- [BIP-39 Tool](https://iancoleman.io/bip39/) (use offline!)
- Hardware wallets (Ledger, Trezor)
- `bitcoin-cli` or other wallet software

## Testing

Run the test script:

```bash
./test-cli.sh
```

Or manually test:

```bash
# Set test seed
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Test 1: Basic derivation
cargo run --bin bip-keychain -- derive examples/test-entity.json

# Test 2: JSON output
cargo run --bin bip-keychain -- derive examples/test-entity.json --format json

# Test 3: Help text
cargo run --bin bip-keychain -- --help
cargo run --bin bip-keychain -- derive --help
```

## Entity JSON Format

Entity JSON files are typically exported from Nickel configuration files.

**Example entity JSON:**
```json
{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/user/repo",
    "name": "My Project"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Git commit signing key"
}
```

**Supported hash functions:**
- `hmac_sha512` - HMAC-SHA-512 (BIP-85 standard)
- `blake2b` - BLAKE2b (Blockchain Commons)
- `sha256` - SHA-256 (not yet implemented)

**Supported schema types:**
- `schema_org` - Schema.org JSON-LD entities
- `did` - W3C Decentralized Identifiers
- `gordian_envelope` - Blockchain Commons Gordian Envelope
- `x509_dn` - X.509 Distinguished Names
- `dns` - DNS/FQDN
- `ipfs_cid` - IPFS Content Identifiers
- `urn` - Uniform Resource Names
- `verifiable_credential` - W3C Verifiable Credentials
- `custom` - Custom entity format

(Note: Currently only basic parsing is implemented; all entity types are stored as generic JSON)

## Security Best Practices

1. **Never expose your seed phrase:**
   - Don't use it in command-line arguments (visible in `ps`)
   - Don't commit it to version control
   - Don't log it
   - Use environment variables or secure vaults

2. **Parent entropy:**
   - In production, derive from your master seed
   - Don't reuse across different applications
   - Document your derivation scheme

3. **Derived keys:**
   - Treat Ed25519 seeds as sensitive data
   - Clear from memory after use
   - Use secure storage for private keys

4. **Testing:**
   - Use the standard test mnemonic for testing only
   - Never use test mnemonics in production

## Troubleshooting

### "BIP_KEYCHAIN_SEED environment variable not set"

Set your seed phrase:
```bash
export BIP_KEYCHAIN_SEED="your twelve word seed phrase here"
```

### "Failed to parse entity JSON"

- Ensure the JSON file is valid
- Check that required fields are present:
  - `schema_type`
  - `entity`
  - `derivation_config`

### "Failed to create keychain from seed phrase"

- Verify your seed phrase is a valid BIP-39 mnemonic
- Should be 12, 15, 18, 21, or 24 words
- Words must be from the BIP-39 wordlist

## Next Steps

- See `examples/derive_key.rs` for Rust library usage
- See Nickel examples in `nickel/examples/` for entity definitions
- Read `CLAUDE.md` for development information
