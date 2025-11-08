# BIP-Keychain Core - Product Roadmap

**Vision**: Build semantic key infrastructure using BIP-Keychain + Blockchain Commons ecosystem.

**Last Updated**: 2025-11-08

---

## 🎯 Project Vision

**Traditional PKI**: Certificates, Certificate Authorities, Complex, Centralized

**Semantic Key Infrastructure**:
- Entities (Gordian Envelopes) instead of certificates
- Self-sovereign derivation instead of CAs
- Simple, decentralized, privacy-preserving

**Key Insight**:
> "The entity definition IS the certificate. Deriving a key from it IS the signing ceremony. The envelope's provenance IS the certificate chain."

You're not just managing keys - you're building **verifiable, auditable, privacy-preserving identity infrastructure** using semantic entities.

---

## 📋 Current Status (v0.1.0)

**Production-Ready MVP** ✅

### Completed
- ✅ Multi-hash support (HMAC-SHA-512, BLAKE2b, SHA-256)
- ✅ BIP-32 hierarchical key derivation
- ✅ Ed25519 keypair generation
- ✅ Multiple output formats (SSH, GPG, hex, JSON)
- ✅ CLI tool with secure seed generation
- ✅ 50 tests passing (unit, integration, property-based)
- ✅ Comprehensive documentation (7 guides)
- ✅ CI/CD pipeline (GitHub Actions)

### Architecture Ready For
- ✅ Gordian Envelope parsing (entity.rs has SchemaType enum)
- ✅ BLAKE2b via libsodium (BC compatibility)
- ✅ Feature flags (Cargo.toml structure supports optional dependencies)
- ✅ Hash-agnostic derivation (any bytes → u32 index → key)

**What Works Right Now**:
```bash
# Generate seed
bip-keychain generate-seed --words 24

# Derive SSH key from semantic entity
export BIP_KEYCHAIN_SEED="..."
bip-keychain derive entity.json
```

---

## 🚀 Release Timeline

### v0.1.1 - Polish (In Progress - 1 week)
**Focus**: Production readiness

- [x] Fix compiler warnings
- [x] Implement `generate-seed` command
- [x] CI/CD pipeline (multi-platform testing)
- [ ] Thorough review & testing (REVIEW-CHECKLIST.md)
- [ ] Publish to crates.io (HOLD - pending review)

**Outcome**: Stable, well-tested foundation

---

### v0.2.0 - Enhanced (3 months)
**Focus**: Usability & Blockchain Commons Foundation

#### Core Features
1. **Batch Keychain Processing**
   - Derive multiple keys from single JSON
   - Progress reporting
   - Per-key error handling
   - **Use Case**: Generate all SSH keys for infrastructure at once

2. **OpenSSH Private Key Format**
   - Full SSH authentication (not just public keys)
   - Passphrase encryption support
   - **Use Case**: Complete SSH workflow without manual conversion

3. **Performance Benchmarks**
   - Criterion.rs integration
   - Track: derivation time, hash time, keygen time
   - CI regression tracking
   - **Use Case**: Verify performance claims

4. **Shell Completions**
   - Bash, Zsh, Fish support
   - Generated via clap
   - **Use Case**: Better CLI UX

#### Blockchain Commons Integration - Phase 1

5. **Gordian Envelope Parsing** (Medium effort)
   ```toml
   [features]
   gordian = ["bc-envelope", "bc-components", "dcbor"]
   ```
   - Read Gordian Envelopes as entities
   - Parse envelope → CBOR → hash → derive
   - Minimal code changes (~30 lines in entity.rs)
   - **Use Case**: BC ecosystem compatibility

6. **UR (Uniform Resources) Encoding** (Low effort)
   - Export entities as UR for QR codes
   - Import entities from UR-encoded QR scans
   - Multipart UR for large payloads
   - **Use Case**: Airgapped key derivation workflows

7. **SSKR Seed Backup** (Low effort)
   ```bash
   bip-keychain generate-seed --sskr --threshold 2 --shares 3
   ```
   - Threshold recovery (2-of-3, 3-of-5, etc.)
   - Geographic distribution of shares
   - **Use Case**: Organizational key management, no single point of failure

