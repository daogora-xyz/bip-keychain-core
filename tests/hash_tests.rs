//! Tests for hash module
//!
//! Uses known test vectors from NIST and other standards bodies.

use bip_keychain::{HashFunction, hash::hash_entity};

#[test]
fn test_hmac_sha512_rfc4231_test_case_1() {
    // RFC 4231 Test Case 1 (HMAC-SHA-512)
    // Test vector from: https://tools.ietf.org/html/rfc4231
    //
    // Key = 0x0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b (20 bytes)
    // Data = "Hi There" (ASCII)
    // Expected HMAC-SHA-512:
    //   87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde
    //   daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854

    let key: [u8; 20] = [0x0b; 20];
    let data = "Hi There";

    let expected = hex::decode(
        "87aa7cdea5ef619d4ff0b4241a1d6cb02379f4e2ce4ec2787ad0b30545e17cde\
         daa833b7d6b8a702038b274eaea3f4e4be9d914eeb61f1702e696c203a126854"
    ).unwrap();

    let result = hash_entity(data, &key, HashFunction::HmacSha512)
        .expect("HMAC-SHA-512 should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "HMAC-SHA-512 output should match RFC 4231 test vector"
    );
}

#[test]
fn test_hmac_sha512_rfc4231_test_case_2() {
    // RFC 4231 Test Case 2 (HMAC-SHA-512)
    // Key = "Jefe" (ASCII)
    // Data = "what do ya want for nothing?" (ASCII)

    let key = b"Jefe";
    let data = "what do ya want for nothing?";

    let expected = hex::decode(
        "164b7a7bfcf819e2e395fbe73b56e0a387bd64222e831fd610270cd7ea250554\
         9758bf75c05a994a6d034f65f8f0e6fdcaeab1a34d4a6b4b636e070a38bce737"
    ).unwrap();

    let result = hash_entity(data, key, HashFunction::HmacSha512)
        .expect("HMAC-SHA-512 should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "HMAC-SHA-512 output should match RFC 4231 test vector"
    );
}

#[test]
fn test_hmac_sha512_with_json_entity() {
    // BIP-Keychain specific test: JSON entity
    // This represents a typical use case with canonical JSON

    let entity_json = r#"{"@context":"https://schema.org","@type":"SoftwareSourceCode","codeRepository":"https://github.com/akarve/bip-keychain","name":"bip-keychain"}"#;
    let parent_entropy = b"test_seed_entropy_32_bytes_long!";

    // This test will pass once HMAC-SHA-512 is implemented
    // The exact expected value doesn't matter for the RED phase,
    // but the function must produce deterministic output
    let result1 = hash_entity(entity_json, parent_entropy, HashFunction::HmacSha512)
        .expect("Should hash entity JSON");

    let result2 = hash_entity(entity_json, parent_entropy, HashFunction::HmacSha512)
        .expect("Should hash entity JSON");

    assert_eq!(
        result1, result2,
        "Same input should produce same output (determinism)"
    );

    assert_eq!(result1.len(), 64, "HMAC-SHA-512 should produce 64 bytes");
}
