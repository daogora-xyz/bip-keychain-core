# Examples Guide

This directory contains comprehensive examples demonstrating BIP-Keychain's semantic key derivation capabilities across various real-world use cases.

## Quick Start

```bash
# Derive a key from any JSON entity
cargo run -- derive examples/person-identity.json

# Or using the release build
./target/release/bip-keychain derive examples/api-service-key.json

# Run comprehensive demo
./examples/comprehensive-demo.sh

# Test batch derivation
./examples/batch-derive-keys.sh
```

## Overview

BIP-Keychain derives cryptographic keys from **semantic entities** (human-readable JSON) rather than numeric indices. Each example demonstrates:

- **Entity definition**: Schema.org, DID, DNS, or custom JSON structure
- **Derivation configuration**: Hash function (HMAC-SHA-512, BLAKE2b, SHA-256)
- **Purpose**: What the key is used for
- **Metadata**: Additional context (rotation policy, criticality, etc.)

## Examples by Category

### Infrastructure & Server Management

#### Production Environment
**File**: `server-prod.json`
**Schema**: DNS
**Hash**: HMAC-SHA-512
**Purpose**: Production API server SSH keys

```json
{
  "schema_type": "dns",
  "entity": {
    "name": "prod.api.example.com",
    "environment": "production",
    "datacenter": "us-east-1"
  },
  "metadata": {
    "tier": "production",
    "critical": true,
    "rotation_policy": "90-days"
  }
}
```

**Expected Output**:
- Derives Ed25519 keypair for SSH access
- Same entity always produces same key (deterministic)
- Critical production system - 90-day rotation policy

**Use Cases**:
- Infrastructure-as-code deployments (Terraform, Ansible)
- CI/CD pipeline server provisioning
- Zero-trust SSH access without storing keys

#### Staging Environment
**File**: `server-staging.json`
**Schema**: DNS
**Hash**: HMAC-SHA-512
**Purpose**: Staging API server SSH keys

```json
{
  "schema_type": "dns",
  "entity": {
    "name": "staging.api.example.com",
    "environment": "staging",
    "datacenter": "us-east-1"
  },
  "metadata": {
    "tier": "staging",
    "critical": false,
    "rotation_policy": "180-days"
  }
}
```

**Expected Output**:
- Different key than production (different entity)
- Less frequent rotation (180 days)
- Same datacenter, different environment

**Use Cases**:
- Pre-production testing environments
- Integration testing with production-like setup
- Environment isolation via semantic derivation

#### Development Environment
**File**: `server-dev.json`
**Schema**: DNS
**Hash**: HMAC-SHA-512
**Purpose**: Development API server SSH keys

```json
{
  "schema_type": "dns",
  "entity": {
    "name": "dev.api.example.com",
    "environment": "development",
    "datacenter": "us-west-2"
  },
  "metadata": {
    "tier": "development",
    "critical": false,
    "rotation_policy": "never"
  }
}
```

**Expected Output**:
- Long-lived keys (no rotation requirement)
- Different datacenter than prod/staging
- Developer workstation access

**Use Cases**:
- Local development environments
- Developer onboarding (derive from entity, not copy keys)
- Ephemeral dev environments (recreate key anytime)

#### API Service Key
**File**: `api-service-key.json`
**Schema**: Schema.org SoftwareApplication
**Hash**: HMAC-SHA-512
**Purpose**: Inter-service authentication

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "SoftwareApplication",
    "identifier": "payment-processing-api",
    "name": "Payment Processing Service"
  }
}
```

**Expected Output**:
- Ed25519 keypair for API authentication
- JWT signing, mutual TLS, or service mesh identity

**Use Cases**:
- Microservices authentication
- API gateway authorization
- Zero-trust service mesh (Istio, Linkerd)

### IoT & Device Management

#### Individual IoT Device
**File**: `iot-device-key.json`
**Schema**: Schema.org Thing
**Hash**: BLAKE2b
**Purpose**: Single device attestation

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "identifier": "sensor-device-001",
    "name": "Temperature Sensor #1"
  }
}
```

**Expected Output**:
- Device-specific Ed25519 keypair
- Same identifier always produces same key

**Use Cases**:
- Device provisioning at manufacturing
- Secure boot attestation
- Over-the-air (OTA) update verification

