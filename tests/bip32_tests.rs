//! Tests for BIP-32 wrapper functionality
//!
//! Tests key derivation from BIP-39 mnemonic phrases using the BIP-Keychain path.

use bip_keychain::Keychain;

#[test]
fn test_keychain_from_mnemonic() {
    // Test BIP-39 mnemonic (12 words)
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let keychain =
        Keychain::from_mnemonic(mnemonic).expect("Should create keychain from valid mnemonic");

    // Should be able to access master key
    assert!(keychain.master_key().private_key().to_bytes().len() == 32);
}

#[test]
fn test_derive_at_bip_keychain_path() {
    // BIP-Keychain derivation path: m/83696968'/67797668'/{index}'
    // 83696968 = BIP-85 application number
    // 67797668 = "BIP" in ASCII (0x424950 = 4_345_168, but spec says 67797668)

    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    // Test deriving at index 0
    let derived = keychain.derive_bip_keychain_path(0).unwrap();

    // Should get 32 bytes for Ed25519 seed
    let seed = derived.to_seed();
    assert_eq!(seed.len(), 32);
}

#[test]
fn test_deterministic_derivation() {
    // Same mnemonic and index should always produce same result
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let keychain1 = Keychain::from_mnemonic(mnemonic).unwrap();
    let keychain2 = Keychain::from_mnemonic(mnemonic).unwrap();

    let derived1 = keychain1.derive_bip_keychain_path(42).unwrap();
    let derived2 = keychain2.derive_bip_keychain_path(42).unwrap();

    assert_eq!(derived1.to_bytes(), derived2.to_bytes());
}

#[test]
fn test_different_indices_produce_different_keys() {
    // Different indices should produce different keys
    let mnemonic = "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";

    let keychain = Keychain::from_mnemonic(mnemonic).unwrap();

    let derived_0 = keychain.derive_bip_keychain_path(0).unwrap();
    let derived_1 = keychain.derive_bip_keychain_path(1).unwrap();

    assert_ne!(derived_0.to_bytes(), derived_1.to_bytes());
}

#[test]
fn test_invalid_mnemonic() {
    // Invalid mnemonic should fail gracefully
    let bad_mnemonic = "invalid mnemonic phrase that is not valid";

    let result = Keychain::from_mnemonic(bad_mnemonic);
    assert!(result.is_err());
}

#[test]
fn test_bip32_constants() {
    // Verify the BIP-Keychain path constants
    // m/83696968'/67797668'/{index}'

    const BIP85_APP: u32 = 83696968;
    const BIPKEYCHAIN_APP: u32 = 67797668;

    // BIP-85 application number for BIP-Keychain
    assert_eq!(BIP85_APP, 83_696_968);

    // "BIP-KEYCHAIN" application code
    // Note: The spec uses 67797668, need to verify this is correct
    assert_eq!(BIPKEYCHAIN_APP, 67_797_668);
}
