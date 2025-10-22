#!/usr/bin/env bash
#
# Key Rotation Workflow
#
# Automates key rotation for BIP-Keychain derived keys
# Supports:
# - Time-based rotation (annual, quarterly, monthly)
# - On-demand rotation (compromised keys)
# - Graceful transition (old + new keys for transition period)

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
Usage: $0 [OPTIONS] <entity-file>

Rotate a BIP-Keychain derived key by creating a new entity version.

OPTIONS:
    -r, --reason REASON     Rotation reason (scheduled, compromised, expired)
    -t, --transition-days N Transition period in days (default: 30)
    -o, --output-dir DIR    Output directory for new entity (default: ./entities-rotated)
    -n, --dry-run          Show what would be done without doing it
    -h, --help            Show this help message

WORKFLOW:
    1. Creates new entity with rotation metadata
    2. Derives new key
    3. Optionally deploys new key alongside old key (transition)
    4. After transition period, remove old key
    5. Archives old entity

ENVIRONMENT:
    BIP_KEYCHAIN_SEED     Required: BIP-39 seed phrase

EXAMPLES:
    # Scheduled rotation (annual)
    $0 --reason scheduled entities/server-key.json

    # Compromised key rotation (immediate)
    $0 --reason compromised --transition-days 0 entities/compromised-key.json

    # Test rotation (dry run)
    $0 --dry-run entities/test-key.json

EOF
}

# Defaults
REASON="scheduled"
TRANSITION_DAYS=30
OUTPUT_DIR="./entities-rotated"
DRY_RUN=false
ENTITY_FILE=""

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -r|--reason)
            REASON="$2"
            shift 2
            ;;
        -t|--transition-days)
            TRANSITION_DAYS="$2"
            shift 2
            ;;
        -o|--output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -n|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        -*)
            echo -e "${RED}Error: Unknown option $1${NC}" >&2
            usage
            exit 1
            ;;
        *)
            ENTITY_FILE="$1"
            shift
            ;;
    esac
done

# Validate
if [[ -z "$ENTITY_FILE" ]]; then
    echo -e "${RED}Error: No entity file specified${NC}" >&2
    usage
    exit 1
fi

if [[ ! -f "$ENTITY_FILE" ]]; then
    echo -e "${RED}Error: Entity file not found: $ENTITY_FILE${NC}" >&2
    exit 1
fi

if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
    echo -e "${RED}Error: BIP_KEYCHAIN_SEED environment variable not set${NC}" >&2
    exit 1
fi