**Outcome**: Enhanced usability + BC ecosystem foundation

---

### v0.3.0 - Collaboration (6 months)
**Focus**: Multi-party workflows & Enterprise governance

#### Blockchain Commons Integration - Phase 2

1. **Provenance Support**
   - Who created the entity?
   - When was it created?
   - Who approved it?
   - When should keys be rotated?

   ```
   Envelope:
     subject: Entity JSON
     assertions:
       - createdBy: did:key:alice
       - createdAt: 2025-11-08
       - approvedBy: [bob, carol]
       - rotationDate: 2026-11-08
   ```
   **Use Case**: Auditable key derivation with governance

2. **Elision Support** (Privacy-Preserving Derivation)
   - Derive keys from full entity
   - Prove you used partial entity
   - Zero-knowledge proof of key derivation

   ```
   Full:    {name: "Alice", email: "alice@secret.com"}
   Elided:  {name: "Alice", email: ELIDED}
   Result:  Same key, but email stays private!
   ```
   **Use Case**: Privacy in collaborative workflows

3. **Gordian Coordinator Integration**
   - Collaborative entity definition
   - Multi-party approval workflows
   - Distributed consensus on entities
   **Use Case**: Teams jointly define keys without centralized authority

4. **Lifehash Visual Verification**
   - Beautiful visual fingerprints of entities
   - Easier verification than hex hashes
   - Preview before key derivation
   **Use Case**: UX improvement for key verification

#### Advanced Features

5. **Hardware Wallet Integration**
   - Ledger Nano S/X support
   - Trezor support
   - COLDCARD support
   - **Use Case**: Derive keys on-device, never expose seed

6. **Key Rotation Automation**
   - Time-based policies from envelope metadata
   - Automated rotation alerts
   - Graceful key transition
   **Use Case**: Enterprise key lifecycle management

**Outcome**: Multi-party collaboration, enterprise-ready

---

### v0.4.0 - Advanced (12 months)
**Focus**: Complete BC integration & Advanced cryptography

#### Blockchain Commons Integration - Phase 3

1. **Envelope Expressions** (Query-based derivation)
   ```
   Single organizational envelope
   Query: "Engineering.Backend.AuthService"
   Result: Derive key for that specific team/service
   ```
   **Use Case**: Single source of truth, many derived keys

2. **Verifiable Credentials as Entities**
   - Derive keys from W3C VCs
   - Keys tied to verifiable authority
   - Auto-rotation on credential revocation

   ```json
   {
     "type": "EmployeeCredential",
     "issuer": "did:web:acme.com",
     "credentialSubject": {
       "id": "did:key:alice",
       "employeeId": "E12345"
     }
   }
   ```
   **Use Case**: Self-sovereign key management with verifiable authority

3. **Distributed Key Recovery**
   - Social recovery with SSKR
   - M-of-N guardians
   - Time-locked recovery
   **Use Case**: Inheritance, account recovery, business continuity

4. **Interop with Gordian Coordinator**
   - Network integration
   - Remote entity synchronization
   - Collaborative workflows over network
   **Use Case**: Distributed teams, organizational coordination

#### Platform Expansion

5. **WebAssembly Build**
   - Compile to WASM for browser use
   - JavaScript bindings
   - Demo web app
   **Use Case**: Browser-based key derivation

6. **GUI Application**
   - Cross-platform desktop app (Tauri/egui)
   - Visual keychain management
   - QR code scanning
   - **Use Case**: Non-technical users

7. **Mobile Apps**
   - iOS and Android
   - Camera QR scanning
   - Biometric seed protection
   **Use Case**: Mobile-first workflows

**Outcome**: Complete BC ecosystem integration, broad platform support

---

## 🎯 High-Value Integration Opportunities

### 1. Progressive Trust & Multi-Party Workflows

**Scenario**: Collaborative entity definition without centralized key generation

```
Alice, Bob, Carol → Define entity collaboratively
                 → Each derives their OWN key from SAME entity
                 → Combine in multisig
                 → Auditable, reproducible
```

**Why it matters**: Coordinate the *entity definition*, not key generation

### 2. Airgapped Key Derivation with UR Codes

