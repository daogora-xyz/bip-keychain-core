#!/bin/bash
# Complete end-to-end workflow test
#
# Tests the full BIP-Keychain workflow:
# 1. Nickel entity definitions (pre-exported to JSON)
# 2. Key derivation using bip-keychain CLI
# 3. Verification of deterministic output

set -e

echo "=== BIP-Keychain Complete Workflow Test ==="
echo ""

# Set test seed phrase
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo "Using test mnemonic: ${BIP_KEYCHAIN_SEED:0:40}..."
echo ""

# Test 1: Schema.org entity (HMAC-SHA-512)
echo "--- Test 1: Schema.org Entity (HMAC-SHA-512) ---"
echo "Entity: GitHub repository signing key"
echo ""

echo "Command: bip-keychain derive examples/github-repo.json"
KEY1=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)
echo "Ed25519 Seed: $KEY1"
echo ""

echo "Testing determinism (derive again)..."
KEY1_AGAIN=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)

if [ "$KEY1" == "$KEY1_AGAIN" ]; then
    echo "✓ Deterministic: Same entity → Same key"
else
    echo "✗ ERROR: Non-deterministic derivation!"
    exit 1
fi
echo ""

# Test 2: DID entity (HMAC-SHA-512)
echo "--- Test 2: DID Entity (HMAC-SHA-512) ---"
echo "Entity: GitHub DID identity"
echo ""

echo "Command: bip-keychain derive examples/did-identity.json"
KEY2=$(cargo run --quiet --bin bip-keychain -- derive examples/did-identity.json)
echo "Ed25519 Seed: $KEY2"
echo ""

# Test 3: Gordian Envelope (BLAKE2b)
echo "--- Test 3: Gordian Envelope (BLAKE2b) ---"
echo "Entity: Blockchain Commons envelope"
echo ""

echo "Command: bip-keychain derive examples/gordian-envelope.json --format json"
cargo run --quiet --bin bip-keychain -- derive examples/gordian-envelope.json --format json | jq .
echo ""

# Test 4: Different entities produce different keys
echo "--- Test 4: Uniqueness Verification ---"
if [ "$KEY1" != "$KEY2" ]; then
    echo "✓ Different entities produce different keys"
    echo "  Schema.org: ${KEY1:0:16}..."
    echo "  DID:        ${KEY2:0:16}..."
else
    echo "✗ ERROR: Same key for different entities!"
    exit 1
fi
echo ""

# Test 5: Custom parent entropy
echo "--- Test 5: Custom Parent Entropy ---"
echo "Testing with custom entropy value"
echo ""

KEY_DEFAULT=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)
KEY_CUSTOM=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json --parent-entropy "deadbeef")

if [ "$KEY_DEFAULT" != "$KEY_CUSTOM" ]; then
    echo "✓ Different parent entropy produces different keys"
    echo "  Default:  ${KEY_DEFAULT:0:16}..."
    echo "  Custom:   ${KEY_CUSTOM:0:16}..."
else
    echo "✗ ERROR: Same key despite different parent entropy!"
    exit 1
fi
echo ""

# Summary
echo "=== All Tests Passed! ==="
echo ""
echo "Workflow verified:"
echo "  ✓ Nickel entities (pre-exported JSON)"
echo "  ✓ BIP-Keychain key derivation"
echo "  ✓ Deterministic output"
echo "  ✓ Unique keys for different entities"
echo "  ✓ HMAC-SHA-512 hash function"
echo "  ✓ BLAKE2b hash function"
echo "  ✓ Parent entropy configuration"
echo ""
echo "Next steps:"
echo "  1. Install Nickel to export your own .ncl files"
echo "  2. See NICKEL-WORKFLOW.md for detailed guide"
echo "  3. Use your own seed phrase (BIP_KEYCHAIN_SEED)"
echo "  4. Derive keys for real use cases!"
echo ""
