# BIP-Keychain Rust Implementation - Requirements

**Project**: bip-keychain (Rust implementation)
**Version**: 0.1.0
**Date**: 2025-10-21
**Spec Type**: Kiro Specs - Requirements (EARS Notation)

## Overview

Build a Rust library and CLI tool that implements BIP-Keychain semantic hierarchical key derivation with multi-schema support beyond the original spec.

## User Stories (EARS Notation)

### US-1: Multi-Hash Entity Derivation

**WHEN** a user provides a JSON entity and parent entropy,
**THE SYSTEM SHALL** hash the entity using the configured hash function (HMAC-SHA-512, BLAKE2b, or SHA-256),
**AND** convert the first 4 bytes to a uint32 BIP-32 child index.

**Acceptance Criteria**:
- Supports HMAC-SHA-512 (default, BIP-85 standard)
- Supports BLAKE2b (Blockchain Commons compatibility)
- Supports SHA-256 (alternative)
- Produces deterministic indices for identical inputs
- Uses canonical JSON serialization

### US-2: Multi-Schema Entity Parsing

**WHEN** a user exports a Nickel schema to JSON,
**THE SYSTEM SHALL** parse all supported schema types:
- schema.org (JSON-LD)
- W3C DIDs
- Blockchain Commons Gordian Envelope
- W3C Verifiable Credentials
- X.509 Distinguished Names
- DNS/FQDN
- IPFS Content Identifiers
- URNs
- Custom schemas

**Acceptance Criteria**:
- Type-safe deserialization with serde
- Clear error messages for invalid schemas
- Preserves all entity metadata

### US-3: BIP-32 Key Derivation

**WHEN** a user provides a seed phrase and entity,
**THE SYSTEM SHALL** derive a BIP-32 extended key at path `m/83696968'/67797668'/{entity_index}'`,
**AND** support both hardened and non-hardened derivation.

**Acceptance Criteria**:
- Compatible with BIP-32 standard
- Supports hardened derivation (default)
- Supports non-hardened derivation (for PKI use cases)
- Validates derivation depth limits

### US-4: CLI Key Derivation

**WHEN** a user runs `bip-keychain derive --input entity.json --seed-phrase "words..."`,
**THE SYSTEM SHALL** output the derived key in the requested format (SSH, GPG, hex, or JSON).

**Acceptance Criteria**:
- Reads Nickel-exported JSON
- Accepts seed phrase via CLI or environment variable
- Outputs SSH public key format
- Outputs raw hex
- Outputs JSON with metadata

### US-5: Keychain Batch Derivation

**WHEN** a user provides a keychain JSON with multiple derivations,
**THE SYSTEM SHALL** derive all keys in the keychain,
**AND** output results with associated metadata.

**Acceptance Criteria**:
- Processes entire keychain from single JSON
- Outputs all derived keys with purposes
- Handles errors gracefully per-key
- Reports progress for large keychains

### US-6: Seed Phrase Generation

**WHEN** a user runs `bip-keychain generate-seed --word-count 24`,
**THE SYSTEM SHALL** generate a BIP-39 mnemonic seed phrase.

**Acceptance Criteria**:
- Supports 12, 15, 18, 21, and 24 word mnemonics
- Uses cryptographically secure randomness
- Validates checksums
- Outputs human-readable format

## Non-Functional Requirements

### NFR-1: Security

**THE SYSTEM SHALL** never log or persist seed phrases or private keys.

### NFR-2: Performance

**THE SYSTEM SHALL** derive a single key in under 100ms on modern hardware.

### NFR-3: Compatibility

**THE SYSTEM SHALL** produce bit-for-bit identical results to reference implementations for BLAKE2b (libsodium).

### NFR-4: Error Handling

**THE SYSTEM SHALL** provide clear error messages with actionable guidance.

## Dependencies

- Nickel (for schema validation, separate from Rust implementation)
- Rust toolchain (1.70+)
- libsodium (for alkali/BLAKE2b)

## Out of Scope (v0.1.0)

- Hardware wallet integration
- Key storage/management
- GUI application
- Network operations
- Smart contract integration

## References

- [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP-85](https://bips.dev/85/)
- [BIP-Keychain](https://github.com/akarve/bip-keychain)
- [Kiro Specs](https://kiro.dev/docs/specs/)
