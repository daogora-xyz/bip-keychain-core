#!/usr/bin/env bash
# SSKR Seed Backup and Recovery Demo
#
# Demonstrates Shamir's Secret Sharing for BIP-39 seed backup.
# Shows different policies for personal, enterprise, and couples backup.
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
echo -e "${BLUE}║  BIP-Keychain SSKR Backup Demo                                ║${NC}"
echo -e "${BLUE}║  Shamir's Secret Sharing for Seed Backup and Recovery         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Check if binary exists
if [ ! -f "$BIP_KEYCHAIN" ]; then
    echo -e "${RED}Error: bip-keychain binary not found at: $BIP_KEYCHAIN${NC}"
    echo -e "${YELLOW}Build it with: cargo build --release --features bc${NC}"
    exit 1
fi

# Check if bc feature is enabled
if ! "$BIP_KEYCHAIN" backup-seed --help > /dev/null 2>&1; then
    echo -e "${RED}Error: bip-keychain was not built with BC feature${NC}"
    echo -e "${YELLOW}Rebuild with: cargo build --release --features bc${NC}"
    exit 1
fi

# Create temporary directory for this demo
DEMO_DIR=$(mktemp -d -t bip-keychain-sskr-XXXXXX)
trap "rm -rf $DEMO_DIR" EXIT

echo -e "${GREEN}━━━ Use Case 1: Personal Backup (2-of-3) ━━━${NC}"
echo ""
echo "Scenario: Individual wants to backup seed with 3 trusted parties"
echo "  • Share 1: Family member (spouse/parent)"
echo "  • Share 2: Trusted friend"
echo "  • Share 3: Safe deposit box or secure storage"
echo ""
echo "Security: Any 2 shares can recover the seed"
echo "  ✓ If one location is compromised, seed is still safe"
echo "  ✓ If one share is lost, seed can still be recovered"
echo ""

# Set test seed for demo
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

echo -e "${YELLOW}Creating 2-of-3 backup...${NC}"
"$BIP_KEYCHAIN" backup-seed --groups 3 --threshold 2 --output-dir "$DEMO_DIR/personal-2of3"
echo ""

echo -e "${YELLOW}Share files created:${NC}"
ls -lh "$DEMO_DIR/personal-2of3/"
echo ""

echo -e "${GREEN}━━━ Use Case 2: Enterprise Backup (3-of-5) ━━━${NC}"
echo ""
echo "Scenario: Company backup requiring 3 of 5 executives"
echo "  • Share 1: CEO"
echo "  • Share 2: CTO"
echo "  • Share 3: CFO"
echo "  • Share 4: Board member"
echo "  • Share 5: Legal counsel"
echo ""
echo "Security: Requires 3 executives to recover (business continuity)"
echo "  ✓ No single person has access"
echo "  ✓ Resistant to insider threats"
echo "  ✓ Survives departure of 2 key people"
echo ""

echo -e "${YELLOW}Creating 3-of-5 backup...${NC}"
"$BIP_KEYCHAIN" backup-seed --groups 5 --threshold 3 --output-dir "$DEMO_DIR/enterprise-3of5"
echo ""

echo -e "${YELLOW}Share files created:${NC}"
ls -lh "$DEMO_DIR/enterprise-3of5/"
echo ""

echo -e "${GREEN}━━━ Use Case 3: Couples/Partners (2-of-2) ━━━${NC}"
echo ""
echo "Scenario: Two business partners requiring both for access"
echo "  • Share 1: Partner A"
echo "  • Share 2: Partner B"
echo ""
echo "Security: BOTH shares required (joint control)"
echo "  ✓ Neither partner can act alone"
echo "  ✓ Requires mutual agreement"
echo "  ✓ Equal authority"
echo ""

echo -e "${YELLOW}Creating 2-of-2 backup...${NC}"
"$BIP_KEYCHAIN" backup-seed --groups 2 --threshold 2 --output-dir "$DEMO_DIR/partners-2of2"
echo ""

echo -e "${YELLOW}Share files created:${NC}"
ls -lh "$DEMO_DIR/partners-2of2/"
echo ""

echo -e "${GREEN}━━━ Recovery Demonstration ━━━${NC}"
echo ""
echo -e "${YELLOW}Testing 2-of-3 recovery with shares 1 and 2...${NC}"
echo ""

# Test recovery with 2 shares
RECOVERED_OUTPUT=$("$BIP_KEYCHAIN" recover-seed \
    "$DEMO_DIR/personal-2of3/share-01-of-03.hex" \
    "$DEMO_DIR/personal-2of3/share-02-of-03.hex" 2>&1)

# Extract the recovered seed (it's printed to stdout, between two decorative lines)
RECOVERED=$(echo "$RECOVERED_OUTPUT" | sed -n '/RECOVERED SEED PHRASE/,/SECURITY REMINDER/p' | grep -v "━" | grep -v "RECOVERED" | grep -v "SECURITY" | grep -v "^$" | xargs)

echo ""
echo -e "${GREEN}✓ Recovery successful!${NC}"
echo ""
echo "Original seed: $BIP_KEYCHAIN_SEED"
echo "Recovered:     $RECOVERED"
echo ""

if [ "$RECOVERED" = "$BIP_KEYCHAIN_SEED" ]; then
    echo -e "${GREEN}✓ Seeds match perfectly!${NC}"
else
    echo -e "${RED}✗ Seed mismatch - this should not happen${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Testing recovery with different shares (1 and 3)...${NC}"
echo ""

