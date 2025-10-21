# Example: Identity Key using W3C DID
#
# Derives a signing key for a decentralized identifier.

let keychain = import "../src/keychain.ncl" in

{
  schema_type = 'did,

  entity = {
    did = "did:github:lessuselesss",
    method = "github",
    identifier = "lessuselesss",
  },

  derivation_config = {
    hash_function = 'hmac_sha512,
    hardened = true,
  },

  purpose = "Personal identity signing key",

  metadata = {
    note = "Used for cryptographic proof of GitHub identity",
  },
} | keychain.KeyDerivation
