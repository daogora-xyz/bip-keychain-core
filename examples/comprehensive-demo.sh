#!/usr/bin/env bash
#
# BIP-Keychain Comprehensive Demo
#
# Demonstrates all features of BIP-Keychain:
# - Multiple hash functions (HMAC-SHA-512, BLAKE2b, SHA-256)
# - All output formats (seed, public-key, private-key, ssh, gpg, json)
# - Multiple entity types
# - Determinism verification
# - Uniqueness verification
# - Real-world use cases

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Helper functions
print_header() {
    echo
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${CYAN}$1${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo
}

print_subheader() {
    echo
    echo -e "${YELLOW}▶ $1${NC}"
    echo
}

print_success() {
    echo -e "${GREEN}✓${NC} $1"
}

print_info() {
    echo -e "${CYAN}ℹ${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

pause() {
    echo
    read -p "Press Enter to continue..." -r
}

# Check prerequisites
check_prerequisites() {
    print_header "Checking Prerequisites"

    if ! command -v bip-keychain &> /dev/null; then
        print_error "bip-keychain not found in PATH"
        echo "Install with: cargo install --path ."
        exit 1
    fi
    print_success "bip-keychain found: $(which bip-keychain)"

    if [[ -z "${BIP_KEYCHAIN_SEED:-}" ]]; then
        print_warning "BIP_KEYCHAIN_SEED not set, using test mnemonic"
        export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
        print_info "Set to: $BIP_KEYCHAIN_SEED"
    else
        print_success "BIP_KEYCHAIN_SEED is set (${#BIP_KEYCHAIN_SEED} characters)"
    fi

    print_success "All prerequisites met"
}

# Demo 1: Output formats
demo_output_formats() {
    print_header "Demo 1: All Output Formats"

    ENTITY_FILE="examples/github-repo.json"

    if [[ ! -f "$ENTITY_FILE" ]]; then
        print_error "Entity file not found: $ENTITY_FILE"
        return
    fi

    print_info "Using entity: $ENTITY_FILE"
    echo

    # Show entity contents
    print_subheader "Entity Definition"
    cat "$ENTITY_FILE"
    echo

    # Format: seed (hex)
    print_subheader "Format: seed (raw 32-byte seed as hex)"
    bip-keychain derive "$ENTITY_FILE" --format seed
    echo

    # Format: public-key
    print_subheader "Format: public-key (Ed25519 public key as hex)"
    bip-keychain derive "$ENTITY_FILE" --format public-key
    echo

    # Format: private-key
    print_subheader "Format: private-key (Ed25519 private key as hex)"
    print_warning "DANGEROUS: Only for demonstration!"
    bip-keychain derive "$ENTITY_FILE" --format private-key
    echo

    # Format: ssh
    print_subheader "Format: ssh (OpenSSH public key)"
    bip-keychain derive "$ENTITY_FILE" --format ssh
    echo

    # Format: gpg
    print_subheader "Format: gpg (GPG-compatible public key info)"
    bip-keychain derive "$ENTITY_FILE" --format gpg
    echo

    # Format: json
    print_subheader "Format: json (complete metadata)"
    bip-keychain derive "$ENTITY_FILE" --format json
    echo

    print_success "All output formats demonstrated"
}

# Demo 2: Hash functions
demo_hash_functions() {
    print_header "Demo 2: Multiple Hash Functions"

    print_info "BIP-Keychain supports three hash functions:"
    echo "  1. HMAC-SHA-512 (BIP-85 standard)"
    echo "  2. BLAKE2b (Blockchain Commons)"
    echo "  3. SHA-256 (compatibility)"
    echo

    # Create temporary entities with different hash functions
    TEMP_DIR=$(mktemp -d)

    # HMAC-SHA-512 entity
    cat > "$TEMP_DIR/hmac-sha512.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test Entity"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Testing HMAC-SHA-512"
}
EOF

    # BLAKE2b entity
    cat > "$TEMP_DIR/blake2b.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test Entity"},
  "derivation_config": {"hash_function": "blake2b", "hardened": true},
  "purpose": "Testing BLAKE2b"
}
EOF

    # SHA-256 entity
    cat > "$TEMP_DIR/sha256.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Thing", "name": "Test Entity"},
  "derivation_config": {"hash_function": "sha256", "hardened": true},
  "purpose": "Testing SHA-256"
}
EOF

    print_subheader "HMAC-SHA-512 (BIP-85 standard)"
    KEY_HMAC=$(bip-keychain derive "$TEMP_DIR/hmac-sha512.json" --format public-key)
    echo "$KEY_HMAC"
    echo

    print_subheader "BLAKE2b (Blockchain Commons)"
    KEY_BLAKE=$(bip-keychain derive "$TEMP_DIR/blake2b.json" --format public-key)
    echo "$KEY_BLAKE"
    echo

    print_subheader "SHA-256 (compatibility)"
    KEY_SHA=$(bip-keychain derive "$TEMP_DIR/sha256.json" --format public-key)
    echo "$KEY_SHA"
    echo

    print_info "Note: Same entity with different hash functions produces different keys"
    print_info "This provides algorithm agility and future-proofing"

    # Cleanup
    rm -rf "$TEMP_DIR"

    print_success "All hash functions demonstrated"
}

