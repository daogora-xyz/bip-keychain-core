# Example: Complete Personal Keychain
#
# Demonstrates a full keychain with multiple derivations using different schemas.
# This represents a real-world use case where one seed manages all cryptographic
# keys across different contexts.

let keychain = import "../src/keychain.ncl" in

{
  master_fingerprint = "a1b2c3d4",  # Would be actual BIP-32 fingerprint

  metadata = {
    name = "Personal Keychain 2025",
    description = "All my keys derived from one seed",
    created = "2025-10-21",
    version = "1.0.0",
  },

  derivations = [
    # GitHub identity
    {
      schema_type = 'did,
      entity = {
        did = "did:github:lessuselesss",
        method = "github",
        identifier = "lessuselesss",
      },
      purpose = "GitHub identity signing",
    },

    # Primary repository signing
    {
      schema_type = 'schema_org,
      entity = {
        "@context" = "https://schema.org",
        "@type" = "SoftwareSourceCode",
        codeRepository = "https://github.com/DAOgora-xyz/bip-keychain-core",
      },
      purpose = "Git commit signing for bip-keychain-core",
    },

    # OpenIntegrity project
    {
      schema_type = 'schema_org,
      entity = {
        "@context" = "https://schema.org",
        "@type" = "SoftwareSourceCode",
        codeRepository = "https://github.com/DAOgora-xyz/openintegrity-nickel-core",
      },
      purpose = "Git commit signing for openintegrity-nickel-core",
    },

    # Personal website TLS
    {
      schema_type = 'x509_dn,
      entity = {
        CN = "lessuselesss.dev",
        O = "Personal",
        C = "US",
      },
      derivation_config = {
        hardened = false,  # For PKI
      },
      purpose = "Personal website TLS certificate",
    },

    # Privacy-preserving credential
    {
      schema_type = 'gordian_envelope,
      entity = {
        envelope = "ur:envelope/example123",
        format = "ur:envelope",
      },
      derivation_config = {
        hash_function = 'blake2b,
      },
      purpose = "Selective disclosure credentials",
    },

    # API server SSH key
    {
      schema_type = 'dns,
      entity = {
        fqdn = "api.example.com",
      },
      purpose = "API server SSH authentication",
    },

    # IPFS content signing
    {
      schema_type = 'ipfs_cid,
      entity = {
        cid = "bafybeigdyrzt5sfp7udm7hu76uh7y26nf3efuylqabf3oclgtqy55fbzdi",
        version = 1,
        codec = "dag-pb",
      },
      purpose = "Sign specific IPFS content version",
    },
  ],
} | keychain.Keychain
