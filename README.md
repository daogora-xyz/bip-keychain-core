# BIP-Keychain Core

Exploration and implementation of semantic hierarchical key derivation based on [BIP-Keychain](https://github.com/akarve/bip-keychain).

## What is BIP-Keychain?

BIP-Keychain is a draft Bitcoin Improvement Proposal that extends BIP-85 with semantic derivation paths. It creates a hierarchical key-value store where:

- **Keys**: Meaningful semantic paths (JSON-LD schema.org entities)
- **Values**: Cryptographic secrets derived from those paths
- **Derivation**: HMAC-SHA-512 converts semantic entities to BIP-32 child indices

## Key Innovation

**Separation of path keys from path values**: If the hot master is compromised, only the derivation paths (metadata) are exposed, not the actual secrets.

## Use Cases

- Password management with semantic organization
- SSH key hierarchies based on infrastructure topology
- Git signing keys organized by repository/organization
- PKI with human-readable certificate paths
- API key management with semantic namespacing

## Project Goals

1. **Implement** BIP-Keychain derivation algorithm
2. **Extend** beyond Bitcoin to general-purpose secret management
3. **Integrate** with existing tools (Git, SSH, GPG)
4. **Research** applications in progressive trust systems
5. **Document** security properties and best practices

## Status

**Early Research Phase** - This repository is for exploration and experimentation.

The original BIP-Keychain proposal is in draft status. This project aims to:
- Provide reference implementations
- Test real-world use cases
- Contribute feedback to the BIP process

## Repository Structure

```
.
├── docs/          # Documentation and research
├── examples/      # Example derivation paths and use cases
├── src/           # Implementation code
└── tests/         # Test vectors and validation
```

## Quick Example

```python
# Semantic path: GitHub repository signing key
repo_path = {
    "@context": "https://schema.org",
    "@type": "SoftwareSourceCode",
    "codeRepository": "https://github.com/user/repo"
}

# Derivation: m/83696968'/67797668'/{hash(repo_path)}'
# Result: Deterministic signing key for that specific repository
```

## Relationship to Other Projects

- **BIP-Keychain**: Original proposal (this is implementation/research)
- **BIP-85**: Parent standard for deterministic entropy
- **BIP-32**: Underlying hierarchical deterministic wallet tech
- **OpenIntegrity-Nickel-Core**: Exploring BIP-Keychain for cryptographic work stream management

## Contributing

This is an exploratory research project. Contributions welcome:

- Implementations in various languages
- Use case documentation
- Security analysis
- Integration examples
- Test vectors

## License

BSD-2-Clause (matching original BIP-Keychain proposal)

## References

- **Original BIP-Keychain**: https://github.com/akarve/bip-keychain
- **BIP-85**: https://bips.dev/85/
- **BIP-32**: https://github.com/bitcoin/bips/blob/master/bip-0032.mediawiki
- **JSON-LD**: https://json-ld.org/
- **Schema.org**: https://schema.org/