#### Fleet-Scale IoT Device
**File**: `iot-fleet-device.json`
**Schema**: Schema.org Thing (extended)
**Hash**: BLAKE2b
**Purpose**: Large-scale fleet management

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "identifier": "device-sensor-temp-001",
    "name": "Temperature Sensor #001",
    "manufacturer": "ACME IoT Corp",
    "model": "TempSense Pro",
    "serialNumber": "TS-001-2025-11-08",
    "location": {
      "@type": "Place",
      "name": "Building A - Floor 3 - Room 301"
    }
  },
  "metadata": {
    "device_type": "temperature_sensor",
    "deployment_date": "2025-11-08",
    "firmware_version": "2.1.0",
    "use_case": "iot_fleet_management"
  }
}
```

**Expected Output**:
- Unique key per device (based on serial number, model, location)
- Supports 10,000+ devices without key database
- Location-aware key derivation

**Use Cases**:
- Smart building sensor networks
- Industrial IoT fleet management (10K-1M devices)
- Automotive fleet telemetry (each vehicle is a unique entity)
- Asset tracking with physical location semantics

### Multi-Tenant SaaS

#### Enterprise Customer
**File**: `saas-customer-acme.json`
**Schema**: Schema.org Organization
**Hash**: HMAC-SHA-512
**Purpose**: Per-customer data encryption

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Organization",
    "identifier": "customer-acme-corp",
    "name": "ACME Corporation",
    "taxID": "12-3456789",
    "foundingDate": "2020-01-15"
  },
  "metadata": {
    "customer_tier": "enterprise",
    "customer_id": "cust_001",
    "onboarding_date": "2025-01-15",
    "use_case": "saas_data_isolation"
  }
}
```

**Expected Output**:
- Customer-specific encryption key
- Verifiable data isolation (different customer = different key)
- Compliance-ready (GDPR, SOC2 data segregation)

**Use Cases**:
- Multi-tenant database encryption (per-customer keys)
- SaaS data isolation (zero-knowledge architecture)
- Customer-specific backups and exports
- Regulatory compliance (GDPR "right to be forgotten" - delete customer entity = unrecoverable data)

#### Startup Customer
**File**: `saas-customer-techstart.json`
**Schema**: Schema.org Organization
**Hash**: HMAC-SHA-512
**Purpose**: Startup tier customer isolation

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Organization",
    "identifier": "customer-techstart-inc",
    "name": "TechStart Inc",
    "taxID": "98-7654321",
    "foundingDate": "2023-06-01"
  },
  "metadata": {
    "customer_tier": "startup",
    "customer_id": "cust_042",
    "onboarding_date": "2025-06-01",
    "use_case": "saas_data_isolation"
  }
}
```

**Expected Output**:
- Different key than enterprise customer
- Tier-aware metadata (startup vs enterprise)
- Same derivation path structure

**Use Cases**:
- Tiered SaaS offerings (startup/pro/enterprise)
- Customer migration (change tier without changing key)
- Customer lifecycle management

### Identity & Authentication

#### Person Identity
**File**: `person-identity.json`
**Schema**: Schema.org Person
**Hash**: HMAC-SHA-512
**Purpose**: Personal identity key

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "identifier": "alice-johnson",
    "givenName": "Alice",
    "familyName": "Johnson",
    "email": "alice@example.com"
  }
}
```

**Expected Output**:
- Personal Ed25519 keypair
- Portable identity across platforms

**Use Cases**:
- Self-sovereign identity (SSI)
- Personal data encryption
- Cross-platform authentication

#### DID Identity
**File**: `did-identity.json`
**Schema**: DID (Decentralized Identifier)
**Hash**: HMAC-SHA-512
**Purpose**: W3C DID key derivation

```json
{
  "schema_type": "did",
  "entity": {
    "did": "did:example:alice123"
  }
}
```

**Expected Output**:
- DID-compliant keypair
- Compatible with DID:key, DID:web methods

**Use Cases**:
- Verifiable credentials
- Decentralized identity systems
- Blockchain identity anchoring

#### Organization Signing
**File**: `organization-signing.json`
**Schema**: Schema.org Organization
**Hash**: HMAC-SHA-512
**Purpose**: Corporate signing key

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Organization",
    "identifier": "acme-corp-signing",
    "name": "ACME Corporation",
    "legalName": "ACME Corporation Inc."
  }
}
```

**Expected Output**:
- Corporate signing keypair
- Verifiable organizational identity

**Use Cases**:
- Document signing (contracts, invoices)
- Software release signing
- Supply chain attestation

### Blockchain & Web3

#### Blockchain Identity
**File**: `blockchain-identity.json`
**Schema**: Schema.org Person
**Hash**: BLAKE2b
**Purpose**: Blockchain address derivation

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "identifier": "blockchain-user-alice"
  }
}
```

