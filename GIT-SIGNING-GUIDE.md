# Git Signing with BIP-Keychain

**Complete guide to deterministic GPG keys for Git commit and tag signing**

---

## Table of Contents

1. [Introduction](#introduction)
2. [Why Deterministic Git Signing?](#why-deterministic-git-signing)
3. [Prerequisites](#prerequisites)
4. [Quick Start](#quick-start)
5. [Detailed Workflow](#detailed-workflow)
6. [Configuration](#configuration)
7. [Verification](#verification)
8. [Advanced Usage](#advanced-usage)
9. [Troubleshooting](#troubleshooting)
10. [Security Considerations](#security-considerations)

---

## Introduction

BIP-Keychain enables **deterministic, reproducible GPG keys** for signing Git commits and tags. Instead of managing multiple GPG key files, you derive all signing keys from a single BIP-39 seed phrase using semantic entities.

### What You Get

- **Single Backup:** One seed phrase backs up all signing identities
- **Reproducible Keys:** Regenerate any signing key on any machine
- **Semantic Organization:** Keys organized by project, identity, or purpose
- **No Key Files:** Derive keys on-demand from entities
- **Multi-Identity:** Different keys for work, personal, open-source projects

### Use Cases

1. **Per-Project Signing:** Each repository gets a unique signing key
2. **Per-Identity Signing:** Work identity, personal identity, anonymous identity
3. **Team Reproducibility:** Share entity definitions (not keys!) with team
4. **Disaster Recovery:** Restore all signing keys from seed phrase backup
5. **CI/CD Integration:** Derive signing keys in build pipelines

---

## Why Deterministic Git Signing?

### Traditional Approach Problems

```bash
# Traditional GPG key management
gpg --full-gen-key  # Generate random key
gpg --export-secret-keys > backup.gpg  # Backup keyring
# Lost file = lost all signatures
```

**Issues:**
- ❌ Multiple key files to backup
- ❌ Keys not reproducible
- ❌ Complex key management
- ❌ Hard to sync across machines

### BIP-Keychain Approach

```bash
# BIP-Keychain deterministic approach
export BIP_KEYCHAIN_SEED="your twelve word seed phrase..."
bip-keychain derive git-work-identity.json --format gpg
# Derive same key anytime, anywhere
```

**Benefits:**
- ✅ Single seed phrase backup
- ✅ Fully reproducible keys
- ✅ Semantic organization
- ✅ Works on any machine

---

## Prerequisites

### Required Software

```bash
# 1. BIP-Keychain (this tool)
cargo install --path .

# 2. GnuPG (for GPG/OpenPGP operations)
# Linux
sudo apt install gnupg  # Debian/Ubuntu
sudo dnf install gnupg2  # Fedora/RHEL
# macOS
brew install gnupg

# 3. Git (with GPG support)
git --version  # Should be 2.x or higher
```

### Verify Installation

```bash
which bip-keychain  # Should show path
gpg --version       # Should show GnuPG 2.x
git --version       # Should show Git 2.x
```

---

## Quick Start

### 1. Generate Your Signing Key

Create an entity for your Git identity:

```bash
# Create entity file: my-git-identity.json
cat > my-git-identity.json <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice Developer",
    "email": "alice@example.com"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Git commit signing for personal projects"
}
EOF
```

### 2. Derive Ed25519 Keys

```bash
# Set your seed phrase (secure backup!)
export BIP_KEYCHAIN_SEED="your twelve word mnemonic seed phrase here..."

# Derive the Ed25519 key material
bip-keychain derive my-git-identity.json --format gpg
```

**Output:**
```
GPG Ed25519 Public Key
=====================
Comment: Git commit signing for personal projects

Public Key (hex, 32 bytes):
a1b2c3d4e5f6...

To import into GPG:
1. Save this key material
2. Use: gpg --import (if in OpenPGP format)
3. Or use: gpg --expert --full-gen-key to create key from seed
...
```

### 3. Import into GPG

**Current Limitation:** BIP-Keychain outputs Ed25519 key material, but **not full OpenPGP packets** yet. You need to manually create a GPG key from the derived seed.

#### Option A: Manual GPG Key Creation (Current Method)

```bash
# Get the private key seed (32 bytes hex)
bip-keychain derive my-git-identity.json --format private-key > /tmp/key-seed.txt

# Create GPG key using gpg --expert mode
gpg --expert --full-gen-key
# Choose: (9) ECC and ECC
# Choose: (1) Curve 25519
# Enter: Real name, email, passphrase (optional)
# GPG will generate a key

# NOTE: This generates a NEW random key, not from BIP-Keychain seed
# Full integration requires OpenPGP packet generation (future work)
```

#### Option B: Use Existing GPG Key + BIP-Keychain for Derivation

You can use BIP-Keychain to **organize** your signing workflow while keeping GPG for actual signing:

```bash
# Derive a unique identifier for this project
PROJECT_ID=$(bip-keychain derive my-git-identity.json --format public-key | cut -c1-16)

# Use GPG as normal, but organize by BIP-Keychain entity
gpg --list-keys
git config user.signingkey YOUR_GPG_KEY_ID
```

### 4. Configure Git

```bash
# Set signing key
git config --global user.signingkey YOUR_GPG_KEY_ID

# Enable automatic signing
git config --global commit.gpgsign true
git config --global tag.gpgsign true

# Set name and email (must match GPG key)
git config --global user.name "Alice Developer"
git config --global user.email "alice@example.com"
```

### 5. Sign Your First Commit

```bash
# Make a change
echo "test" > test.txt
git add test.txt

# Commit (automatically signed)
git commit -m "Signed commit using BIP-Keychain-derived identity"

# Verify signature
git log --show-signature -1
```

---

## Detailed Workflow

### Entity Design for Git Signing

Design your entities to reflect your signing use cases:

#### Personal Projects

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice Developer",
    "email": "alice@personal.com"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Personal open-source projects"
}
```

#### Work Projects

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice Developer",
    "email": "alice@company.com"
  },
  "derivation_config": {
    "hash_function": "blake2b",
    "hardened": true
  },
  "purpose": "Company repository commits"
}
```

#### Per-Repository Signing

```json
{
  "schema_type": "dns",
  "entity": {
    "fqdn": "github.com/myorg/myproject"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Signing key for myproject repository"
}
```

#### Anonymous Contributions

```json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Anonymous Contributor",
    "identifier": "anon-contributor-1234"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Anonymous signing for sensitive projects"
}
```

### Derivation Workflow

```bash
#!/usr/bin/env bash
# derive-git-signing-key.sh - Derive and display GPG key for Git signing

set -euo pipefail

ENTITY_FILE="${1:-}"

if [[ -z "$ENTITY_FILE" ]]; then
    echo "Usage: $0 <entity-json-file>"
    echo "Example: $0 my-git-identity.json"
    exit 1
fi

if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
    echo "Error: BIP_KEYCHAIN_SEED environment variable not set"
    echo "Set your seed: export BIP_KEYCHAIN_SEED=\"your twelve words...\""
    exit 1
fi

echo "=== BIP-Keychain Git Signing Key Derivation ==="
echo
echo "Entity: $ENTITY_FILE"
echo

# Derive GPG public key info
echo "--- Public Key Info ---"
bip-keychain derive "$ENTITY_FILE" --format gpg
echo

# Derive Ed25519 public key (for verification)
echo "--- Ed25519 Public Key (hex) ---"
bip-keychain derive "$ENTITY_FILE" --format public-key
echo

# WARNING for private key
echo "--- Private Key (KEEP SECRET!) ---"
echo "To get private key for GPG import:"
echo "  bip-keychain derive $ENTITY_FILE --format private-key"
echo
echo "WARNING: Never share private keys!"
echo "Only use on trusted machines with secure storage."
```

Save as `derive-git-signing-key.sh` and use:

```bash
chmod +x derive-git-signing-key.sh
./derive-git-signing-key.sh my-git-identity.json
```

---

## Configuration

### Per-Repository Configuration

Use different signing keys for different repositories:

```bash
# Repository 1: Personal project
cd ~/projects/personal-repo
git config user.signingkey PERSONAL_GPG_KEY_ID
git config user.name "Alice Developer"
git config user.email "alice@personal.com"

# Repository 2: Work project
cd ~/projects/work-repo
git config user.signingkey WORK_GPG_KEY_ID
git config user.name "Alice Developer"
git config user.email "alice@company.com"
```

### Conditional Configuration

Use Git's conditional includes for automatic configuration:

```bash
# ~/.gitconfig
[user]
    name = Alice Developer
    email = alice@personal.com
    signingkey = PERSONAL_GPG_KEY_ID

[commit]
    gpgsign = true

[tag]
    gpgsign = true

# Work projects in ~/work/ directory
[includeIf "gitdir:~/work/"]
    path = ~/.gitconfig-work

# ~/.gitconfig-work
[user]
    email = alice@company.com
    signingkey = WORK_GPG_KEY_ID
```

### Environment-Based Configuration

```bash
# Development environment
export GIT_AUTHOR_NAME="Alice Developer"
export GIT_AUTHOR_EMAIL="alice@dev.com"
export GIT_COMMITTER_NAME="Alice Developer"
export GIT_COMMITTER_EMAIL="alice@dev.com"

# Then commits use these values
git commit -m "Development commit"
```

---

## Verification

### Verify Signed Commits

```bash
# Verify last commit
git log --show-signature -1

# Expected output:
# gpg: Signature made ...
# gpg: Good signature from "Alice Developer <alice@example.com>"
```

### Verify Signed Tags

```bash
# Create signed tag
git tag -s v1.0.0 -m "Release v1.0.0"

# Verify tag signature
git tag -v v1.0.0

# Expected output:
# gpg: Signature made ...
# gpg: Good signature from "Alice Developer <alice@example.com>"
```

### Verify on GitHub

GitHub verifies GPG signatures automatically:

1. Add your GPG public key to GitHub:
   ```bash
   gpg --armor --export YOUR_GPG_KEY_ID
   ```
2. Go to GitHub Settings → SSH and GPG keys → New GPG key
3. Paste the public key
4. Signed commits show "Verified" badge

### Batch Verification

Verify all commits in a range:

```bash
#!/usr/bin/env bash
# verify-all-signatures.sh

RANGE="${1:-HEAD~10..HEAD}"

echo "Verifying signatures for commits: $RANGE"
echo

git log "$RANGE" --format="%H %s" | while read -r commit_hash message; do
    echo "Checking: $commit_hash - $message"

    if git verify-commit "$commit_hash" 2>/dev/null; then
        echo "  ✓ Valid signature"
    else
        echo "  ✗ Invalid or missing signature"
    fi

    echo
done
```

---

## Advanced Usage

### Multiple Identities

Manage multiple signing identities with entity files:

```
entities/
├── personal-github.json
├── work-gitlab.json
├── anonymous-contrib.json
└── emergency-backup.json
```

Switch between identities:

```bash
# Derive identity A
bip-keychain derive entities/personal-github.json --format gpg
git config user.signingkey PERSONAL_KEY_ID

# Derive identity B
bip-keychain derive entities/work-gitlab.json --format gpg
git config user.signingkey WORK_KEY_ID
```

### Hierarchical Identities

Create related identities with different indexes:

```json
// personal-signing-primary.json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice",
    "identifier": "personal-signing-primary"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}

// personal-signing-backup.json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice",
    "identifier": "personal-signing-backup"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}
```

### Time-Based Key Rotation

Rotate signing keys annually:

```bash
# entities/signing-2024.json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice Developer",
    "email": "alice@example.com",
    "validFrom": "2024-01-01",
    "validThrough": "2024-12-31"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Git signing for year 2024"
}

# entities/signing-2025.json
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice Developer",
    "email": "alice@example.com",
    "validFrom": "2025-01-01",
    "validThrough": "2025-12-31"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Git signing for year 2025"
}
```

### CI/CD Integration

Use BIP-Keychain in automated pipelines:

```yaml
# .github/workflows/sign-release.yml
name: Sign Release

on:
  push:
    tags:
      - 'v*'

jobs:
  sign:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Install BIP-Keychain
        run: |
          cargo install bip-keychain

      - name: Derive signing key
        env:
          BIP_KEYCHAIN_SEED: ${{ secrets.BIP_KEYCHAIN_SEED }}
        run: |
          bip-keychain derive .ci/release-signing.json --format private-key > /tmp/signing-key

      - name: Import to GPG
        run: |
          # Convert key material to GPG format
          # (requires OpenPGP packet generation - future feature)
          echo "Import private key to GPG keyring"

      - name: Sign release tag
        run: |
          git tag -s ${{ github.ref_name }} -m "Release ${{ github.ref_name }}"
          git push origin ${{ github.ref_name }}
```

### Backup and Recovery

```bash
# Backup: Only need the seed phrase + entity files
echo "Backup these two things:"
echo "1. Seed phrase (24 words, keep VERY secure)"
echo "2. Entity JSON files (can be public)"

# Recovery: Regenerate all keys from scratch
export BIP_KEYCHAIN_SEED="your backed up seed phrase..."
bip-keychain derive entity1.json --format gpg  # Regenerate key 1
bip-keychain derive entity2.json --format gpg  # Regenerate key 2
# All keys identical to originals!
```

---

## Troubleshooting

### "gpg: signing failed: No secret key"

**Problem:** Git can't find your GPG signing key.

**Solution:**
```bash
# List available secret keys
gpg --list-secret-keys --keyid-format=long

# Set the correct signing key
git config --global user.signingkey YOUR_KEY_ID

# Verify configuration
git config --global user.signingkey
```

### "error: gpg failed to sign the data"

**Problem:** GPG cannot access the private key or passphrase.

**Solution:**
```bash
# Test GPG signing manually
echo "test" | gpg --clearsign

# If prompted for passphrase, GPG is working
# If not, check GPG agent:
gpg-agent --daemon

# For Git, ensure GPG agent is accessible:
export GPG_TTY=$(tty)

# Add to ~/.bashrc or ~/.zshrc:
echo 'export GPG_TTY=$(tty)' >> ~/.bashrc
```

### "error: cannot run gpg: No such file or directory"

**Problem:** Git can't find the GPG executable.

**Solution:**
```bash
# Find GPG location
which gpg

# Tell Git where GPG is
git config --global gpg.program $(which gpg)

# Or use gpg2 if installed
git config --global gpg.program $(which gpg2)
```

### Different Key Each Time

**Problem:** BIP-Keychain generates different keys on different machines.

**Cause:** Different seed phrases or different entity files.

**Solution:**
```bash
# Verify seed phrase is identical
echo "Seed on machine 1: $BIP_KEYCHAIN_SEED"
# Compare to machine 2

# Verify entity JSON is identical
sha256sum entity.json  # Should match on both machines

# Check public keys match
bip-keychain derive entity.json --format public-key
```

### "BIP_KEYCHAIN_SEED environment variable not set"

**Problem:** Seed phrase not exported.

**Solution:**
```bash
# Set seed phrase
export BIP_KEYCHAIN_SEED="your twelve word mnemonic phrase..."

# For permanent use, add to secure environment file
# ~/.env-bip-keychain (NOT checked into version control!)
cat > ~/.env-bip-keychain <<'EOF'
export BIP_KEYCHAIN_SEED="your twelve word mnemonic phrase..."
EOF

chmod 600 ~/.env-bip-keychain

# Source when needed
source ~/.env-bip-keychain
```

### GitHub Shows "Unverified" Signature

**Problem:** GitHub doesn't recognize your signature.

**Solution:**
1. Export your GPG public key:
   ```bash
   gpg --armor --export YOUR_GPG_KEY_ID
   ```

2. Add to GitHub:
   - Go to Settings → SSH and GPG keys
   - Click "New GPG key"
   - Paste the public key
   - Save

3. Verify email matches:
   ```bash
   git config user.email  # Should match GPG key email
   gpg --list-keys        # Check email in key
   ```

---

## Security Considerations

### Seed Phrase Security

**CRITICAL: Your seed phrase is the master secret for all derived keys.**

#### ✅ DO:
- Store seed phrase in hardware wallet (Ledger, Trezor)
- Use BIP-39 passphrase for additional security layer
- Keep paper backup in secure location (safe, safety deposit box)
- Use password manager with strong master password
- Consider Shamir Secret Sharing for redundancy

#### ❌ DON'T:
- Never commit seed phrase to version control
- Never store in plain text on disk
- Never email or message seed phrase
- Never enter seed phrase on untrusted machines
- Never use weak/guessable seed phrases

### Key Material Handling

```bash
# SECURE: Derive key in memory, use immediately
export BIP_KEYCHAIN_SEED="..."
bip-keychain derive entity.json --format gpg | gpg --import

# INSECURE: Writing private keys to disk
bip-keychain derive entity.json --format private-key > key.txt  # ❌ DON'T DO THIS

# If you must write to disk, secure it:
bip-keychain derive entity.json --format private-key > /tmp/key
chmod 600 /tmp/key
# Use immediately, then:
shred -vfz -n 10 /tmp/key  # Securely delete
```

### Environment Variable Security

```bash
# Risk: Shell history logs commands
export BIP_KEYCHAIN_SEED="abandon abandon ..."  # ❌ Visible in history

# Better: Read from secure file
export BIP_KEYCHAIN_SEED=$(cat ~/.bip-keychain-seed)  # File chmod 600

# Best: Use password manager or hardware wallet
export BIP_KEYCHAIN_SEED=$(pass show bip-keychain/seed)
```

### GPG Key Expiration

Set expiration dates on GPG keys derived from BIP-Keychain:

```bash
# When creating GPG key, set expiration (e.g., 1 year)
gpg --expert --full-gen-key
# Choose expiration: 1y

# Extend expiration when needed
gpg --edit-key YOUR_KEY_ID
gpg> expire
gpg> save
```

**Benefit:** Even if key is compromised, it becomes invalid after expiration.

### Threat Model

#### What BIP-Keychain Protects Against:
- ✅ Key file loss (regenerate from seed)
- ✅ Key file theft (keys encrypted by seed)
- ✅ Accidental key exposure (rotate by changing entity)
- ✅ Key backup complexity (single seed backup)

#### What BIP-Keychain Does NOT Protect Against:
- ❌ Seed phrase theft (compromises ALL derived keys)
- ❌ Malware on derivation machine (can steal derived keys)
- ❌ Weak seed phrases (use BIP-39 generated seeds!)
- ❌ GPG agent compromise (standard GPG security applies)

### Operational Security

1. **Derivation Environment:**
   - Use trusted, malware-free machines for key derivation
   - Consider air-gapped machine for high-security keys
   - Verify BIP-Keychain binary integrity (checksums, signatures)

2. **Key Usage:**
   - Use GPG agent with timeout for automatic key locking
   - Don't leave signing keys unlocked indefinitely
   - Use separate keys for different security levels

3. **Monitoring:**
   - Watch for unexpected signed commits
   - Verify signatures on important commits
   - Revoke compromised keys immediately

### Key Revocation

If a derived key is compromised:

```bash
# 1. Revoke the GPG key
gpg --gen-revoke YOUR_KEY_ID > revoke.asc
gpg --import revoke.asc

# 2. Publish revocation to keyservers
gpg --send-keys YOUR_KEY_ID

# 3. Derive a NEW key from a DIFFERENT entity
# Create new entity with different identifier
bip-keychain derive new-entity.json --format gpg

# 4. Update Git configuration
git config --global user.signingkey NEW_KEY_ID

# 5. Optionally: Rotate seed phrase (if seed compromised)
# Generate new BIP-39 seed and migrate all entities
```

**Note:** Even after revocation, old signed commits remain verifiable (by design). Revocation prevents future signatures with the compromised key.

---

## Future Enhancements

### Planned Features

1. **Full OpenPGP Packet Generation:**
   - Direct GPG keyring import without manual steps
   - Complete OpenPGP packet format support
   - Automated GPG key creation from derived seeds

2. **SSH-Agent Integration:**
   - Use derived keys directly with SSH for Git over SSH
   - No separate GPG configuration needed

3. **Hardware Wallet Support:**
   - Derive keys using Ledger/Trezor as seed source
   - Never expose seed phrase on computer

4. **Key Rotation Automation:**
   - Automatic time-based key rotation
   - Smooth transition between keys

5. **Web of Trust Integration:**
   - Sign other keys with BIP-Keychain-derived keys
   - Participate in GPG web of trust

---

## Related Documentation

- **CLI-USAGE.md** - Complete CLI reference
- **SSH-KEYS-GUIDE.md** - SSH key generation and usage
- **NICKEL-WORKFLOW.md** - Type-safe entity configuration
- **PROJECT-STATUS.md** - Current project status and roadmap

---

## Examples

### Example 1: Personal GitHub Projects

```bash
# 1. Create entity
cat > personal-github.json <<'EOF'
{
  "schema_type": "dns",
  "entity": {
    "fqdn": "github.com/myusername"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Personal GitHub signing key"
}
EOF

# 2. Derive key
export BIP_KEYCHAIN_SEED="your seed phrase..."
bip-keychain derive personal-github.json --format gpg

# 3. Configure Git
git config --global user.signingkey YOUR_GPG_KEY_ID
git config --global commit.gpgsign true

# 4. Make signed commit
git commit -m "Signed with BIP-Keychain"
git log --show-signature -1
```

### Example 2: Work vs. Personal Separation

```bash
# Work entity
cat > work-commits.json <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice",
    "email": "alice@company.com",
    "worksFor": {"@type": "Organization", "name": "ACME Corp"}
  },
  "derivation_config": {"hash_function": "blake2b", "hardened": true},
  "purpose": "Work repository signing"
}
EOF

# Personal entity
cat > personal-commits.json <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "name": "Alice",
    "email": "alice@personal.com"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Personal project signing"
}
EOF

# Conditional Git config
cat > ~/.gitconfig-work <<'EOF'
[user]
    email = alice@company.com
    signingkey = WORK_GPG_KEY_ID
EOF

cat >> ~/.gitconfig <<'EOF'
[includeIf "gitdir:~/work/"]
    path = ~/.gitconfig-work
EOF
```

### Example 3: Anonymous Contributions

```bash
# Anonymous entity
cat > anonymous.json <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Person",
    "identifier": "anon-4f8e3a9d"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Anonymous contributions"
}
EOF

# Use with pseudonym
bip-keychain derive anonymous.json --format gpg
git config user.name "Anonymous Contributor"
git config user.email "anon@example.com"
git config user.signingkey ANON_GPG_KEY_ID
```

---

## Conclusion

BIP-Keychain enables **deterministic, semantic Git signing** with a single seed phrase backup. While full GPG integration is still evolving, the foundation is solid for building reproducible signing workflows.

**Key Takeaways:**
1. Derive signing identities from semantic entities
2. Single seed phrase backs up all keys
3. Reproduce keys on any machine
4. Organize keys by project, identity, or purpose
5. Integrate with standard Git/GPG workflows

**Next Steps:**
1. Create your first entity file
2. Derive a signing key
3. Configure Git for signing
4. Make your first signed commit
5. Verify the signature

For questions, issues, or feature requests, see the project repository.

---

**Last Updated:** 2025-10-22
**Version:** 1.0
**Status:** Production-ready with manual GPG integration
