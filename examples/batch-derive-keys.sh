#!/usr/bin/env bash
#
# Batch Key Derivation Script
#
# Derives keys from multiple entity files in batch mode
# Useful for:
# - Initial setup of multiple keys
# - Key regeneration after disaster recovery
# - Automated provisioning

set -euo pipefail

# Configuration
OUTPUT_DIR="${OUTPUT_DIR:-./derived-keys}"
OUTPUT_FORMAT="${OUTPUT_FORMAT:-ssh}"  # seed, public-key, private-key, ssh, gpg, json

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

usage() {
    cat <<EOF
Usage: $0 [OPTIONS] <entity-files...>

Batch derive keys from multiple entity files.

OPTIONS:
    -o, --output-dir DIR     Output directory (default: ./derived-keys)
    -f, --format FORMAT      Output format: seed, public-key, private-key, ssh, gpg, json
                             (default: ssh)
    -h, --help              Show this help message

ENVIRONMENT:
    BIP_KEYCHAIN_SEED       Required: BIP-39 seed phrase

EXAMPLES:
    # Derive SSH keys for all servers
    $0 entities/server-*.json

    # Derive GPG keys for all identities
    OUTPUT_FORMAT=gpg $0 entities/identity-*.json

    # Custom output directory
    $0 -o /tmp/keys -f public-key entities/*.json

EOF
}

# Parse arguments
ENTITY_FILES=()
while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output-dir)
            OUTPUT_DIR="$2"
            shift 2
            ;;
        -f|--format)
            OUTPUT_FORMAT="$2"
            shift 2
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
            ENTITY_FILES+=("$1")
            shift
            ;;
    esac
done

# Validate
if [[ ${#ENTITY_FILES[@]} -eq 0 ]]; then
    echo -e "${RED}Error: No entity files specified${NC}" >&2
    usage
    exit 1
fi

if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
    echo -e "${RED}Error: BIP_KEYCHAIN_SEED environment variable not set${NC}" >&2
    exit 1
fi

# Create output directory
mkdir -p "$OUTPUT_DIR"

echo "Batch Key Derivation"
echo "===================="
echo "Output directory: $OUTPUT_DIR"
echo "Output format: $OUTPUT_FORMAT"
echo "Entity files: ${#ENTITY_FILES[@]}"
echo

# Process each entity file
SUCCESS_COUNT=0
FAIL_COUNT=0

for entity_file in "${ENTITY_FILES[@]}"; do
    if [[ ! -f "$entity_file" ]]; then
        echo -e "${YELLOW}⚠${NC} Skipping (not found): $entity_file"
        ((FAIL_COUNT++)) || true
        continue
    fi

    # Extract basename for output file
    basename=$(basename "$entity_file" .json)
    output_file="$OUTPUT_DIR/${basename}.${OUTPUT_FORMAT}"

    echo -n "Deriving: $entity_file ... "

    if bip-keychain derive "$entity_file" --format "$OUTPUT_FORMAT" > "$output_file" 2>/dev/null; then
        echo -e "${GREEN}✓${NC}"
        echo "  → $output_file"
        ((SUCCESS_COUNT++)) || true
    else
        echo -e "${RED}✗${NC}"
        rm -f "$output_file"
        ((FAIL_COUNT++)) || true
    fi
done

# Summary
echo
echo "Summary"
echo "======="
echo -e "Success: ${GREEN}${SUCCESS_COUNT}${NC}"
echo -e "Failed:  ${RED}${FAIL_COUNT}${NC}"
echo -e "Total:   $((SUCCESS_COUNT + FAIL_COUNT))"
echo
echo "Output directory: $OUTPUT_DIR"

if [[ $FAIL_COUNT -eq 0 ]]; then
    echo -e "${GREEN}All keys derived successfully!${NC}"
    exit 0
else
    echo -e "${YELLOW}Some keys failed to derive${NC}"
    exit 1
fi
