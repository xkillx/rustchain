use crate::block::Block;
use crate::transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH};

/// Difference between two blockchains
#[derive(Debug, Clone)]
pub struct ChainDiff {
    pub blocks_different: usize,
    pub first_divergence: Option<usize>,
}

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

    // =========================================================================
    // Day 5: Attack Simulation Methods
    // =========================================================================
    // WARNING: These methods are for educational purposes only!
    // They allow direct manipulation of the chain for attack simulation.
    // In production, these methods should NOT exist or be strictly access-controlled.

    /// Gets a mutable reference to a block by index (for attack simulation)
    /// WARNING: This is dangerous! Only use for educational attack demonstrations
    pub fn get_block_mut(&mut self, index: usize) -> Option<&mut Block> {
        self.chain.get_mut(index)
    }

    /// Gets a reference to a block by index (for inspection)
    pub fn get_block(&self, index: usize) -> Option<&Block> {
        self.chain.get(index)
    }

    /// Tamper with a block's transactions (attack simulation)
    /// WARNING: This breaks the chain! Use for demonstration only.
    pub fn tamper_with_transactions(&mut self, index: usize, new_transactions: Vec<Transaction>) {
        if let Some(block) = self.get_block_mut(index) {
            block.transactions = new_transactions;
            // Note: We DON'T recalculate the hash, so the chain will be invalid
            // This simulates an attacker trying to change history
        }
    }

    /// Tamper with a block's hash directly (attack simulation)
    /// WARNING: This breaks the chain! Use for demonstration only.
    pub fn tamper_with_hash(&mut self, index: usize, new_hash: String) {
        if let Some(block) = self.get_block_mut(index) {
            block.hash = new_hash;
        }
    }

    /// Tamper with a block's nonce (attack simulation)
    /// WARNING: This breaks the chain! Use for demonstration only.
    pub fn tamper_with_nonce(&mut self, index: usize, new_nonce: u64) {
        if let Some(block) = self.get_block_mut(index) {
            block.nonce = new_nonce;
        }
    }

    /// Tamper with a block's previous_hash (attack simulation)
    /// WARNING: This breaks the chain! Use for demonstration only.
    pub fn tamper_with_previous_hash(&mut self, index: usize, new_previous_hash: String) {
        if let Some(block) = self.get_block_mut(index) {
            block.previous_hash = new_previous_hash;
        }
    }

    /// Checks if this blockchain is longer than another
    pub fn is_longer_than(&self, other: &Blockchain) -> bool {
        self.len() > other.len()
    }

    /// Compares two blockchains and returns the differences
    pub fn compare_chains(&self, other: &Blockchain) -> ChainDiff {
        let min_len = self.len().min(other.len());
        let mut first_divergence = None;

        for i in 0..min_len {
            if self.chain[i].hash != other.chain[i].hash {
                first_divergence = Some(i);
                break;
            }
        }

        // If all common blocks match, check if one chain is longer
        if first_divergence.is_none() && self.len() != other.len() {
            first_divergence = Some(min_len);
        }

        // Count blocks different from the point of first divergence
        let blocks_different = if let Some(divergence) = first_divergence {
            (self.len() - divergence) + (other.len() - divergence)
        } else {
            0
        };

        ChainDiff {
            blocks_different,
            first_divergence,
        }
    }

    /// Replaces the current chain with a new one if it's valid and longer
    /// Simulates chain reorganization in blockchain consensus
    pub fn replace_chain(&mut self, new_chain: Blockchain) -> Result<(), String> {
        // Validate the new chain
        if !new_chain.is_valid() {
            return Err("Cannot replace with invalid chain".to_string());
        }

        // Only replace if new chain is longer
        if new_chain.len() <= self.len() {
            return Err("Cannot replace with shorter or equal-length chain".to_string());
        }

        // Replace the chain
        self.chain = new_chain.chain;
        self.difficulty = new_chain.difficulty;
        // Note: We don't copy pending_transactions as they're local to this node

        Ok(())
    }

    /// Re-mines a block and all subsequent blocks
    /// This demonstrates the cost of rewriting history
    /// Returns the number of blocks that were re-mined
    pub fn remine_from(&mut self, index: usize) -> Result<usize, String> {
        if index >= self.len() {
            return Err("Index out of bounds".to_string());
        }

        if index == 0 {
            return Err("Cannot re-mine genesis block".to_string());
        }

        let mut blocks_remined = 0;
        let chain_len = self.len();

        // Re-mine each block starting from the specified index
        for i in index..chain_len {
            // Re-calculate the hash with current nonce
            self.chain[i].hash = self.chain[i].calculate_hash();

            // Re-mine to find new valid nonce
            self.chain[i].mine_block();

            // If this isn't the last block, update the next block's previous_hash
            if i < chain_len - 1 {
                self.chain[i + 1].previous_hash = self.chain[i].hash.clone();
            }

            blocks_remined += 1;
        }

        Ok(blocks_remined)
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

    // Day 5: Attack Simulation Tests

    #[test]
    fn test_tamper_with_transactions_detected() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        // Chain should be valid
        assert!(blockchain.is_valid());

        // Tamper with the block
        blockchain.tamper_with_transactions(1, vec![
            Transaction::new_unvalidated(String::from("Eve"), String::from("Eve"), 999999.0),
        ]);

        // Chain should now be invalid
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_tamper_with_hash_detected() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        // Chain should be valid
        assert!(blockchain.is_valid());

        // Tamper with the hash
        blockchain.tamper_with_hash(1, String::from("fake_hash"));

        // Chain should now be invalid
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_tamper_with_previous_hash_detected() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();
        blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();
        blockchain.mine_block();

        // Chain should be valid
        assert!(blockchain.is_valid());

        // Break the chain link
        blockchain.tamper_with_previous_hash(2, String::from("wrong_hash"));

        // Chain should now be invalid
        assert!(!blockchain.is_valid());
    }

    #[test]
    fn test_cascading_validation_failure() {
        let mut blockchain = Blockchain::new();

        // Create a chain with multiple blocks
        for i in 1..=4 {
            blockchain.add_transaction(
                String::from("Alice"),
                String::from(&format!("User{}", i)),
                10.0,
            ).unwrap();
            blockchain.mine_block();
        }

        // Chain should be valid
        assert!(blockchain.is_valid());

        // Tamper with an early block (block 1)
        blockchain.chain[1].transactions[0].amount = 999.0;

        // All subsequent blocks should be invalid
        // The validation should fail at block 1
        assert!(!blockchain.is_valid());

        // Specifically, block 2's previous_hash should point to the now-invalid block 1
        // and block 2 itself is now also invalid because it references a changed hash
    }

    #[test]
    fn test_compare_chains_identical() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let blockchain2 = blockchain1.clone();

        let diff = blockchain1.compare_chains(&blockchain2);
        assert_eq!(diff.blocks_different, 0);
        assert!(diff.first_divergence.is_none());
    }

    #[test]
    fn test_compare_chains_different() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let mut blockchain2 = Blockchain::new();
        blockchain2.add_transaction(String::from("Different"), String::from("User"), 10.0).unwrap();
        blockchain2.mine_block();

        let diff = blockchain1.compare_chains(&blockchain2);
        assert!(diff.blocks_different > 0);
        assert!(diff.first_divergence.is_some());
    }

    #[test]
    fn test_is_longer_than() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let blockchain2 = Blockchain::new();

        assert!(blockchain1.is_longer_than(&blockchain2));
        assert!(!blockchain2.is_longer_than(&blockchain1));
    }

    #[test]
    fn test_replace_chain_with_valid_longer() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let mut blockchain2 = Blockchain::new();
        blockchain2.add_transaction(String::from("Different"), String::from("User"), 10.0).unwrap();
        blockchain2.mine_block();
        blockchain2.add_transaction(String::from("User"), String::from("Another"), 5.0).unwrap();
        blockchain2.mine_block();

        let original_len = blockchain1.len();
        let result = blockchain1.replace_chain(blockchain2);

        assert!(result.is_ok());
        assert!(blockchain1.len() > original_len);
        assert!(blockchain1.is_valid());
    }

    #[test]
    fn test_replace_chain_with_invalid() {
        let mut blockchain1 = Blockchain::new();

        let mut blockchain2 = Blockchain::new();
        blockchain2.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain2.mine_block();

        // Tamper with blockchain2 to make it invalid
        blockchain2.chain[1].transactions[0].amount = 999.0;

        let result = blockchain1.replace_chain(blockchain2);
        assert!(result.is_err());
    }

    #[test]
    fn test_replace_chain_with_shorter() {
        let mut blockchain1 = Blockchain::new();
        blockchain1.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain1.mine_block();

        let blockchain2 = Blockchain::new();

        let result = blockchain1.replace_chain(blockchain2);
        assert!(result.is_err());
    }

    #[test]
    fn test_remine_from() {
        let mut blockchain = Blockchain::new();

        // Create a chain with 3 blocks
        for i in 1..=3 {
            blockchain.add_transaction(
                String::from("Alice"),
                String::from(&format!("User{}", i)),
                10.0,
            ).unwrap();
            blockchain.mine_block();
        }

        assert!(blockchain.is_valid());

        // Tamper with block 1
        blockchain.chain[1].transactions[0].amount = 999.0;
        assert!(!blockchain.is_valid());

        // Re-mine from block 1
        let result = blockchain.remine_from(1);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3); // Should have re-mined 3 blocks

        // Chain should be valid again
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_get_block() {
        let blockchain = Blockchain::new();

        let block = blockchain.get_block(0);
        assert!(block.is_some());
        assert_eq!(block.unwrap().index, 0);

        let block = blockchain.get_block(99);
        assert!(block.is_none());
    }

    #[test]
    fn test_get_block_mut() {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
        blockchain.mine_block();

        if let Some(block) = blockchain.get_block_mut(1) {
            block.transactions[0].amount = 999.0;
        }

        // The tampering should have worked
        assert_eq!(blockchain.chain[1].transactions[0].amount, 999.0);
        // And the chain should now be invalid
        assert!(!blockchain.is_valid());
    }
}
