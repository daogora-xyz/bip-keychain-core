# BIP-Keychain Core Implementation in Nickel
#
# Multi-schema semantic hierarchical key derivation based on BIP-85.
# Extends the original BIP-keychain spec to support multiple schema types
# beyond schema.org (Blockchain Commons, DIDs, X.509, etc.)

{
  # BIP-Keychain Constants
  bip85_prefix = "m/83696968'",
  bip_keychain_app_code = "67797668'",

  # Base derivation path for all BIP-keychain operations
  base_path
    | doc "Base BIP-32 derivation path for BIP-keychain"
    | String
    = "%{bip85_prefix}/%{bip_keychain_app_code}",

  # Supported schema types
  SchemaType
    | doc "Supported schema types for semantic entity derivation"
    = [|
        'schema_org,            # JSON-LD schema.org entities
        'gordian_envelope,      # Blockchain Commons Gordian Envelope
        'did,                   # W3C Decentralized Identifiers
        'verifiable_credential, # W3C Verifiable Credentials
        'x509_dn,              # X.509 Distinguished Names
        'dns,                  # DNS/FQDN
        'ipfs_cid,             # IPFS Content Identifiers
        'urn,                  # Uniform Resource Names
        'custom,               # User-defined schema
      |],

  # Hash function types
  HashFunction = [| 'hmac_sha512, 'blake2b, 'sha256 |],

  # Derivation configuration
  DerivationConfig = {
    hash_function
      | doc "Hash function for entity to index conversion"
      | HashFunction
      | default
      = 'hmac_sha512,

    hardened
      | doc "Use hardened derivation (recommended for most use cases)"
      | Boolean
      | default
      = true,

    max_depth
      | doc "Maximum derivation path depth (safety limit)"
      | Number
      | default
      = 1000,
  },

  # Schema.org entity schema
  SchemaOrgEntity = {
    "@context"
      | doc "JSON-LD context (typically https://schema.org)"
      | String,

    "@type"
      | doc "Schema.org type (e.g., SoftwareSourceCode, Organization)"
      | String,

    # Additional fields are flexible (schema.org is extensible)
    # Use Record(Dyn) for arbitrary additional properties
  },

  # Blockchain Commons Gordian Envelope
  GordianEnvelopeEntity = {
    envelope
      | doc "UR-encoded Gordian Envelope"
      | String,

    format
      | doc "Envelope format identifier"
      | String
      | default
      = "ur:envelope",

    elision_proof
      | doc "Optional elision proof for privacy"
      | String
      | optional,
  },

  # W3C Decentralized Identifier
  DIDEntity = {
    did
      | doc "Full DID string (e.g., did:github:username)"
      | String,

    method
      | doc "DID method (e.g., github, web, key)"
      | String,

    identifier
      | doc "Method-specific identifier"
      | String,
  },

  # W3C Verifiable Credential
  VerifiableCredentialEntity = {
    "@context"
      | doc "JSON-LD context for VC"
      | Array String,

    type
      | doc "Credential type(s)"
      | Array String,

    credentialSubject
      | doc "Subject of the credential"
      | { id | String },

    issuer
      | doc "Credential issuer DID"
      | String,
  },

  # X.509 Distinguished Name
  X509DNEntity = {
    CN
      | doc "Common Name"
      | String
      | optional,

    O
      | doc "Organization"
      | String
      | optional,

    OU
      | doc "Organizational Unit"
      | String
      | optional,

    L
      | doc "Locality"
      | String
      | optional,

    ST
      | doc "State/Province"
      | String
      | optional,

    C
      | doc "Country"
      | String
      | optional,
  },

  # DNS/FQDN
  DNSEntity = {
    fqdn
      | doc "Fully Qualified Domain Name"
      | String,

    labels
      | doc "Domain labels in hierarchical order (e.g., ['com', 'example', 'www'])"
      | Array String
      | optional,
  },

  # IPFS Content Identifier
  IPFSCIDEntity = {
    cid
      | doc "IPFS Content Identifier (CIDv0 or CIDv1)"
      | String,

    version
      | doc "CID version (0 or 1)"
      | Number
      | optional,

    codec
      | doc "Content codec (e.g., 'dag-pb', 'raw')"
      | String
      | optional,
  },

  # URN
  URNEntity = {
    urn
      | doc "Full URN string (e.g., urn:isbn:0451450523)"
      | String,

    namespace
      | doc "URN namespace identifier"
      | String,

    nss
      | doc "Namespace-specific string"
      | String,
  },

  # Custom schema (user-defined)
  CustomEntity = {
    schema_id
      | doc "Custom schema identifier"
      | String,

    data
      | doc "Schema-specific data (arbitrary structure)"
      | Dyn,
  },

  # Polymorphic entity based on schema type
  SemanticEntity = fun schema_type =>
    if schema_type == 'schema_org then
      SchemaOrgEntity
    else if schema_type == 'gordian_envelope then
      GordianEnvelopeEntity
    else if schema_type == 'did then
      DIDEntity
    else if schema_type == 'verifiable_credential then
      VerifiableCredentialEntity
    else if schema_type == 'x509_dn then
      X509DNEntity
    else if schema_type == 'dns then
      DNSEntity
    else if schema_type == 'ipfs_cid then
      IPFSCIDEntity
    else if schema_type == 'urn then
      URNEntity
    else if schema_type == 'custom then
      CustomEntity
    else
      # Fallback to dynamic type
      Dyn,

  # Key derivation specification
  KeyDerivation = {
    schema_type
      | doc "Type of schema being used for derivation"
      | SchemaType,

    entity
      | doc "Semantic entity data (type depends on schema_type)"
      | SemanticEntity schema_type,

    derivation_config
      | doc "Derivation configuration options"
      | DerivationConfig
      | default
      = {},

    purpose
      | doc "Human-readable purpose for this key"
      | String
      | optional,

    metadata
      | doc "Additional metadata (not used in derivation)"
      | Dyn
      | optional,
  },

  # Derived key output
  DerivedKey = {
    derivation_path
      | doc "Full BIP-32 derivation path"
      | String,

    schema_type
      | doc "Schema type used for derivation"
      | SchemaType,

    entity_hash
      | doc "Hash of the semantic entity (for verification)"
      | String
      | optional,

    public_key
      | doc "Derived public key (hex or base58)"
      | String
      | optional,

    key_type
      | doc "Type of key derived (ssh, gpg, signing, encryption)"
      | [| 'ssh, 'gpg, 'signing, 'encryption, 'generic |]
      | default
      = 'generic,

    purpose
      | doc "Purpose of this key"
      | String
      | optional,
  },

  # Complete keychain configuration
  Keychain = {
    master_fingerprint
      | doc "BIP-32 master key fingerprint (for verification)"
      | String
      | optional,

    derivations
      | doc "List of key derivations in this keychain"
      | Array KeyDerivation,

    metadata
      | doc "Keychain-level metadata"
      | {
          name | String | optional,
          description | String | optional,
          created | String | optional,
          version | String | default = "1.0.0",
        }
      | default
      = {},
  },
}
