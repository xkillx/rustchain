use crate::block::Block;
use std::time::{SystemTime, UNIX_EPOCH};

/// Blockchain struct that manages the chain of blocks
#[derive(Debug, Clone)]
pub struct Blockchain {
    /// Vector storing all blocks in order
    pub chain: Vec<Block>,
    /// Mining difficulty (number of leading zeros required) - for Day 4
    pub difficulty: u32,
    /// Pending transaction pool - for Day 3
    pub pending_transactions: Vec<String>,
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
        Block::new(
            0,
            0,
            String::from("Genesis Block - The Beginning of RustChain"),
            String::from("0"),
        )
    }

    /// Returns a reference to the latest block in the chain
    pub fn get_latest_block(&self) -> &Block {
        self.chain.last().expect("Chain should always have at least genesis block")
    }

    /// Adds a new block with the given data to the blockchain
    pub fn add_block(&mut self, data: String) {
        // Get the previous block's hash
        let previous_hash = self.get_latest_block().hash.clone();

        // Calculate the new block's index
        let new_index = self.chain.len() as u64;

        // Get current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        // Create the new block
        let new_block = Block::new(new_index, timestamp, data, previous_hash);

        // Add the block to the chain
        self.chain.push(new_block);
    }

    /// Validates the integrity of the blockchain
    /// Checks that each block's hash is correct and links are valid
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
        }

        true
    }

    /// Returns the number of blocks in the chain
    pub fn len(&self) -> usize {
        self.chain.len()
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
        println!("Chain valid: {}\n", self.is_valid());

        for block in &self.chain {
            Self::display_block(block);
            println!();
        }
    }

    /// Displays a single block
    fn display_block(block: &Block) {
        println!("Block #{}", block.index);
        println!("  Timestamp:     {}", block.timestamp);
        println!("  Data:          {}", block.data);
        println!("  Previous Hash: {}", block.previous_hash);
        println!("  Nonce:         {}", block.nonce);
        println!("  Hash:          {}", block.hash);
    }

    /// Returns a summary of the blockchain
    pub fn summary(&self) {
        println!("\n=== Blockchain Summary ===");
        println!("Total blocks:   {}", self.len());
        println!("Latest block:   #{}", self.get_latest_block().index);
        println!("Latest hash:    {}", self.get_latest_block().hash);
        println!("Chain valid:    {}", self.is_valid());
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
    }

    #[test]
    fn test_add_block() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("First transaction"));
        assert_eq!(blockchain.len(), 2);
        assert_eq!(blockchain.chain[1].data, "First transaction");
    }

    #[test]
    fn test_block_linking() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Block 1"));
        blockchain.add_block(String::from("Block 2"));

        // Verify second block points to first
        assert_eq!(
            blockchain.chain[1].previous_hash,
            blockchain.chain[0].hash
        );
        // Verify third block points to second
        assert_eq!(
            blockchain.chain[2].previous_hash,
            blockchain.chain[1].hash
        );
    }

    #[test]
    fn test_chain_validation() {
        let mut blockchain = Blockchain::new();
        blockchain.add_block(String::from("Transaction 1"));
        blockchain.add_block(String::from("Transaction 2"));
        assert!(blockchain.is_valid());
    }

    #[test]
    fn test_genesis_block_is_first() {
        let blockchain = Blockchain::new();
        assert_eq!(blockchain.chain[0].index, 0);
        assert_eq!(blockchain.chain[0].data, "Genesis Block - The Beginning of RustChain");
    }
}