# Demo 3: Determinism
demo_determinism() {
    print_header "Demo 3: Determinism Verification"

    print_info "Deriving the same entity multiple times..."
    echo

    ENTITY_FILE="examples/github-repo.json"

    print_subheader "Derivation #1"
    KEY1=$(bip-keychain derive "$ENTITY_FILE" --format public-key)
    echo "$KEY1"

    print_subheader "Derivation #2"
    KEY2=$(bip-keychain derive "$ENTITY_FILE" --format public-key)
    echo "$KEY2"

    print_subheader "Derivation #3"
    KEY3=$(bip-keychain derive "$ENTITY_FILE" --format public-key)
    echo "$KEY3"

    echo
    if [[ "$KEY1" == "$KEY2" && "$KEY2" == "$KEY3" ]]; then
        print_success "DETERMINISM VERIFIED: All three derivations are identical"
        print_info "Same entity + same seed = same key (always)"
    else
        print_error "DETERMINISM FAILED: Keys differ!"
        exit 1
    fi
}

# Demo 4: Uniqueness
demo_uniqueness() {
    print_header "Demo 4: Uniqueness Verification"

    print_info "Deriving keys from different entities..."
    echo

    # Create temporary entities with different content
    TEMP_DIR=$(mktemp -d)

    cat > "$TEMP_DIR/alice.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Person", "name": "Alice"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}
EOF

    cat > "$TEMP_DIR/bob.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Person", "name": "Bob"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}
EOF

    cat > "$TEMP_DIR/charlie.json" <<'EOF'
{
  "schema_type": "schema_org",
  "entity": {"@type": "Person", "name": "Charlie"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true}
}
EOF

    print_subheader "Entity: Alice"
    KEY_ALICE=$(bip-keychain derive "$TEMP_DIR/alice.json" --format public-key)
    echo "$KEY_ALICE"

    print_subheader "Entity: Bob"
    KEY_BOB=$(bip-keychain derive "$TEMP_DIR/bob.json" --format public-key)
    echo "$KEY_BOB"

    print_subheader "Entity: Charlie"
    KEY_CHARLIE=$(bip-keychain derive "$TEMP_DIR/charlie.json" --format public-key)
    echo "$KEY_CHARLIE"

    echo
    if [[ "$KEY_ALICE" != "$KEY_BOB" && "$KEY_BOB" != "$KEY_CHARLIE" && "$KEY_ALICE" != "$KEY_CHARLIE" ]]; then
        print_success "UNIQUENESS VERIFIED: All keys are different"
        print_info "Different entities = different keys (collision-resistant)"
    else
        print_error "UNIQUENESS FAILED: Keys collision detected!"
        exit 1
    fi

    # Cleanup
    rm -rf "$TEMP_DIR"
}

# Demo 5: Real-world use cases
demo_use_cases() {
    print_header "Demo 5: Real-World Use Cases"

    print_subheader "Use Case 1: SSH Server Access"
    if [[ -f "examples/sha256-example.json" ]]; then
        print_info "Deriving SSH key for backup server..."
        bip-keychain derive examples/sha256-example.json --format ssh
        echo
        print_info "This key can be added to server's ~/.ssh/authorized_keys"
    fi

    print_subheader "Use Case 2: GitHub Deploy Key"
    if [[ -f "examples/github-repo.json" ]]; then
        print_info "Deriving GitHub deploy key..."
        bip-keychain derive examples/github-repo.json --format ssh
        echo
        print_info "Add to GitHub: Settings → Deploy keys → Add deploy key"
    fi

    print_subheader "Use Case 3: Personal Identity (DID)"
    if [[ -f "examples/did-identity.json" ]]; then
        print_info "Deriving DID-based identity key..."
        bip-keychain derive examples/did-identity.json --format json | head -n 10
        echo
        print_info "Complete JSON output contains all key material"
    fi

    print_subheader "Use Case 4: Email Signing"
    if [[ -f "examples/email-signing.json" ]]; then
        print_info "Deriving email signing key..."
        bip-keychain derive examples/email-signing.json --format gpg | head -n 15
        echo
        print_info "Use with GPG for S/MIME or PGP email signing"
    fi

    print_subheader "Use Case 5: Code Signing"
    if [[ -f "examples/software-app-signing.json" ]]; then
        print_info "Deriving software application signing key..."
        bip-keychain derive examples/software-app-signing.json --format public-key
        echo
        print_info "Use for Authenticode, codesign, or other code signing"
    fi

    print_success "Real-world use cases demonstrated"
}

