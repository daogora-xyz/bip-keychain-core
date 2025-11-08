# BIP-Keychain Core - Review & Testing Checklist

**Purpose**: Systematic review of all functionality before publication.
**Date**: 2025-11-08
**Reviewer**: (Your name)

---

## 📋 Quick Summary of What We Built

This project implements BIP-Keychain: deriving cryptographic keys from semantic JSON entities using BIP-32 hierarchical deterministic derivation.

**Core Features**:
- Multi-hash support (HMAC-SHA-512, BLAKE2b, SHA-256)
- BIP-32 key derivation with semantic entities
- Ed25519 keypair generation
- Multiple output formats (SSH, GPG, hex, JSON)
- CLI tool with seed generation
- 50 tests (unit, integration, property-based)

**Commits in this session**:
1. `315186e` - Fixed alkali 0.3.0 compatibility and overflow issues
2. `eabee03` - Fixed compiler warnings
3. `22a73e8` - Replaced spec/tasks.md with TODO.md
4. `1b3e110` - Implemented generate-seed command
5. `3154689` - Updated TODO.md with Gordian Envelope plan
6. `125b8e7` - Added CI/CD pipeline and updated README

---

## ✅ Testing Checklist

### 1. Build & Test Suite

**Basic Build**:
- [ ] `cargo build` completes without errors
- [ ] `cargo build --release` completes without errors
- [ ] No compiler warnings
- [ ] Binary created at `target/debug/bip-keychain`

**Test Suite**:
- [ ] `cargo test` - All tests pass (should be 50 tests)
- [ ] `cargo test --release` - Tests pass in release mode
- [ ] `cargo test -- --nocapture` - Review test output
- [ ] Check specific test modules:
  - [ ] `cargo test hash::` - Hash function tests
  - [ ] `cargo test derivation::` - Derivation tests
  - [ ] `cargo test property_tests::` - Property-based tests

**Expected Results**:
```
test result: ok. 50 passed; 0 failed; 0 ignored
```

---

### 2. Manual Testing - generate-seed Command

**Test 1: Basic 24-word generation**
```bash
cargo run -- generate-seed --words 24
```
- [ ] Generates 24 words
- [ ] Words are from BIP-39 wordlist (recognizable English words)
- [ ] Security warning displays on stderr
- [ ] Different output each time (run 3 times, compare)

**Test 2: Different word counts**
```bash
cargo run -- generate-seed --words 12
cargo run -- generate-seed --words 15
cargo run -- generate-seed --words 18
cargo run -- generate-seed --words 21
cargo run -- generate-seed --words 24
```
- [ ] Each generates correct number of words
- [ ] All are valid BIP-39 mnemonics

**Test 3: Invalid word counts**
```bash
cargo run -- generate-seed --words 13
cargo run -- generate-seed --words 10
cargo run -- generate-seed --words 0
```
- [ ] Shows helpful error message
- [ ] Suggests valid word counts (12, 15, 18, 21, 24)
- [ ] Exit with error code (check with `echo $?`)

**Test 4: Piping output**
```bash
cargo run -- generate-seed --words 24 2>/dev/null
```
- [ ] Only mnemonic on stdout (no warnings)
- [ ] Can be piped to file or other commands

---

### 3. Manual Testing - derive Command

**Test 1: Basic SSH key derivation**
```bash
# Generate and set seed
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Derive from example entity
cargo run -- derive examples/github-repo.json
```
- [ ] Outputs SSH public key (starts with `ssh-ed25519`)
- [ ] Key is base64 encoded
- [ ] Comment matches entity name
- [ ] No errors or warnings

**Test 2: Determinism (same seed → same key)**
```bash
# Run twice with same seed and entity
cargo run -- derive examples/github-repo.json > key1.txt
cargo run -- derive examples/github-repo.json > key2.txt
diff key1.txt key2.txt
```
- [ ] Files are identical (diff shows no differences)
- [ ] Same output every time

**Test 3: Uniqueness (different entities → different keys)**
```bash
cargo run -- derive examples/github-repo.json > key_github.txt
cargo run -- derive examples/server-prod.json > key_server.txt
diff key_github.txt key_server.txt
```
- [ ] Files are different
- [ ] Each entity produces unique key

