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

// BLAKE2b tests

#[test]
fn test_blake2b_empty_string() {
    // BLAKE2b test vector from official BLAKE2 repository
    // Input: "" (empty string)
    // Output (64 bytes): BLAKE2b-512 hash of empty string
    // Source: https://github.com/BLAKE2/BLAKE2/blob/master/testvectors/blake2b-kat.txt

    let data = "";
    let dummy_entropy = &[0u8; 32]; // BLAKE2b doesn't use parent entropy

    let expected = hex::decode(
        "786a02f742015903c6c6fd852552d272912f4740e15847618a86e217f71f5419\
         d25e1031afee585313896444934eb04b903a685b1448b755d56f701afe9be2ce"
    ).unwrap();

    let result = hash_entity(data, dummy_entropy, HashFunction::Blake2b)
        .expect("BLAKE2b should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "BLAKE2b output should match official test vector for empty string"
    );
}

#[test]
fn test_blake2b_abc() {
    // BLAKE2b test vector from official BLAKE2 repository
    // Input: "abc"
    // Output (64 bytes): BLAKE2b-512 hash of "abc"

    let data = "abc";
    let dummy_entropy = &[0u8; 32]; // BLAKE2b doesn't use parent entropy

    let expected = hex::decode(
        "ba80a53f981c4d0d6a2797b69f12f6e94c212f14685ac4b74b12bb6fdbffa2d1\
         7d87c5392aab792dc252d5de4533cc9518d38aa8dbf1925ab92386edd4009923"
    ).unwrap();

    let result = hash_entity(data, dummy_entropy, HashFunction::Blake2b)
        .expect("BLAKE2b should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "BLAKE2b output should match official test vector for 'abc'"
    );
}

#[test]
fn test_blake2b_with_json_entity() {
    // BIP-Keychain specific test: JSON entity with BLAKE2b
    // Used by Blockchain Commons

    let entity_json = r#"{"@context":"https://schema.org","@type":"Organization","name":"Blockchain Commons"}"#;
    let dummy_entropy = &[0u8; 32]; // BLAKE2b doesn't use parent entropy

    // Test determinism
    let result1 = hash_entity(entity_json, dummy_entropy, HashFunction::Blake2b)
        .expect("Should hash entity JSON with BLAKE2b");

    let result2 = hash_entity(entity_json, dummy_entropy, HashFunction::Blake2b)
        .expect("Should hash entity JSON with BLAKE2b");

    assert_eq!(
        result1, result2,
        "Same input should produce same output (determinism)"
    );

    assert_eq!(result1.len(), 64, "BLAKE2b should produce 64 bytes");
}

// SHA-256 tests

#[test]
fn test_sha256_empty_string() {
    // SHA-256 test vector from NIST
    // Input: "" (empty string)
    // Output (32 bytes): SHA-256 hash of empty string
    // Source: NIST FIPS 180-4

    let data = "";
    let dummy_entropy = &[0u8; 32]; // SHA-256 doesn't use parent entropy in our implementation

    // SHA-256 of empty string:
    // e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
    let expected_32 = hex::decode(
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    ).unwrap();

    // Our implementation pads to 64 bytes
    let mut expected = vec![0u8; 64];
    expected[..32].copy_from_slice(&expected_32);

    let result = hash_entity(data, dummy_entropy, HashFunction::Sha256)
        .expect("SHA-256 should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "SHA-256 output should match NIST test vector for empty string"
    );
}

#[test]
fn test_sha256_abc() {
    // SHA-256 test vector from NIST
    // Input: "abc"
    // Output (32 bytes): SHA-256 hash of "abc"

    let data = "abc";
    let dummy_entropy = &[0u8; 32];

    // SHA-256 of "abc":
    // ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
    let expected_32 = hex::decode(
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    ).unwrap();

    let mut expected = vec![0u8; 64];
    expected[..32].copy_from_slice(&expected_32);

    let result = hash_entity(data, dummy_entropy, HashFunction::Sha256)
        .expect("SHA-256 should succeed");

    assert_eq!(
        result.as_slice(),
        expected.as_slice(),
        "SHA-256 output should match NIST test vector for 'abc'"
    );
}

#[test]
fn test_sha256_with_json_entity() {
    // BIP-Keychain specific test: JSON entity with SHA-256

    let entity_json = r#"{"@context":"https://schema.org","@type":"Thing","name":"Test"}"#;
    let dummy_entropy = &[0u8; 32];

    // Test determinism
    let result1 = hash_entity(entity_json, dummy_entropy, HashFunction::Sha256)
        .expect("Should hash entity JSON with SHA-256");

    let result2 = hash_entity(entity_json, dummy_entropy, HashFunction::Sha256)
        .expect("Should hash entity JSON with SHA-256");

    assert_eq!(
        result1, result2,
        "Same input should produce same output (determinism)"
    );

    assert_eq!(result1.len(), 64, "SHA-256 (padded) should produce 64 bytes");

    // Verify first 32 bytes are non-zero (the actual hash)
    // and last 32 bytes are zero (the padding)
    let has_nonzero_in_first_half = result1[..32].iter().any(|&b| b != 0);
    let all_zero_in_second_half = result1[32..].iter().all(|&b| b == 0);

    assert!(has_nonzero_in_first_half, "First 32 bytes should contain the hash");
    assert!(all_zero_in_second_half, "Last 32 bytes should be zero padding");
}