# Demo 6: Entity types
demo_entity_types() {
    print_header "Demo 6: Diverse Entity Types"

    print_info "BIP-Keychain supports many semantic entity types:"
    echo

    EXAMPLES=(
        "person-identity.json:Schema.org Person"
        "organization-signing.json:Schema.org Organization"
        "x509-distinguished-name.json:X.509 Distinguished Name"
        "ipfs-content-signing.json:IPFS CID"
        "urn-namespace.json:URN Identifier"
        "verifiable-credential.json:W3C Verifiable Credential"
        "email-signing.json:Email Address"
        "software-app-signing.json:Software Application"
        "api-service-key.json:API Service (DNS)"
        "iot-device-key.json:IoT Device"
        "blockchain-identity.json:Blockchain Identity"
    )

    for example in "${EXAMPLES[@]}"; do
        IFS=':' read -r file description <<< "$example"
        filepath="examples/$file"

        if [[ -f "$filepath" ]]; then
            print_subheader "$description"
            PUBKEY=$(bip-keychain derive "$filepath" --format public-key 2>/dev/null || echo "N/A")
            echo "Public Key: ${PUBKEY:0:32}...${PUBKEY: -32}"

            # Extract purpose from JSON
            PURPOSE=$(grep -o '"purpose": "[^"]*"' "$filepath" | cut -d'"' -f4 || echo "N/A")
            echo "Purpose: $PURPOSE"
        fi
    done

    echo
    print_success "Diverse entity types demonstrated"
}

# Demo 7: Parent entropy
demo_parent_entropy() {
    print_header "Demo 7: Custom Parent Entropy"

    print_info "Parent entropy adds an additional layer of derivation"
    print_info "Same entity + different parent entropy = different keys"
    echo

    ENTITY_FILE="examples/github-repo.json"

    print_subheader "Default parent entropy"
    KEY_DEFAULT=$(bip-keychain derive "$ENTITY_FILE" --format public-key)
    echo "$KEY_DEFAULT"

    print_subheader "Custom parent entropy: 'production'"
    CUSTOM_ENTROPY_1=$(echo -n "production" | xxd -p)
    KEY_CUSTOM_1=$(bip-keychain derive "$ENTITY_FILE" --parent-entropy "$CUSTOM_ENTROPY_1" --format public-key)
    echo "$KEY_CUSTOM_1"

    print_subheader "Custom parent entropy: 'development'"
    CUSTOM_ENTROPY_2=$(echo -n "development" | xxd -p)
    KEY_CUSTOM_2=$(bip-keychain derive "$ENTITY_FILE" --parent-entropy "$CUSTOM_ENTROPY_2" --format public-key)
    echo "$KEY_CUSTOM_2"

    echo
    if [[ "$KEY_DEFAULT" != "$KEY_CUSTOM_1" && "$KEY_CUSTOM_1" != "$KEY_CUSTOM_2" ]]; then
        print_success "PARENT ENTROPY VERIFIED: Different entropy = different keys"
        print_info "Use for: environment separation, multi-tenancy, etc."
    else
        print_error "PARENT ENTROPY FAILED!"
        exit 1
    fi
}

# Summary
print_summary() {
    print_header "Demo Complete: Summary"

    print_success "BIP-Keychain Features Demonstrated:"
    echo
    echo "  ✓ All output formats (seed, public-key, private-key, ssh, gpg, json)"
    echo "  ✓ Multiple hash functions (HMAC-SHA-512, BLAKE2b, SHA-256)"
    echo "  ✓ Determinism (same input = same output)"
    echo "  ✓ Uniqueness (different inputs = different outputs)"
    echo "  ✓ Real-world use cases (SSH, GitHub, DIDs, email, code signing)"
    echo "  ✓ Diverse entity types (Schema.org, DNS, DIDs, X.509, IPFS, etc.)"
    echo "  ✓ Parent entropy customization"
    echo

    print_info "Key Advantages:"
    echo "  • Single seed phrase backs up all keys"
    echo "  • Fully reproducible on any machine"
    echo "  • Semantic organization (human-readable entities)"
    echo "  • Standards-compliant (BIP-32, BIP-39, BIP-85, Ed25519)"
    echo

    print_info "Next Steps:"
    echo "  1. Read: CLI-USAGE.md for complete command reference"
    echo "  2. Read: SSH-KEYS-GUIDE.md for SSH key workflows"
    echo "  3. Read: GIT-SIGNING-GUIDE.md for Git signing integration"
    echo "  4. Read: NICKEL-WORKFLOW.md for type-safe config files"
    echo "  5. Explore: examples/ directory for more entity definitions"
    echo

    print_header "Thank you for trying BIP-Keychain!"
}

# Main execution
main() {
    clear

    print_header "BIP-Keychain Comprehensive Demo"
    print_info "This demo showcases all features of BIP-Keychain"
    print_info "Version: $(bip-keychain --version 2>/dev/null || echo 'development')"
    echo
    print_warning "This demo uses a test seed phrase (publicly known)"
    print_warning "Never use test seed phrases for real keys!"

    pause

    check_prerequisites
    pause

    demo_output_formats
    pause

    demo_hash_functions
    pause

    demo_determinism
    pause

    demo_uniqueness
    pause

    demo_use_cases
    pause

    demo_entity_types
    pause

    demo_parent_entropy
    pause

    print_summary
}

# Run demo
main "$@"
