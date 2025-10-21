# BIP-Keychain Examples

Nickel-based examples demonstrating multi-schema semantic key derivation.

## Overview

Each example shows how to derive cryptographic keys from semantic entities using different schema types. The Nickel contracts provide compile-time type safety and validation.

## Examples

### Schema.org

**github-repo-schema-org.ncl** - Derive Git signing key for a repository
```bash
nickel export github-repo-schema-org.ncl
```

Uses schema.org `SoftwareSourceCode` to create a deterministic signing key tied to a specific GitHub repository.

### Decentralized Identifiers (DIDs)

**did-identity.ncl** - Derive key from W3C DID
```bash
nickel export did-identity.ncl
```

Uses a DID to create an identity-based signing key. Great for self-sovereign identity use cases.

### Blockchain Commons

**gordian-envelope.ncl** - Privacy-preserving key derivation
```bash
nickel export gordian-envelope.ncl
```

Uses Gordian Envelope (UR-encoded CBOR) with selective disclosure support. Demonstrates integration with Blockchain Commons standards.

### Infrastructure

**infrastructure-keys.ncl** - Keys for servers and certificates
```bash
nickel export infrastructure-keys.ncl
```

Shows how to derive multiple infrastructure keys using DNS and X.509 schemas. Includes both hardened (SSH) and non-hardened (PKI) derivation.

### Complete Keychain

**complete-keychain.ncl** - Full personal keychain
```bash
nickel export complete-keychain.ncl
```

Comprehensive example with multiple derivations across different schema types:
- GitHub identity (DID)
- Repository signing keys (schema.org)
- TLS certificates (X.509)
- Privacy credentials (Gordian Envelope)
- Server authentication (DNS)
- Content signing (IPFS CID)

## Supported Schema Types

| Schema Type | Use Case | Hardened? | Example Entity |
|-------------|----------|-----------|----------------|
| `schema_org` | General-purpose entities | Yes | GitHub repos, organizations |
| `gordian_envelope` | Privacy-preserving | Yes | Selective disclosure credentials |
| `did` | Identity | Yes | `did:github:username` |
| `verifiable_credential` | Credentials | Yes | W3C VCs |
| `x509_dn` | PKI | No* | TLS certificates |
| `dns` | Infrastructure | Yes | Server SSH keys |
| `ipfs_cid` | Content-addressed | Yes | Signing specific content versions |
| `urn` | Persistent identifiers | Yes | ISBNs, UUIDs, etc. |
| `custom` | User-defined | Configurable | Any structured data |

\* X.509 typically uses non-hardened derivation for PKI use cases

## How It Works

1. **Define Entity**: Describe what you want a key for using a semantic schema
2. **Nickel Validation**: Type checking ensures entity matches schema contract
3. **Export to JSON**: `nickel export` produces validated JSON
4. **Derive Key**: External tool hashes JSON and derives BIP-32 child key

## Derivation Algorithm

```
entity (JSON/CBOR)
  → serialize (canonical)
  → HMAC-SHA-512(parent_entropy, serialized_entity)
  → first 4 bytes as uint32
  → BIP-32 child index
```

Each schema type may use different:
- Serialization (JSON-LD, CBOR, etc.)
- Hash function (HMAC-SHA-512, BLAKE2b, etc.)
- Hardening policy (based on use case)

## Next Steps

After validating with Nickel:
1. Export to JSON: `nickel export example.ncl > output.json`
2. Use Python/Rust/Go implementation to derive actual keys
3. Store only the seed phrase - keys are regenerated on demand

## References

- [BIP-Keychain Spec](https://github.com/akarve/bip-keychain)
- [BIP-85](https://bips.dev/85/)
- [BIP-32](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [Schema.org](https://schema.org/)
- [Blockchain Commons](https://www.blockchaincommons.com/)
- [W3C DIDs](https://www.w3.org/TR/did-core/)
