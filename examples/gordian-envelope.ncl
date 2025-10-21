# Example: Blockchain Commons Gordian Envelope
#
# Derives a key using a Gordian Envelope with selective disclosure support.

let keychain = import "../src/keychain.ncl" in

{
  schema_type = 'gordian_envelope,

  entity = {
    envelope = "ur:envelope/lftpsptansisftykisioihoyhsjzhsjtpmcxfwfgfwlkfthsjtiohsjyhsjzfwlkfx",
    format = "ur:envelope",
    elision_proof = "proof-of-selective-disclosure-abc123",
  },

  derivation_config = {
    hash_function = 'blake2b,  # Blockchain Commons often uses BLAKE
    hardened = true,
  },

  purpose = "Privacy-preserving credential signing",

  metadata = {
    use_case = "Selective disclosure with hashed elision",
    blockchain_commons_version = "2025",
  },
} | keychain.KeyDerivation