**Test 4: Different output formats**
```bash
# SSH format (default)
cargo run -- derive examples/github-repo.json --format ssh

# Hex seed
cargo run -- derive examples/github-repo.json --format seed

# Public key hex
cargo run -- derive examples/github-repo.json --format public-key

# Private key hex (careful!)
cargo run -- derive examples/github-repo.json --format private-key

# GPG format
cargo run -- derive examples/github-repo.json --format gpg

# JSON format
cargo run -- derive examples/github-repo.json --format json
```
- [ ] Each format produces valid output
- [ ] JSON is valid JSON (can pipe to `jq`)
- [ ] Hex outputs are valid hex strings

**Test 5: Error handling - missing seed**
```bash
unset BIP_KEYCHAIN_SEED
cargo run -- derive examples/github-repo.json
```
- [ ] Shows helpful error about missing BIP_KEYCHAIN_SEED
- [ ] Suggests how to set it
- [ ] Exits with error code

**Test 6: Error handling - invalid entity file**
```bash
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
cargo run -- derive nonexistent.json
```
- [ ] Shows "file not found" error
- [ ] Error message is clear

**Test 7: Error handling - invalid JSON**
```bash
echo "invalid json {" > /tmp/bad.json
cargo run -- derive /tmp/bad.json
```
- [ ] Shows JSON parsing error
- [ ] Error message is helpful

---

### 4. Testing Different Hash Functions

**Test with HMAC-SHA-512 (default)**:
```bash
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Check example entity uses HMAC-SHA-512
cat examples/github-repo.json | grep hashFunction

# Derive
cargo run -- derive examples/github-repo.json
```
- [ ] Uses HMAC-SHA-512 hash function
- [ ] Key is derived successfully

**Test with BLAKE2b**:
Create test entity:
```bash
cat > /tmp/blake2b-test.json << 'EOF'
{
  "schemaType": "DNS",
  "name": "example.com",
  "derivation": {
    "hashFunction": "BLAKE2b",
    "purpose": "Test BLAKE2b"
  }
}
EOF

cargo run -- derive /tmp/blake2b-test.json
```
- [ ] Derives key with BLAKE2b
- [ ] Different output than HMAC-SHA-512 for same entity name

**Test with SHA-256**:
```bash
cat > /tmp/sha256-test.json << 'EOF'
{
  "schemaType": "DNS",
  "name": "example.com",
  "derivation": {
    "hashFunction": "SHA256",
    "purpose": "Test SHA-256"
  }
}
EOF

cargo run -- derive /tmp/sha256-test.json
```
- [ ] Derives key with SHA-256
- [ ] Different output than other hash functions

---

### 5. Real-World Integration Test

**Test 1: SSH Key Usage**
```bash
# Generate seed
export BIP_KEYCHAIN_SEED=$(cargo run -- generate-seed --words 24 2>/dev/null)

# Derive SSH key
cargo run -- derive examples/server-prod.json > /tmp/test-key.pub

# Check format
ssh-keygen -l -f /tmp/test-key.pub
```
- [ ] ssh-keygen recognizes the key
- [ ] Shows fingerprint and key type (ED25519)
- [ ] Key size is 256 bits

**Test 2: GPG Key Info**
```bash
cargo run -- derive examples/person-identity.json --format gpg
```
- [ ] Outputs GPG-compatible public key info
- [ ] Includes Ed25519 algorithm
- [ ] Readable and properly formatted

**Test 3: JSON Output Validation**
```bash
cargo run -- derive examples/github-repo.json --format json | jq .
```
- [ ] Valid JSON (jq parses it)
- [ ] Contains all expected fields:
  - [ ] `publicKey`
  - [ ] `privateKey`
  - [ ] `sshPublicKey`
  - [ ] `entity` metadata

---

### 6. Example Entities Review

Check all provided examples work:
```bash
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Test each example
for file in examples/*.json; do
    echo "Testing: $file"
    cargo run -- derive "$file" || echo "FAILED: $file"
done
```
- [ ] All examples derive successfully
- [ ] No errors or panics
- [ ] Each produces valid output

Review example entity files:
- [ ] `examples/github-repo.json` - Valid schema
- [ ] `examples/server-prod.json` - Valid schema
- [ ] `examples/person-identity.json` - Valid schema
- [ ] All other examples are properly formatted

---

### 7. Code Quality Review

**Cargo.toml**:
- [ ] Version is `0.1.0`
- [ ] All dependencies have versions specified
- [ ] License is `BSD-2-Clause`
- [ ] Repository URL is correct
- [ ] Keywords and categories are appropriate

