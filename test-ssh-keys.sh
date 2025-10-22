#!/bin/bash
# Test SSH key generation with BIP-Keychain

set -e

echo "=== Testing SSH Key Generation ==="
echo ""

# Set test seed phrase
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo "Using test mnemonic: ${BIP_KEYCHAIN_SEED:0:40}..."
echo ""

# Test 1: Generate SSH public key (default format)
echo "--- Test 1: SSH Public Key (Default Format) ---"
echo "Command: bip-keychain derive examples/github-repo.json"
SSH_KEY=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)
echo "Output: $SSH_KEY"
echo ""

# Verify SSH key format
if [[ $SSH_KEY == ssh-ed25519* ]]; then
    echo "✓ Starts with 'ssh-ed25519'"
else
    echo "✗ ERROR: Does not start with 'ssh-ed25519'"
    exit 1
fi

# Verify it ends with a comment
if [[ $SSH_KEY == *"Git commit signing key for bip-keychain-core repository" ]]; then
    echo "✓ Contains purpose as comment"
else
    echo "✗ ERROR: Missing purpose comment"
    exit 1
fi

# Count parts (should be 3: algorithm, key, comment)
PARTS=$(echo "$SSH_KEY" | wc -w)
if [ "$PARTS" -ge 3 ]; then
    echo "✓ Has correct number of parts ($PARTS)"
else
    echo "✗ ERROR: Wrong number of parts ($PARTS, expected >= 3)"
    exit 1
fi
echo ""

# Test 2: All output formats
echo "--- Test 2: All Output Formats ---"
echo ""

echo "2a. Raw seed:"
cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json --format seed
echo ""

echo "2b. Public key hex:"
cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json --format public-key
echo ""

echo "2c. SSH public key:"
cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json --format ssh
echo ""

echo "2d. JSON (with all keys):"
cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json --format json | jq .
echo ""

# Test 3: Determinism
echo "--- Test 3: Deterministic SSH Key Generation ---"
KEY1=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)
KEY2=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)

if [ "$KEY1" == "$KEY2" ]; then
    echo "✓ Deterministic: Same entity → Same SSH key"
else
    echo "✗ ERROR: Non-deterministic SSH key generation!"
    exit 1
fi
echo ""

# Test 4: Different entities produce different keys
echo "--- Test 4: Unique SSH Keys for Different Entities ---"
KEY_GITHUB=$(cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json)
KEY_DID=$(cargo run --quiet --bin bip-keychain -- derive examples/did-identity.json)

if [ "$KEY_GITHUB" != "$KEY_DID" ]; then
    echo "✓ Different entities produce different SSH keys"
else
    echo "✗ ERROR: Same SSH key for different entities!"
    exit 1
fi
echo ""

# Test 5: Verify with ssh-keygen (if available)
echo "--- Test 5: Verify with ssh-keygen ---"
if command -v ssh-keygen &> /dev/null; then
    TEMP_KEY=$(mktemp)
    cargo run --quiet --bin bip-keychain -- derive examples/github-repo.json > "$TEMP_KEY"

    echo "Running: ssh-keygen -lf $TEMP_KEY"
    if ssh-keygen -lf "$TEMP_KEY"; then
        echo "✓ Valid SSH key format (verified by ssh-keygen)"
    else
        echo "✗ ssh-keygen validation failed"
        rm "$TEMP_KEY"
        exit 1
    fi

    rm "$TEMP_KEY"
else
    echo "⚠ ssh-keygen not found, skipping validation"
    echo "  (SSH key format is correct, but not verified by ssh-keygen)"
fi
echo ""

# Test 6: BLAKE2b hash function
echo "--- Test 6: BLAKE2b with Gordian Envelope ---"
echo "Testing different hash function (BLAKE2b):"
KEY_BLAKE2=$(cargo run --quiet --bin bip-keychain -- derive examples/gordian-envelope.json)
echo "$KEY_BLAKE2"

if [[ $KEY_BLAKE2 == ssh-ed25519* ]]; then
    echo "✓ BLAKE2b produces valid SSH key"
else
    echo "✗ ERROR: BLAKE2b did not produce valid SSH key"
    exit 1
fi
echo ""

# Summary
echo "=== All SSH Key Tests Passed! ==="
echo ""
echo "Verified:"
echo "  ✓ SSH public key format (ssh-ed25519)"
echo "  ✓ Deterministic generation"
echo "  ✓ Unique keys for different entities"
echo "  ✓ All output formats working"
echo "  ✓ HMAC-SHA-512 and BLAKE2b hash functions"
if command -v ssh-keygen &> /dev/null; then
    echo "  ✓ Valid SSH key (verified by ssh-keygen)"
fi
echo ""
echo "You can now use BIP-Keychain to generate SSH keys!"
echo "See SSH-KEYS-GUIDE.md for usage examples."
echo ""
