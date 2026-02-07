mod block;
mod blockchain;
mod crypto;
mod transaction;

use blockchain::Blockchain;

fn main() {
    println!("=== RustChain Day 3: Transactions ===\n");

    // Test 1: Create a blockchain with genesis block
    println!("--- Test 1: Create Blockchain with Genesis Block ---");
    let mut blockchain = Blockchain::new();
    blockchain.summary();
    print_separator();

    // Test 2: Display the genesis block
    println!("--- Test 2: Display Genesis Block ---");
    blockchain.display();
    print_separator();

    // Test 3: Add transactions to the pending pool
    println!("--- Test 3: Add Transactions to Pending Pool ---");
    println!("Adding transaction: Alice -> Bob : 10.0");
    blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();

    println!("Adding transaction: Bob -> Charlie : 5.0");
    blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();

    println!("Adding transaction: Charlie -> Dave : 2.0");
    blockchain.add_transaction(String::from("Charlie"), String::from("Dave"), 2.0).unwrap();

    println!("Pending transactions: {}", blockchain.pending_transaction_count());
    print_separator();

    // Test 4: Display pending transactions
    println!("--- Test 4: Display Pending Transactions ---");
    blockchain.display_pending_transactions();
    print_separator();

    // Test 5: Mine a block with pending transactions
    println!("--- Test 5: Mine Block with Pending Transactions ---");
    println!("Mining block #1...");
    blockchain.mine_block();
    println!("Block mined! Pending pool cleared: {}", blockchain.pending_transaction_count() == 0);
    print_separator();

    // Test 6: Display the full chain
    println!("--- Test 6: Display Full Blockchain ---");
    blockchain.display();
    print_separator();

    // Test 7: Add more transactions and mine another block
    println!("--- Test 7: Add More Transactions and Mine ---");
    blockchain.add_transaction(String::from("Dave"), String::from("Eve"), 1.5).unwrap();
    blockchain.add_transaction(String::from("Eve"), String::from("Frank"), 0.5).unwrap();
    println!("Added 2 more transactions. Pending: {}", blockchain.pending_transaction_count());
    blockchain.mine_block();
    println!("Block #2 mined!");
    print_separator();

    // Test 8: Verify block linking
    println!("--- Test 8: Verify Block Linking ---");
    println!("Genesis block hash: {}", blockchain.chain[0].hash);
    println!("Block #1 previous_hash: {}", blockchain.chain[1].previous_hash);
    println!("Links match: {}", blockchain.chain[0].hash == blockchain.chain[1].previous_hash);
    println!("Block #2 previous_hash: {}", blockchain.chain[2].previous_hash);
    println!("Links match: {}", blockchain.chain[1].hash == blockchain.chain[2].previous_hash);
    print_separator();

    // Test 9: Chain validation
    println!("--- Test 9: Chain Validation ---");
    println!("Chain is valid: {}", blockchain.is_valid());
    print_separator();

    // Test 10: Show blockchain summary
    println!("--- Test 10: Blockchain Summary ---");
    blockchain.summary();
    print_separator();

    // Test 11: Transaction validation
    println!("--- Test 11: Transaction Validation ---");
    println!("Attempting invalid transaction (zero amount):");
    let result = blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 0.0);
    println!("Result: {:?}", result);
    println!("\nAttempting invalid transaction (empty sender):");
    let result = blockchain.add_transaction(String::from(""), String::from("Bob"), 10.0);
    println!("Result: {:?}", result);
    println!("\nAttempting invalid transaction (self-transfer):");
    let result = blockchain.add_transaction(String::from("Alice"), String::from("Alice"), 10.0);
    println!("Result: {:?}", result);
    print_separator();

    // Test 12: Hash determinism with transactions
    println!("--- Test 12: Hash Determinism ---");
    println!("Each block has a unique hash:");
    for block in &blockchain.chain {
        println!("  Block #{} ({} tx): {}",
            block.index,
            block.transaction_count(),
            &block.hash[..16]
        );
    }
    print_separator();

    // Test 13: Chain structure visualization
    println!("--- Test 13: Chain Structure with Transactions ---");
    visualize_chain(&blockchain);
    print_separator();

    // Test 14: Mine empty block
    println!("--- Test 14: Mine Empty Block ---");
    println!("Mining block with no pending transactions...");
    blockchain.mine_block();
    println!("Block #3 mined with {} transactions", blockchain.chain[3].transaction_count());
    print_separator();

    println!("=== Day 3 Complete: Transactions ===");
    println!("Key observations:");
    println!("  Transactions are structured data (sender, receiver, amount)");
    println!("  Transactions wait in a mempool before being mined");
    println!("  Blocks can contain multiple transactions");
    println!("  Mining clears the pending pool and adds a block to the chain");
    println!("  Transaction validation prevents invalid data");
    println!("  Transaction order affects the block hash");
    println!("  The blockchain is now a proper ledger of value transfers");
}

fn visualize_chain(blockchain: &Blockchain) {
    println!("Block structure:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        if i == 0 {
            println!("  [Genesis] ({} tx) → hash: {}",
                block.transaction_count(),
                &block.hash[..16]
            );
        } else {
            println!("  [Block {}] ({} tx) ← prev: {} | hash: {}",
                block.index,
                block.transaction_count(),
                &block.previous_hash[..16],
                &block.hash[..16]
            );
        }
    }
}

fn print_separator() {
    println!();
}
