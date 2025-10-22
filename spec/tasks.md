# BIP-Keychain Implementation Tasks

**Project**: bip-keychain (Rust implementation)
**Methodology**: TDD (RED-GREEN-REFACTOR)
**Date Started**: 2025-10-21

## Task Status Legend

- **RED**: Test written, currently failing
- **GREEN**: Test passing, functionality implemented
- **REFACTOR**: Code improved, tests still passing
- **Complete**: All phases done

## Current Sprint: Core Functionality

### Task 1: Project Setup ✅

**TDD Phase**: Complete
**Status**: Complete

#### Subtasks

- [x] **Setup**: Create Cargo.toml (commit: 6441ddd)
- [x] **Setup**: Create Kiro Specs structure (commit: 6441ddd)
- [x] **Setup**: Create development workflow docs (commit: 6441ddd)

**Acceptance Criteria**:
- [x] Cargo.toml with all dependencies
- [x] spec/ directory with requirements.md, design.md, tasks.md
- [x] .claude/development-workflow.md

---

### Task 2: Multi-Hash Support

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-1 from spec/requirements.md
**Design**: See spec/design.md#hash-module

#### Subtasks

##### 2.1: HMAC-SHA-512
- [x] **RED**: Write test with NIST test vector (commit: fdc803b)
- [x] **GREEN**: Implement using hmac + sha2 crates (commit: d81bc93)
- [x] **REFACTOR**: Extract canonicalize_json helper (commit: c916b8d)

##### 2.2: BLAKE2b (Blockchain Commons)
- [x] **RED**: Write test with BLAKE2 official test vector (commit: 555822f)
- [x] **GREEN**: Implement using alkali crate (libsodium bindings) (commit: f67ae3f)
- [x] **REFACTOR**: Optimize for large entities (commit: eaae33a)

##### 2.3: SHA-256
- [ ] **RED**: Write test with NIST test vector
- [ ] **GREEN**: Implement using sha2 crate
- [ ] **REFACTOR**: Add error handling

**Acceptance Criteria**:
- [ ] All three hash functions implemented
- [ ] Tests pass with known vectors
- [ ] Performance < 10ms per hash
- [ ] Canonical JSON serialization