**Workflow**:
```
Hot Machine:   Define entity → Encode as UR → Display QR
Airgapped:     Scan QR → Decode entity → Derive key → Export pubkey via QR
Hot Machine:   Scan pubkey QR
```

**Why it matters**: Hardware wallet-level security without specialized hardware

### 3. Elision: Privacy-Preserving Derivation

**Scenario**: Derive from sensitive data, prove without revealing

```
Full Entity:    {name, email, SSN, department}
Derive key from full entity ✓

Elided Entity:  {name, email, ELIDED, department}
Prove you used this entity ✓
Others verify without knowing SSN ✓
```

**Why it matters**: Zero-knowledge proof of key derivation

### 4. Provenance & Auditable Key Derivation

**Every key has metadata**:
- Who authorized it?
- When was it created?
- When should it rotate?
- What's it for?
- Who approved it?

**Why it matters**: Enterprise governance, compliance, audit trails

### 5. SSKR for Organizational Key Management

**Enterprise policies via cryptography**:
```
Critical infrastructure: 3-of-5 executives
Development keys:       2-of-3 leads
Testing keys:           1-of-2 developers
```

**Why it matters**: Cryptographic policy enforcement, business continuity

### 6. Verifiable Credentials as Authority

**Keys tied to credentials**:
```
Employer issues credential → Employee derives key from it
Credential revoked → Employee knows to rotate
Self-sovereign but verifiable!
```

**Why it matters**: Self-sovereign key management with organizational authority

### 7. Envelope Expressions: Single Source of Truth

**Query complex structures**:
```
Organizational envelope (one)
  ├─ Engineering.Backend.AuthService
  ├─ Engineering.Frontend.WebApp
  ├─ Sales.WestCoast.Q4Campaign
  └─ ...

Query determines key, not separate entities!
```

**Why it matters**: Scalable key management from unified structure

---

## 🏗️ Architecture Principles

### 1. Modular Design
- Core derivation is hash-agnostic (any bytes → index → key)
- BC features as optional cargo features
- Zero-cost abstractions

### 2. Unix Philosophy
- Do one thing well (derive keys from entities)
- Compose with other tools
- Text-based, pipe-friendly

### 3. Security First
- Seed phrases via env vars, never CLI args
- Private keys never logged
- Secure RNG (getrandom)
- Standard crypto libraries (audited)

### 4. Standards Compliance
- BIP-32, BIP-39, BIP-85
- W3C DIDs, Verifiable Credentials
- Blockchain Commons specifications
- Schema.org vocabulary

### 5. User Control
- Self-sovereign (you control your seed)
- No network requirements (offline-first)
- Open source, auditable
- No vendor lock-in

---

## 🎓 Use Cases by Timeline

### Available Now (v0.1.0)
- ✅ SSH key management (per-server keys)
- ✅ Git commit signing (GPG integration)
- ✅ GitHub deploy keys (per-repository)
- ✅ Personal identity (DID-based)

### Near Term (v0.2.0 - 3 months)
- 🔜 Batch infrastructure provisioning
- 🔜 Airgapped workflows (UR + QR)
- 🔜 Organizational seed backup (SSKR)
- 🔜 BC ecosystem compatibility

### Medium Term (v0.3.0 - 6 months)
- 🔜 Multi-party key coordination
- 🔜 Privacy-preserving derivation
- 🔜 Enterprise governance & audit
- 🔜 Hardware wallet integration

### Long Term (v0.4.0 - 12 months)
- 🔜 Credential-based authority
- 🔜 Organizational hierarchies
- 🔜 Social recovery
- 🔜 Browser & mobile support

---

## 🤝 Community & Ecosystem

### Blockchain Commons Collaboration
- Already using BLAKE2b via libsodium (BC standard)
- Architecture designed for Gordian Envelopes
- SSKR reference in generate-seed warnings
- Opportunity to be THE reference implementation

### Potential Partnerships
1. **Blockchain Commons** - Core ecosystem alignment
2. **Schema.org** - Semantic web integration
3. **W3C DID/VC communities** - Identity standards
4. **Hardware wallet manufacturers** - Device integration