**Source Code Structure**:
```bash
# Review file organization
ls -la src/
ls -la src/bin/
ls -la tests/
```
- [ ] `src/lib.rs` - Library entry point
- [ ] `src/entity.rs` - Entity parsing
- [ ] `src/hash.rs` - Hash functions
- [ ] `src/derivation.rs` - Core derivation
- [ ] `src/bip32_wrapper.rs` - BIP-32 wrapper
- [ ] `src/output.rs` - Output formatting
- [ ] `src/error.rs` - Error types
- [ ] `src/bin/bip-keychain.rs` - CLI tool

**Code Review Focus Areas**:
1. **Security-critical code**:
   - [ ] `src/hash.rs` - Hash implementations
   - [ ] `src/bip32_wrapper.rs` - Key derivation
   - [ ] `src/bin/bip-keychain.rs` - Seed generation

2. **Test coverage**:
   - [ ] `tests/hash_tests.rs` - Hash function tests
   - [ ] `tests/integration_test.rs` - End-to-end tests
   - [ ] `tests/property_tests.rs` - Property-based tests

3. **Error handling**:
   - [ ] All `Result<T>` types have proper error messages
   - [ ] `context()` used for helpful error messages
   - [ ] No unwraps in production code (only in tests)

---

### 8. Security Review

**Seed Phrase Handling**:
```bash
# Verify seed not in process list
export BIP_KEYCHAIN_SEED="test seed phrase"
cargo run -- derive examples/github-repo.json &
ps aux | grep bip-keychain
```
- [ ] Seed phrase NOT visible in process arguments
- [ ] Only passed via environment variable

**Private Key Handling**:
- [ ] Private keys only output when explicitly requested
- [ ] Warning shown for private key output
- [ ] No private keys logged or persisted

**Randomness Quality**:
```bash
# Check getrandom is used
grep -r "getrandom" src/
```
- [ ] `getrandom` crate used for entropy
- [ ] No custom/weak RNG implementations

**Cryptographic Libraries**:
- [ ] `bip32` crate (0.5) - Well-known, audited
- [ ] `bip39` crate (2.0) - Standard implementation
- [ ] `ed25519-dalek` (2.0) - Widely used
- [ ] `alkali` (0.3.0) - libsodium bindings
- [ ] All use standard, vetted implementations

**Test Vectors**:
- [ ] HMAC-SHA-512 uses RFC 4231 test vectors
- [ ] BLAKE2b uses official BLAKE2 test vectors
- [ ] BIP-39 test mnemonic is standard

---

### 9. Documentation Review

**README.md**:
- [ ] Accurate project description
- [ ] Build instructions work
- [ ] Examples are correct
- [ ] Links are valid
- [ ] Status is accurate ("Production-Ready MVP")

**TODO.md**:
- [ ] Completed items are marked
- [ ] Priorities are reasonable
- [ ] Gordian Envelope plan is clear
- [ ] Release roadmap makes sense

**CLI-USAGE.md**:
- [ ] All commands documented
- [ ] Examples work as written
- [ ] Troubleshooting section is helpful

**SSH-KEYS-GUIDE.md**:
- [ ] Instructions are clear
- [ ] Examples are accurate
- [ ] Security warnings are appropriate

**GIT-SIGNING-GUIDE.md**:
- [ ] GPG integration explained
- [ ] Git setup instructions correct

**CLAUDE.md**:
- [ ] Development workflow documented
- [ ] Architecture is accurate
- [ ] TDD process described

**PROJECT-STATUS.md**:
- [ ] Current status accurate
- [ ] Metrics are correct (50 tests, etc.)
- [ ] Performance claims reasonable

---

### 10. CI/CD Review

**GitHub Actions Workflow** (`.github/workflows/ci.yml`):
- [ ] Jobs are properly configured
- [ ] Multi-platform testing (Linux, macOS, Windows)
- [ ] libsodium installation for all platforms
- [ ] Caching strategy is efficient
- [ ] Linting with clippy
- [ ] Format checking with rustfmt
- [ ] Security audit with cargo-audit
- [ ] MSRV check (Rust 1.70.0)

**Triggers**:
- [ ] Runs on push to main/master
- [ ] Runs on pull requests
- [ ] Can be manually triggered

---

## 🔍 Deep Dive Areas

### A. Hash Function Correctness

**HMAC-SHA-512 Test Vectors**:
```bash
cargo test test_hmac_sha512 -- --nocapture
```
Review the output:
- [ ] Uses RFC 4231 test vector
- [ ] Input: "Hi There", Key: 0x0b repeated
- [ ] Output matches expected hash

