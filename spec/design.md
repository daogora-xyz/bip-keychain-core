# BIP-Keychain Rust Implementation - Design

**Project**: bip-keychain (Rust implementation)
**Version**: 0.1.0
**Date**: 2025-10-21
**Spec Type**: Kiro Specs - Technical Design

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    CLI (bip-keychain)                       │
│  clap-based argument parsing, user interaction              │
└───────────────────────────┬─────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Core Library (bip_keychain)               │
├─────────────────────────────────────────────────────────────┤
│  entity.rs      │ Entity type definitions (serde)           │
│  hash.rs        │ Multi-hash (HMAC-SHA-512, BLAKE2b, SHA256)│
│  derivation.rs  │ BIP-keychain derivation algorithm         │
│  bip32_wrapper.rs│ BIP-32 key derivation wrapper            │
│  output.rs      │ Key formatting (SSH, GPG, hex, JSON)      │
│  error.rs       │ Error types (thiserror)                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              External Dependencies                          │
├─────────────────────────────────────────────────────────────┤
│  bip32          │ BIP-32 hierarchical deterministic keys    │
│  bip39          │ Mnemonic seed phrases                     │
│  alkali         │ libsodium bindings (BLAKE2b)              │
│  hmac + sha2    │ HMAC-SHA-512, SHA-256                     │
│  serde          │ JSON serialization/deserialization        │
└─────────────────────────────────────────────────────────────┘
```

## Module Design

### entity.rs - Entity Type Definitions

**Purpose**: Parse Nickel-exported JSON into type-safe Rust structs.

```rust
#[derive(Debug, Deserialize)]
#[serde(tag = "schema_type")]
pub enum Entity {
    #[serde(rename = "schema_org")]
    SchemaOrg(SchemaOrgEntity),

    #[serde(rename = "did")]
    DID(DIDEntity),

    #[serde(rename = "gordian_envelope")]
    GordianEnvelope(GordianEnvelopeEntity),

    // ... other schema types
}

#[derive(Debug, Deserialize)]
pub struct KeyDerivation {
    pub schema_type: String,
    pub entity: Entity,
    pub derivation_config: DerivationConfig,
    pub purpose: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
```

**Design Decisions**:
- Use serde's `tag` attribute for discriminated unions
- Each entity type has its own struct for type safety
- Optional fields use `Option<T>`

### hash.rs - Multi-Hash Support

**Purpose**: Provide multiple hash functions for entity→index conversion.

```rust
pub enum HashFunction {
    HmacSha512,
    Blake2b,
    Sha256,
}

pub fn hash_entity(
    entity_json: &str,
    parent_entropy: &[u8],
    hash_fn: HashFunction,
) -> Result<[u8; 64], HashError> {
    match hash_fn {
        HashFunction::HmacSha512 => hmac_sha512(entity_json, parent_entropy),
        HashFunction::Blake2b => blake2b_hash(entity_json),
        HashFunction::Sha256 => sha256_padded(entity_json, parent_entropy),
    }
}
```

**Design Decisions**:
- Enum for hash function selection
- All hash functions return 64-byte arrays (pad if needed)
- Canonical JSON serialization before hashing

### derivation.rs - BIP-Keychain Algorithm

**Purpose**: Implement the core BIP-keychain entity→key derivation.

**Algorithm**:
```
1. Canonicalize entity JSON
2. Hash with selected function + parent entropy
3. Extract first 4 bytes as u32 index
4. Derive BIP-32 child at m/83696968'/67797668'/{index}'
5. Return extended key
```

**Sequence Diagram**:
```
User -> CLI: bip-keychain derive entity.json
CLI -> entity::parse: Parse JSON
entity::parse -> CLI: KeyDerivation struct
CLI -> hash::hash_entity: Hash with config
hash::hash_entity -> CLI: 64-byte digest
CLI -> derivation::entity_to_index: Extract u32
derivation::entity_to_index -> CLI: child_index
CLI -> bip32_wrapper::derive: Derive key at index
bip32_wrapper::derive -> CLI: ExtendedKey
CLI -> output::format: Format as SSH/GPG/hex
output::format -> User: Formatted key
```

### bip32_wrapper.rs - BIP-32 Wrapper

**Purpose**: Simplify BIP-32 operations with sensible defaults.

```rust
pub struct Keychain {
    master_key: ExtendedKey,
}

impl Keychain {
    pub fn from_mnemonic(phrase: &str) -> Result<Self, Bip32Error>;

