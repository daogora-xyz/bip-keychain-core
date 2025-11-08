# BIP-Keychain TODO

**Project**: bip-keychain-core
**Current Version**: 0.1.0
**Status**: Production-Ready MVP ✅
**Last Updated**: 2025-11-08

## Overview

This TODO tracks remaining work for BIP-Keychain Core. The core MVP is **complete and functional** - users can derive SSH/GPG keys from semantic entities using a single seed phrase. This document focuses on polish, enhancements, and production readiness.

---

## ✅ Completed (v0.1.0 MVP)

### Core Functionality
- [x] **Multi-Hash Support** - HMAC-SHA-512, BLAKE2b, SHA-256
- [x] **Entity Parsing** - JSON entity parsing with serde
- [x] **BIP-32 Derivation** - Keychain from mnemonic, hardened derivation
- [x] **Ed25519 Keypair Generation** - Full keypair support
- [x] **Output Formats** - SSH, GPG, hex (seed/public/private), JSON
- [x] **CLI Tool** - `derive` command with all output formats
- [x] **Integration Tests** - End-to-end, property-based tests (50 tests passing)
- [x] **Documentation** - 7 comprehensive guides (CLAUDE.md, CLI-USAGE.md, SSH-KEYS-GUIDE.md, etc.)
- [x] **Examples** - 11 entity examples, 6 automation scripts
- [x] **alkali 0.3.0 Compatibility** - Fixed BLAKE2b-512 hash size issue (commit: 315186e)
- [x] **Overflow Safety** - Fixed BIP-32 hardened index overflow (commit: 315186e)

**Latest Commits:**
- `315186e` - fix: resolve alkali 0.3.0 compatibility and overflow issues
- `837661a` - feat: add GPG signing, comprehensive examples, and production automation
- `44a6a75` - docs: add comprehensive project status document
- `9107ff5` - refactor: production hardening with better errors and property tests
- `4e4116c` - feat: implement SHA-256 hash function

---

## 🚧 In Progress

### Code Quality
- [ ] **Fix compiler warnings** (Priority: High)
  - [ ] Remove unused imports in `src/bip32_wrapper.rs` (DerivationPath, ExtendedKey)
  - [ ] Fix unused variable in `src/bin/bip-keychain.rs` (words parameter)
  - Run: `cargo fix --lib --bin bip-keychain --allow-dirty`

---

## 📋 TODO (Priority Order)

### High Priority - Core Features

#### 1. Implement `generate-seed` Command
**Status**: Stubbed (returns error)
**Effort**: 2-4 hours
**Files**: `src/bin/bip-keychain.rs`

Currently the command exists but isn't implemented:
```rust
fn generate_seed_command(words: usize) -> Result<()> {
    anyhow::bail!("generate-seed command not yet implemented...")
}
```

**Tasks:**
- [ ] Implement BIP-39 mnemonic generation (12/15/18/21/24 word support)
- [ ] Use `bip39::Mnemonic::generate()` with secure randomness
- [ ] Add word count validation
- [ ] Add `--word-count` CLI flag (default: 24)
- [ ] Print mnemonic to stdout with security warning
- [ ] Add tests for word count validation
- [ ] Update CLI-USAGE.md with examples

**Acceptance Criteria:**
- User can run `bip-keychain generate-seed --word-count 24`
- Output includes security warning about seed phrase storage
- Different invocations produce different seeds (using secure RNG)
- Tests verify word count options work correctly

---

#### 2. Batch Keychain Processing
**Status**: Not implemented
**Effort**: 4-8 hours
**Files**: `src/bin/bip-keychain.rs`, `src/derivation.rs`

Currently can only derive one key at a time. Need support for:
```json
{
  "keychain": [
    {"entity": {...}, "purpose": "GitHub"},
    {"entity": {...}, "purpose": "Server"},
    ...
  ]
}
```

