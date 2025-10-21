# Example: GitHub Repository Signing Key using Schema.org
#
# Derives a Git signing key for a specific repository using schema.org vocabulary.

let keychain = import "../src/keychain.ncl" in

{
  # Use schema.org to describe the repository
  schema_type = 'schema_org,

  entity = {
    "@context" = "https://schema.org",
    "@type" = "SoftwareSourceCode",
    codeRepository = "https://github.com/DAOgora-xyz/bip-keychain-core",
    programmingLanguage = "Nickel",
    name = "BIP-Keychain Core",
  },

  derivation_config = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },

  purpose = "Git commit signing key for bip-keychain-core repository",

  metadata = {
    created = "2025-10-21",
    owner = "DAOgora-xyz",
  },
} | keychain.KeyDerivation
