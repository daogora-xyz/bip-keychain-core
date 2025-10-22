# SSH Keys with BIP-Keychain

Complete guide for generating and using SSH keys with BIP-Keychain semantic hierarchical key derivation.

## Quick Start

### Generate an SSH Key

```bash
# 1. Set your seed phrase
export BIP_KEYCHAIN_SEED="your twelve word seed phrase here"

# 2. Create an entity JSON for your server
cat > server.json << 'EOF'
{
  "schema_type": "dns",
  "entity": {
    "fqdn": "server.example.com"
  },
  "derivation_config": {
    "hash_function": "hmac_sha512",
    "hardened": true
  },
  "purpose": "SSH access to server.example.com"
}
EOF

# 3. Generate SSH public key
bip-keychain derive server.json

# Output:
# ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAbc123... SSH access to server.example.com
```

That's it! You now have an SSH public key that you can add to `~/.ssh/authorized_keys` on the server.

## Understanding the Output Formats

### SSH Public Key (default, most useful)

```bash
bip-keychain derive server.json --format ssh
```

Output:
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIAbc123def456... SSH access to server.example.com
```

**Use case:** Add this directly to `~/.ssh/authorized_keys` on servers

### Public Key Hex

```bash
bip-keychain derive server.json --format public-key
```

Output:
```
abc123def456...  (64 hex characters = 32 bytes)
```

**Use case:** When you need the raw public key bytes for custom applications

### Raw Seed Hex

```bash
bip-keychain derive server.json --format seed
```

Output:
```
789def012abc...  (64 hex characters = 32 bytes)
```

**Use case:** When you need the underlying seed for manual key derivation

### Complete JSON

```bash
bip-keychain derive server.json --format json
```

Output:
```json
{
  "seed_hex": "789def012abc...",
  "ed25519_public_key": "abc123def456...",
  "ed25519_private_key": "fedcba987...",
  "ssh_public_key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAA...",
  "schema_type": "dns",
  "hash_function": "HmacSha512",
  "purpose": "SSH access to server.example.com"
}
```

**Use case:** When you need all key material and metadata for automation

## Real-World Use Cases

### 1. SSH Server Access

Generate a unique key for each server based on its DNS name:

```bash
# Server 1
cat > prod-server.json << 'EOF'
{
  "schema_type": "dns",
  "entity": {"fqdn": "prod.example.com"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Production server access"
}
EOF

bip-keychain derive prod-server.json > prod.pub

# Server 2
cat > staging-server.json << 'EOF'
{
  "schema_type": "dns",
  "entity": {"fqdn": "staging.example.com"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Staging server access"
}
EOF

bip-keychain derive staging-server.json > staging.pub

# Add to servers:
# cat prod.pub | ssh user@prod.example.com 'cat >> ~/.ssh/authorized_keys'
```

### 2. GitHub Repository Access

Generate SSH keys for specific GitHub repositories:

```bash
cat > github-repo.json << 'EOF'
{
  "schema_type": "schema_org",
  "entity": {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/myuser/myrepo"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "GitHub deploy key for myrepo"
}
EOF

bip-keychain derive github-repo.json

# Add as GitHub deploy key:
# 1. Copy the output
# 2. Go to: https://github.com/myuser/myrepo/settings/keys
# 3. Add as deploy key
```

### 3. Personal Identity Keys

DID-based keys for personal identity:

```bash
cat > my-identity.json << 'EOF'
{
  "schema_type": "did",
  "entity": {
    "did": "did:github:myusername",
    "method": "github",
    "identifier": "myusername"
  },
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Personal GitHub identity"
}
EOF

bip-keychain derive my-identity.json
```

### 4. Infrastructure as Code

Use with Terraform, Ansible, etc.:

```bash
# Generate keys for entire infrastructure
for server in $(cat servers.txt); do
  cat > "${server}.json" << EOF
{
  "schema_type": "dns",
  "entity": {"fqdn": "${server}"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "SSH access to ${server}"
}
EOF

  bip-keychain derive "${server}.json" > "${server}.pub"
  echo "Generated key for $server"
done
```

## Important Security Notes

### ⚠️ Private Keys

BIP-Keychain currently outputs the Ed25519 seed/private key material. In production:

1. **Never expose private keys** - Only share public keys
2. **Store seeds securely** - Use hardware wallets or encrypted storage for your BIP-39 seed
3. **Use SSH agent** - Load private keys into ssh-agent, don't write them to disk

### Current Limitations

**SSH Private Key Format:** The current implementation outputs Ed25519 keypairs but doesn't format private keys in OpenSSH private key format. To use for SSH authentication:

**Option 1: Manual conversion (advanced)**
```bash
# Get the private key hex
PRIV_KEY=$(bip-keychain derive server.json --format private-key)

# Convert to OpenSSH format using external tools
# (This requires additional tooling not included in bip-keychain)
```

**Option 2: Use for authorized_keys only (recommended)**
```bash
# Generate public keys with bip-keychain
# Use traditional SSH keys for authentication
# Or use hardware wallets/SSH agents for private key management
```

**Option 3: Future enhancement**
A future version may add full OpenSSH private key format export with proper encryption.

## Deterministic Key Generation

The power of BIP-Keychain is **deterministic derivation**:

```bash
# Same entity + same seed = same key (always!)
export BIP_KEYCHAIN_SEED="abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"

# Generate key today
bip-keychain derive server.json > key1.pub

# Generate same key tomorrow (or on different machine)
bip-keychain derive server.json > key2.pub

# They're identical!
diff key1.pub key2.pub
# No differences
```

**Benefits:**
- **No key backup needed** - Just backup your seed phrase
- **Reproducible** - Regenerate keys on any machine
- **Semantic organization** - Keys organized by their meaning, not random IDs

## Verifying SSH Keys

### Check SSH Key Format

```bash
bip-keychain derive server.json | ssh-keygen -lf -
```

Should output:
```
256 SHA256:abc123def456... SSH access to server.example.com (ED25519)
```

### Test SSH Connection

```bash
# Add key to test server
bip-keychain derive server.json | ssh user@server.example.com 'cat >> ~/.ssh/authorized_keys'

# Test connection (you'll still need the private key to actually connect)
ssh user@server.example.com
```

## Automation Examples

### Bash Script

```bash
#!/bin/bash
# Generate SSH keys for all production servers

export BIP_KEYCHAIN_SEED="$(cat ~/.bip-keychain-seed)"

for server in prod-{1..5}.example.com; do
  cat > "/tmp/${server}.json" << EOF
{
  "schema_type": "dns",
  "entity": {"fqdn": "${server}"},
  "derivation_config": {"hash_function": "hmac_sha512", "hardened": true},
  "purpose": "Production server ${server}"
}
EOF

  bip-keychain derive "/tmp/${server}.json" | \
    ssh root@${server} 'cat >> ~/.ssh/authorized_keys'

  echo "✓ Deployed key to ${server}"
done
```

### Python Integration

```python
import subprocess
import json

def generate_ssh_key(entity_json):
    """Generate SSH key from entity JSON"""
    result = subprocess.run(
        ['bip-keychain', 'derive', '-'],
        input=json.dumps(entity_json),
        capture_output=True,
        text=True
    )
    return result.stdout.strip()

# Example usage
entity = {
    "schema_type": "dns",
    "entity": {"fqdn": "api.example.com"},
    "derivation_config": {"hash_function": "hmac_sha512", "hardened": True},
    "purpose": "API server access"
}

ssh_key = generate_ssh_key(entity)
print(f"Generated: {ssh_key}")
```

## Best Practices

1. **One key per server/service** - Don't reuse keys
2. **Semantic entity naming** - Use meaningful entity descriptions
3. **Document your entities** - Keep a registry of what each entity represents
4. **Backup your seed phrase** - Use hardware wallets or secure storage
5. **Test key generation** - Verify deterministic reproduction
6. **Use purpose field** - Makes SSH keys self-documenting
7. **Rotate keys regularly** - Generate new keys by changing entity attributes

## Troubleshooting

### "Permission denied (publickey)"

The generated public key was added to `authorized_keys`, but you need the corresponding private key to authenticate. BIP-Keychain currently focuses on public key generation for authorization, not full SSH authentication setup.

### "Invalid format"

Ensure the SSH key output is complete:
```bash
# Should be one line starting with "ssh-ed25519"
bip-keychain derive server.json | wc -l
# Output: 1
```

### "Key type not supported"

Ed25519 keys require OpenSSH 6.5+ (released 2014). Update OpenSSH if you see this error.

## Next Steps

- See `CLI-USAGE.md` for complete CLI reference
- See `NICKEL-WORKFLOW.md` for writing entities in Nickel
- See `examples/` for pre-made entity JSON files
- Try generating keys for your infrastructure!

## Future Enhancements

Coming soon:
- Full OpenSSH private key format export
- SSH agent integration
- GPG key generation
- Git commit signing
- Certificate generation (X.509)

## Resources

- **Ed25519 Keys**: https://ed25519.cr.yp.to/
- **OpenSSH**: https://www.openssh.com/
- **BIP-32**: Hierarchical Deterministic Wallets
- **BIP-39**: Mnemonic seed phrases