**Tasks:**
- [ ] Design JSON keychain format (array of entities with purposes)
- [ ] Add `keychain` subcommand to CLI
- [ ] Implement `process_keychain()` function in `derivation.rs`
- [ ] Add progress reporting for large batches
- [ ] Handle per-key errors gracefully (continue on error)
- [ ] Output all keys with associated metadata
- [ ] Add integration test for batch processing
- [ ] Create example keychain JSON files
- [ ] Document in CLI-USAGE.md

**Acceptance Criteria:**
- User can run `bip-keychain keychain keychains/work.json`
- Progress indicator shows N/M keys derived
- Failures on individual keys don't stop entire batch
- Output clearly associates each key with its purpose

---

### Medium Priority - Output Formats

#### 3. OpenSSH Private Key Format
**Status**: Not implemented (can output private key as hex)
**Effort**: 6-12 hours
**Files**: `src/output.rs`

Currently can output Ed25519 private key as hex, but not in standard OpenSSH format (`id_ed25519` file format).

**Tasks:**
- [ ] Research OpenSSH private key format spec
- [ ] Implement OpenSSH private key serialization
- [ ] Add passphrase encryption support (optional)
- [ ] Add `--format ssh-private` option to CLI
- [ ] Add security warnings to CLI output
- [ ] Add tests with known SSH private keys
- [ ] Document security considerations
- [ ] Update SSH-KEYS-GUIDE.md

**Acceptance Criteria:**
- Output works with `ssh-add id_ed25519`
- Output works with `ssh -i id_ed25519 user@host`
- Compatible with OpenSSH 7.0+
- Passphrase encryption optional but supported

**Security Note:**
This is lower priority because private keys should ideally never be written to disk in plain text. Current hex format + manual conversion is acceptable for MVP.

---

### Medium Priority - Production Readiness

#### 4. CI/CD Pipeline
**Status**: Not started
**Effort**: 4-6 hours
**Files**: `.github/workflows/ci.yml` (new)

**Tasks:**
- [ ] Create GitHub Actions workflow
  - [ ] Run `cargo build` on push
  - [ ] Run `cargo test` on push
  - [ ] Run `cargo clippy` for lints
  - [ ] Run `cargo fmt --check` for formatting
  - [ ] Test on Linux, macOS, Windows
  - [ ] Cache Cargo dependencies
- [ ] Add status badges to README.md
- [ ] Set up branch protection rules (require CI passing)
- [ ] Add pre-commit hooks (optional)

**Acceptance Criteria:**
- All PRs must pass CI before merge
- CI runs on all supported platforms
- Badge shows build status on README

---

#### 5. Publish to crates.io
**Status**: Not started
**Effort**: 2-4 hours
**Prerequisites**: CI/CD pipeline, clean warnings

**Tasks:**
- [ ] Review and finalize Cargo.toml metadata
  - [ ] Add categories, keywords
  - [ ] Add license file (BSD-2-Clause)
  - [ ] Add repository URL
  - [ ] Add homepage URL
- [ ] Write crates.io description (optimize for search)
- [ ] Test `cargo publish --dry-run`
- [ ] Create git tag for v0.1.0
- [ ] Publish to crates.io
- [ ] Verify installation works: `cargo install bip-keychain`
- [ ] Update README with installation instructions

**Acceptance Criteria:**
- Package available on crates.io
- Users can install with `cargo install bip-keychain`
- Documentation link works on crates.io

---

#### 6. Performance Benchmarks
**Status**: Claims met but not measured
**Effort**: 3-5 hours
**Files**: `benches/derivation_bench.rs` (new)

PROJECT-STATUS.md claims:
- Single key derivation: < 100ms ✅
- Hash operations: < 10ms ✅
- Ed25519 key generation: < 5ms ✅

Need to actually measure and track these.

**Tasks:**
- [ ] Set up Criterion.rs for benchmarking
- [ ] Add benchmark for full derivation pipeline
- [ ] Add benchmark for each hash function
- [ ] Add benchmark for Ed25519 keypair generation
- [ ] Add benchmark for SSH key formatting
- [ ] Document performance results in PROJECT-STATUS.md
- [ ] Add CI step to run benchmarks (track regressions)

