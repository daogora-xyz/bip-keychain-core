#!/usr/bin/env bash
#
# SSH Server Provisioning Script
#
# Automatically provision SSH keys to multiple servers
# Derives keys from entity files and deploys to authorized_keys

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

usage() {
    cat <<EOF
Usage: $0 [OPTIONS] <entity-file> <ssh-targets...>

Provision SSH keys to remote servers using BIP-Keychain.

ARGUMENTS:
    entity-file             Entity JSON file to derive SSH key from
    ssh-targets             SSH targets (user@host format)

OPTIONS:
    -n, --dry-run          Show what would be done without doing it
    -h, --help            Show this help message

ENVIRONMENT:
    BIP_KEYCHAIN_SEED     Required: BIP-39 seed phrase

EXAMPLES:
    # Provision key to single server
    $0 entities/github-deploy.json user@github-server.com

    # Provision to multiple servers
    $0 entities/backup-key.json user@backup1.com user@backup2.com

    # Dry run (test without deploying)
    $0 --dry-run entities/test.json user@test-server.com

EOF
}

# Parse arguments
DRY_RUN=false
ENTITY_FILE=""
SSH_TARGETS=()

while [[ $# -gt 0 ]]; do
    case $1 in
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
            if [[ -z "$ENTITY_FILE" ]]; then
                ENTITY_FILE="$1"
            else
                SSH_TARGETS+=("$1")
            fi
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

if [[ ${#SSH_TARGETS[@]} -eq 0 ]]; then
    echo -e "${RED}Error: No SSH targets specified${NC}" >&2
    usage
    exit 1
fi

if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
    echo -e "${RED}Error: BIP_KEYCHAIN_SEED environment variable not set${NC}" >&2
    exit 1
fi

# Header
echo "SSH Server Provisioning"
echo "======================="
echo "Entity: $ENTITY_FILE"
echo "Targets: ${#SSH_TARGETS[@]}"
if [[ "$DRY_RUN" == "true" ]]; then
    echo -e "${YELLOW}Mode: DRY RUN (no changes will be made)${NC}"
fi
echo

# Derive SSH public key
echo -n "Deriving SSH public key... "
SSH_KEY=$(bip-keychain derive "$ENTITY_FILE" --format ssh)
if [[ -z "$SSH_KEY" ]]; then
    echo -e "${RED}✗ Failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓${NC}"
echo
echo -e "${CYAN}SSH Public Key:${NC}"
echo "$SSH_KEY"
echo

# Provision to each target
SUCCESS_COUNT=0
FAIL_COUNT=0

for target in "${SSH_TARGETS[@]}"; do
    echo "Provisioning: $target"

    if [[ "$DRY_RUN" == "true" ]]; then
        echo "  [DRY RUN] Would execute:"
        echo "    ssh $target 'mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo \"$SSH_KEY\" >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys'"
        echo -e "  ${GREEN}✓${NC} (dry run)"
        ((SUCCESS_COUNT++)) || true
    else
        # Deploy key
        if ssh "$target" "mkdir -p ~/.ssh && chmod 700 ~/.ssh && echo '$SSH_KEY' >> ~/.ssh/authorized_keys && chmod 600 ~/.ssh/authorized_keys" 2>/dev/null; then
            echo -e "  ${GREEN}✓${NC} Key added to authorized_keys"
            ((SUCCESS_COUNT++)) || true
        else
            echo -e "  ${RED}✗${NC} Failed to provision"
            ((FAIL_COUNT++)) || true
        fi
    fi
    echo
done

# Summary
echo "Summary"
echo "======="
echo -e "Success: ${GREEN}${SUCCESS_COUNT}${NC}"
echo -e "Failed:  ${RED}${FAIL_COUNT}${NC}"
echo -e "Total:   $((SUCCESS_COUNT + FAIL_COUNT))"

if [[ $FAIL_COUNT -eq 0 ]]; then
    echo
    echo -e "${GREEN}All servers provisioned successfully!${NC}"

    if [[ "$DRY_RUN" == "false" ]]; then
        echo
        echo "Next steps:"
        echo "  1. Test SSH access: ssh <target>"
        echo "  2. Verify key fingerprint"
        echo "  3. Disable password authentication (optional)"
    fi
    exit 0
else
    echo
    echo -e "${YELLOW}Some servers failed to provision${NC}"
    exit 1
fi
