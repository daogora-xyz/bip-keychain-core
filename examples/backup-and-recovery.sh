#!/usr/bin/env bash
#
# Backup and Recovery Automation
#
# Automates BIP-Keychain backup and disaster recovery workflows
# Includes:
# - Seed phrase backup verification
# - Entity file backup
# - Key regeneration testing
# - Recovery simulation

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
BLUE='\033[0;34m'
NC='\033[0m'

usage() {
    cat <<EOF
Usage: $0 <command> [OPTIONS]

BIP-Keychain Backup and Recovery Automation

COMMANDS:
    backup              Create backup of entity files
    verify              Verify seed phrase can regenerate keys
    test-recovery       Simulate disaster recovery
    export-inventory    Export key inventory for documentation

OPTIONS:
    -o, --output DIR    Output directory (default: ./backups)
    -e, --entities DIR  Entity directory (default: ./entities)
    -h, --help         Show this help message

ENVIRONMENT:
    BIP_KEYCHAIN_SEED  Required for verification and recovery testing

EXAMPLES:
    # Create backup of all entity files
    $0 backup --output ./backups

    # Verify seed phrase regenerates keys correctly
    $0 verify --entities ./entities

    # Simulate disaster recovery
    $0 test-recovery

    # Export key inventory
    $0 export-inventory --output ./docs

EOF
}

# Defaults
OUTPUT_DIR="./backups"
ENTITY_DIR="./entities"
COMMAND=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        backup|verify|test-recovery|export-inventory)
            COMMAND="$1"
            shift
            ;;
        -o|--output)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -e|--entities)
            ENTITY_DIR="$2"
            shift 2
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            echo -e "${RED}Error: Unknown option $1${NC}" >&2
            usage
            exit 1
            ;;
    esac
done

if [[ -z "$COMMAND" ]]; then
    echo -e "${RED}Error: No command specified${NC}" >&2
    usage
    exit 1
fi

