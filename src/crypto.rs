use sha2::{Digest, Sha256};
use hex;

/// Calculates SHA-256 hash of the given input string
/// Returns hexadecimal encoded hash string
pub fn calculate_hash(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    let result = hasher.finalize();
    hex::encode(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_determinism() {
        let input = "test data";
        let hash1 = calculate_hash(input);
        let hash2 = calculate_hash(input);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_length() {
        let input = "test data";
        let hash = calculate_hash(input);
        assert_eq!(hash.len(), 64);
    }

    #[test]
    fn test_avalanche_effect() {
        let hash1 = calculate_hash("test data");
        let hash2 = calculate_hash("test data.");
        assert_ne!(hash1, hash2);
    }
}
