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

- `--format <FORMAT>` - Output format (default: `ssh`)
  - `seed` - Raw 32-byte seed as hex
  - `public-key` - Ed25519 public key as hex
  - `private-key` - Ed25519 private key as hex (use with caution!)
  - `ssh` - OpenSSH public key format (default, most useful)
  - `gpg` - GPG-compatible public key info
  - `json` - JSON with all key data and metadata
  - `ur-entity` - UR-encoded entity (for airgapped transfer) **[requires --features bc]**
  - `ur-pubkey` - UR-encoded public key (for returning from airgapped) **[requires --features bc]**
  - `qr-entity` - QR code with UR-encoded entity **[requires --features bc]**
  - `qr-pubkey` - QR code with UR-encoded public key **[requires --features bc]**
  - `qr-animated` - Animated QR sequence for large entities (fountain codes) **[requires --features bc]**

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

Creates a cryptographically secure random BIP-39 mnemonic seed phrase.

**Syntax:**
```bash
bip-keychain generate-seed [OPTIONS]
```

**Options:**
- `--words <N>` - Number of words (12, 15, 18, 21, or 24) [default: 24]

**Examples:**
```bash
# Generate 24-word seed (256 bits entropy)
bip-keychain generate-seed

# Generate 12-word seed (128 bits entropy)
bip-keychain generate-seed --words 12
```

**Security Warning:**
- Write down the seed phrase on paper IMMEDIATELY
- Never store digitally (no screenshots, photos, or files)
- Never share with anyone
- Store securely (fireproof safe, bank vault, etc.)
- Consider SSKR backup for redundancy (see below)

### `backup-seed` - SSKR Seed Backup (Blockchain Commons feature)

**Requires:** Build with `--features bc`

Splits a BIP-39 seed into N shares using Shamir's Secret Sharing, where M shares are required to recover.

**Syntax:**
```bash
bip-keychain backup-seed [OPTIONS]
```

**Options:**
- `--groups <N>` - Total number of shares to generate (2-16) [default: 3]
- `--threshold <M>` - Number of shares required to recover (1-groups) [default: 2]
- `--output-dir <DIR>` - Output directory for share files [default: ./sskr-shares]

**Environment Variables:**
- `BIP_KEYCHAIN_SEED` - (Required) BIP-39 mnemonic to backup

**Examples:**

```bash
# 2-of-3 backup (personal backup)
export BIP_KEYCHAIN_SEED="your twelve word seed phrase here"
bip-keychain backup-seed --groups 3 --threshold 2

# 3-of-5 backup (enterprise backup)
bip-keychain backup-seed --groups 5 --threshold 3 --output-dir ./company-backup

# 2-of-2 backup (couples/partners requiring both)
bip-keychain backup-seed --groups 2 --threshold 2
```

**Output:**
Creates hex-encoded share files:
```
./sskr-shares/
  share-01-of-03.hex
  share-02-of-03.hex
  share-03-of-03.hex
```

**Distribution Best Practices:**
1. **2-of-3 Personal Backup:**
   - Share 1: Family member (spouse/parent)
   - Share 2: Trusted friend
   - Share 3: Safe deposit box or secure storage

2. **3-of-5 Enterprise Backup:**
   - Distribute to 5 executives/board members
   - Requires 3 to recover (business continuity)
   - Survives departure of 2 key people

3. **2-of-2 Joint Control:**
   - Both partners required for access
   - Maximum protection against unilateral action

**Security Properties:**
- Information-theoretically secure
- M-1 shares reveal NOTHING about the secret
- Any M-of-N combination can recover
- Based on Shamir's Secret Sharing (provably secure)

### `recover-seed` - Recover Seed from SSKR Shares

**Requires:** Build with `--features bc`

Combines M-of-N SSKR shares to recover the original BIP-39 seed phrase.

**Syntax:**
```bash
bip-keychain recover-seed <SHARE_FILES>...
```

**Arguments:**
- `<SHARE_FILES>...` - Paths to hex-encoded share files (at least threshold required)

**Examples:**

