# Example: Infrastructure Keys (DNS and X.509)
#
# Shows how to derive keys for infrastructure components
# using DNS and X.509 Distinguished Names.

let keychain = import "../src/keychain.ncl" in

{
  metadata = {
    name = "Infrastructure Keychain",
    description = "Keys for servers and certificates",
    created = "2025-10-21",
  },

  derivations = [
    # SSH key for web server
    {
      schema_type = 'dns,
      entity = {
        fqdn = "api.daogora.xyz",
        labels = ["xyz", "daogora", "api"],
      },
      derivation_config = {
        hardened = true,
      },
      purpose = "SSH host key for API server",
    },

    # Certificate signing key
    {
      schema_type = 'x509_dn,
      entity = {
        CN = "*.daogora.xyz",
        O = "DAOgora",
        OU = "Infrastructure",
        C = "US",
      },
      derivation_config = {
        hardened = false,  # Non-hardened for PKI use
      },
      purpose = "TLS certificate signing",
    },

    # Database server key
    {
      schema_type = 'dns,
      entity = {
        fqdn = "db.internal.daogora.xyz",
        labels = ["xyz", "daogora", "internal", "db"],
      },
      derivation_config = {
        hardened = true,
      },
      purpose = "Database server authentication",
    },
  ],
} | keychain.Keychain