**BLAKE2b Test Vectors**:
```bash
cargo test test_blake2b -- --nocapture
```
Review the output:
- [ ] Uses official BLAKE2 test vector
- [ ] 64-byte output (BLAKE2b-512)
- [ ] Output matches expected hash

**SHA-256 Implementation**:
```bash
cargo test test_sha256 -- --nocapture
```
Review the output:
- [ ] Produces 32-byte hash
- [ ] Padded to 64 bytes with zeros
- [ ] Deterministic output

### B. BIP-32 Derivation Path

Check the derivation path constants:
```bash
grep -A 5 "BIP85_APP\|BIPKEYCHAIN_APP" src/bip32_wrapper.rs
```
- [ ] `BIP85_APP = 83696968` (correct)
- [ ] `BIPKEYCHAIN_APP = 67797668` (correct)
- [ ] Path format: `m/83696968'/67797668'/{index}'`

### C. Overflow Safety

The overflow fix in commit `315186e`:
```bash
git show 315186e | grep -A 3 "wrapping_add"
```
- [ ] Uses `wrapping_add()` for hardened indices
- [ ] Prevents panic when entity_index is large
- [ ] Maintains correct BIP-32 behavior

### D. alkali 0.3.0 Compatibility

Check the BLAKE2b fix:
```bash
git show 315186e | grep -A 5 "hash_custom"
```
- [ ] Uses `hash_custom()` instead of `hash()`
- [ ] Specifies 64-byte output explicitly
- [ ] Returns BLAKE2b-512 (not BLAKE2b-256)

---

## 🚨 Red Flags to Look For

**Code Issues**:
- [ ] ❌ No `unwrap()` in production code (only tests OK)
- [ ] ❌ No `expect()` in production code
- [ ] ❌ No hardcoded secrets or test seeds in production paths
- [ ] ❌ No `println!` of sensitive data (seeds, private keys)
- [ ] ❌ No unsafe blocks (except well-justified)

**Security Issues**:
- [ ] ❌ Private keys never written to disk unintentionally
- [ ] ❌ Seed phrases never logged
- [ ] ❌ No weak random number generation
- [ ] ❌ No timing attacks in key comparison (use constant-time)

**Documentation Issues**:
- [ ] ❌ No broken links in documentation
- [ ] ❌ No outdated examples
- [ ] ❌ No misleading claims about security
- [ ] ❌ No missing security warnings

---

## ✅ Final Checklist

**Before Considering Publication**:
- [ ] All tests above completed
- [ ] All features work as expected
- [ ] No critical bugs found
- [ ] Documentation is accurate
- [ ] Security review passed
- [ ] Comfortable explaining all code
- [ ] Used the tool yourself for real keys
- [ ] Tested on your infrastructure

**Personal Confidence**:
- [ ] I understand how BIP-32 derivation works
- [ ] I understand how the hash functions work
- [ ] I trust the cryptographic libraries used
- [ ] I've reviewed the security-critical code
- [ ] I'm comfortable putting my name on this
- [ ] I would use this tool myself (and have)

**Next Steps After Review**:
- [ ] Document any issues found
- [ ] Fix critical bugs
- [ ] Improve documentation where unclear
- [ ] Add more tests if needed
- [ ] Test in production (non-critical) environment
- [ ] Wait 1-4 weeks, use daily, then reconsider publication

---

## 📝 Review Notes

Use this section to document your findings:

### Issues Found:
```
(List any bugs, unclear documentation, or concerns)
```

### Questions:
```
(List any questions about implementation or design)
```

### Improvements Needed:
```
(List any changes before publication)
```

### Overall Assessment:
```
(Your overall thoughts on code quality, security, readability)
```

---

## 🎯 Recommended Testing Order

1. **Day 1**: Build & test suite (Section 1)
2. **Day 2**: Manual testing of generate-seed and derive commands (Sections 2-3)
3. **Day 3**: Hash functions and real-world integration (Sections 4-5)
4. **Day 4**: Code review and security review (Sections 7-8)
5. **Day 5**: Documentation review and deep dives (Sections 9-10, A-D)
6. **Week 2+**: Use the tool yourself with real (non-critical) keys

Take your time. There's no rush.

---

**Last Updated**: 2025-11-08
**Next Review**: After using in production for 1-2 weeks
