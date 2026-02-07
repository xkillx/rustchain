mod block;
mod blockchain;
mod crypto;

use blockchain::Blockchain;

fn main() {
    println!("=== RustChain Day 2: Blockchain Core ===\n");

    // Test 1: Create a blockchain with genesis block
    println!("--- Test 1: Create Blockchain with Genesis Block ---");
    let mut blockchain = Blockchain::new();
    blockchain.summary();
    print_separator();

    // Test 2: Display the genesis block
    println!("--- Test 2: Display Genesis Block ---");
    blockchain.display();
    print_separator();

    // Test 3: Add blocks to the chain
    println!("--- Test 3: Adding Blocks to Chain ---");
    println!("Adding block: 'Alice sent 10 BTC to Bob'");
    blockchain.add_block(String::from("Alice sent 10 BTC to Bob"));

    println!("Adding block: 'Bob sent 5 BTC to Charlie'");
    blockchain.add_block(String::from("Bob sent 5 BTC to Charlie"));

    println!("Adding block: 'Charlie sent 2 BTC to Dave'");
    blockchain.add_block(String::from("Charlie sent 2 BTC to Dave"));
    println!("Chain now has {} blocks", blockchain.len());
    print_separator();

    // Test 4: Display the full chain
    println!("--- Test 4: Display Full Blockchain ---");
    blockchain.display();
    print_separator();

    // Test 5: Verify block linking
    println!("--- Test 5: Verify Block Linking ---");
    println!("Genesis block hash: {}", blockchain.chain[0].hash);
    println!("Block 1 previous_hash: {}", blockchain.chain[1].previous_hash);
    println!("Links match: {}", blockchain.chain[0].hash == blockchain.chain[1].previous_hash);
    print_separator();

    // Test 6: Chain validation
    println!("--- Test 6: Chain Validation ---");
    println!("Chain is valid: {}", blockchain.is_valid());
    print_separator();

    // Test 7: Show blockchain summary
    println!("--- Test 7: Blockchain Summary ---");
    blockchain.summary();
    print_separator();

    // Test 8: Demonstrate hash uniqueness
    println!("--- Test 8: Hash Uniqueness ---");
    println!("Each block has a unique hash:");
    for block in &blockchain.chain {
        println!("  Block #{}: {}", block.index, &block.hash[..16]);
    }
    print_separator();

    // Test 9: Chain structure visualization
    println!("--- Test 9: Chain Structure ---");
    visualize_chain(&blockchain);
    print_separator();

    println!("=== Day 2 Complete: Blockchain Core ===");
    println!("Key observations:");
    println!("  Genesis block is automatically created on initialization");
    println!("  Each block cryptographically links to the previous block");
    println!("  previous_hash of block N equals hash of block N-1");
    println!("  Chain is append-only: blocks are added, never removed");
    println!("  Validation ensures all hash links are intact");
    println!("  The 'chain' in 'blockchain' comes from these hash links");
}

fn visualize_chain(blockchain: &Blockchain) {
    println!("Block structure:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        if i == 0 {
            println!("  [Genesis] → hash: {}", &block.hash[..16]);
        } else {
            println!("  [Block {}] ← prev: {} | hash: {}",
                block.index,
                &block.previous_hash[..16],
                &block.hash[..16]
            );
        }
    }
}

fn print_separator() {
    println!();
}
