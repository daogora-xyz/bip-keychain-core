#!/bin/bash
# Export all Nickel examples to JSON
#
# This script exports all .ncl files in nickel/examples/ to JSON format
# suitable for use with the bip-keychain CLI.
#
# Requirements: Nickel must be installed (nickel-lang-cli or via nix)
#
# Usage: ./export-nickel-examples.sh

set -e

echo "=== Exporting Nickel Examples to JSON ==="
echo ""

# Check if nickel is installed
if ! command -v nickel &> /dev/null; then
    echo "ERROR: nickel command not found!"
    echo ""
    echo "Please install Nickel:"
    echo "  - Using Nix: nix-env -iA nixpkgs.nickel"
    echo "  - Using Cargo: cargo install nickel-lang-cli"
    echo "  - Download binary: https://github.com/tweag/nickel/releases"
    echo ""
    echo "See NICKEL-WORKFLOW.md for detailed installation instructions."
    exit 1
fi

echo "Found Nickel: $(nickel --version)"
echo ""

# Create output directory if needed
mkdir -p examples

# Counter for exported files
count=0

# Export each .ncl file
for ncl_file in nickel/examples/*.ncl; do
    if [ -f "$ncl_file" ]; then
        # Get basename without extension
        basename=$(basename "$ncl_file" .ncl)

        # Output JSON file
        json_file="examples/${basename}.json"

        echo "Exporting: $ncl_file"
        echo "       to: $json_file"

        # Export to JSON
        nickel export "$ncl_file" > "$json_file"

        # Validate JSON
        if jq empty "$json_file" 2>/dev/null; then
            echo "       ✓ Valid JSON"
        else
            echo "       ✗ Invalid JSON (but exported)"
        fi

        echo ""
        count=$((count + 1))
    fi
done

echo "=== Export Complete ==="
echo "Exported $count files to examples/"
echo ""
echo "To use with bip-keychain:"
echo "  export BIP_KEYCHAIN_SEED=\"your twelve word seed phrase...\""
echo "  bip-keychain derive examples/github-repo.json"
echo ""
