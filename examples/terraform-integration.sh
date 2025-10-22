#!/usr/bin/env bash
#
# Terraform Integration Script for BIP-Keychain
#
# Generates Terraform variable files with BIP-Keychain derived keys
# Enables Infrastructure as Code with deterministic key management

set -euo pipefail

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
CYAN='\033[0;36m'
NC='\033[0m'

usage() {
    cat <<EOF
Usage: $0 [OPTIONS] <entity-files...>

Generate Terraform variable files with BIP-Keychain derived SSH keys.

OPTIONS:
    -o, --output FILE       Output Terraform tfvars file (default: terraform.tfvars.json)
    -h, --help             Show this help message

ENVIRONMENT:
    BIP_KEYCHAIN_SEED      Required: BIP-39 seed phrase

EXAMPLE TERRAFORM USAGE:
    # variables.tf
    variable "ssh_public_keys" {
      type = map(string)
      description = "SSH public keys for servers"
    }

    # main.tf
    resource "aws_key_pair" "servers" {
      for_each   = var.ssh_public_keys
      key_name   = each.key
      public_key = each.value
    }

EXAMPLES:
    # Generate tfvars for all server entities
    $0 entities/server-*.json

    # Custom output file
    $0 -o prod.tfvars.json entities/prod-*.json

EOF
}

# Parse arguments
OUTPUT_FILE="terraform.tfvars.json"
ENTITY_FILES=()

while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_FILE="$2"
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

echo "Terraform BIP-Keychain Integration"
echo "==================================="
echo "Output file: $OUTPUT_FILE"
echo "Entity files: ${#ENTITY_FILES[@]}"
echo

# Start JSON output
cat > "$OUTPUT_FILE" <<'EOF'
{
  "ssh_public_keys": {
EOF

FIRST=true
for entity_file in "${ENTITY_FILES[@]}"; do
    if [[ ! -f "$entity_file" ]]; then
        echo -e "${YELLOW}⚠${NC} Skipping (not found): $entity_file"
        continue
    fi

    # Extract key name from filename
    key_name=$(basename "$entity_file" .json)

    # Derive SSH public key
    echo -n "Deriving: $key_name ... "
    ssh_key=$(bip-keychain derive "$entity_file" --format ssh 2>/dev/null)

    if [[ -n "$ssh_key" ]]; then
        echo -e "${GREEN}✓${NC}"

        # Add to JSON (handle comma for multiple entries)
        if [[ "$FIRST" == "true" ]]; then
            FIRST=false
        else
            echo "," >> "$OUTPUT_FILE"
        fi

        # Write JSON entry
        cat >> "$OUTPUT_FILE" <<EOF
    "$key_name": "$ssh_key"
EOF
    else
        echo -e "${RED}✗${NC}"
    fi
done

# Close JSON
cat >> "$OUTPUT_FILE" <<'EOF'

  }
}
EOF

echo
echo -e "${GREEN}Terraform variables generated: $OUTPUT_FILE${NC}"
echo

# Show usage instructions
cat <<EOF
${CYAN}Next steps:${NC}

1. Review the generated file:
   ${YELLOW}cat $OUTPUT_FILE${NC}

2. Use in Terraform:
   ${YELLOW}terraform apply -var-file=$OUTPUT_FILE${NC}

3. Example Terraform configuration:

   ${YELLOW}# variables.tf${NC}
   variable "ssh_public_keys" {
     type = map(string)
   }

   ${YELLOW}# main.tf (AWS example)${NC}
   resource "aws_key_pair" "servers" {
     for_each   = var.ssh_public_keys
     key_name   = each.key
     public_key = each.value
   }

   resource "aws_instance" "servers" {
     for_each      = var.ssh_public_keys
     ami           = "ami-12345678"
     instance_type = "t2.micro"
     key_name      = aws_key_pair.servers[each.key].key_name
   }

${CYAN}Key Management:${NC}
- All keys reproducible from BIP_KEYCHAIN_SEED
- Version control $OUTPUT_FILE (safe - public keys only)
- Regenerate anytime: $0 entities/*.json
- Rotate keys: change entity definitions, re-run script

EOF