**Expected Output**:
- Ed25519 keypair for blockchain signing
- Compatible with Solana, Polkadot, Substrate chains

**Use Cases**:
- Wallet key derivation
- DAO member keys
- NFT signing keys

#### Verifiable Credential
**File**: `verifiable-credential.json`
**Schema**: W3C Verifiable Credential
**Hash**: HMAC-SHA-512
**Purpose**: Credential signing key

```json
{
  "schema_type": "verifiable_credential",
  "entity": {
    "id": "https://example.edu/credentials/3732",
    "type": ["VerifiableCredential", "AlumniCredential"],
    "issuer": "https://example.edu/issuers/565049"
  }
}
```

**Expected Output**:
- Credential issuer signing key
- Verifiable by credential holder and verifiers

**Use Cases**:
- Educational credentials
- Professional certifications
- Government-issued documents (digital ID, licenses)

### Content & Collaboration

#### Email Signing
**File**: `email-signing.json`
**Schema**: Schema.org EmailMessage
**Hash**: HMAC-SHA-512
**Purpose**: Email signing (PGP/GPG alternative)

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "EmailMessage",
    "sender": "alice@example.com"
  }
}
```

**Expected Output**:
- Ed25519 keypair for email signing
- Replaces PGP key management

**Use Cases**:
- Email authentication (DKIM alternative)
- Encrypted email (ProtonMail, Tutanota)
- Secure mailing lists

#### GitHub Repository
**File**: `github-repo.json`
**Schema**: Schema.org SoftwareSourceCode
**Hash**: HMAC-SHA-512
**Purpose**: Repository-specific signing

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "SoftwareSourceCode",
    "identifier": "github-acme-corp-api",
    "codeRepository": "https://github.com/acme-corp/api"
  }
}
```

**Expected Output**:
- Repository-specific commit signing key
- Per-repo access keys (deploy keys)

**Use Cases**:
- Git commit signing (GPG alternative)
- GitHub Actions deploy keys
- Repository access tokens

#### Software Application Signing
**File**: `software-app-signing.json`
**Schema**: Schema.org SoftwareApplication
**Hash**: HMAC-SHA-512
**Purpose**: Software release signing

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "SoftwareApplication",
    "identifier": "acme-desktop-app",
    "name": "ACME Desktop App"
  }
}
```

**Expected Output**:
- Application signing key (code signing)
- Verifiable software releases

**Use Cases**:
- macOS/Windows code signing
- Docker image signing (Notary, Cosign)
- APK/IPA mobile app signing

#### IPFS Content Signing
**File**: `ipfs-content-signing.json`
**Schema**: Schema.org DigitalDocument
**Hash**: BLAKE2b
**Purpose**: IPFS content authentication

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "DigitalDocument",
    "identifier": "ipfs-QmXoypizjW3WknFiJnKLwHCnL72vedxjQkDDP1mXWo6uco"
  }
}
```

**Expected Output**:
- Ed25519 keypair for IPFS content signing
- Verifiable content authorship

**Use Cases**:
- IPFS content signing
- NFT metadata attestation
- Decentralized storage verification

### Namespace & Standards

#### URN Namespace
**File**: `urn-namespace.json`
**Schema**: URN
**Hash**: HMAC-SHA-512
**Purpose**: URN-based key derivation

```json
{
  "schema_type": "urn",
  "entity": {
    "namespace": "example",
    "identifier": "user:alice"
  }
}
```

**Expected Output**:
- URN-compliant keypair
- Supports IETF RFC 8141 URN syntax

**Use Cases**:
- Legacy system integration (URN namespaces)
- ISBN/ISSN digital signing
- Standards-based identifiers

#### X.509 Distinguished Name
**File**: `x509-distinguished-name.json`
**Schema**: X.509 DN
**Hash**: HMAC-SHA-512
**Purpose**: PKI-compatible key derivation

```json
{
  "schema_type": "x509_dn",
  "entity": {
    "CN": "example.com",
    "O": "Example Organization",
    "C": "US"
  }
}
```

**Expected Output**:
- X.509-compliant Ed25519 keypair
- Can be used for TLS certificates (Ed25519 support required)

**Use Cases**:
- TLS certificate signing requests
- Mutual TLS (mTLS) authentication
- PKI infrastructure integration

### Blockchain Commons Ecosystem

#### Gordian Envelope
**File**: `gordian-envelope.json`
**Schema**: Gordian Envelope (CBOR-based)
**Hash**: BLAKE2b
**Purpose**: Privacy-preserving key derivation