RECOVERED2_OUTPUT=$("$BIP_KEYCHAIN" recover-seed \
    "$DEMO_DIR/personal-2of3/share-01-of-03.hex" \
    "$DEMO_DIR/personal-2of3/share-03-of-03.hex" 2>&1)

RECOVERED2=$(echo "$RECOVERED2_OUTPUT" | sed -n '/RECOVERED SEED PHRASE/,/SECURITY REMINDER/p' | grep -v "━" | grep -v "RECOVERED" | grep -v "SECURITY" | grep -v "^$" | xargs)

echo ""
if [ "$RECOVERED2" = "$BIP_KEYCHAIN_SEED" ]; then
    echo -e "${GREEN}✓ Recovery with shares 1+3 successful!${NC}"
else
    echo -e "${RED}✗ Recovery failed${NC}"
    exit 1
fi

echo ""
echo -e "${YELLOW}Testing insufficient shares (only 1 of 3)...${NC}"
echo ""

# This should fail - we need 2 shares but only provide 1
set +e
INSUFFICIENT_OUTPUT=$("$BIP_KEYCHAIN" recover-seed \
    "$DEMO_DIR/personal-2of3/share-01-of-03.hex" 2>&1)
INSUFFICIENT_EXIT=$?
set -e

if [ $INSUFFICIENT_EXIT -ne 0 ] && echo "$INSUFFICIENT_OUTPUT" | grep -q "Failed to recover"; then
    echo -e "${GREEN}✓ Correctly rejected insufficient shares${NC}"
else
    echo -e "${RED}✗ Should have rejected single share${NC}"
    echo "Exit code: $INSUFFICIENT_EXIT"
    echo "Output: $INSUFFICIENT_OUTPUT"
    exit 1
fi

echo ""
echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  SSKR Security Properties                                      ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  ✓ Information-theoretically secure                           ║${NC}"
echo -e "${BLUE}║  ✓ Threshold shares reveal NOTHING about the secret           ║${NC}"
echo -e "${BLUE}║  ✓ Any M-of-N combination can recover                         ║${NC}"
echo -e "${BLUE}║  ✓ M-1 shares provide ZERO information                        ║${NC}"
echo -e "${BLUE}║  ✓ Based on Shamir's Secret Sharing (provably secure)         ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Best Practices                                                ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  1. DISTRIBUTE shares to different physical locations          ║${NC}"
echo -e "${BLUE}║  2. LABEL shares clearly (\"Share 1 of 3\", etc.)                ║${NC}"
echo -e "${BLUE}║  3. DOCUMENT the policy (threshold/total) for inheritors       ║${NC}"
echo -e "${BLUE}║  4. TEST recovery immediately after creation                   ║${NC}"
echo -e "${BLUE}║  5. STORE shares securely (fireproof safe, bank vault, etc.)   ║${NC}"
echo -e "${BLUE}║  6. NEVER store threshold or more shares together              ║${NC}"
echo -e "${BLUE}║  7. REVIEW distribution annually (update if needed)            ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${BLUE}╔════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║  Policy Selection Guide                                        ║${NC}"
echo -e "${BLUE}╠════════════════════════════════════════════════════════════════╣${NC}"
echo -e "${BLUE}║  2-of-3: Personal backup (recommended for individuals)         ║${NC}"
echo -e "${BLUE}║    → Distribute to: family, friend, safe deposit box           ║${NC}"
echo -e "${BLUE}║    → Survives: 1 share loss or compromise                      ║${NC}"
echo -e "${BLUE}║                                                                ║${NC}"
echo -e "${BLUE}║  3-of-5: Enterprise/high-value (requires multiple parties)     ║${NC}"
echo -e "${BLUE}║    → Distribute to: executives, board members, legal           ║${NC}"
echo -e "${BLUE}║    → Survives: 2 share losses or compromises                   ║${NC}"
echo -e "${BLUE}║                                                                ║${NC}"
echo -e "${BLUE}║  2-of-2: Joint control (couples, business partners)            ║${NC}"
echo -e "${BLUE}║    → Both parties required for access                          ║${NC}"
echo -e "${BLUE}║    → Maximum protection against unilateral action              ║${NC}"
echo -e "${BLUE}║                                                                ║${NC}"
echo -e "${BLUE}║  4-of-7: High-security enterprise                              ║${NC}"
echo -e "${BLUE}║    → Distributed across multiple executives/locations          ║${NC}"
echo -e "${BLUE}║    → Survives: 3 share losses                                  ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════════════╝${NC}"
echo ""

echo -e "${YELLOW}Real-world workflow:${NC}"
echo ""
echo "1. Generate seed phrase:"
echo "   bip-keychain generate-seed --words 24"
echo ""
echo "2. Set environment variable:"
echo "   export BIP_KEYCHAIN_SEED=\"your generated phrase...\""
echo ""
echo "3. Create SSKR backup:"
echo "   bip-keychain backup-seed --groups 3 --threshold 2"
echo ""
echo "4. Distribute shares to trusted parties/locations"
echo ""
echo "5. Test recovery immediately:"
echo "   bip-keychain recover-seed share-01-of-03.hex share-02-of-03.hex"
echo ""
echo "6. Store original seed securely (metal backup recommended)"
echo ""
echo "7. Document the policy for inheritors:"
echo "   \"This wallet uses 2-of-3 SSKR backup. Any 2 shares can recover.\""
echo ""

echo -e "${GREEN}━━━ Demo Complete ━━━${NC}"
echo ""
echo -e "${GREEN}Demo files saved to: $DEMO_DIR${NC}"
echo -e "${YELLOW}(Will be automatically cleaned up on exit)${NC}"
echo ""
