# Nickel Integration Workflow

This guide explains how to use Nickel configuration language with BIP-Keychain.

## Overview

The complete workflow:
1. Write entity definitions in Nickel (.ncl files)
2. Export Nickel to JSON using `nickel export`
3. Derive keys from JSON using `bip-keychain derive`

**Benefits of using Nickel:**
- Type-safe configuration with validation
- Reusable schemas and contracts
- Better error messages at export time
- IDE support and documentation

## Installing Nickel

### Option 1: Using Nix (Recommended)

```bash
# Add Nickel to your environment
nix-env -iA nixpkgs.nickel

# Or use nix-shell for temporary usage
nix-shell -p nickel
```

### Option 2: Using Cargo

```bash
cargo install nickel-lang-cli
```

### Option 3: Download Binary

Visit: https://github.com/tweag/nickel/releases

Download the appropriate binary for your platform.

### Verify Installation

```bash
nickel --version
# Should output: nickel x.y.z
```

## Nickel Schema Structure

The BIP-Keychain Nickel schemas are in `nickel/src/keychain.ncl`.

**Key components:**
- `SchemaType` - Supported schema types (schema_org, did, gordian_envelope, etc.)
- `HashFunction` - Hash function selection (hmac_sha512, blake2b, sha256)
- `DerivationConfig` - Configuration for key derivation
- `KeyDerivation` - Main contract for entity definitions

## Writing Entities in Nickel

### Example 1: Schema.org Entity

File: `nickel/examples/github-repo-schema-org.ncl`

```nickel
let keychain = import "../src/keychain.ncl" in

{
  schema_type = 'schema_org,

  entity = {
    "@context" = "https://schema.org",
    "@type" = "SoftwareSourceCode",
    codeRepository = "https://github.com/user/repo",
    name = "My Project",
  },

  derivation_config = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },

  purpose = "Git commit signing key",
} | keychain.KeyDerivation
```

### Example 2: DID Entity

File: `nickel/examples/did-identity.ncl`

```nickel
let keychain = import "../src/keychain.ncl" in

{
  schema_type = 'did,

  entity = {
    did = "did:github:username",
    method = "github",
    identifier = "username",
  },

  derivation_config = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },

  purpose = "Personal identity signing key",
} | keychain.KeyDerivation
```

### Example 3: Gordian Envelope (with BLAKE2b)

File: `nickel/examples/gordian-envelope.ncl`

```nickel
let keychain = import "../src/keychain.ncl" in

{
  schema_type = 'gordian_envelope,

  entity = {
    envelope = "ur:envelope/...",
    format = "ur:envelope",
  },

  derivation_config = {
    hash_function = 'blake2b,  # Blockchain Commons compatibility
    hardened = true,
  },

  purpose = "Privacy-preserving credential signing",
} | keychain.KeyDerivation
```

## Exporting to JSON

### Basic Export

```bash
# Export a single entity
nickel export nickel/examples/github-repo-schema-org.ncl > github-repo.json

# Verify the JSON
cat github-repo.json | jq .
```

### Export All Examples

```bash
# Create output directory
mkdir -p exported-entities

# Export all examples
for file in nickel/examples/*.ncl; do
  basename=$(basename "$file" .ncl)
  nickel export "$file" > "exported-entities/${basename}.json"
  echo "Exported: ${basename}.json"
done
```

### Export Script

We provide a convenient script: `export-nickel-examples.sh`

```bash
./export-nickel-examples.sh
```

This exports all Nickel examples to the `examples/` directory.

## Deriving Keys from Exported JSON

Once you have exported JSON files, use them with `bip-keychain`:

```bash
# Set your seed phrase
export BIP_KEYCHAIN_SEED="your twelve word seed phrase here"

# Derive a key from exported JSON
bip-keychain derive examples/github-repo.json

# Output: 64-character hex string (Ed25519 seed)
```

### JSON Output

```bash
bip-keychain derive examples/github-repo.json --format json
```

Output:
```json
{
  "ed25519_seed": "a1b2c3d4e5f6...",
  "schema_type": "schema_org",
  "hash_function": "HmacSha512",
  "purpose": "Git commit signing key for bip-keychain-core repository"
}
```