    pub fn derive_child(
        &self,
        index: u32,
        hardened: bool,
    ) -> Result<ExtendedKey, Bip32Error>;

    pub fn derive_bip_keychain_path(
        &self,
        entity_index: u32,
        hardened: bool,
    ) -> Result<ExtendedKey, Bip32Error> {
        // Derive m/83696968'/67797668'/{entity_index}'
        // Uses BIP-85 prefix + BIP-keychain app code
    }
}
```

### output.rs - Key Formatting

**Purpose**: Format derived keys for different use cases.

```rust
pub enum OutputFormat {
    SSH,
    GPG,
    Hex,
    JSON,
}

pub fn format_key(
    key: &ExtendedKey,
    format: OutputFormat,
    metadata: &Derivation Metadata,
) -> Result<String, OutputError>;
```

**Formats**:
- **SSH**: OpenSSH public key format (`ssh-ed25519 AAAA...`)
- **GPG**: GPG-compatible key export
- **Hex**: Raw hex encoding
- **JSON**: Structured output with metadata

## Data Flow

### Single Key Derivation Flow

```
Nickel File (.ncl)
    ↓ nickel export
JSON Entity
    ↓ bip-keychain CLI
Entity Struct (Rust)
    ↓ canonicalize + hash
64-byte Digest
    ↓ first 4 bytes
u32 Child Index
    ↓ BIP-32 derive
Extended Key
    ↓ format
Output (SSH/GPG/hex)
```

### Keychain Batch Flow

```
Nickel Keychain (.ncl)
    ↓ nickel export
JSON Keychain (multiple derivations)
    ↓ bip-keychain CLI
Vec<KeyDerivation>
    ↓ for each
Multiple Extended Keys
    ↓ format each
Multiple Outputs
```

## Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum BipKeychainError {
    #[error("Invalid entity JSON: {0}")]
    InvalidEntity(#[from] serde_json::Error),

    #[error("Hash function error: {0}")]
    HashError(String),

    #[error("BIP-32 derivation error: {0}")]
    Bip32Error(#[from] bip32::Error),

    #[error("Invalid seed phrase: {0}")]
    InvalidSeedPhrase(String),

    #[error("Output format error: {0}")]
    OutputError(String),
}
```

**Strategy**: Use `thiserror` for ergonomic error handling, provide context for all errors.

## Security Considerations

1. **Seed Phrase Handling**:
   - Never log seed phrases
   - Clear from memory after use (use `zeroize` crate)
   - Accept via environment variable for automation

2. **Entropy Sources**:
   - Use `getrandom` for BIP-39 generation
   - Validate all entropy inputs

3. **Canonical JSON**:
   - Sort keys alphabetically
   - No whitespace
   - UTF-8 encoding

## Testing Strategy

### Unit Tests
- Each hash function with known test vectors
- Entity parsing for all schema types
- Index extraction correctness

### Integration Tests
- Full derivation path with known seed → known key
- Multi-schema keychain processing
- Output format validation

### Property Tests
- Determinism: same input → same output
- Uniqueness: different entities → different indices (high probability)

## Performance Targets

- Single key derivation: < 100ms
- 100-key keychain: < 5 seconds
- Memory: < 50MB for typical operations

## Future Considerations

- Hardware wallet integration (Ledger/Trezor)
- Key caching/memoization
- Parallel keychain processing
- WebAssembly compilation
