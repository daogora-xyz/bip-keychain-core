#!/usr/bin/env bash
# Airgapped Key Derivation Workflow using UR Encoding
#
# This demonstrates hardware-wallet-level security without specialized hardware.
# You can use an old laptop, Raspberry Pi, or any computer as an airgapped device.
#
# Workflow:
# 1. Hot machine: Create entity definition → Encode as UR → Show QR code
# 2. Airgapped machine: Scan QR code → Decode entity → Derive key → Export pubkey as QR
# 3. Hot machine: Scan pubkey QR code → Use public key for SSH, deployment, etc.
#
# Required: Build with BC feature enabled: cargo build --release --features bc

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Binary path
BIP_KEYCHAIN="${BIP_KEYCHAIN_BIN:-./target/release/bip-keychain}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  BIP-Keychain Airgapped Workflow Demo                         ║${NC}"
echo -e "${BLUE}║  Hardware-wallet-level security without specialized hardware  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BIP_KEYCHAIN" ]; then
    echo -e "${RED}Error: bip-keychain binary not found at: $BIP_KEYCHAIN${NC}"
    echo -e "${YELLOW}Build it with: cargo build --release --features bc${NC}"
    exit 1
fi

# Check if bc feature is enabled
if ! "$BIP_KEYCHAIN" derive --help | grep -q "ur-entity"; then
    echo -e "${RED}Error: bip-keychain was not built with BC feature${NC}"
    echo -e "${YELLOW}Rebuild with: cargo build --release --features bc${NC}"
    exit 1
fi

# Create temporary directory for this demo
DEMO_DIR=$(mktemp -d -t bip-keychain-airgap-XXXXXX)
trap "rm -rf $DEMO_DIR" EXIT

echo -e "${GREEN}━━━ STEP 1: Hot Machine - Prepare Entity for Airgapped Device ━━━${NC}"
echo ""

# Set a test seed for the demo (in production, the hot machine wouldn't need the seed)
# The seed is only needed for the airgapped machine
export BIP_KEYCHAIN_SEED="test test test test test test test test test test test junk"
echo -e "${YELLOW}Note: In production, the hot machine doesn't need the seed${NC}"
echo -e "${YELLOW}      We're setting it here only for demo purposes${NC}"
echo ""

# Example entity: Production server SSH key
ENTITY_FILE="$DEMO_DIR/server-prod.json"
cat > "$ENTITY_FILE" <<'EOF'
{
  "schema_type": "dns",
  "entity": {
    "name": "prod.api.example.com",
    "environment": "production",
    "zone": "example.com",
    "service": "api",
    "datacenter": "us-east-1"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Production API server SSH access key"
}
EOF

echo -e "Created entity definition:"
cat "$ENTITY_FILE" | head -10
echo ""

# Generate UR-encoded entity
echo -e "${YELLOW}Generating UR-encoded entity...${NC}"
UR_ENTITY=$("$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ur-entity)
echo -e "${GREEN}✓${NC} UR-encoded entity:"
echo "$UR_ENTITY"
echo ""

# Generate QR code
echo -e "${YELLOW}Generating QR code for airgapped transfer...${NC}"
echo ""
"$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format qr-entity > "$DEMO_DIR/entity-qr.txt"
cat "$DEMO_DIR/entity-qr.txt"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  Now you would scan this QR code with your airgapped device${NC}"
echo -e "${BLUE}  The airgapped device would:${NC}"
echo -e "${BLUE}    1. Decode the UR entity${NC}"
echo -e "${BLUE}    2. Derive the key using ITS OWN seed phrase${NC}"
echo -e "${BLUE}    3. Export ONLY the public key as a UR/QR code${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${GREEN}━━━ STEP 2: Airgapped Machine - Derive Key Securely ━━━${NC}"
echo ""

# Simulate airgapped machine with a different seed
echo -e "${YELLOW}Setting up airgapped environment (using test seed)...${NC}"
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
echo -e "${GREEN}✓${NC} Airgapped seed loaded (would be stored ONLY on airgapped device)"
echo ""

# Derive key on airgapped machine
echo -e "${YELLOW}Deriving key from entity (ON AIRGAPPED MACHINE)...${NC}"
DERIVED_KEY=$("$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format json)
echo -e "${GREEN}✓${NC} Key derived successfully (private key never leaves airgapped device)"
echo ""

# Extract public key only
PUBKEY=$(echo "$DERIVED_KEY" | grep "ed25519_public_key" | cut -d'"' -f4)
echo -e "Public key (hex): ${GREEN}$PUBKEY${NC}"
echo ""

# Generate UR-encoded public key for return to hot machine
echo -e "${YELLOW}Generating UR-encoded public key for hot machine...${NC}"
UR_PUBKEY=$("$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ur-pubkey)
echo -e "${GREEN}✓${NC} UR-encoded public key:"
echo "$UR_PUBKEY"
echo ""

# Generate QR code for public key
echo -e "${YELLOW}Generating QR code for public key return...${NC}"
echo ""
"$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format qr-pubkey > "$DEMO_DIR/pubkey-qr.txt"
cat "$DEMO_DIR/pubkey-qr.txt"
echo ""

echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}  The hot machine would now scan this QR code to receive${NC}"
echo -e "${BLUE}  ONLY the public key (private key remains on airgapped device)${NC}"
echo -e "${BLUE}═══════════════════════════════════════════════════════════════${NC}"
echo ""

echo -e "${GREEN}━━━ STEP 3: Hot Machine - Use Public Key ━━━${NC}"
echo ""

# Show SSH public key format
SSH_PUBKEY=$("$BIP_KEYCHAIN" derive "$ENTITY_FILE" --format ssh)
echo -e "${YELLOW}SSH public key (ready to add to authorized_keys):${NC}"
echo "$SSH_PUBKEY"
echo ""

echo -e "${GREEN}✓ Workflow complete!${NC}"
echo ""

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Security Benefits                                             ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  ✓ Private key NEVER leaves airgapped device                   ║${NC}"
echo -e "${BLUE}║  ✓ Seed phrase stored ONLY on airgapped machine                ║${NC}"
echo -e "${BLUE}║  ✓ Entity transferred via QR (no USB, no network)              ║${NC}"
echo -e "${BLUE}║  ✓ Public key returned via QR (camera-based transfer)          ║${NC}"
echo -e "${BLUE}║  ✓ Hardware-wallet-level security without special hardware     ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${YELLOW}Real-world usage:${NC}"
echo -e "  1. Old laptop as airgapped device (no WiFi/Ethernet)"
echo -e "  2. Raspberry Pi Zero (no network connectivity)"
echo -e "  3. Dedicated signing box (physically disconnected)"
echo -e ""
echo -e "${YELLOW}Operational security:${NC}"
echo -e "  • Airgapped device boots from read-only media (Live USB)"
echo -e "  • Seed phrase backed up with SSKR (Shamir's Secret Sharing)"
echo -e "  • Camera-based transfer (no attack surface via USB/network)"
echo -e "  • Public keys stored on hot machine, private keys airgapped"
echo ""

echo -e "${GREEN}Demo files saved to: $DEMO_DIR${NC}"
echo -e "  - entity-qr.txt: QR code for entity (send to airgapped)"
echo -e "  - pubkey-qr.txt: QR code for public key (return from airgapped)"
echo ""
