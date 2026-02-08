use crate::block::Block;
use crate::blockchain::Blockchain;
use std::fmt;

/// Validation errors that can occur during chain validation
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationError {
    /// The stored hash doesn't match the computed hash
    InvalidHash { index: usize, stored: String, computed: String },
    /// The previous_hash doesn't match the actual previous block's hash
    BrokenLink { index: usize, previous_hash: String, expected: String },
    /// The hash doesn't meet the difficulty requirement
    InvalidProofOfWork { index: usize, hash: String, difficulty: u32 },
    /// The block index is not sequential
    InvalidIndex { index: usize, expected: usize },
    /// The genesis block doesn't meet requirements
    InvalidGenesis { reason: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::InvalidHash { index, stored, computed } => {
                write!(f, "Block #{}: Invalid hash\n  Stored:   {}\n  Computed: {}", index, stored, computed)
            }
            ValidationError::BrokenLink { index, previous_hash, expected } => {
                write!(f, "Block #{}: Broken chain link\n  Previous hash: {}\n  Expected:      {}", index, previous_hash, expected)
            }
            ValidationError::InvalidProofOfWork { index, hash, difficulty } => {
                write!(f, "Block #{}: Invalid proof-of-work\n  Hash:       {}\n  Difficulty: {} (requires {} leading zeros)",
                    index, hash, difficulty, difficulty)
            }
            ValidationError::InvalidIndex { index, expected } => {
                write!(f, "Block #{}: Invalid index (expected {})", index, expected)
            }
            ValidationError::InvalidGenesis { reason } => {
                write!(f, "Genesis block: {}", reason)
            }
        }
    }
}

/// Detailed validation result that includes all validation errors
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub errors: Vec<ValidationError>,
}

impl ValidationResult {
    pub fn valid() -> Self {
        ValidationResult {
            is_valid: true,
            errors: Vec::new(),
        }
    }

    pub fn invalid(errors: Vec<ValidationError>) -> Self {
        ValidationResult {
            is_valid: false,
            errors,
        }
    }

    pub fn get_first_error(&self) -> Option<&ValidationError> {
        self.errors.first()
    }

    pub fn display_errors(&self) {
        if self.is_valid {
            println!("Chain is valid ✓");
        } else {
            println!("Chain is invalid ✗");
            println!("\nValidation errors:");
            for (i, error) in self.errors.iter().enumerate() {
                println!("  {}. {}", i + 1, error);
            }
        }
    }
}

/// Validates a single block's hash
pub fn verify_block_hash(block: &Block) -> Result<(), ValidationError> {
    let computed_hash = block.calculate_hash();
    if block.hash != computed_hash {
        return Err(ValidationError::InvalidHash {
            index: block.index as usize,
            stored: block.hash.clone(),
            computed: computed_hash,
        });
    }
    Ok(())
}

/// Validates the chain link between two consecutive blocks
pub fn verify_chain_link(current_block: &Block, previous_block: &Block) -> Result<(), ValidationError> {
    if current_block.previous_hash != previous_block.hash {
        return Err(ValidationError::BrokenLink {
            index: current_block.index as usize,
            previous_hash: current_block.previous_hash.clone(),
            expected: previous_block.hash.clone(),
        });
    }
    Ok(())
}

/// Validates proof-of-work for a block
pub fn verify_proof_of_work(block: &Block) -> Result<(), ValidationError> {
    if !Block::is_hash_valid(&block.hash, block.difficulty) {
        return Err(ValidationError::InvalidProofOfWork {
            index: block.index as usize,
            hash: block.hash.clone(),
            difficulty: block.difficulty,
        });
    }
    Ok(())
}

/// Validates the genesis block
pub fn verify_genesis_block(block: &Block) -> Result<(), ValidationError> {
    if block.index != 0 {
        return Err(ValidationError::InvalidGenesis {
            reason: format!("Invalid index: expected 0, got {}", block.index),
        });
    }

    if block.previous_hash != "0" {
        return Err(ValidationError::InvalidGenesis {
            reason: format!("Invalid previous_hash: expected '0', got '{}'", block.previous_hash),
        });
    }

    Ok(())
}

/// Validates block index sequencing
pub fn verify_block_index(block: &Block, expected_index: usize) -> Result<(), ValidationError> {
    if block.index as usize != expected_index {
        return Err(ValidationError::InvalidIndex {
            index: block.index as usize,
            expected: expected_index,
        });
    }
    Ok(())
}