### Open Source Strategy
- BSD-2-Clause license (permissive)
- Thorough documentation
- Comprehensive tests
- Public roadmap (this document)
- Community-driven development

---

## 📊 Success Metrics

### Technical Excellence
- Test coverage > 80%
- Zero critical security issues
- Sub-100ms key derivation
- Multi-platform support (Linux, macOS, Windows)

### Adoption
- 100+ stars on GitHub (v0.2.0)
- 1000+ crate downloads (v0.3.0)
- Used in production by 10+ organizations (v0.4.0)
- Reference implementation for BC ecosystem

### Ecosystem Impact
- Contributions to BC specifications
- Collaboration with BC team
- Integration with other BC tools
- Standards proposals (BIP process)

---

## 🚫 Out of Scope

These are explicitly **not** planned:

- ❌ Key storage/management (use ssh-agent, gpg-agent)
- ❌ Network operations (fetching schemas, syncing)
- ❌ Smart contract integration
- ❌ Cryptocurrency wallet features (use dedicated wallets)
- ❌ Cloud backup services (security risk)
- ❌ Proprietary features or vendor lock-in

**Philosophy**: Build focused, composable tools that work with existing infrastructure.

---

## 💭 Open Questions

### Research Areas
1. **Performance at Scale**: How does derivation perform with 10,000+ entities?
2. **Envelope Elision Patterns**: What are common elision use cases?
3. **Key Rotation Policies**: What automation makes sense?
4. **Multi-Signature Workflows**: How to coordinate entity definitions?
5. **Credential Integration**: What VC formats matter most?

### Standards Work
1. Should BIP-Keychain become a formal BIP?
2. W3C DID Method for BIP-Keychain?
3. Schema.org extensions for key metadata?
4. BC specification alignment?

### Community
1. How to attract contributors?
2. What documentation is most valuable?
3. Which platforms to prioritize?
4. Enterprise vs. individual user focus?

---

## 🎯 How to Contribute

**Immediate Needs** (v0.2.0):
- Batch keychain processing implementation
- Performance benchmarking
- BC crate integration (bc-envelope, bc-ur, sskr)
- Additional entity type examples

**Medium Term** (v0.3.0):
- Gordian Coordinator integration research
- Hardware wallet drivers
- Elision pattern documentation
- Enterprise use case studies

**Long Term** (v0.4.0):
- WebAssembly port
- Mobile app development
- GUI design
- Standards proposals

**Always Welcome**:
- Bug reports and fixes
- Documentation improvements
- Test coverage expansion
- Security audits

See [TODO.md](TODO.md) for detailed task breakdown.

---

## 📚 References

### Specifications
- [BIP-Keychain Proposal](https://github.com/akarve/bip-keychain)
- [BIP-32: HD Wallets](https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki)
- [BIP-39: Mnemonic Code](https://github.com/bitcoin/bips/blob/master/bip-0039.mediawiki)
- [BIP-85: Deterministic Entropy](https://bips.dev/85/)

### Blockchain Commons
- [Gordian Envelope](https://github.com/BlockchainCommons/Gordian)
- [SSKR Specification](https://github.com/BlockchainCommons/bc-sskr)
- [UR Specification](https://github.com/BlockchainCommons/Research/blob/master/papers/bcr-2020-005-ur.md)
- [Gordian Principles](https://github.com/BlockchainCommons/Gordian#gordian-principles)

### Standards
- [W3C DIDs](https://www.w3.org/TR/did-core/)
- [W3C Verifiable Credentials](https://www.w3.org/TR/vc-data-model/)
- [Schema.org](https://schema.org/)
- [JSON-LD](https://json-ld.org/)

### Related Projects
- [Blockchain Commons Repositories](https://github.com/BlockchainCommons)
- [seedtool-cli (Rust)](https://github.com/BlockchainCommons/seedtool-cli-rust)
- [Gordian Coordinator](https://github.com/BlockchainCommons/GordianCoordinator-iOS)

---

**Last Updated**: 2025-11-08
**Next Review**: After v0.2.0 release

---

*This roadmap is a living document. Priorities may shift based on community feedback, security considerations, and ecosystem evolution.*
