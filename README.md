# BIP-Keychain Core

[![CI](https://github.com/daogora-xyz/bip-keychain-core/workflows/CI/badge.svg)](https://github.com/daogora-xyz/bip-keychain-core/actions)
[![License](https://img.shields.io/badge/license-BSD--2--Clause-blue.svg)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org)

**Production-ready Rust implementation of semantic hierarchical key derivation based on [BIP-Keychain](https://github.com/akarve/bip-keychain).**

## What is BIP-Keychain?

BIP-Keychain is a draft Bitcoin Improvement Proposal that extends BIP-85 with semantic derivation paths. It creates a hierarchical key-value store where:

- **Keys**: Meaningful semantic paths (JSON-LD schema.org entities)
- **Values**: Cryptographic secrets derived from those paths
- **Derivation**: HMAC-SHA-512 converts semantic entities to BIP-32 child indices

## Key Innovation

**Separation of path keys from path values**: If the hot master is compromised, only the derivation paths (metadata) are exposed, not the actual secrets.

## Use Cases

- Password management with semantic organization
- SSH key hierarchies based on infrastructure topology
- Git signing keys organized by repository/organization
- PKI with human-readable certificate paths
- API key management with semantic namespacing

## Project Goals

1. **Implement** BIP-Keychain derivation algorithm
2. **Extend** beyond Bitcoin to general-purpose secret management
3. **Integrate** with existing tools (Git, SSH, GPG)
4. **Research** applications in progressive trust systems
5. **Document** security properties and best practices

## Status

**Production-Ready MVP (v0.1.0)** ✅

Core functionality complete and tested:
- ✅ Multi-hash support (HMAC-SHA-512, BLAKE2b, SHA-256)
- ✅ BIP-32 hierarchical key derivation
- ✅ Ed25519 keypair generation
- ✅ SSH & GPG output formats
- ✅ CLI tool with secure seed generation
- ✅ 50 tests passing (unit, integration, property-based)
- ✅ Comprehensive documentation

See [PROJECT-STATUS.md](PROJECT-STATUS.md) for details.

## Installation

### Option 1: Nix Flake (Recommended)

If you use Nix, this is the easiest and most reproducible method:

```bash
# Try it without installing
nix run github:daogora-xyz/bip-keychain-core -- --help

# Install to your profile
nix profile install github:daogora-xyz/bip-keychain-core

# Or add to your NixOS/home-manager configuration
# flake.nix:
# inputs.bip-keychain.url = "github:daogora-xyz/bip-keychain-core";
# environment.systemPackages = [ inputs.bip-keychain.packages.${system}.default ];
```

### Option 2: Cargo (Traditional Rust)

```bash
# Install from source
git clone https://github.com/daogora-xyz/bip-keychain-core
cd bip-keychain-core
cargo install --path .

# Generate a seed phrase
bip-keychain generate-seed --words 24

# Set your seed (use the one you just generated)
export BIP_KEYCHAIN_SEED="your twelve word seed phrase here..."

# Derive an SSH key from a semantic entity
bip-keychain derive examples/github-repo.json

# Output: ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA... github-repo
```

All your keys are reproducible from a single seed phrase!

## Documentation

- **[CLI-USAGE.md](CLI-USAGE.md)** - Complete CLI reference and examples
- **[SSH-KEYS-GUIDE.md](SSH-KEYS-GUIDE.md)** - Using BIP-Keychain for SSH authentication
- **[GIT-SIGNING-GUIDE.md](GIT-SIGNING-GUIDE.md)** - Git commit signing with GPG
- **[NICKEL-WORKFLOW.md](NICKEL-WORKFLOW.md)** - Type-safe entity definitions
- **[TODO.md](TODO.md)** - Roadmap and future enhancements
- **[CLAUDE.md](CLAUDE.md)** - Development guide for contributors

## Features

### Multi-Schema Support

Derive keys from semantic entities in multiple formats:
- **schema.org** (JSON-LD) - Person, Organization, SoftwareSourceCode
- **W3C DIDs** - Decentralized Identifiers
- **Blockchain Commons Gordian Envelope** (planned - see TODO.md)
- **X.509 Distinguished Names** - TLS/SSL certificates
- **DNS/FQDN** - Server infrastructure
- **Custom schemas** - Your own semantic structures

### Multi-Hash Functions

Choose the hash function for your ecosystem:
- **HMAC-SHA-512** - BIP-85 standard (default)
- **BLAKE2b** - Blockchain Commons compatibility (via libsodium)
- **SHA-256** - Alternative for specific use cases

### Output Formats

Generate keys in multiple formats:
- **SSH public keys** (OpenSSH format)
- **GPG public keys** (for Git signing)
- **Raw hex** (seed, public key, private key)
- **JSON** (with metadata)

## Architecture

```
JSON Entity → Canonicalize → Hash (HMAC-SHA-512/BLAKE2b/SHA-256)
  → Extract first 4 bytes as u32 → BIP-32 derive at m/83696968'/67797668'/{index}'
  → Generate Ed25519 keypair → Format output (SSH/GPG/hex/JSON)
```

**Key Innovation**: Separation of path keys from path values - if a hot master is compromised, only derivation paths (metadata) are exposed, not the actual secrets.

## Real-World Use Cases

- ✅ **SSH Server Access** - Unique key per server, organized by DNS name
- ✅ **GitHub Deploy Keys** - Per-repository keys, reproducible across teams
- ✅ **Git Commit Signing** - GPG keys for verified commits
- ✅ **Personal Identity** - DID-based keys, consistent across platforms
- ✅ **Infrastructure as Code** - Terraform/Ansible integration
- 🚧 **Email Signing** - S/MIME, PGP (future)
- 🚧 **Code Signing** - Software releases (future)

See [examples/](examples/) for 11 entity examples and 6 automation scripts.

## Security

### What's Secure
- ✅ Seed phrase via environment variable (not CLI args)
- ✅ Cryptographically secure hash functions
- ✅ Standard BIP-32/39 implementations
- ✅ Test vectors from official sources (NIST, RFC 4231, BLAKE2)
- ✅ No key logging or persistence

### User Responsibilities
- ⚠️ Secure seed phrase storage (hardware wallet recommended)
- ⚠️ Private key handling (use ssh-agent, don't write to disk)
- ⚠️ Regular key rotation (generate new entities)

See [PROJECT-STATUS.md](PROJECT-STATUS.md) for security considerations.

## Development

### Using Nix (Recommended)

```bash
# Enter development shell (includes Rust, cargo tools, libsodium)
nix develop

# Or use direnv for automatic shell loading
echo "use flake" > .envrc
direnv allow

# Inside the dev shell:
cargo build
cargo test
cargo run -- derive examples/person-identity.json

# Run checks (tests, clippy, fmt)
nix flake check
```

### Traditional Cargo Workflow

```bash
# Build
cargo build

# Run tests (50 tests)
cargo test

# Run specific test
cargo test test_hmac_sha512

# Build release binary
cargo build --release

# Run CLI
cargo run -- derive entity.json
```

**Note**: If not using Nix, you must install libsodium manually:
- macOS: `brew install libsodium`
- Ubuntu/Debian: `apt install libsodium-dev`
- Arch: `pacman -S libsodium`

See [CLAUDE.md](CLAUDE.md) for development workflow and architecture details.

## Repository Structure

```
.
├── docs/          # Documentation and research
├── examples/      # Example derivation paths and use cases
├── src/           # Implementation code
└── tests/         # Test vectors and validation
```

## Quick Example

```python
# Semantic path: GitHub repository signing key
repo_path = {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/user/repo"
}

# Derivation: m/83696968'/67797668'/{hash(repo_path)}'
# Result: Deterministic signing key for that specific repository
```

## Relationship to Other Projects

- **BIP-Keychain**: Original proposal (this is implementation/research)
- **BIP-85**: Parent standard for deterministic entropy
- **BIP-32**: Underlying hierarchical deterministic wallet tech
- **OpenIntegrity-Nickel-Core**: Exploring BIP-Keychain for cryptographic work stream management

## Contributing

Contributions welcome! Priority areas:
- CI/CD improvements
- Additional entity type examples
- Security audits
- Performance benchmarks
- Integration with other tools

See [TODO.md](TODO.md) for planned features and [CLAUDE.md](CLAUDE.md) for development workflow.

## License

BSD-2-Clause (matching original BIP-Keychain proposal)

## References

- **Original BIP-Keychain**: https://github.com/akarve/bip-keychain
- **BIP-85**: https://bips.dev/85/
- **BIP-32**: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
- **JSON-LD**: https://json-ld.org/
- **Schema.org**: https://schema.org/