**Acceptance Criteria:**
- Benchmarks run with `cargo bench`
- Performance targets documented and verified
- CI tracks performance regressions

---

### Lower Priority - Quality of Life

#### 7. Improved Error Messages
**Status**: Good but could be better
**Effort**: 2-3 hours

**Tasks:**
- [ ] Review all error messages for clarity
- [ ] Add suggestions to common errors
- [ ] Add examples to error messages where helpful
- [ ] Test error messages with new users
- [ ] Document common errors in troubleshooting guide

---

#### 8. Shell Completions
**Status**: Not implemented
**Effort**: 1-2 hours
**Files**: `src/bin/bip-keychain.rs`

**Tasks:**
- [ ] Generate shell completions using clap
- [ ] Add completions for bash, zsh, fish
- [ ] Document installation in README
- [ ] Add to installation instructions

**Acceptance Criteria:**
- Tab completion works for commands and options
- Installable via package managers

---

#### 9. Key Metadata Cache (Optional)
**Status**: Not started
**Effort**: 6-10 hours

For users deriving many keys, cache entity hash → index mapping to avoid re-hashing.

**Tasks:**
- [ ] Design cache format (JSON or SQLite)
- [ ] Implement cache read/write
- [ ] Add `--cache` flag to CLI
- [ ] Add cache invalidation strategy
- [ ] Document cache behavior

**Note:** This is optional optimization - current performance is acceptable for most use cases.

---

## 🔮 Future Enhancements (v0.2.0+)

### Major Features

#### Hardware Wallet Integration
- Ledger Nano S/X support
- Trezor support
- COLDCARD support
- Derive keys on-device, never expose seed phrase

#### Advanced Derivation
- Non-hardened derivation support (for PKI use cases)
- Custom derivation paths
- BIP-44/49/84 compatibility mode

#### WebAssembly Build
- Compile to WASM for browser use
- JavaScript bindings
- Demo web app

#### GUI Application
- Cross-platform desktop app (Tauri/egui)
- Visual keychain management
- QR code support for mobile

### Minor Features
- Key rotation automation
- Backup verification tools
- Integration with password managers
- Docker container for CLI
- Homebrew tap for macOS
- APT/RPM packages for Linux

---

## 🚫 Out of Scope

These are explicitly **not** planned:

- Key storage/management (use existing tools like ssh-agent, gpg-agent)
- Network operations (fetching schemas, syncing)
- Smart contract integration
- Cryptocurrency wallet features (use dedicated wallets)
- Cloud backup services (security risk)

---

## 📊 Release Roadmap

### v0.1.0 (Current - MVP) ✅
Core functionality complete, ready for use

### v0.1.1 (Next - Polish)
- Fix compiler warnings
- Implement `generate-seed` command
- Add CI/CD pipeline
- Publish to crates.io
- **Target: 1-2 weeks**

### v0.2.0 (Enhanced)
- Batch keychain processing
- OpenSSH private key format
- Performance benchmarks
- Shell completions
- **Target: 1-2 months**

### v0.3.0 (Production)
- Hardware wallet integration
- External security audit
- WebAssembly build
- Comprehensive test coverage
- **Target: 3-6 months**

---

## 🤝 Contributing

Priority for external contributors:
1. CI/CD pipeline setup
2. Performance benchmarks
3. Shell completions
4. Additional entity examples
5. Security audit

See CONTRIBUTING.md (to be created) for guidelines.

---

## 📝 Notes

- This file tracks work items. For development process, see `.claude/development-workflow.md`
- For architecture and design decisions, see `spec/design.md`
- For user stories and requirements, see `spec/requirements.md`
- For project status and metrics, see `PROJECT-STATUS.md`

**Next Steps:**
1. Fix compiler warnings (quick win)
2. Implement `generate-seed` command (high value)
3. Set up CI/CD (production readiness)
4. Publish to crates.io (distribution)