## Complete End-to-End Workflow

### Step 1: Write Nickel Configuration

```nickel
# my-key.ncl
let keychain = import "nickel/src/keychain.ncl" in

{
  schema_type = 'schema_org,
  entity = {
    "@context" = "https://schema.org",
    "@type" = "SoftwareSourceCode",
    codeRepository = "https://github.com/myuser/myrepo",
    name = "My Repository",
  },
  derivation_config = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },
  purpose = "Repository signing key",
} | keychain.KeyDerivation
```

### Step 2: Validate and Export

```bash
# Type-check the Nickel file
nickel typecheck my-key.ncl

# Export to JSON
nickel export my-key.ncl > my-key.json
```

### Step 3: Derive Key

```bash
# Set seed phrase
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Derive the key
bip-keychain derive my-key.json
```

Output:
```
a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456
```

### Step 4: Use the Derived Seed

The 32-byte seed can now be used to generate Ed25519 keypairs for:
- Git commit signing
- SSH authentication
- GPG signatures
- Any Ed25519-based cryptography

## Pre-Exported Examples

For convenience, we've pre-exported the Nickel examples to JSON:

- `examples/github-repo.json` - Schema.org repository key
- `examples/did-identity.json` - DID-based identity key
- `examples/gordian-envelope.json` - Blockchain Commons envelope (BLAKE2b)

You can use these immediately without Nickel installed:

```bash
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
bip-keychain derive examples/github-repo.json
```

## Nickel Schema Features

### Type Safety

Nickel validates your entities at export time:

```nickel
{
  schema_type = 'invalid_type,  # ERROR: Not in SchemaType enum
  # ...
}
```

### Default Values

```nickel
{
  derivation_config = {
    # hash_function defaults to 'hmac_sha512
    # hardened defaults to true
  },
}
```

### Documentation

```nickel
{
  purpose
    | doc "Human-readable description of this key's purpose"
    | String
    = "My key description",
}
```

### Reusable Configurations

```nickel
# common-config.ncl
{
  standard_derivation = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },
}

# my-key.ncl
let common = import "common-config.ncl" in
{
  derivation_config = common.standard_derivation,
  # ...
}
```

## Troubleshooting

### "nickel: command not found"

Install Nickel using one of the methods above.

### "import path not found"

Ensure you're running nickel export from the correct directory:
```bash
cd /path/to/bip-keychain-core
nickel export nickel/examples/github-repo-schema-org.ncl
```

### "type error in Nickel file"

Nickel provides helpful error messages. Read the error and fix the type mismatch:
```bash
nickel typecheck your-file.ncl
```

### "Failed to parse entity JSON"

Ensure the exported JSON is valid:
```bash
nickel export my-key.ncl | jq .
```

## Advanced Usage

### Batch Processing

```bash
# Export multiple entities
for entity in entities/*.ncl; do
  nickel export "$entity" | \
  bip-keychain derive /dev/stdin --format json >> all-keys.jsonl
done
```

### Custom Schemas

Create your own schema types:

```nickel
{
  schema_type = 'custom,
  entity = {
    # Your custom fields
    my_field = "value",
  },
  # ...
}
```

### Environment-Specific Configurations

```nickel
let env = std.env.get "ENVIRONMENT" in

{
  entity = {
    codeRepository =
      if env == "prod" then
        "https://github.com/org/prod-repo"
      else
        "https://github.com/org/dev-repo",
  },
}
```

## Next Steps

- Explore examples in `nickel/examples/`
- Read the Nickel schema in `nickel/src/keychain.ncl`
- Create your own entity definitions
- See `CLI-USAGE.md` for `bip-keychain` command reference

## Resources

- **Nickel Documentation**: https://nickel-lang.org/
- **BIP-Keychain Spec**: https://github.com/akarve/bip-keychain
- **Schema.org Vocabulary**: https://schema.org/
- **W3C DIDs**: https://www.w3.org/TR/did-core/
- **Blockchain Commons**: https://github.com/BlockchainCommons