/// Comprehensive validation of the entire blockchain
/// Returns a detailed ValidationResult with all errors found
pub fn validate_chain(blockchain: &Blockchain) -> ValidationResult {
    let mut errors = Vec::new();

    // Validate genesis block
    if let Some(genesis) = blockchain.chain.first() {
        if let Err(e) = verify_genesis_block(genesis) {
            errors.push(e);
        }
    }

    // Validate each block in the chain
    for i in 1..blockchain.chain.len() {
        let current_block = &blockchain.chain[i];
        let previous_block = &blockchain.chain[i - 1];

        // Check index sequencing
        if let Err(e) = verify_block_index(current_block, i) {
            errors.push(e);
        }

        // Verify hash integrity
        if let Err(e) = verify_block_hash(current_block) {
            errors.push(e);
        }

        // Verify chain link
        if let Err(e) = verify_chain_link(current_block, previous_block) {
            errors.push(e);
        }

        // Verify proof-of-work
        if let Err(e) = verify_proof_of_work(current_block) {
            errors.push(e);
        }
    }

    if errors.is_empty() {
        ValidationResult::valid()
    } else {
        ValidationResult::invalid(errors)
    }
}

/// Quick validation check (stops at first error)
pub fn validate_chain_quick(blockchain: &Blockchain) -> bool {
    for i in 1..blockchain.chain.len() {
        let current_block = &blockchain.chain[i];
        let previous_block = &blockchain.chain[i - 1];

        // Quick checks: hash, link, and proof-of-work
        if current_block.hash != current_block.calculate_hash() {
            return false;
        }

        if current_block.previous_hash != previous_block.hash {
            return false;
        }

        if !Block::is_hash_valid(&current_block.hash, current_block.difficulty) {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transaction::Transaction;

    #[test]
    fn test_verify_block_hash_valid() {
        let block = Block::new(
            1,
            1234567890,
            vec![],
            String::from("prev"),
            2,
        );
        assert!(verify_block_hash(&block).is_ok());
    }

    #[test]
    fn test_verify_block_hash_invalid() {
        let mut block = Block::new(
            1,
            1234567890,
            vec![],
            String::from("prev"),
            2,
        );
        block.hash = String::from("fake_hash");
        assert!(verify_block_hash(&block).is_err());
    }

    #[test]
    fn test_verify_chain_link_valid() {
        let block1 = Block::new(0, 1234567890, vec![], String::from("0"), 0);
        let block2 = Block::new(1, 1234567891, vec![], block1.hash.clone(), 2);

        assert!(verify_chain_link(&block2, &block1).is_ok());
    }

    #[test]
    fn test_verify_chain_link_invalid() {
        let block1 = Block::new(0, 1234567890, vec![], String::from("0"), 0);
        let mut block2 = Block::new(1, 1234567891, vec![], block1.hash.clone(), 2);
        block2.previous_hash = String::from("wrong");

        assert!(verify_chain_link(&block2, &block1).is_err());
    }

    #[test]
    fn test_verify_proof_of_work_valid() {
        let mut block = Block::new_unmined(1, 1234567890, vec![], String::from("prev"), 2);
        block.mine_block();
        assert!(verify_proof_of_work(&block).is_ok());
    }

    #[test]
    fn test_verify_proof_of_work_invalid() {
        let block = Block::new(1, 1234567890, vec![], String::from("prev"), 2);
        // This hash won't meet difficulty 2
        assert!(verify_proof_of_work(&block).is_err());
    }

    #[test]
    fn test_verify_genesis_block_valid() {
        let genesis = Block::genesis();
        assert!(verify_genesis_block(&genesis).is_ok());
    }

    #[test]
    fn test_verify_genesis_block_invalid_index() {
        let mut block = Block::new(0, 1234567890, vec![], String::from("0"), 0);
        block.index = 5;
        assert!(verify_genesis_block(&block).is_err());
    }

    #[test]
    fn test_verify_genesis_block_invalid_previous_hash() {
        let block = Block::new(0, 1234567890, vec![], String::from("not_zero"), 0);
        assert!(verify_genesis_block(&block).is_err());
    }

    #[test]
    fn test_validate_chain_valid() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        let result = validate_chain(&blockchain);
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_validate_chain_tampered_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        // Tamper with the block
        blockchain.chain[1].transactions[0].amount = 999.0;

        let result = validate_chain(&blockchain);
        assert!(!result.is_valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_validate_chain_quick() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        assert!(validate_chain_quick(&blockchain));

        // Tamper with the block
        blockchain.chain[1].transactions[0].amount = 999.0;

        assert!(!validate_chain_quick(&blockchain));
    }
}
