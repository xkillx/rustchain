use crate::crypto::calculate_hash;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub data: String,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Creates a new block and calculates its hash
    pub fn new(index: u64, timestamp: u128, data: String, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            data,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
    }

    /// Calculates the hash of the block based on its contents
    pub fn calculate_hash(&self) -> String {
        let block_string = format!(
            "{}{}{}{}{}",
            self.index, self.timestamp, self.data, self.previous_hash, self.nonce
        );
        calculate_hash(&block_string)
    }

    /// Creates the genesis block (first block in the chain)
    pub fn genesis() -> Self {
        Block::new(
            0,
            0,
            String::from("Genesis Block - The Beginning of RustChain"),
            String::from("0"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_creation() {
        let block = Block::new(
            1,
            1234567890,
            String::from("Test data"),
            String::from("previous_hash"),
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.timestamp, 1234567890);
        assert_eq!(block.data, "Test data");
        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.nonce, 0);
        assert_ne!(block.hash, "");
    }

    #[test]
    fn test_genesis_block() {
        let genesis = Block::genesis();
        assert_eq!(genesis.index, 0);
        assert_eq!(genesis.previous_hash, "0");
        assert_ne!(genesis.hash, "");
    }

    #[test]
    fn test_hash_determinism() {
        let block1 = Block::new(
            1,
            1234567890,
            String::from("Test data"),
            String::from("prev"),
        );
        let block2 = Block::new(
            1,
            1234567890,
            String::from("Test data"),
            String::from("prev"),
        );

        assert_eq!(block1.hash, block2.hash);
    }

    #[test]
    fn test_avalanche_effect() {
        let block1 = Block::new(
            1,
            1234567890,
            String::from("Test data"),
            String::from("prev"),
        );
        let block2 = Block::new(
            1,
            1234567890,
            String::from("Test data."),
            String::from("prev"),
        );

        assert_ne!(block1.hash, block2.hash);
    }
}