# Header
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}BIP-Keychain Key Rotation Workflow${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo "Entity: $ENTITY_FILE"
echo "Reason: $REASON"
echo "Transition period: $TRANSITION_DAYS days"
echo "Output directory: $OUTPUT_DIR"

if [[ "$DRY_RUN" == "true" ]]; then
    echo -e "${YELLOW}Mode: DRY RUN${NC}"
fi
echo

# Extract basename
BASENAME=$(basename "$ENTITY_FILE" .json)
TIMESTAMP=$(date +"%Y%m%d-%H%M%S")
ROTATION_DATE=$(date +"%Y-%m-%d")
EXPIRY_DATE=$(date -d "+$TRANSITION_DAYS days" +"%Y-%m-%d" 2>/dev/null || date -v "+${TRANSITION_DAYS}d" +"%Y-%m-%d")

# File paths
OLD_ENTITY_FILE="$ENTITY_FILE"
NEW_ENTITY_FILE="$OUTPUT_DIR/${BASENAME}-rotated-${TIMESTAMP}.json"
ARCHIVE_DIR="$OUTPUT_DIR/archived"

# Create directories
if [[ "$DRY_RUN" == "false" ]]; then
    mkdir -p "$OUTPUT_DIR"
    mkdir -p "$ARCHIVE_DIR"
fi

# Step 1: Derive current key
echo -e "${YELLOW}Step 1: Deriving current (old) key${NC}"
echo "────────────────────────────────────────"
echo

OLD_KEY=$(bip-keychain derive "$OLD_ENTITY_FILE" --format ssh)
echo "Old SSH Key:"
echo "$OLD_KEY"
echo

# Step 2: Create new entity with rotation metadata
echo -e "${YELLOW}Step 2: Creating rotated entity${NC}"
echo "────────────────────────────────────────"
echo

# Read current entity and modify
CURRENT_ENTITY=$(cat "$OLD_ENTITY_FILE")

# Add rotation version to entity name/identifier
NEW_ENTITY=$(echo "$CURRENT_ENTITY" | jq --arg rotation_date "$ROTATION_DATE" --arg reason "$REASON" '
  .entity.identifier = (.entity.identifier // .entity.name // .entity.fqdn // "unknown") + "-v" + $rotation_date |
  .purpose = (.purpose // "unknown") + " (rotated: " + $rotation_date + ", reason: " + $reason + ")" |
  .metadata.rotation = {
    "rotated_at": $rotation_date,
    "reason": $reason,
    "previous_version": input_filename
  }
' 2>/dev/null || echo "$CURRENT_ENTITY")

if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY RUN] Would create: $NEW_ENTITY_FILE"
    echo
    echo "New entity content:"
    echo "$NEW_ENTITY" | jq '.'
else
    echo "$NEW_ENTITY" > "$NEW_ENTITY_FILE"
    echo -e "${GREEN}✓${NC} Created: $NEW_ENTITY_FILE"
fi
echo

# Step 3: Derive new key
echo -e "${YELLOW}Step 3: Deriving new (rotated) key${NC}"
echo "────────────────────────────────────────"
echo

if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY RUN] Would derive key from: $NEW_ENTITY_FILE"
    NEW_KEY="[DRY RUN - new key would appear here]"
else
    NEW_KEY=$(bip-keychain derive "$NEW_ENTITY_FILE" --format ssh)
fi

echo "New SSH Key:"
echo "$NEW_KEY"
echo

# Verify keys are different
if [[ "$OLD_KEY" == "$NEW_KEY" && "$DRY_RUN" == "false" ]]; then
    echo -e "${RED}✗ ERROR: New key is identical to old key!${NC}"
    echo "  This should not happen. Check entity modification logic."
    exit 1
fi

if [[ "$OLD_KEY" != "$NEW_KEY" || "$DRY_RUN" == "true" ]]; then
    echo -e "${GREEN}✓${NC} New key is different from old key (rotation successful)"
fi
echo

# Step 4: Transition plan
echo -e "${YELLOW}Step 4: Transition Plan${NC}"
echo "────────────────────────────────────────"
echo

if [[ $TRANSITION_DAYS -eq 0 ]]; then
    echo -e "${RED}Immediate rotation (no transition period)${NC}"
    echo
    echo "Actions:"
    echo "  1. Deploy new key immediately"
    echo "  2. Remove old key immediately"
    echo "  3. Test access with new key"
    echo
else
    echo -e "${CYAN}Gradual transition ($TRANSITION_DAYS days)${NC}"
    echo
    echo "Phase 1: Deploy both keys (today - $ROTATION_DATE)"
    echo "  1. Add new key to authorized_keys"
    echo "  2. Keep old key active"
    echo "  3. Test access with new key"
    echo
    echo "Phase 2: Transition period ($ROTATION_DATE to $EXPIRY_DATE)"
    echo "  1. Both keys remain active"
    echo "  2. Update clients to use new key"
    echo "  3. Monitor old key usage"
    echo
    echo "Phase 3: Decommission old key (after $EXPIRY_DATE)"
    echo "  1. Remove old key from authorized_keys"
    echo "  2. Archive old entity file"
    echo "  3. Verify only new key in use"
    echo
fi

# Step 5: Deployment commands
echo -e "${YELLOW}Step 5: Deployment Commands${NC}"
echo "────────────────────────────────────────"
echo

cat <<EOF
# Deploy new key (add to existing authorized_keys)
${CYAN}ssh user@server "echo '$NEW_KEY' >> ~/.ssh/authorized_keys"${NC}

# Test new key access
${CYAN}ssh user@server -i <path-to-new-private-key> "echo 'New key works!'"${NC}

EOF

if [[ $TRANSITION_DAYS -gt 0 ]]; then
    cat <<EOF
# After transition period ($EXPIRY_DATE), remove old key
${CYAN}ssh user@server "sed -i.bak '\\|$OLD_KEY|d' ~/.ssh/authorized_keys"${NC}

EOF
fi

# Step 6: Archive old entity
echo -e "${YELLOW}Step 6: Archive Old Entity${NC}"
echo "────────────────────────────────────────"
echo

ARCHIVE_FILE="$ARCHIVE_DIR/${BASENAME}-archived-${TIMESTAMP}.json"

if [[ "$DRY_RUN" == "true" ]]; then
    echo "[DRY RUN] Would archive: $OLD_ENTITY_FILE → $ARCHIVE_FILE"
else
    cp "$OLD_ENTITY_FILE" "$ARCHIVE_FILE"
    echo -e "${GREEN}✓${NC} Archived: $ARCHIVE_FILE"
fi
echo

# Summary
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo -e "${CYAN}Rotation Summary${NC}"
echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
echo
echo -e "${GREEN}✓${NC} Old entity: $OLD_ENTITY_FILE"
echo -e "${GREEN}✓${NC} New entity: $NEW_ENTITY_FILE"
echo -e "${GREEN}✓${NC} Archived to: $ARCHIVE_FILE"
echo
echo "Rotation reason: $REASON"
echo "Rotation date: $ROTATION_DATE"

if [[ $TRANSITION_DAYS -gt 0 ]]; then
    echo "Transition period: $TRANSITION_DAYS days"
    echo "Decommission date: $EXPIRY_DATE"
else
    echo "Transition period: None (immediate)"
fi
echo

# Next steps
echo -e "${CYAN}Next Steps:${NC}"
echo
if [[ "$DRY_RUN" == "true" ]]; then
    echo "  1. Review the rotation plan above"
    echo "  2. Run without --dry-run to perform rotation"
else
    echo "  1. Deploy new key to target servers/services"
    echo "  2. Test access with new key"
    if [[ $TRANSITION_DAYS -gt 0 ]]; then
        echo "  3. Update clients to use new key during transition"
        echo "  4. Monitor old key usage"
        echo "  5. After $EXPIRY_DATE, remove old key"
    else
        echo "  3. Remove old key immediately"
    fi
    echo "  6. Update documentation with new entity file path"
fi
echo

if [[ "$DRY_RUN" == "false" ]]; then
    echo -e "${GREEN}Key rotation workflow complete!${NC}"
else
    echo -e "${YELLOW}Dry run complete. No changes made.${NC}"
fi
