#!/usr/bin/env bash
set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Default binary location
BIP_KEYCHAIN="${BIP_KEYCHAIN:-$PROJECT_ROOT/target/release/bip-keychain}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  UR Round-trip Encode/Decode Test                             ║${NC}"
echo -e "${BLUE}║  Single-part & Multi-part (Fountain Codes)                    ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check binary exists
if [ ! -f "$BIP_KEYCHAIN" ]; then
    echo -e "${RED}Error: bip-keychain binary not found at: $BIP_KEYCHAIN${NC}"
    echo -e "${YELLOW}Build it with: cargo build --release --features bc${NC}"
    exit 1
fi

if ! "$BIP_KEYCHAIN" decode-ur --help > /dev/null 2>&1; then
    echo -e "${RED}Error: bip-keychain was not built with BC feature${NC}"
    echo -e "${YELLOW}Rebuild with: cargo build --release --features bc${NC}"
    exit 1
fi

# Create temporary directory
TEMP_DIR=$(mktemp -d -t bip-keychain-roundtrip-XXXXXX)
trap "rm -rf $TEMP_DIR" EXIT

# Set test seed
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo -e "${GREEN}━━━ Test 1: Single-part UR Round-trip ━━━${NC}"
echo ""
echo "This tests encoding and decoding of a single UR string."
echo "Entity → UR string → Entity"
echo ""

# Use test entity
ENTITY_FILE="examples/test-entity.json"

if [ ! -f "$ENTITY_FILE" ]; then
    echo -e "${RED}Error: Test entity not found: $ENTITY_FILE${NC}"
    exit 1
fi

echo -e "${YELLOW}Step 1: Encode entity to UR...${NC}"
echo ""
"$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ur-entity > "$TEMP_DIR/encoded.ur" 2>&1
UR_LENGTH=$(wc -c < "$TEMP_DIR/encoded.ur")
echo "✓ Encoded to UR:"
echo "  $(head -c 60 "$TEMP_DIR/encoded.ur")..."
echo "  Length: $UR_LENGTH characters"
echo ""

echo -e "${YELLOW}Step 2: Decode UR back to entity...${NC}"
echo ""
cat "$TEMP_DIR/encoded.ur" | xargs "$BIP_KEYCHAIN" decode-ur 2>/dev/null > "$TEMP_DIR/decoded-raw.json"
echo "✓ Decoded entity JSON"
echo ""

echo -e "${YELLOW}Step 3: Verify round-trip...${NC}"
echo ""

# Save original and decoded for comparison
jq -S . < "$ENTITY_FILE" > "$TEMP_DIR/original.json"
jq -S . < "$TEMP_DIR/decoded-raw.json" > "$TEMP_DIR/decoded.json"

if diff -u "$TEMP_DIR/original.json" "$TEMP_DIR/decoded.json" > /dev/null; then
    echo -e "${GREEN}✓ Round-trip successful! Entities match perfectly.${NC}"
else
    echo -e "${RED}✗ Round-trip failed! Entities don't match.${NC}"
    echo ""
    echo "Differences:"
    diff -u "$TEMP_DIR/original.json" "$TEMP_DIR/decoded.json" || true
    exit 1
fi

echo ""
echo -e "${GREEN}━━━ Test 2: Public Key UR Round-trip ━━━${NC}"
echo ""
echo "This tests encoding and decoding of public keys."
echo "Entity → Public Key → UR → Public Key"
echo ""

echo -e "${YELLOW}Step 1: Derive public key and encode to UR...${NC}"
echo ""
"$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ur-pubkey 2>&1 | tail -1 > "$TEMP_DIR/pubkey.ur"
echo "✓ Encoded public key to UR:"
echo "  $(head -c 60 "$TEMP_DIR/pubkey.ur")..."
echo ""

echo -e "${YELLOW}Step 2: Decode UR back to public key...${NC}"
echo ""
cat "$TEMP_DIR/pubkey.ur" | xargs "$BIP_KEYCHAIN" decode-ur 2>&1 | grep -E "^[0-9a-f]{64}$" > "$TEMP_DIR/decoded-pubkey.txt"
DECODED_PUBKEY=$(cat "$TEMP_DIR/decoded-pubkey.txt")
echo "✓ Decoded public key"
echo ""

echo -e "${YELLOW}Step 3: Verify round-trip...${NC}"
echo ""
DIRECT_PUBKEY=$("$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format public-key)

if [ "$DECODED_PUBKEY" = "$DIRECT_PUBKEY" ]; then
    echo -e "${GREEN}✓ Public key round-trip successful!${NC}"
else
    echo -e "${RED}✗ Public key round-trip failed!${NC}"
    echo "Direct:  $DIRECT_PUBKEY"
    echo "Decoded: $DECODED_PUBKEY"
    exit 1
fi

echo ""
echo -e "${GREEN}━━━ Test 3: Airgapped Workflow Simulation ━━━${NC}"
echo ""
echo "Simulates complete airgapped key derivation:"
echo "Hot machine → Cold machine (derive) → Hot machine"
echo ""

echo -e "${YELLOW}Step 1: Hot machine encodes entity...${NC}"
echo ""
"$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ur-entity > "$TEMP_DIR/hot-entity.ur" 2>&1
echo "✓ Entity encoded for transfer to cold machine"
echo ""

echo -e "${YELLOW}Step 2: Cold machine decodes, derives key, encodes pubkey...${NC}"
echo ""
cat "$TEMP_DIR/hot-entity.ur" | xargs "$BIP_KEYCHAIN" decode-ur 2>/dev/null > "$TEMP_DIR/cold-entity.json"
"$BIP_KEYCHAIN" derive "$TEMP_DIR/cold-entity.json" --format ur-pubkey 2>&1 | tail -1 > "$TEMP_DIR/cold-pubkey.ur"
echo "✓ Cold machine derived key and encoded pubkey"
echo ""

echo -e "${YELLOW}Step 3: Hot machine receives and decodes pubkey...${NC}"
echo ""
cat "$TEMP_DIR/cold-pubkey.ur" | xargs "$BIP_KEYCHAIN" decode-ur 2>&1 | grep -E "^[0-9a-f]{64}$" > "$TEMP_DIR/hot-received.txt"
HOT_RECEIVED=$(cat "$TEMP_DIR/hot-received.txt")
echo "✓ Hot machine received public key"
echo ""

echo -e "${YELLOW}Step 4: Verify airgapped workflow...${NC}"
echo ""
if [ "$HOT_RECEIVED" = "$DIRECT_PUBKEY" ]; then
    echo -e "${GREEN}✓ Airgapped workflow successful!${NC}"
    echo "  Public key correctly transferred through airgap"
else
    echo -e "${RED}✗ Airgapped workflow failed!${NC}"
    exit 1
fi

echo ""
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${GREEN}  All Tests Passed!${NC}"
echo -e "${GREEN}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo ""
echo "Summary:"
echo "  ✓ Entity round-trip (UR encode/decode)"
echo "  ✓ Public key round-trip (UR encode/decode)"
echo "  ✓ Complete airgapped workflow simulation"
echo ""
echo "Note: Multi-part fountain code testing would require additional"
echo "      CLI commands for animated QR generation and capture."
