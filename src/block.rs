use crate::crypto::calculate_hash;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: String,
}

impl Block {
    /// Creates a new block and calculates its hash
    pub fn new(index: u64, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String, difficulty: u32) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            difficulty,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Creates a new block without mining (for testing)
    #[cfg(test)]
    pub fn new_unmined(index: u64, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String, difficulty: u32) -> Self {
        Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            difficulty,
            hash: String::new(),
        }
    }

    /// Calculates the hash of the block based on its contents
    pub fn calculate_hash(&self) -> String {
        // Create a deterministic string representation of transactions
        let transactions_string: String = self.transactions
            .iter()
            .map(|tx| format!("{}{}{}", tx.sender, tx.receiver, tx.amount))
            .collect();

        let block_string = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, transactions_string, self.previous_hash, self.nonce
        );
        calculate_hash(&block_string)
    }

    /// Checks if a hash meets the difficulty requirement
    /// Returns true if the hash starts with the specified number of zeros
    pub fn is_hash_valid(hash: &str, difficulty: u32) -> bool {
        let prefix = "0".repeat(difficulty as usize);
        hash.starts_with(&prefix)
    }

    /// Mines the block by finding a nonce that produces a valid hash
    /// This is the proof-of-work algorithm - brute force search for valid hash
    pub fn mine_block(&mut self) {
        // Target string with required leading zeros
        let target = "0".repeat(self.difficulty as usize);

        // Mining loop: increment nonce until we find a valid hash
        // This is the "burning electricity" part
        while !self.hash.starts_with(&target) {
            self.nonce += 1;
            self.hash = self.calculate_hash();
        }

        // When we exit the loop, we've found a valid hash
        // The nonce proves we did the work
    }

    /// Creates the genesis block (first block in the chain)
    pub fn genesis() -> Self {
        Block::new(
            0,
            0,
            Vec::new(), // Empty transactions for genesis block
            String::from("0"),
            0, // Genesis block has no difficulty requirement
        )
    }

    /// Returns the number of transactions in this block
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Displays the block with its transactions
    pub fn display(&self) {
        println!("Block #{}", self.index);
        println!("  Timestamp:     {}", self.timestamp);
        println!("  Transactions:  {}", self.transaction_count());
        for (i, tx) in self.transactions.iter().enumerate() {
            println!("    {}. {}", i + 1, tx);
        }
        if self.transaction_count() == 0 {
            println!("    (No transactions)");
        }
        println!("  Previous Hash: {}", self.previous_hash);
        println!("  Difficulty:    {}", self.difficulty);
        println!("  Nonce:         {}", self.nonce);
        println!("  Hash:          {}", self.hash);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation_empty() {
        let block = Block::new(
            1,
            1234567890,
            Vec::new(),
            String::from("previous_hash"),
            2,
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.timestamp, 1234567890);
        assert_eq!(block.transaction_count(), 0);
        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.nonce, 0);
        assert_eq!(block.difficulty, 2);
        assert_ne!(block.hash, "");
    }

    #[test]
    fn test_block_creation_with_transactions() {
        let tx1 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        let tx2 = Transaction::new_unvalidated(
            String::from("Bob"),
            String::from("Charlie"),
            5.0,
        );

        let block = Block::new(
            1,
            1234567890,
            vec![tx1, tx2],
            String::from("previous_hash"),
            2,
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.transaction_count(), 2);
        assert_eq!(block.transactions[0].sender, "Alice");
        assert_eq!(block.transactions[1].sender, "Bob");
        assert_ne!(block.hash, "");
    }

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert_eq!(genesis.transaction_count(), 0);
        assert_ne!(genesis.hash, "");
    }

    #[test]
    fn test_hash_determinism() {
        let tx1 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        let tx2 = Transaction::new_unvalidated(
            String::from("Bob"),
            String::from("Charlie"),
            5.0,
        );

        let block1 = Block::new(
            1,
            1234567890,
            vec![tx1.clone(), tx2.clone()],
            String::from("prev"),
            2,
        );
        let block2 = Block::new(
            1,
            1234567890,
            vec![tx1, tx2],
            String::from("prev"),
            2,
        );

        assert_eq!(block1.hash, block2.hash);
    }

    #[test]
    fn test_transaction_order_affects_hash() {
        let tx1 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        let tx2 = Transaction::new_unvalidated(
            String::from("Bob"),
            String::from("Charlie"),
            5.0,
        );

        let block1 = Block::new(
            1,
            1234567890,
            vec![tx1.clone(), tx2.clone()],
            String::from("prev"),
            2,
        );
        let block2 = Block::new(
            1,
            1234567890,
            vec![tx2, tx1], // Different order
            String::from("prev"),
            2,
        );

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn test_avalanche_effect() {
        let tx1 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );

        let block1 = Block::new(
            1,
            1234567890,
            vec![tx1.clone()],
            String::from("prev"),
            2,
        );

        let tx2 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.1, // Different amount
        );

        let block2 = Block::new(
            1,
            1234567890,
            vec![tx2],
            String::from("prev"),
            2,
        );

        assert_ne!(block1.hash, block2.hash);
    }

    #[test]
    fn test_hash_validation() {
        // Test hash validation with different difficulties
        assert!(Block::is_hash_valid("0000abc123", 4));
        assert!(Block::is_hash_valid("000abc123", 3));
        assert!(Block::is_hash_valid("00abc123", 2));
        assert!(Block::is_hash_valid("0abc123", 1));
        assert!(!Block::is_hash_valid("abc123", 1));
        assert!(!Block::is_hash_valid("000abc123", 4));
    }

    #[test]
    fn test_basic_mining() {
        let tx = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );

        let mut block = Block::new_unmined(
            1,
            1234567890,
            vec![tx],
            String::from("prev"),
            1, // Low difficulty for fast testing
        );

        assert_eq!(block.nonce, 0);
        assert_eq!(block.hash, "");

        // Mine the block
        block.mine_block();

        // Verify the block was mined
        assert_ne!(block.nonce, 0);
        assert_ne!(block.hash, "");
        assert!(Block::is_hash_valid(&block.hash, 1));
    }

    #[test]
    fn test_mining_with_different_difficulties() {
        let tx = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );

        // Mine with difficulty 1
        let mut block1 = Block::new_unmined(
            1,
            1234567890,
            vec![tx.clone()],
            String::from("prev"),
            1,
        );
        block1.mine_block();
        assert!(Block::is_hash_valid(&block1.hash, 1));

        // Mine with difficulty 2
        let mut block2 = Block::new_unmined(
            1,
            1234567890,
            vec![tx.clone()],
            String::from("prev"),
            2,
        );
        block2.mine_block();
        assert!(Block::is_hash_valid(&block2.hash, 2));

        // Higher difficulty should result in higher nonce (more work)
        assert!(block2.nonce > block1.nonce);
    }

    #[test]
    fn test_mining_determinism() {
        let tx = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );

        // Mine two identical blocks
        let mut block1 = Block::new_unmined(
            1,
            1234567890,
            vec![tx.clone()],
            String::from("prev"),
            2,
        );
        block1.mine_block();

        let mut block2 = Block::new_unmined(
            1,
            1234567890,
            vec![tx],
            String::from("prev"),
            2,
        );
        block2.mine_block();

        // Same inputs should produce same result
        assert_eq!(block1.hash, block2.hash);
        assert_eq!(block1.nonce, block2.nonce);
    }

    #[test]
    fn test_mining_changes_hash() {
        let tx = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );

        let mut block = Block::new_unmined(
            1,
            1234567890,
            vec![tx],
            String::from("prev"),
            1,
        );

        // Get the initial hash (with nonce = 0)
        let initial_hash = block.calculate_hash();
        block.hash = initial_hash.clone();

        // Mine the block
        block.mine_block();

        // The mined hash should be different from initial
        assert_ne!(block.hash, initial_hash);
        // And should meet difficulty requirement
        assert!(Block::is_hash_valid(&block.hash, 1));
    }

    #[test]
    fn test_mining_with_transactions() {
        let tx1 = Transaction::new_unvalidated(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        let tx2 = Transaction::new_unvalidated(
            String::from("Bob"),
            String::from("Charlie"),
            5.0,
        );

        let mut block = Block::new_unmined(
            1,
            1234567890,
            vec![tx1, tx2],
            String::from("prev"),
            2,
        );

        block.mine_block();

        assert!(Block::is_hash_valid(&block.hash, 2));
        assert_ne!(block.nonce, 0);
        assert_eq!(block.transaction_count(), 2);
    }
}