```json
{
  "schema_type": "gordian_envelope",
  "entity": {
    "type": "Identity",
    "id": "ur:crypto-seed/abc123"
  }
}
```

**Expected Output**:
- BLAKE2b-derived keypair (BC ecosystem compatibility)
- Supports elision (privacy-preserving redaction)

**Use Cases**:
- Gordian Envelope signing and encryption
- Privacy-preserving verifiable credentials
- Multi-party workflows (progressive trust)
- Airgapped wallet integration (QR codes, UR encoding)

### Test & Development

#### Test Entity
**File**: `test-entity.json`
**Schema**: Schema.org Thing
**Hash**: HMAC-SHA-512
**Purpose**: Simple test case

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "identifier": "test-entity-001"
  }
}
```

**Use Cases**:
- Integration testing
- CI/CD pipeline validation
- Reproducibility verification

#### SHA-256 Example
**File**: `sha256-example.json`
**Schema**: Schema.org Thing
**Hash**: SHA-256 (padded to 64 bytes)
**Purpose**: SHA-256 hash function test

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "identifier": "sha256-test"
  },
  "derivation_config": {
    "hash_function": "sha256",
    "hardened": true
  }
}
```

**Expected Output**:
- SHA-256 hash (padded to 64 bytes)
- First 4 bytes converted to u32 index

**Use Cases**:
- Testing SHA-256 derivation path
- Legacy system compatibility (SHA-256 requirement)

## Shell Scripts & Automation

### Comprehensive Demo
**File**: `comprehensive-demo.sh`
**Description**: Full demonstration of BIP-Keychain capabilities

**What it does**:
1. Generates BIP-39 seed phrase
2. Derives keys for multiple entity types
3. Shows JSON canonicalization and hashing
4. Demonstrates multi-hash support
5. Validates reproducibility

**Usage**:
```bash
./examples/comprehensive-demo.sh
```

### Batch Derivation
**File**: `batch-derive-keys.sh`
**Description**: Batch derive keys from multiple entities

**What it does**:
- Processes all JSON entities in `examples/` directory
- Parallel derivation for performance
- Outputs CSV or JSON results

**Usage**:
```bash
./examples/batch-derive-keys.sh
```

### SSH Server Provisioning
**File**: `ssh-provision-servers.sh`
**Description**: Provision SSH keys for server fleet

**What it does**:
- Derives SSH keys for prod/staging/dev servers
- Generates `authorized_keys` files
- Creates SSH config entries

**Usage**:
```bash
./examples/ssh-provision-servers.sh
```

**Use Cases**:
- Infrastructure-as-code deployments
- Ansible/Terraform integration
- Zero-trust SSH access

### Key Rotation Workflow
**File**: `key-rotation-workflow.sh`
**Description**: Demonstrates key rotation best practices

**What it does**:
1. Derives current key (version N)
2. Derives next key (version N+1) with updated metadata
3. Grace period with both keys active
4. Deactivates old key

**Usage**:
```bash
./examples/key-rotation-workflow.sh
```

**Use Cases**:
- 90-day rotation policies (compliance)
- Zero-downtime key rotation
- Audit trail of key versions

### Backup and Recovery
**File**: `backup-and-recovery.sh`
**Description**: Seed backup and recovery workflow

**What it does**:
- Demonstrates BIP-39 seed backup
- Shows seed recovery process
- Validates key re-derivation from recovered seed

**Usage**:
```bash
./examples/backup-and-recovery.sh
```

