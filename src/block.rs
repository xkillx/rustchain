use crate::crypto::calculate_hash;
use crate::transaction::Transaction;

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub hash: String,
}

impl Block {
    /// Creates a new block and calculates its hash
    pub fn new(index: u64, timestamp: u128, transactions: Vec<Transaction>, previous_hash: String) -> Self {
        let mut block = Block {
            index,
            timestamp,
            transactions,
            previous_hash,
            nonce: 0,
            hash: String::new(),
        };
        block.hash = block.calculate_hash();
        block
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

    /// Creates the genesis block (first block in the chain)
    pub fn genesis() -> Self {
        Block::new(
            0,
            0,
            Vec::new(), // Empty transactions for genesis block
            String::from("0"),
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
        );

        assert_eq!(block.index, 1);
        assert_eq!(block.timestamp, 1234567890);
        assert_eq!(block.transaction_count(), 0);
        assert_eq!(block.previous_hash, "previous_hash");
        assert_eq!(block.nonce, 0);
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
        );
        let block2 = Block::new(
            1,
            1234567890,
            vec![tx1, tx2],
            String::from("prev"),
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
        );
        let block2 = Block::new(
            1,
            1234567890,
            vec![tx2, tx1], // Different order
            String::from("prev"),
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
        );

        assert_ne!(block1.hash, block2.hash);
    }
}
