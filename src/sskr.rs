//! SSKR (Shamir's Secret Sharing for Key Recovery) support
//!
//! Implements Blockchain Commons SSKR for seed backup and recovery.
//! Allows splitting a BIP-39 seed into N shares where M are required to recover.
//!
//! Use cases:
//! - Disaster recovery (distribute shares to trusted parties)
//! - Business continuity (require multiple executives)
//! - Inheritance planning
//! - Reducing single points of failure

#[cfg(feature = "bc")]
use crate::error::{BipKeychainError, Result};

#[cfg(feature = "bc")]
use sskr::{sskr_combine, sskr_generate, GroupSpec, Secret, Spec};

/// Policy for seed sharding
///
/// Defines how many shares are created and how many are required for recovery.
#[cfg(feature = "bc")]
#[derive(Debug, Clone, Copy)]
pub struct SskrPolicy {
    /// Total number of shares to generate
    pub groups: u8,
    /// Number of shares required to recover the secret
    pub threshold: u8,
}

#[cfg(feature = "bc")]
impl SskrPolicy {
    /// Create a new SSKR policy
    ///
    /// # Arguments
    /// * `groups` - Total number of shares to generate (2-16)
    /// * `threshold` - Number of shares required to recover (1-groups)
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// // 2-of-3: Require any 2 of 3 shares
    /// let policy = SskrPolicy::new(3, 2)?;
    ///
    /// // 3-of-5: Require any 3 of 5 shares (executives)
    /// let policy = SskrPolicy::new(5, 3)?;
    /// ```
    pub fn new(groups: u8, threshold: u8) -> Result<Self> {
        if groups < 2 || groups > 16 {
            return Err(BipKeychainError::OutputError(
                "SSKR groups must be between 2 and 16".to_string(),
            ));
        }

        if threshold < 1 || threshold > groups {
            return Err(BipKeychainError::OutputError(format!(
                "SSKR threshold must be between 1 and {} (number of groups)",
                groups
            )));
        }

        Ok(Self { groups, threshold })
    }

    /// Common policy: 2-of-3 (any 2 of 3 shares)
    ///
    /// Good for personal backup with 3 trusted parties.
    pub fn two_of_three() -> Self {
        Self {
            groups: 3,
            threshold: 2,
        }
    }

    /// Common policy: 3-of-5 (any 3 of 5 shares)
    ///
    /// Good for enterprise backup with 5 executives.
    pub fn three_of_five() -> Self {
        Self {
            groups: 5,
            threshold: 3,
        }
    }

    /// Common policy: 2-of-2 (both shares required)
    ///
    /// Good for couples or business partners.
    pub fn two_of_two() -> Self {
        Self {
            groups: 2,
            threshold: 2,
        }
    }
}

/// Shard a BIP-39 seed into SSKR shares
///
/// Splits the seed entropy into N shares where M are required to recover.
///
/// # Arguments
/// * `seed_entropy` - The raw seed entropy (16, 20, 24, 28, or 32 bytes)
/// * `policy` - The SSKR sharding policy (groups and threshold)
///
/// # Returns
/// Vector of SSKR shares (as byte vectors)
///
/// # Example
///
/// ```rust,ignore
/// use bip_keychain::sskr::{shard_seed, SskrPolicy};
///
/// let seed = b"abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about";
/// let policy = SskrPolicy::two_of_three();
/// let shares = shard_seed(seed, &policy)?;
/// // Returns 3 shares, any 2 can recover the seed
/// ```
#[cfg(feature = "bc")]
pub fn shard_seed(seed_entropy: &[u8], policy: &SskrPolicy) -> Result<Vec<Vec<u8>>> {
    // Validate seed entropy length (BIP-39 valid lengths)
    match seed_entropy.len() {
        16 | 20 | 24 | 28 | 32 => {}
        _ => {
            return Err(BipKeychainError::OutputError(format!(
                "Invalid seed entropy length: {} bytes. Must be 16, 20, 24, 28, or 32 bytes.",
                seed_entropy.len()
            )))
        }
    }

    // Create SSKR secret from seed entropy
    let secret = Secret::new(seed_entropy.to_vec())
        .map_err(|e| BipKeychainError::OutputError(format!("Failed to create secret: {:?}", e)))?;

    // Create specification: 1 group with policy.groups members, policy.threshold required
    let group_spec = GroupSpec::new(policy.threshold as usize, policy.groups as usize)
        .map_err(|e| BipKeychainError::OutputError(format!("Failed to create group spec: {:?}", e)))?;

    let spec = Spec::new(1, vec![group_spec])
        .map_err(|e| BipKeychainError::OutputError(format!("Failed to create SSKR spec: {:?}", e)))?;

    // Generate shares (returns Vec<Vec<Vec<u8>>> - groups of shares)
    let groups = sskr_generate(&spec, &secret)
        .map_err(|e| BipKeychainError::OutputError(format!("Failed to shard seed: {:?}", e)))?;

    // Since we use 1 group, extract the first (and only) group's shares
    let share_bytes = groups
        .into_iter()
        .next()
        .ok_or_else(|| BipKeychainError::OutputError("No shares generated".to_string()))?;

    Ok(share_bytes)
}