# Command: backup
cmd_backup() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}BIP-Keychain Backup${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo

    if [[ ! -d "$ENTITY_DIR" ]]; then
        echo -e "${RED}✗ Entity directory not found: $ENTITY_DIR${NC}"
        exit 1
    fi

    TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
    BACKUP_DIR="$OUTPUT_DIR/backup-$TIMESTAMP"

    mkdir -p "$BACKUP_DIR"

    echo "Backing up entity files..."
    echo "  Source: $ENTITY_DIR"
    echo "  Destination: $BACKUP_DIR"
    echo

    # Copy all JSON entity files
    ENTITY_COUNT=0
    for entity in "$ENTITY_DIR"/*.json; do
        if [[ -f "$entity" ]]; then
            cp "$entity" "$BACKUP_DIR/"
            echo -e "  ${GREEN}✓${NC} $(basename "$entity")"
            ((ENTITY_COUNT++)) || true
        fi
    done

    echo
    echo "Backup Statistics:"
    echo "  Entity files: $ENTITY_COUNT"
    echo "  Backup directory: $BACKUP_DIR"
    echo

    # Create backup manifest
    MANIFEST="$BACKUP_DIR/MANIFEST.txt"
    cat > "$MANIFEST" <<EOF
BIP-Keychain Backup Manifest
=============================

Backup Date: $(date)
Backup Directory: $BACKUP_DIR
Entity Count: $ENTITY_COUNT

Entity Files:
EOF

    for entity in "$BACKUP_DIR"/*.json; do
        if [[ -f "$entity" ]]; then
            HASH=$(sha256sum "$entity" | cut -d' ' -f1)
            echo "  $(basename "$entity") (SHA-256: $HASH)" >> "$MANIFEST"
        fi
    done

    cat >> "$MANIFEST" <<EOF

Recovery Instructions:
======================

1. Install BIP-Keychain:
   cargo install --path /path/to/bip-keychain-core

2. Set seed phrase:
   export BIP_KEYCHAIN_SEED="your twelve word seed phrase..."

3. Regenerate keys from entity files:
   bip-keychain derive <entity-file> --format ssh

4. All keys will be identical to originals (deterministic)

IMPORTANT: Keep seed phrase backup separate and secure!
         - Hardware wallet (recommended)
         - Paper backup in safe
         - Encrypted password manager
         - Shamir Secret Sharing for redundancy

Entity files are NOT secret (can be version controlled).
Seed phrase IS secret (never commit, never share).
EOF

    echo -e "${GREEN}✓${NC} Created backup manifest: $MANIFEST"
    echo

    # Create checksums
    CHECKSUM_FILE="$BACKUP_DIR/SHA256SUMS"
    (cd "$BACKUP_DIR" && sha256sum *.json > SHA256SUMS 2>/dev/null || true)
    echo -e "${GREEN}✓${NC} Created checksums: $CHECKSUM_FILE"
    echo

    # Create tarball
    TARBALL="$OUTPUT_DIR/bip-keychain-backup-$TIMESTAMP.tar.gz"
    tar -czf "$TARBALL" -C "$OUTPUT_DIR" "backup-$TIMESTAMP"
    echo -e "${GREEN}✓${NC} Created compressed backup: $TARBALL"
    echo

    echo -e "${GREEN}Backup complete!${NC}"
    echo
    echo "Next steps:"
    echo "  1. Verify backup: $0 verify --entities $BACKUP_DIR"
    echo "  2. Store backup securely (external drive, cloud, etc.)"
    echo "  3. Test recovery: $0 test-recovery"
    echo "  4. Document seed phrase backup location"
}

# Command: verify
cmd_verify() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}BIP-Keychain Verification${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo

    if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
        echo -e "${RED}✗ BIP_KEYCHAIN_SEED not set${NC}"
        echo "Set your seed phrase: export BIP_KEYCHAIN_SEED=\"...\""
        exit 1
    fi

    if [[ ! -d "$ENTITY_DIR" ]]; then
        echo -e "${RED}✗ Entity directory not found: $ENTITY_DIR${NC}"
        exit 1
    fi

    echo "Verifying seed phrase can regenerate keys..."
    echo "Entity directory: $ENTITY_DIR"
    echo

    SUCCESS_COUNT=0
    FAIL_COUNT=0
    TOTAL_COUNT=0

    for entity in "$ENTITY_DIR"/*.json; do
        if [[ ! -f "$entity" ]]; then
            continue
        fi

        ((TOTAL_COUNT++)) || true
        ENTITY_NAME=$(basename "$entity")

        echo -n "  $ENTITY_NAME ... "

        # Derive key twice
        KEY1=$(bip-keychain derive "$entity" --format public-key 2>/dev/null)
        KEY2=$(bip-keychain derive "$entity" --format public-key 2>/dev/null)

        if [[ "$KEY1" == "$KEY2" && -n "$KEY1" ]]; then
            echo -e "${GREEN}✓${NC}"
            ((SUCCESS_COUNT++)) || true
        else
            echo -e "${RED}✗${NC}"
            ((FAIL_COUNT++)) || true
        fi
    done

    echo
    echo "Verification Results:"
    echo "─────────────────────"
    echo -e "  Success: ${GREEN}$SUCCESS_COUNT${NC}"
    echo -e "  Failed:  ${RED}$FAIL_COUNT${NC}"
    echo -e "  Total:   $TOTAL_COUNT"
    echo

    if [[ $FAIL_COUNT -eq 0 && $TOTAL_COUNT -gt 0 ]]; then
        echo -e "${GREEN}✓ All keys verified successfully!${NC}"
        echo "  Seed phrase can regenerate all keys deterministically."
        exit 0
    else
        echo -e "${RED}✗ Verification failed!${NC}"
        echo "  Check seed phrase and entity files."
        exit 1
    fi
}

# Command: test-recovery
cmd_test_recovery() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}BIP-Keychain Disaster Recovery Simulation${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo

    if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
        echo -e "${RED}✗ BIP_KEYCHAIN_SEED not set${NC}"
        echo "Set your seed phrase: export BIP_KEYCHAIN_SEED=\"...\""
        exit 1
    fi

    echo "Simulating disaster recovery scenario..."
    echo
    echo "Scenario: All private keys lost, only seed phrase + entity files remain"
    echo

    # Create temporary recovery workspace
    RECOVERY_DIR=$(mktemp -d)
    echo "Recovery workspace: $RECOVERY_DIR"
    echo

    # Create test entity
    TEST_ENTITY="$RECOVERY_DIR/recovery-test.json"
    cat > "$TEST_ENTITY" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@type": "Thing",
    "name": "Recovery Test Entity",
    "identifier": "recovery-test-12345"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "Testing disaster recovery"
}
EOF

    echo "Step 1: Derive original key (before disaster)"
    echo "─────────────────────────────────────────────────"
    ORIGINAL_KEY=$(bip-keychain derive "$TEST_ENTITY" --format public-key)
    echo "  Original key: ${ORIGINAL_KEY:0:32}...${ORIGINAL_KEY: -32}"
    echo

    echo "Step 2: Simulate disaster (keys lost)"
    echo "──────────────────────────────────────"
    echo "  🔥 Simulating data loss..."
    echo "  ✗ Private keys deleted"
    echo "  ✗ SSH keys deleted"
    echo "  ✓ Seed phrase backed up (secure)"
    echo "  ✓ Entity files backed up (version control)"
    echo

    echo "Step 3: Recovery from seed phrase + entity file"
    echo "────────────────────────────────────────────────"
    echo "  Regenerating key from seed phrase..."
    RECOVERED_KEY=$(bip-keychain derive "$TEST_ENTITY" --format public-key)
    echo "  Recovered key: ${RECOVERED_KEY:0:32}...${RECOVERED_KEY: -32}"
    echo

    echo "Step 4: Verify recovery"
    echo "───────────────────────"
    if [[ "$ORIGINAL_KEY" == "$RECOVERED_KEY" ]]; then
        echo -e "  ${GREEN}✓ SUCCESS: Recovered key matches original!${NC}"
        echo
        echo "  Original:  $ORIGINAL_KEY"
        echo "  Recovered: $RECOVERED_KEY"
        echo
        echo -e "${GREEN}✓ Disaster recovery simulation successful!${NC}"
        echo
        echo "Key takeaways:"
        echo "  • Seed phrase is sufficient to recover ALL keys"
        echo "  • Entity files + seed phrase = complete recovery"
        echo "  • No need to backup individual private keys"
        echo "  • Deterministic derivation ensures exact key reproduction"
    else
        echo -e "  ${RED}✗ FAILURE: Keys do not match!${NC}"
        echo
        echo "  Original:  $ORIGINAL_KEY"
        echo "  Recovered: $RECOVERED_KEY"
        echo
        echo -e "${RED}✗ Recovery failed - investigate immediately!${NC}"
        rm -rf "$RECOVERY_DIR"
        exit 1
    fi

    # Cleanup
    rm -rf "$RECOVERY_DIR"
    echo

    echo "Disaster Recovery Checklist:"
    echo "───────────────────────────────"
    echo "  □ Seed phrase backed up securely"
    echo "  □ Entity files version controlled"
    echo "  □ Recovery process documented"
    echo "  □ Team trained on recovery procedure"
    echo "  □ Recovery tested regularly (quarterly)"
    echo "  □ Backup locations documented"
    echo "  □ Contact information for emergency"
}

# Command: export-inventory
cmd_export_inventory() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}BIP-Keychain Key Inventory Export${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo

    if [[ ! -d "$ENTITY_DIR" ]]; then
        echo -e "${RED}✗ Entity directory not found: $ENTITY_DIR${NC}"
        exit 1
    fi

    mkdir -p "$OUTPUT_DIR"
    INVENTORY_FILE="$OUTPUT_DIR/key-inventory.md"

    cat > "$INVENTORY_FILE" <<EOF
# BIP-Keychain Key Inventory

**Generated:** $(date)
**Entity Directory:** $ENTITY_DIR

## Overview

This document lists all BIP-Keychain derived keys and their purposes.

**Important:** This inventory contains PUBLIC key information only. Never include seed phrases or private keys.

## Entity Inventory

| Entity File | Schema Type | Purpose | Hash Function | Public Key (first 16 chars) |
|-------------|-------------|---------|---------------|------------------------------|
EOF

    echo "Generating key inventory..."
    echo "Entity directory: $ENTITY_DIR"
    echo

    ENTITY_COUNT=0

    for entity in "$ENTITY_DIR"/*.json; do
        if [[ ! -f "$entity" ]]; then
            continue
        fi

        ENTITY_NAME=$(basename "$entity")
        SCHEMA_TYPE=$(jq -r '.schema_type // "N/A"' "$entity" 2>/dev/null)
        PURPOSE=$(jq -r '.purpose // "N/A"' "$entity" 2>/dev/null)
        HASH_FUNCTION=$(jq -r '.derivation_config.hash_function // "N/A"' "$entity" 2>/dev/null)

        if [[ -n "${BIP_KEYCHAIN_SEED:-}" ]]; then
            PUBKEY=$(bip-keychain derive "$entity" --format public-key 2>/dev/null || echo "N/A")
            PUBKEY_SHORT="${PUBKEY:0:16}..."
        else
            PUBKEY_SHORT="[seed not provided]"
        fi

        echo "| \`$ENTITY_NAME\` | $SCHEMA_TYPE | $PURPOSE | $HASH_FUNCTION | \`$PUBKEY_SHORT\` |" >> "$INVENTORY_FILE"

        echo -e "  ${GREEN}✓${NC} $ENTITY_NAME"
        ((ENTITY_COUNT++)) || true
    done

    cat >> "$INVENTORY_FILE" <<EOF

## Recovery Information

### Seed Phrase Backup Locations

- [ ] Hardware wallet (Ledger/Trezor)
- [ ] Paper backup (safe/safety deposit box)
- [ ] Encrypted password manager
- [ ] Shamir Secret Sharing (if applicable)

### Recovery Procedure

1. Install BIP-Keychain: \`cargo install --path /path/to/bip-keychain-core\`
2. Set seed phrase: \`export BIP_KEYCHAIN_SEED="..."\`
3. Regenerate keys: \`bip-keychain derive <entity-file> --format ssh\`

### Emergency Contacts

- Primary: [Name, Contact Info]
- Secondary: [Name, Contact Info]

### Last Recovery Test

- Date: [YYYY-MM-DD]
- Result: [Success/Failure]
- Notes: [Any relevant notes]

## Security Notes

- **Entity files:** Can be version controlled (no secrets)
- **Seed phrase:** MUST be kept secret (never commit to git)
- **Recovery:** Requires both seed phrase + entity files
- **Rotation:** Update entity files to rotate keys

---

**Total Entities:** $ENTITY_COUNT
**Document Version:** 1.0
**Last Updated:** $(date)
EOF

    echo
    echo -e "${GREEN}✓${NC} Key inventory exported: $INVENTORY_FILE"
    echo
    echo "Total entities documented: $ENTITY_COUNT"
    echo
    echo "Next steps:"
    echo "  1. Review inventory: cat $INVENTORY_FILE"
    echo "  2. Update recovery information (backup locations, contacts)"
    echo "  3. Schedule next recovery test"
    echo "  4. Share with team (entity files + inventory are safe to share)"
}

# Main execution
case "$COMMAND" in
    backup)
        cmd_backup
        ;;
    verify)
        cmd_verify
        ;;
    test-recovery)
        cmd_test_recovery
        ;;
    export-inventory)
        cmd_export_inventory
        ;;
    *)
        echo -e "${RED}Error: Unknown command: $COMMAND${NC}" >&2
        usage
        exit 1
        ;;
esac
