#!/usr/bin/env bash
# Animated QR Code Demo
#
# Demonstrates fountain codes for transmitting large entities via
# animated QR code sequences. Uses UR (Uniform Resources) encoding
# with Luby transform fountain coding.
#
# Required: Build with BC feature: cargo build --release --features bc

set -euo pipefail

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Binary path
BIP_KEYCHAIN="${BIP_KEYCHAIN_BIN:-./target/release/bip-keychain}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Animated QR Codes with Fountain Encoding                     ║${NC}"
echo -e "${BLUE}║  Multi-part UR transmission for large entities                ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BIP_KEYCHAIN" ]; then
    echo -e "${RED}Error: bip-keychain binary not found at: $BIP_KEYCHAIN${NC}"
    echo -e "${YELLOW}Build it with: cargo build --release --features bc${NC}"
    exit 1
fi

# Set test seed
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo -e "${GREEN}━━━ What are Animated QR Codes? ━━━${NC}"
echo ""
echo "Problem: QR codes have size limits (~2,953 bytes for standard readers)"
echo "Solution: Split large data into multiple QR frames using fountain codes"
echo ""
echo "How it works:"
echo "  1. Entity JSON is split into N fragments"
echo "  2. Fountain encoder generates infinite stream of parts"
echo "  3. Receiver needs ~1.5x minimum fragments to decode"
echo "  4. Any subset of parts can reconstruct the original"
echo ""
echo "Benefits:"
echo "  ✓ No fixed order required"
echo "  ✓ Resistant to packet loss"
echo "  ✓ Can miss frames and still decode"
echo "  ✓ Standard UR format (Blockchain Commons)"
echo ""

echo -e "${GREEN}━━━ Use Cases ━━━${NC}"
echo ""
echo "1. Large Entity Transfer:"
echo "   - Complex schema.org entities"
echo "   - Multi-key derivation configs"
echo "   - Entities with extensive metadata"
echo ""
echo "2. Unreliable Scanning:"
echo "   - Poor camera quality"
echo "   - Bad lighting conditions"
echo "   - Moving displays"
echo ""
echo "3. Airgapped Workflows:"
echo "   - Hot wallet → Cold wallet"
echo "   - Smartphone → Hardware wallet"
echo "   - Desktop → Mobile"
echo ""

echo -e "${GREEN}━━━ Technical Details ━━━${NC}"
echo ""
echo "Fountain Coding (Luby Transform):"
echo "  - Generates potentially infinite sequence of parts"
echo "  - Each part encodes random combination of fragments"
echo "  - Receiver collects parts until decode succeeds"
echo "  - Typically needs 1.5x minimum fragments"
echo ""
echo "UR Encoding:"
echo "  - Standard: ur:<type>/<sequence-id>/<fragment-data>"
echo "  - Type: crypto-entity, crypto-pubkey, etc."
echo "  - Sequence ID: Identifies which frame in sequence"
echo "  - Fragment Data: Base45-encoded fountain part"
echo ""

echo -e "${YELLOW}━━━ Demonstration ━━━${NC}"
echo ""
echo "We'll demonstrate with a test entity from examples/test-entity.json"
echo ""
echo "The entity will be:"
echo "  1. Encoded as multi-part UR with fountain codes"
echo "  2. Split into QR-sized fragments (~200 bytes each)"
echo "  3. Generated as sequence of QR frames"
echo "  4. Animated in terminal (cycling forever)"
echo ""
echo "Press Ctrl+C to stop the animation."
echo ""

read -p "Press ENTER to start animated QR code sequence..."
echo ""

echo -e "${YELLOW}Starting animated QR display...${NC}"
echo ""
echo "Scan these QR codes with a UR-compatible wallet or scanner."
echo "You don't need to scan all frames - just enough to reach the threshold!"
echo ""
sleep 2

# Run animated QR (this will loop forever until Ctrl+C)
"$BIP_KEYCHAIN" derive examples/test-entity.json --format qr-animated || true

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Fountain Code Properties                                     ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  ✓ Rateless: Generate infinite parts                          ║${NC}"
echo -e "${BLUE}║  ✓ Resilient: Tolerates packet loss                           ║${NC}"
echo -e "${BLUE}║  ✓ Efficient: ~1.5x overhead typical                          ║${NC}"
echo -e "${BLUE}║  ✓ Unordered: Parts can arrive in any order                   ║${NC}"
echo -e "${BLUE}║  ✓ Incremental: Decode as soon as enough parts collected      ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Real-World Workflow                                           ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  Hot Machine (Online):                                         ║${NC}"
echo -e "${BLUE}║    1. Create entity definition                                 ║${NC}"
echo -e "${BLUE}║    2. Generate animated QR sequence:                           ║${NC}"
echo -e "${BLUE}║       bip-keychain derive entity.json --format qr-animated     ║${NC}"
echo -e "${BLUE}║                                                                ║${NC}"
echo -e "${BLUE}║  Cold Machine (Airgapped):                                     ║${NC}"
echo -e "${BLUE}║    1. Scan QR frames with camera                               ║${NC}"
echo -e "${BLUE}║    2. UR decoder reconstructs entity                           ║${NC}"
echo -e "${BLUE}║    3. Derive keys securely offline                             ║${NC}"
echo -e "${BLUE}║    4. Export public key as QR                                  ║${NC}"
echo -e "${BLUE}║                                                                ║${NC}"
echo -e "${BLUE}║  Hot Machine (Online):                                         ║${NC}"
echo -e "${BLUE}║    1. Scan public key QR from cold machine                     ║${NC}"
echo -e "${BLUE}║    2. Use for verification/deployment                          ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${YELLOW}Performance characteristics:${NC}"
echo ""
echo "Fragment Size: 200 bytes (recommended for QR codes)"
echo "  - Fits in standard QR code capacity"
echo "  - Good balance between frame count and reliability"
echo "  - Compatible with most smartphone cameras"
echo ""
echo "Overhead: ~50% (1.5x minimum fragments)"
echo "  - Ensures high probability of successful decode"
echo "  - Accounts for missed frames during scanning"
echo "  - Standard for fountain codes"
echo ""
echo "Frame Rate: 500ms per frame (default)"
echo "  - Fast enough for quick transfer"
echo "  - Slow enough for reliable scanning"
echo "  - Configurable in code"
echo ""

echo -e "${YELLOW}Comparison with single QR:${NC}"
echo ""
echo "Single Static QR:"
echo "  ✓ Simple (one scan)"
echo "  ✗ Limited size (~3KB max)"
echo "  ✗ Must scan perfectly"
echo "  ✗ No error recovery"
echo ""
echo "Animated Multi-part QR:"
echo "  ✓ Unlimited size"
echo "  ✓ Error resistant"
echo "  ✓ Can miss frames"
echo "  ✓ No fixed order"
echo "  ✗ More complex (multiple scans)"
echo ""

echo -e "${GREEN}━━━ Demo Complete ━━━${NC}"
echo ""