/// Recover a BIP-39 seed from SSKR shares
///
/// Combines M-of-N SSKR shares to recover the original seed entropy.
///
/// # Arguments
/// * `shares` - Vector of SSKR share bytes (at least threshold required)
///
/// # Returns
/// The recovered seed entropy
///
/// # Example
///
/// ```rust,ignore
/// use bip_keychain::sskr::{shard_seed, recover_seed, SskrPolicy};
///
/// let seed = b"test seed entropy";
/// let policy = SskrPolicy::two_of_three();
/// let shares = shard_seed(seed, &policy)?;
///
/// // Recover using any 2 of 3 shares
/// let recovered = recover_seed(&shares[0..2])?;
/// assert_eq!(recovered, seed);
/// ```
#[cfg(feature = "bc")]
pub fn recover_seed(share_bytes: &[Vec<u8>]) -> Result<Vec<u8>> {
    if share_bytes.is_empty() {
        return Err(BipKeychainError::OutputError(
            "No shares provided for recovery".to_string(),
        ));
    }

    // Combine the shares to recover the secret
    let secret = sskr_combine(share_bytes)
        .map_err(|e| BipKeychainError::OutputError(format!("Failed to recover seed: {:?}", e)))?;

    Ok(secret.data().to_vec())
}

#[cfg(all(test, feature = "bc"))]
mod tests {
    use super::*;

    #[test]
    fn test_sskr_policy_validation() {
        // Valid policies
        assert!(SskrPolicy::new(2, 2).is_ok());
        assert!(SskrPolicy::new(3, 2).is_ok());
        assert!(SskrPolicy::new(5, 3).is_ok());

        // Invalid: groups too small
        assert!(SskrPolicy::new(1, 1).is_err());

        // Invalid: groups too large
        assert!(SskrPolicy::new(17, 10).is_err());

        // Invalid: threshold > groups
        assert!(SskrPolicy::new(3, 4).is_err());

        // Invalid: threshold = 0
        assert!(SskrPolicy::new(3, 0).is_err());
    }

    #[test]
    fn test_sskr_common_policies() {
        let two_of_three = SskrPolicy::two_of_three();
        assert_eq!(two_of_three.groups, 3);
        assert_eq!(two_of_three.threshold, 2);

        let three_of_five = SskrPolicy::three_of_five();
        assert_eq!(three_of_five.groups, 5);
        assert_eq!(three_of_five.threshold, 3);

        let two_of_two = SskrPolicy::two_of_two();
        assert_eq!(two_of_two.groups, 2);
        assert_eq!(two_of_two.threshold, 2);
    }

    #[test]
    fn test_shard_and_recover_2_of_3() {
        // 16 bytes = 128 bits (12-word BIP-39)
        let seed = b"test seed 16byte";

        let policy = SskrPolicy::two_of_three();
        let shares = shard_seed(seed, &policy).expect("Should shard seed");

        assert_eq!(shares.len(), 3, "Should generate 3 shares");

        // Recover with shares 0 and 1
        let recovered = recover_seed(&shares[0..2]).expect("Should recover with 2 shares");
        assert_eq!(recovered, seed, "Recovered seed should match original");

        // Recover with shares 1 and 2
        let recovered = recover_seed(&shares[1..3]).expect("Should recover with different 2 shares");
        assert_eq!(recovered, seed, "Recovered seed should match original");

        // Cannot recover with only 1 share
        let result = recover_seed(&shares[0..1]);
        assert!(result.is_err(), "Should not recover with only 1 share");
    }

    #[test]
    fn test_shard_and_recover_3_of_5() {
        // 32 bytes = 256 bits (24-word BIP-39)
        let seed = [42u8; 32];

        let policy = SskrPolicy::three_of_five();
        let shares = shard_seed(&seed, &policy).expect("Should shard seed");

        assert_eq!(shares.len(), 5, "Should generate 5 shares");

        // Recover with shares 0, 1, 2
        let recovered = recover_seed(&shares[0..3]).expect("Should recover with 3 shares");
        assert_eq!(recovered, seed, "Recovered seed should match original");

        // Recover with shares 2, 3, 4
        let recovered = recover_seed(&shares[2..5]).expect("Should recover with different 3 shares");
        assert_eq!(recovered, seed, "Recovered seed should match original");

        // Cannot recover with only 2 shares
        let result = recover_seed(&shares[0..2]);
        assert!(result.is_err(), "Should not recover with only 2 shares");
    }

    #[test]
    fn test_invalid_seed_length() {
        let invalid_seed = b"too short"; // Not a valid BIP-39 length

        let policy = SskrPolicy::two_of_three();
        let result = shard_seed(invalid_seed, &policy);

        assert!(result.is_err(), "Should reject invalid seed length");
    }

    #[test]
    fn test_recovery_with_no_shares() {
        let shares: Vec<Vec<u8>> = vec![];
        let result = recover_seed(&shares);

        assert!(result.is_err(), "Should reject empty shares");
    }
}