**Test Vectors**:
- HMAC-SHA-512: [NIST CAVP](https://csrc.nist.gov/projects/cryptographic-algorithm-validation-program)
- BLAKE2b: https://github.com/BLAKE2/BLAKE2/tree/master/testvectors
- SHA-256: NIST test vectors

---

### Task 3: Entity Type Definitions

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-2 from spec/requirements.md
**Design**: See spec/design.md#entity-module

#### Subtasks

##### 3.1: Base Entity Enum
- [ ] **RED**: Write test parsing schema.org entity JSON
- [ ] **GREEN**: Implement Entity enum with serde
- [ ] **REFACTOR**: Add validation helpers

##### 3.2: Schema.org Entity
- [ ] **RED**: Write test for SoftwareSourceCode entity
- [ ] **GREEN**: Implement SchemaOrgEntity struct
- [ ] **REFACTOR**: Add JSON-LD context validation

##### 3.3: DID Entity
- [ ] **RED**: Write test for did:github entity
- [ ] **GREEN**: Implement DIDEntity struct
- [ ] **REFACTOR**: Add DID format validation

##### 3.4: Gordian Envelope Entity
- [ ] **RED**: Write test for UR-encoded envelope
- [ ] **GREEN**: Implement GordianEnvelopeEntity struct
- [ ] **REFACTOR**: Add UR format validation

##### 3.5: X.509 DN Entity
- [ ] **RED**: Write test for certificate DN
- [ ] **GREEN**: Implement X509DNEntity struct
- [ ] **REFACTOR**: Add DN validation

##### 3.6: Remaining Entity Types
- [ ] DNS, IPFS CID, URN, VerifiableCredential, Custom

**Acceptance Criteria**:
- [ ] All 9 schema types supported
- [ ] Serde deserialization working
- [ ] Clear error messages for invalid JSON
- [ ] Example JSON for each type

---

### Task 4: BIP-32 Wrapper

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-3 from spec/requirements.md
**Design**: See spec/design.md#bip32-wrapper-module

#### Subtasks

##### 4.1: Keychain Struct
- [ ] **RED**: Write test deriving from BIP-39 mnemonic
- [ ] **GREEN**: Implement Keychain::from_mnemonic()
- [ ] **REFACTOR**: Add seed phrase validation

##### 4.2: Child Derivation
- [ ] **RED**: Write test deriving hardened child
- [ ] **GREEN**: Implement derive_child()
- [ ] **REFACTOR**: Add non-hardened support

##### 4.3: BIP-Keychain Path
- [ ] **RED**: Write test for m/83696968'/67797668'/{index}'
- [ ] **GREEN**: Implement derive_bip_keychain_path()
- [ ] **REFACTOR**: Add path validation

**Acceptance Criteria**:
- [ ] Derives keys from BIP-39 mnemonics
- [ ] Supports hardened and non-hardened paths
- [ ] BIP-Keychain base path correct
- [ ] Compatible with BIP-32 test vectors

**Test Vectors**:
- BIP-32: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki#test-vectors

---

### Task 5: Core Derivation Algorithm

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-1, US-2, US-3 from spec/requirements.md
**Design**: See spec/design.md#derivation-module

#### Subtasks

##### 5.1: Entity to Index Conversion
- [ ] **RED**: Write test converting hash to u32 index
- [ ] **GREEN**: Implement entity_to_index()
- [ ] **REFACTOR**: Add bounds checking

##### 5.2: Full Derivation Pipeline
- [ ] **RED**: Write test: entity JSON → extended key
- [ ] **GREEN**: Implement derive_key_from_entity()
- [ ] **REFACTOR**: Extract error handling

##### 5.3: Keychain Batch Processing
- [ ] **RED**: Write test deriving multiple keys
- [ ] **GREEN**: Implement process_keychain()
- [ ] **REFACTOR**: Add progress reporting

**Acceptance Criteria**:
- [ ] End-to-end derivation working
- [ ] Deterministic (same input → same output)
- [ ] Handles all entity types
- [ ] Batch processing efficient

---

### Task 6: Output Formatting

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-4 from spec/requirements.md
**Design**: See spec/design.md#output-module

#### Subtasks

##### 6.1: SSH Format
- [ ] **RED**: Write test generating ssh-ed25519 key
- [ ] **GREEN**: Implement SSH public key format
- [ ] **REFACTOR**: Add comment field support

##### 6.2: Hex Format
- [ ] **RED**: Write test outputting raw hex
- [ ] **GREEN**: Implement hex encoding
- [ ] **REFACTOR**: Add optional key type prefix

##### 6.3: JSON Format
- [ ] **RED**: Write test outputting structured JSON
- [ ] **GREEN**: Implement JSON output with metadata
- [ ] **REFACTOR**: Add pretty-print option

**Acceptance Criteria**:
- [ ] SSH format compatible with OpenSSH
- [ ] Hex output correct
- [ ] JSON includes all metadata
- [ ] Format selection via enum

---

### Task 7: CLI Tool

**TDD Phase**: Pending
**Status**: Not Started

**Requirements**: US-4, US-5, US-6 from spec/requirements.md
**Design**: See spec/design.md#cli-design

#### Subtasks

##### 7.1: Argument Parsing
- [ ] **RED**: Write test parsing CLI args
- [ ] **GREEN**: Implement clap-based parser
- [ ] **REFACTOR**: Add help text and examples

##### 7.2: Derive Command
- [ ] **RED**: Write integration test for derive
- [ ] **GREEN**: Implement derive subcommand
- [ ] **REFACTOR**: Add progress output

##### 7.3: Keychain Command
- [ ] **RED**: Write integration test for keychain
- [ ] **GREEN**: Implement keychain subcommand
- [ ] **REFACTOR**: Add batch progress

##### 7.4: Generate-Seed Command
- [ ] **RED**: Write test for seed generation
- [ ] **GREEN**: Implement generate-seed subcommand
- [ ] **REFACTOR**: Add word count options

**Acceptance Criteria**:
- [ ] All subcommands working
- [ ] Helpful error messages
- [ ] Examples in --help
- [ ] Environment variable support for seed

---

### Task 8: Integration Tests

**TDD Phase**: Pending
**Status**: Not Started

#### Subtasks

##### 8.1: End-to-End Derivation
- [ ] **RED**: Write test: Nickel JSON → SSH key
- [ ] **GREEN**: Make test pass with real implementation
- [ ] **REFACTOR**: Add more test cases

##### 8.2: Multi-Schema Support
- [ ] **RED**: Write test deriving from all 9 schemas
- [ ] **GREEN**: Ensure all schemas work
- [ ] **REFACTOR**: Parametrize test

##### 8.3: Performance Tests
- [ ] **RED**: Write test for <100ms single derivation
- [ ] **GREEN**: Profile and optimize
- [ ] **REFACTOR**: Add benchmarks

**Acceptance Criteria**:
- [ ] Full workflow tested
- [ ] All schema types validated
- [ ] Performance targets met

---

### Task 9: Documentation

**TDD Phase**: N/A (not TDD)
**Status**: Not Started

#### Subtasks

- [ ] Write README.md for Rust implementation
- [ ] Generate rustdoc API documentation
- [ ] Create usage examples
- [ ] Write integration guide (Nickel → Rust)

**Acceptance Criteria**:
- [ ] README with quickstart
- [ ] API docs for all public functions
- [ ] Example workflows documented

---

## Backlog (Future Sprints)

### Hardware Wallet Integration
- Ledger support
- Trezor support
- COLDCARD support

### Advanced Features
- Key caching/memoization
- Parallel keychain processing
- WebAssembly compilation

### Security Enhancements
- Seed phrase zeroization
- Encrypted keychain storage
- Audit logging

---

## TDD Commit Tracking

### Commit Format

```
<type>: <description>

<TDD phase>: <details>

TDD-Phase: RED|GREEN|REFACTOR
TDD-Previous: <commit-hash> (if GREEN or REFACTOR)
Related: spec/tasks.md#<task-id>
```

### Commit Log

*Commits will be tracked here as implementation progresses*

---

## Notes

- Follow .claude/development-workflow.md for TDD cycle
- Update this file immediately after each commit
- Use proper test vectors from official sources
- Reference design.md for implementation guidance