**Use Cases**:
- Disaster recovery planning
- Seed phrase backup (paper, metal, Shamir's Secret Sharing)
- Key recovery testing

### Terraform Integration
**File**: `terraform-integration.sh`
**Description**: Integrate with Terraform for infrastructure deployment

**What it does**:
- Derives SSH keys from Terraform resources
- Generates Terraform variables file
- Provisions cloud infrastructure with derived keys

**Usage**:
```bash
./examples/terraform-integration.sh
```

**Use Cases**:
- AWS/GCP/Azure infrastructure deployment
- GitOps workflows (ArgoCD, Flux)
- Infrastructure-as-code with semantic keys

## Integration Examples

### CI/CD Pipeline
**File**: `cicd-integration-example.yml`
**Description**: GitHub Actions workflow using BIP-Keychain

**What it does**:
- Installs BIP-Keychain in CI environment
- Derives deployment keys from repository metadata
- Uses keys for SSH access, container signing, etc.

**Usage**:
```yaml
# .github/workflows/deploy.yml
- name: Install BIP-Keychain
  run: |
    cargo install bip-keychain

- name: Derive deployment key
  run: |
    bip-keychain derive entity.json > deploy_key.json
```

**Use Cases**:
- GitHub Actions, GitLab CI, CircleCI
- Secure deployment without storing keys in CI
- Per-repository or per-environment keys

### Rust Integration
**File**: `derive_key.rs`
**Description**: Embed BIP-Keychain as Rust library

**What it does**:
```rust
use bip_keychain_core::{Entity, derive_key};

let entity = Entity::from_json(r#"{"schema_type": "schema_org", ...}"#)?;
let keypair = derive_key(&entity, &seed)?;
```

**Use Cases**:
- Embed in Rust applications
- Custom key management systems
- Integration with existing Rust crypto libraries

## Schema Type Reference

| Schema Type | Examples | Hash Function | Use Cases |
|-------------|----------|---------------|-----------|
| `schema_org` | person-identity, organization-signing | HMAC-SHA-512 | General-purpose identities, SEO-friendly |
| `did` | did-identity | HMAC-SHA-512 | W3C Decentralized Identifiers |
| `dns` | server-prod, server-staging, server-dev | HMAC-SHA-512 | Infrastructure, server management |
| `urn` | urn-namespace | HMAC-SHA-512 | Legacy systems, IETF standards |
| `x509_dn` | x509-distinguished-name | HMAC-SHA-512 | PKI integration, TLS certificates |
| `verifiable_credential` | verifiable-credential | HMAC-SHA-512 | W3C Verifiable Credentials |
| `gordian_envelope` | gordian-envelope | BLAKE2b | Blockchain Commons ecosystem, privacy-preserving |

## Hash Function Comparison

| Hash Function | Output Size | Use Cases | Ecosystem |
|---------------|-------------|-----------|-----------|
| **HMAC-SHA-512** | 64 bytes | BIP-85 standard, general-purpose | Bitcoin, Ethereum, most blockchains |
| **BLAKE2b** | 64 bytes | High-performance, Blockchain Commons | Polkadot, Zcash, Gordian tools |
| **SHA-256** | 32 bytes (padded to 64) | Legacy compatibility | Bitcoin (historical) |

## Best Practices

### 1. Schema Type Selection
- **Use schema.org** for maximum interoperability (Google, Microsoft support)
- **Use DID** for W3C verifiable credentials and SSI
- **Use DNS** for infrastructure and server management
- **Use gordian_envelope** for Blockchain Commons integration

### 2. Hash Function Selection
- **HMAC-SHA-512**: Default choice (BIP-85 standard)
- **BLAKE2b**: Blockchain Commons ecosystem, high performance
- **SHA-256**: Only if required by legacy systems

### 3. Hardened Derivation
- Always use `"hardened": true` for security
- Hardened indices prevent parent key exposure

### 4. Metadata Usage
- Track `rotation_policy` for compliance
- Use `critical` flag for prioritization
- Include `use_case` for auditability

### 5. Semantic Versioning
- Update entity `version` or `@context` for key rotation
- Maintain backward compatibility during migration

## Troubleshooting

### Keys don't match expected output
- Verify JSON canonicalization (sorted keys, no whitespace)
- Check hash function matches (HMAC-SHA-512 vs BLAKE2b)
- Ensure hardened derivation is enabled
- Validate seed phrase is correct

### Entity validation fails
- Check schema type is supported
- Verify JSON structure matches schema
- Ensure required fields are present

### Performance issues with large fleets
- Use batch derivation script
- Consider caching derived keys
- Pre-generate keys for known entities

## Contributing Examples

To add a new example:

1. Create JSON entity file in `examples/`
2. Add description to this EXAMPLES.md
3. Include expected output
4. Document use case
5. Test reproducibility

Example template:
```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "YourType",
    "identifier": "unique-id"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Brief description",
  "metadata": {
    "use_case": "category"
  }
}
```

## Next Steps

- Explore the examples that match your use case
- Run `comprehensive-demo.sh` to see end-to-end workflow
- Integrate BIP-Keychain into your infrastructure (see `terraform-integration.sh`)
- Consult `ROADMAP.md` for upcoming features (Gordian Envelope, SSKR, UR encoding)

## Support

For questions or issues:
- GitHub Issues: https://github.com/daogora-xyz/bip-keychain-core/issues
- Documentation: See `README.md`, `ROADMAP.md`, `spec/design.md`
- Community: Blockchain Commons forum for BC-specific integration
