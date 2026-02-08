use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

/// Blockchain struct that manages the chain of blocks
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Vector storing all blocks in order
    pub chain: Vec<Block>,
    /// Mining difficulty (number of leading zeros required) - for Day 4
    pub difficulty: u32,
    /// Pending transaction pool (mempool)
    pub pending_transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Self {
        let mut blockchain = Blockchain {
            chain: Vec::new(),
            difficulty: 4, // Default difficulty: 4 leading zeros
            pending_transactions: Vec::new(),
        };

        // Create and add the genesis block
        let genesis_block = Self::create_genesis_block();
        blockchain.chain.push(genesis_block);

        blockchain
    }

    /// Creates the genesis block (first block in the chain)
    fn create_genesis_block() -> Block {
        Block::genesis()
    }

    /// Returns a reference to the latest block in the chain
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Chain should always have at least genesis block")
    }

    /// Adds a transaction to the pending pool (mempool)
    pub fn add_transaction(&mut self, sender: String, receiver: String, amount: f64) -> Result<(), String> {
        // Validate and create the transaction
        let transaction = Transaction::new(sender, receiver, amount)?;

        // Add to pending pool
        self.pending_transactions.push(transaction);

        Ok(())
    }

    /// Returns a reference to the pending transactions
    pub fn get_pending_transactions(&self) -> &Vec<Transaction> {
        &self.pending_transactions
    }

    /// Returns the number of pending transactions
    pub fn pending_transaction_count(&self) -> usize {
        self.pending_transactions.len()
    }

    /// Clears the pending transaction pool
    pub fn clear_pending_transactions(&mut self) {
        self.pending_transactions.clear();
    }

    /// Mines a new block with pending transactions using proof-of-work
    pub fn mine_block(&mut self) {
        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        // Get the previous block's hash
        let previous_hash = self.get_latest_block().hash.clone();

        // Calculate the new block's index
        let new_index = self.chain.len() as u64;

        // Take pending transactions and clear the pool
        let transactions = std::mem::take(&mut self.pending_transactions);

        // Create the new block with the blockchain's difficulty
        let mut new_block = Block::new(new_index, timestamp, transactions, previous_hash, self.difficulty);

        // Mine the block (this is where proof-of-work happens)
        new_block.mine_block();

        // Add the mined block to the chain
        self.chain.push(new_block);
    }

    /// Validates the integrity of the blockchain
    /// Checks that each block's hash is correct, links are valid, and proof-of-work is met
    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current_block = &self.chain[i];
            let previous_block = &self.chain[i - 1];

            // Verify the current block's hash is correct
            if current_block.hash != current_block.calculate_hash() {
                return false;
            }

            // Verify the current block points to the previous block
            if current_block.previous_hash != previous_block.hash {
                return false;
            }

            // Verify proof-of-work (hash meets difficulty requirement)
            if !Block::is_hash_valid(&current_block.hash, current_block.difficulty) {
                return false;
            }
        }

        true
    }

    /// Returns the number of blocks in the chain
    pub fn len(&self) -> usize {
        self.chain.len()
    }

    /// Sets the mining difficulty
    pub fn set_difficulty(&mut self, difficulty: u32) {
        self.difficulty = difficulty;
    }

    /// Gets the current mining difficulty
    pub fn get_difficulty(&self) -> u32 {
        self.difficulty
    }

    /// Checks if the chain is empty (should always be false due to genesis block)
    pub fn is_empty(&self) -> bool {
        self.chain.is_empty()
    }

    /// Displays the entire blockchain in a readable format
    pub fn display(&self) {
        println!("\n=== Blockchain ===");
        println!("Total blocks: {}", self.len());
        println!("Difficulty: {}", self.difficulty);
        println!("Pending transactions: {}", self.pending_transaction_count());
        println!("Chain valid: {}\n", self.is_valid());

        for block in &self.chain {
            block.display();
            println!();
        }
    }

    /// Displays pending transactions
    pub fn display_pending_transactions(&self) {
        println!("\n=== Pending Transactions ({}) ===", self.pending_transaction_count());
        if self.pending_transaction_count() == 0 {
            println!("No pending transactions");
        } else {
            for (i, tx) in self.pending_transactions.iter().enumerate() {
                println!("  {}. {}", i + 1, tx);
            }
        }
    }

    /// Returns a summary of the blockchain
    pub fn summary(&self) {
        println!("\n=== Blockchain Summary ===");
        println!("Total blocks:           {}", self.len());
        println!("Latest block:           #{}", self.get_latest_block().index);
        println!("Latest hash:            {}", self.get_latest_block().hash);
        println!("Pending transactions:   {}", self.pending_transaction_count());
        println!("Chain valid:            {}", self.is_valid());
    }
}

