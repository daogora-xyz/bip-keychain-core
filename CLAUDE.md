# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**bip-keychain-core** is a Rust implementation of BIP-Keychain, a semantic hierarchical key derivation system that extends BIP-85. It allows deriving cryptographic keys from human-readable JSON entities (schema.org, DIDs, etc.) rather than numeric indices.

**Key Innovation**: Separation of path keys from path values - if a hot master is compromised, only derivation paths (metadata) are exposed, not the actual secrets.

## Development Methodology

This project follows **Kiro Specs** + **strict TDD (Test-Driven Development)**:

1. **Read spec files first**: `spec/requirements.md` (EARS user stories), `spec/design.md` (architecture), `spec/tasks.md` (TDD task tracking)
2. **Follow RED-GREEN-REFACTOR cycle**: Write failing test → Implement minimal code → Refactor
3. **Update tasks.md**: Mark each TDD phase complete with commit hash
4. **Use known test vectors**: NIST, RFC, BLAKE2 official vectors for crypto implementations

See `.claude/development-workflow.md` for complete TDD workflow.

## Common Commands

### Building and Testing
```bash
# Build the project
cargo build

# Run all tests
cargo test

# Run specific test
cargo test test_hmac_sha512

# Run tests with output
cargo test -- --nocapture

# Run tests for specific module
cargo test hash::

# Build release binary
cargo build --release

# Run CLI tool (once implemented)
cargo run -- derive entity.json
```

### Development Workflow
```bash
# Check what needs to be done
cat spec/tasks.md

# Run single test file
cargo test --test hash_tests
```

## Architecture Overview

### Module Structure

The codebase is organized into focused modules:

- **entity.rs** (planned): Type-safe Rust structs for JSON entities (schema.org, DID, Gordian Envelope, etc.)
- **hash.rs**: Multi-hash support (HMAC-SHA-512, BLAKE2b, SHA-256) with canonical JSON
- **derivation.rs** (planned): Core BIP-Keychain algorithm (entity → hash → u32 index → BIP-32 key)
- **bip32_wrapper.rs** (planned): BIP-32 wrapper with keychain path `m/83696968'/67797668'/{index}'`
- **output.rs** (planned): Key formatting (SSH, GPG, hex, JSON)
- **error.rs**: Unified error types using thiserror

### Derivation Flow

```
JSON Entity → Canonicalize → Hash (HMAC-SHA-512/BLAKE2b/SHA-256)
  → Extract first 4 bytes as u32 → BIP-32 derive at m/83696968'/67797668'/{index}'
  → Format output (SSH/GPG/hex)
```

### Hash Function Design

**Multi-hash support** for different ecosystem compatibility:
- **HMAC-SHA-512**: BIP-85 standard (uses parent entropy as HMAC key)
- **BLAKE2b**: Blockchain Commons compatibility (uses libsodium via alkali crate)
- **SHA-256**: Future support (padded to 64 bytes)

All hash functions return 64-byte arrays. JSON is canonicalized (sorted keys, no whitespace) before hashing for determinism.

### Current Implementation Status (from spec/tasks.md)

**Completed**:
- ✅ Project setup (Cargo.toml, Kiro Specs)
- ✅ HMAC-SHA-512 implementation (RED → GREEN → REFACTOR complete)
- ✅ BLAKE2b implementation (RED phase complete with alkali crate, GREEN in progress)

**In Progress**:
- Task 2.2: BLAKE2b GREEN phase (implement using alkali crate)

**Pending**:
- Entity type definitions (entity.rs)
- BIP-32 wrapper (bip32_wrapper.rs)
- Core derivation algorithm (derivation.rs)
- Output formatting (output.rs)
- CLI tool (bin/bip-keychain.rs)

## Code Patterns

### Canonical JSON
All JSON entities must be canonicalized before hashing. The `canonicalize_json()` helper in `hash.rs`:
- Parses JSON and re-serializes with sorted keys
- Falls back to raw string for non-JSON test vectors
- Ensures deterministic hashing

### Error Handling
Use `BipKeychainError` enum (thiserror-based) for all errors:
```rust
pub type Result<T> = std::result::Result<T, BipKeychainError>;
```

### Test Vectors
Always use official test vectors from standards bodies:
- **HMAC-SHA-512**: RFC 4231 (see tests/hash_tests.rs)
- **BLAKE2b**: Official BLAKE2 repository (github.com/BLAKE2/BLAKE2/tree/master/testvectors)
- **BIP-32**: Test vectors from BIP-32 specification

## Key Dependencies

- **bip32 (0.5)**: BIP-32 hierarchical deterministic keys
- **bip39 (2.0)**: Mnemonic seed phrase generation
- **alkali (0.4)**: libsodium bindings for BLAKE2b (Blockchain Commons compatibility)
- **hmac + sha2**: HMAC-SHA-512 and SHA-256
- **serde + serde_json**: JSON entity serialization
- **thiserror**: Error handling
- **clap (4.0)**: CLI argument parsing (planned)

## Nickel Integration

The `nickel/` directory contains Nickel configuration language files that export to JSON entities. The Rust code consumes these JSON exports for key derivation. This separation allows:
- Type-safe entity definitions in Nickel
- Validation at export time
- Clean Rust integration via JSON

## Security Considerations

1. **Seed phrase handling**: Never log seed phrases, clear from memory after use
2. **Canonical JSON**: Sort keys alphabetically, no whitespace, UTF-8 encoding
3. **Test vectors**: Validate against known vectors to ensure correctness
4. **Determinism**: Same input must always produce same output

## Working with This Codebase

### When starting a new task:

1. **Check spec/tasks.md** for current status and next task
2. **Read spec/design.md** for architectural guidance
3. **Follow TDD**: Write test first (RED) → Implement (GREEN) → Clean up (REFACTOR)
4. **Update tasks.md** immediately after each commit with commit hash
5. **Use proper commit messages**:
   - RED: `test: add failing test for X`
   - GREEN: `feat: implement X`
   - REFACTOR: `refactor: improve X`
   - Include `TDD-Phase:` metadata and reference to `spec/tasks.md#task-id`

### Test-first development is mandatory:

- Never write implementation before test
- Verify test fails before implementing
- Verify test passes after implementing
- Keep tests passing during refactoring

### Where to find things:

- **Requirements**: `spec/requirements.md` (EARS user stories)
- **Architecture**: `spec/design.md` (modules, data flow, sequence diagrams)
- **Task tracking**: `spec/tasks.md` (TDD phases, commit hashes)
- **Workflow guide**: `.claude/development-workflow.md` (complete TDD process)
- **Hash implementation**: `src/hash.rs` (multi-hash with canonical JSON)
- **Hash tests**: `tests/hash_tests.rs` (RFC 4231, BLAKE2 official vectors)

## Related Projects

- **BIP-Keychain proposal**: https://github.com/akarve/bip-keychain (original specification)
- **BIP-85**: https://bips.dev/85/ (parent standard for deterministic entropy)
- **BIP-32**: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki (HD wallets)
- **Blockchain Commons**: Uses BLAKE2b for entity hashing
