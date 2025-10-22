#!/bin/bash
# Test script for BIP-Keychain CLI

set -e

echo "=== Testing BIP-Keychain CLI ==="
echo ""

# Set test seed phrase
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo "1. Testing derive command with hex output (default):"
cargo run --quiet --bin bip-keychain -- derive examples/test-entity.json
echo ""

echo "2. Testing derive command with JSON output:"
cargo run --quiet --bin bip-keychain -- derive examples/test-entity.json --format json
echo ""

echo "3. Testing with custom parent entropy:"
cargo run --quiet --bin bip-keychain -- derive examples/test-entity.json --parent-entropy "deadbeef"
echo ""

echo "4. Testing error handling (missing environment variable):"
unset BIP_KEYCHAIN_SEED
cargo run --quiet --bin bip-keychain -- derive examples/test-entity.json 2>&1 || true
echo ""

echo "5. Testing help output:"
cargo run --quiet --bin bip-keychain -- --help
echo ""

echo "=== All tests complete ==="