impl Default for Blockchain {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blockchain_creation() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.len(), 1);
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert_eq!(blockchain.pending_transaction_count(), 0);
    }

    #[test]
    fn test_add_transaction() {
        let mut blockchain = Blockchain::new();
        let result = blockchain.add_transaction(
            String::from("Alice"),
            String::from("Bob"),
            10.0,
        );
        assert!(result.is_ok());
        assert_eq!(blockchain.pending_transaction_count(), 1);
    }

    #[test]
    fn test_add_invalid_transaction() {
        let mut blockchain = Blockchain::new();
        // Zero amount should fail
        let result = blockchain.add_transaction(
            String::from("Alice"),
            String::from("Bob"),
            0.0,
        );
        assert!(result.is_err());
        assert_eq!(blockchain.pending_transaction_count(), 0);
    }

    #[test]
    fn test_mine_block_with_transactions() {
        let mut blockchain = Blockchain::new();

        // Add some transactions
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();

        assert_eq!(blockchain.pending_transaction_count(), 2);

        // Mine a block
        blockchain.mine_block();

        // Verify block was added
        assert_eq!(blockchain.len(), 2);
        assert_eq!(blockchain.chain[1].transaction_count(), 2);
        assert_eq!(blockchain.pending_transaction_count(), 0); // Pool should be cleared
    }

    #[test]
    fn test_mine_empty_block() {
        let mut blockchain = Blockchain::new();
        assert_eq!(blockchain.len(), 1);

        // Mine with no pending transactions
        blockchain.mine_block();

        assert_eq!(blockchain.len(), 2);
        assert_eq!(blockchain.chain[1].transaction_count(), 0);
    }

    #[test]
    fn test_clear_pending_transactions() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();

        assert_eq!(blockchain.pending_transaction_count(), 2);

        blockchain.clear_pending_transactions();

        assert_eq!(blockchain.pending_transaction_count(), 0);
    }

    #[test]
    fn test_chain_validation_with_transactions() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();
        blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();
        blockchain.mine_block();

        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_genesis_block_is_first() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].previous_hash, "0");
        assert_eq!(blockchain.chain[0].transaction_count(), 0);
    }

    #[test]
    fn test_transaction_order_preserved_in_block() {
        let mut blockchain = Blockchain::new();

        // Add transactions in a specific order
        blockchain.add_transaction(String::from("A"), String::from("B"), 1.0).unwrap();
        blockchain.add_transaction(String::from("B"), String::from("C"), 2.0).unwrap();
        blockchain.add_transaction(String::from("C"), String::from("D"), 3.0).unwrap();

        blockchain.mine_block();

        let block = &blockchain.chain[1];
        assert_eq!(block.transaction_count(), 3);
        assert_eq!(block.transactions[0].sender, "A");
        assert_eq!(block.transactions[1].sender, "B");
        assert_eq!(block.transactions[2].sender, "C");
    }

    #[test]
    fn test_get_pending_transactions() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();

        let pending = blockchain.get_pending_transactions();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].sender, "Alice");
        assert_eq!(pending[0].receiver, "Bob");
    }

    #[test]
    fn test_default_difficulty() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.get_difficulty(), 4);
    }

    #[test]
    fn test_set_difficulty() {
        let mut blockchain = Blockchain::new();
        blockchain.set_difficulty(2);
        assert_eq!(blockchain.get_difficulty(), 2);

        blockchain.set_difficulty(5);
        assert_eq!(blockchain.get_difficulty(), 5);
    }

    #[test]
    fn test_mining_creates_valid_proof_of_work() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();

        blockchain.mine_block();

        let block = &blockchain.chain[1];
        assert!(Block::is_hash_valid(&block.hash, block.difficulty));
        assert_ne!(block.nonce, 0);
    }

    #[test]
    fn test_mining_with_different_difficulties() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.set_difficulty(1);
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let mut blockchain2 = Blockchain::new();
        blockchain2.set_difficulty(2);
        blockchain2.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain2.mine_block();

        // Higher difficulty should result in higher nonce
        assert!(blockchain2.chain[1].nonce > blockchain1.chain[1].nonce);

        // Both should have valid hashes for their difficulty
        assert!(Block::is_hash_valid(&blockchain1.chain[1].hash, 1));
        assert!(Block::is_hash_valid(&blockchain2.chain[1].hash, 2));
    }

    #[test]
    fn test_chain_validation_checks_proof_of_work() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        // Chain should be valid
        assert!(blockchain.is_valid());

        // Tamper with a block's hash (invalidate proof-of-work)
        blockchain.chain[1].hash = String::from("invalid");

        // Chain should now be invalid
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_mining_determinism() {
        // Note: Mining is deterministic only if all inputs are the same
        // Since timestamps differ in real mining, this test verifies
        // that the mining algorithm itself is deterministic

        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        let block = &blockchain.chain[1];

        // Verify that re-calculating the hash produces the same result
        let recalculated_hash = block.calculate_hash();
        assert_eq!(block.hash, recalculated_hash);

        // Verify the hash meets difficulty requirement
        assert!(Block::is_hash_valid(&block.hash, block.difficulty));

        // The key insight: given the same block data (index, timestamp, transactions,
        // previous_hash, nonce), we get the same hash every time
    }

    #[test]
    fn test_multiple_blocks_with_mining() {
        let mut blockchain = Blockchain::new();

        // Mine multiple blocks
        for i in 1..=3 {
            blockchain.add_transaction(
                String::from("Alice"),
                String::from(&format!("Bob{}", i)),
                10.0,
            ).unwrap();
            blockchain.mine_block();
        }

        assert_eq!(blockchain.len(), 4); // Genesis + 3 blocks

        // All blocks should have valid proof-of-work
        for block in &blockchain.chain {
            assert!(Block::is_hash_valid(&block.hash, block.difficulty));
        }

        // Chain should be valid
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_difficulty_affects_mining_time() {
        let mut blockchain = Blockchain::new();

        // Mine with low difficulty
        blockchain.set_difficulty(1);
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();
        let nonce1 = blockchain.chain[1].nonce;

        // Mine with higher difficulty
        blockchain.set_difficulty(3);
        blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();
        blockchain.mine_block();
        let nonce2 = blockchain.chain[2].nonce;

        // Higher difficulty should require more attempts
        assert!(nonce2 > nonce1);
    }
}