```bash
# Recover using 2 of 3 shares
bip-keychain recover-seed \
  ./sskr-shares/share-01-of-03.hex \
  ./sskr-shares/share-02-of-03.hex

# Recover using different shares
bip-keychain recover-seed \
  ./sskr-shares/share-01-of-03.hex \
  ./sskr-shares/share-03-of-03.hex

# Recover using wildcard (if you have threshold or more shares)
bip-keychain recover-seed ./sskr-shares/share-*.hex
```

**Output:**
```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  RECOVERED SEED PHRASE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
  SECURITY REMINDER
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WRITE DOWN this seed phrase on paper immediately.
NEVER store digitally or share with anyone.
```

**SSKR Workflow:**
```bash
# 1. Generate new seed
NEW_SEED=$(bip-keychain generate-seed --words 24)
export BIP_KEYCHAIN_SEED="$NEW_SEED"

# 2. Create SSKR backup (2-of-3)
bip-keychain backup-seed --groups 3 --threshold 2

# 3. Distribute shares to trusted parties/locations

# 4. TEST recovery immediately (critical!)
bip-keychain recover-seed \
  ./sskr-shares/share-01-of-03.hex \
  ./sskr-shares/share-02-of-03.hex

# 5. Store original seed securely (metal backup recommended)

# 6. Document the policy for inheritors:
#    "This wallet uses 2-of-3 SSKR backup. Any 2 shares can recover."
```

**Demo:**
Run the complete SSKR demonstration:
```bash
./examples/sskr-backup.sh
```

## Animated QR Codes (Fountain Encoding)

**Requires:** Build with `--features bc`

For entities too large to fit in a single QR code, bip-keychain uses fountain codes to split the data into multiple animated QR frames.

### How It Works

**Fountain Coding (Luby Transform):**
- Generates infinite sequence of UR parts
- Each part encodes random combination of fragments
- Receiver collects parts until decode succeeds
- Typically needs ~1.5x minimum fragments
- Parts can arrive in any order
- Resistant to packet loss

**Benefits:**
- ✓ No size limit on entities
- ✓ Can miss frames during scanning
- ✓ No fixed scan order required
- ✓ Error resistant
- ✓ Standard UR format (Blockchain Commons)

### Usage

```bash
# Generate animated QR sequence
bip-keychain derive entity.json --format qr-animated
```

This will:
1. Encode entity as multi-part UR with fountain codes
2. Split into ~200-byte fragments (QR-optimized)
3. Generate sequence of QR frames
4. Animate in terminal (loops until Ctrl+C)

### Technical Details

**Fragment Size:** 200 bytes (recommended)
- Fits in standard QR code capacity (~2,953 bytes)
- Good balance between frame count and reliability
- Compatible with most smartphone cameras

**Overhead:** ~50% (1.5x minimum fragments)
- Ensures high probability of successful decode
- Accounts for missed frames during scanning
- Standard for fountain codes

**Frame Rate:** 500ms per frame (default)
- Fast enough for quick transfer
- Slow enough for reliable scanning

### Use Cases

**1. Large Entities:**
- Complex schema.org definitions
- Multi-key derivation configs
- Entities with extensive metadata

**2. Unreliable Scanning:**
- Poor camera quality
- Bad lighting conditions
- Moving displays

**3. Airgapped Workflows:**
- Hot wallet → Cold wallet
- Smartphone → Hardware wallet
- Desktop → Mobile

### Airgapped Workflow with Animated QR

```bash
# Hot Machine (Online):
export BIP_KEYCHAIN_SEED="<your seed>"
bip-keychain derive large-entity.json --format qr-animated

# Cold Machine (Airgapped):
# 1. Scan animated QR frames with UR-compatible wallet
# 2. UR decoder reconstructs entity automatically
# 3. Derive keys securely offline
# 4. Export public key as QR

# Hot Machine:
# 1. Scan public key QR from cold machine
# 2. Use for verification/deployment
```

### Demo

Run the complete animated QR demonstration:
```bash
./examples/animated-qr.sh
```

### Comparison: Single vs Animated QR

**Single Static QR:**
- ✓ Simple (one scan)
- ✗ Limited size (~3KB max)
- ✗ Must scan perfectly
- ✗ No error recovery

**Animated Multi-part QR:**
- ✓ Unlimited size
- ✓ Error resistant
- ✓ Can miss frames
- ✓ No fixed order
- ✗ More complex (multiple scans)

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
