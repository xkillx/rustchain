mod block;
mod blockchain;
mod crypto;
mod transaction;

use blockchain::Blockchain;
use std::time::Instant;

fn main() {
    println!("=== RustChain Day 4: Proof of Work (Mining) ===\n");

    // Test 1: Create a blockchain and show difficulty
    println!("--- Test 1: Create Blockchain with Difficulty ---");
    let mut blockchain = Blockchain::new();
    println!("Default difficulty: {}", blockchain.get_difficulty());
    blockchain.summary();
    print_separator();

    // Test 2: Add transactions and mine with proof-of-work
    println!("--- Test 2: Mine Block with Proof-of-Work ---");
    blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
    blockchain.add_transaction(String::from("Bob"), String::from("Charlie"), 5.0).unwrap();

    println!("Mining block #1 with difficulty {}...", blockchain.get_difficulty());
    let start = Instant::now();
    blockchain.mine_block();
    let duration = start.elapsed();

    println!("Block mined in {:?}", duration);
    println!("Nonce: {}", blockchain.chain[1].nonce);
    println!("Hash starts with: {}", &blockchain.chain[1].hash[..blockchain.get_difficulty() as usize]);
    print_separator();

    // Test 3: Verify proof-of-work requirement
    println!("--- Test 3: Verify Proof-of-Work ---");
    let block = &blockchain.chain[1];
    println!("Block #1 hash: {}", block.hash);
    println!("Meets difficulty requirement: {}", block.hash.starts_with("0000"));
    println!("First 8 chars: {}", &block.hash[..8]);
    print_separator();

    // Test 4: Compare different difficulties
    println!("--- Test 4: Compare Different Difficulties ---");

    // Create a new blockchain with difficulty 1
    let mut blockchain_easy = Blockchain::new();
    blockchain_easy.set_difficulty(1);
    blockchain_easy.add_transaction(String::from("Test"), String::from("User"), 1.0).unwrap();

    println!("Mining with difficulty 1...");
    let start = Instant::now();
    blockchain_easy.mine_block();
    let duration_easy = start.elapsed();
    println!("Time: {:?}, Nonce: {}", duration_easy, blockchain_easy.chain[1].nonce);

    // Create a new blockchain with difficulty 2
    let mut blockchain_medium = Blockchain::new();
    blockchain_medium.set_difficulty(2);
    blockchain_medium.add_transaction(String::from("Test"), String::from("User"), 1.0).unwrap();

    println!("Mining with difficulty 2...");
    let start = Instant::now();
    blockchain_medium.mine_block();
    let duration_medium = start.elapsed();
    println!("Time: {:?}, Nonce: {}", duration_medium, blockchain_medium.chain[1].nonce);

    // Create a new blockchain with difficulty 3
    let mut blockchain_hard = Blockchain::new();
    blockchain_hard.set_difficulty(3);
    blockchain_hard.add_transaction(String::from("Test"), String::from("User"), 1.0).unwrap();

    println!("Mining with difficulty 3...");
    let start = Instant::now();
    blockchain_hard.mine_block();
    let duration_hard = start.elapsed();
    println!("Time: {:?}, Nonce: {}", duration_hard, blockchain_hard.chain[1].nonce);

    println!("\nObservations:");
    println!("  Higher difficulty = more work (higher nonce)");
    println!("  Difficulty 1 vs 2: {:.2}x more work", blockchain_medium.chain[1].nonce as f64 / blockchain_easy.chain[1].nonce as f64);
    println!("  Difficulty 2 vs 3: {:.2}x more work", blockchain_hard.chain[1].nonce as f64 / blockchain_medium.chain[1].nonce as f64);
    print_separator();

    // Test 5: Continue mining on main blockchain
    println!("--- Test 5: Mine Multiple Blocks ---");
    blockchain.add_transaction(String::from("Charlie"), String::from("Dave"), 2.0).unwrap();
    blockchain.add_transaction(String::from("Dave"), String::from("Eve"), 1.0).unwrap();

    println!("Mining block #2...");
    blockchain.mine_block();
    println!("Block #2 nonce: {}", blockchain.chain[2].nonce);
    println!("Block #2 hash: {}", &blockchain.chain[2].hash[..16]);

    blockchain.add_transaction(String::from("Eve"), String::from("Frank"), 0.5).unwrap();
    println!("Mining block #3...");
    blockchain.mine_block();
    println!("Block #3 nonce: {}", blockchain.chain[3].nonce);
    println!("Block #3 hash: {}", &blockchain.chain[3].hash[..16]);
    print_separator();

    // Test 6: Display full blockchain
    println!("--- Test 6: Display Full Blockchain ---");
    blockchain.display();
    print_separator();

    // Test 7: Verify chain with proof-of-work validation
    println!("--- Test 7: Chain Validation with Proof-of-Work ---");
    println!("Chain is valid: {}", blockchain.is_valid());
    println!("(This validates hashes, links, AND proof-of-work)");
    print_separator();

    // Test 8: Demonstrate tamper resistance
    println!("--- Test 8: Demonstrate Tamper Resistance ---");
    println!("Modifying Block #1 transaction amount...");
    blockchain.chain[1].transactions[0].amount = 999.0;
    println!("New transaction amount: {}", blockchain.chain[1].transactions[0].amount);

    // Recalculate hash to try to cover tracks
    let tampered_hash = blockchain.chain[1].calculate_hash();
    println!("If we recalculate: {}", &tampered_hash[..16]);
    println!("Original hash:     {}", &blockchain.chain[1].hash[..16]);
    println!("Hashes match: {}", tampered_hash == blockchain.chain[1].hash);

    // Check if chain is still valid
    println!("\nChain is valid after tampering: {}", blockchain.is_valid());
    println!("(Proof-of-work makes tampering detectable)");
    print_separator();

    // Test 9: Demonstrate re-mining cost
    println!("--- Test 9: Re-mining Cost Demonstration ---");
    println!("To fix the chain, we must re-mine the tampered block...");
    println!("Original nonce: {}", blockchain.chain[1].nonce);

    // Show that we'd need to find a new valid nonce
    let mut test_block = blockchain.chain[1].clone();
    test_block.hash = test_block.calculate_hash();
    println!("Tampered hash (invalid): {}", &test_block.hash[..8]);

    let start = Instant::now();
    test_block.mine_block();
    let duration = start.elapsed();

    println!("Re-mined in {:?} with nonce {}", duration, test_block.nonce);
    println!("New valid hash: {}", &test_block.hash[..8]);
    println!("\nConclusion: Re-mining is required to 'fix' tampering,");
    println!("but this is computationally expensive!");
    print_separator();

    // Test 10: Blockchain summary
    println!("--- Test 10: Blockchain Summary ---");
    blockchain.summary();
    print_separator();

    // Test 11: Difficulty management
    println!("--- Test 11: Dynamic Difficulty Adjustment ---");
    let mut blockchain_dynamic = Blockchain::new();
    println!("Starting difficulty: {}", blockchain_dynamic.get_difficulty());

    blockchain_dynamic.set_difficulty(2);
    println!("Adjusted to: {}", blockchain_dynamic.get_difficulty());

    blockchain_dynamic.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
    blockchain_dynamic.mine_block();
    println!("Mined block with difficulty {}", blockchain_dynamic.get_difficulty());
    println!("Hash starts with: {}", &blockchain_dynamic.chain[1].hash[..2]);
    print_separator();

    // Test 12: Chain structure visualization
    println!("--- Test 12: Chain Structure with Mining Info ---");
    visualize_chain_with_mining(&blockchain);
    print_separator();

    println!("=== Day 4 Complete: Proof of Work (Mining) ===");
    println!("Key observations:");
    println!("  Proof of Work requires computational effort to create blocks");
    println!("  The nonce proves work was done (brute-force search)");
    println!("  Higher difficulty = exponentially more work required");
    println!("  Mining makes tampering expensive (must re-mine)");
    println!("  Anyone can verify the proof instantly");
    println!("  This asymmetry (hard to create, easy to verify) secures the chain");
    println!("  Real blockchains adjust difficulty to maintain target block time");
}

fn visualize_chain_with_mining(blockchain: &Blockchain) {
    println!("Block structure with mining info:");
    for (i, block) in blockchain.chain.iter().enumerate() {
        if i == 0 {
            println!("  [Genesis] (difficulty: {}) → hash: {}",
                block.difficulty,
                &block.hash[..16]
            );
        } else {
            println!("  [Block {}] (difficulty: {}, nonce: {}) ← prev: {} | hash: {}",
                block.index,
                block.difficulty,
                block.nonce,
                &block.previous_hash[..16],
                &block.hash[..16]
            );
        }
    }
}

fn print_separator() {
    println!();
}
