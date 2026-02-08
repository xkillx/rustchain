mod block;
mod blockchain;
mod crypto;
mod transaction;
mod validation;

use blockchain::Blockchain;
use std::time::Instant;
use validation::validate_chain;

fn main() {
    println!("=== RustChain Day 5: Validation & Attacks ===\n");

    // Test 1: Create a valid blockchain and verify it
    println!("--- Test 1: Create Valid Blockchain ---");
    let mut blockchain = create_test_blockchain(3);
    println!("Created blockchain with {} blocks", blockchain.len());
    println!("Chain is valid: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 2: Modify block data - detect with hash verification
    println!("--- Test 2: Tamper with Block Data ---");
    println!("Original amount: {}", blockchain.chain[1].transactions[0].amount);
    println!("Modifying transaction amount in Block #1...");

    blockchain.tamper_with_transactions(1, vec![
        transaction::Transaction::new(
            String::from("Attacker"),
            String::from("Attacker2"),
            999999.0,
        ).unwrap(),
    ]);

    println!("New amount: {}", blockchain.chain[1].transactions[0].amount);
    println!("Chain is valid after tampering: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 3: Demonstrate cascading failures
    println!("--- Test 3: Cascading Validation Failures ---");
    let mut blockchain = create_test_blockchain(5);
    println!("Created blockchain with {} blocks", blockchain.len());
    println!("All blocks valid: {}", blockchain.is_valid());

    println!("\nTampering with Block #1 (early in chain)...");
    blockchain.chain[1].transactions[0].amount = 777.0;

    println!("Checking each block:");
    for i in 0..blockchain.len() {
        let block = &blockchain.chain[i];
        let computed_hash = block.calculate_hash();
        let hash_valid = block.hash == computed_hash;
        println!("  Block #{}: stored hash matches computed = {}", i, hash_valid);
        if !hash_valid {
            println!("    Stored:   {}", &block.hash[..16]);
            println!("    Computed: {}", &computed_hash[..16]);
        }
    }

    println!("\nKey insight: Changing Block #1 makes ALL subsequent blocks invalid!");
    print_separator();

    // Test 4: Modify block hash directly
    println!("--- Test 4: Direct Hash Tampering ---");
    let mut blockchain = create_test_blockchain(3);
    println!("Original Block #1 hash: {}", &blockchain.chain[1].hash[..16]);

    blockchain.tamper_with_hash(1, String::from("fake_hash_123456789"));
    println!("Tampered hash: {}", &blockchain.chain[1].hash[..16]);

    println!("Chain is valid: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 5: Break chain link (previous_hash)
    println!("--- Test 5: Break Chain Link ---");
    let mut blockchain = create_test_blockchain(4);
    println!("Block #2 points to Block #1: {}",
        blockchain.chain[2].previous_hash == blockchain.chain[1].hash);

    println!("Breaking the link by changing previous_hash...");
    blockchain.tamper_with_previous_hash(2, String::from("wrong_hash"));

    println!("Block #2 now points to: {}", &blockchain.chain[2].previous_hash);
    println!("Block #1 actual hash:    {}", &blockchain.chain[1].hash[..16]);

    println!("\nChain is valid: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 6: Swap blocks experiment
    println!("--- Test 6: Block Swap Attack ---");
    let mut blockchain = create_test_blockchain(3);
    println!("Original chain: Block #0 → Block #1 → Block #2");
    println!("Block #1 hash: {}", &blockchain.chain[1].hash[..16]);
    println!("Block #2 hash: {}", &blockchain.chain[2].hash[..16]);

    // Swap blocks 1 and 2
    println!("\nSwapping Block #1 and Block #2...");
    let temp = blockchain.chain[1].clone();
    blockchain.chain[1] = blockchain.chain[2].clone();
    blockchain.chain[2] = temp;

    println!("Chain is valid: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 7: Modify genesis block
    println!("--- Test 7: Genesis Block Tampering ---");
    let mut blockchain = Blockchain::new();
    blockchain.add_transaction(String::from("Alice"), String::from("Bob"), 10.0).unwrap();
    blockchain.mine_block();

    println!("Original genesis hash: {}", &blockchain.chain[0].hash[..16]);
    println!("Chain is valid: {}", blockchain.is_valid());

    println!("\nTampering with genesis block...");
    blockchain.chain[0].transactions.push(
        transaction::Transaction::new(
            String::from("Fake"),
            String::from("Genesis2"),
            1000.0,
        ).unwrap()
    );

    println!("Chain is valid: {}", blockchain.is_valid());

    let validation_result = validate_chain(&blockchain);
    validation_result.display_errors();
    print_separator();

    // Test 8: Re-mining cost demonstration
    println!("--- Test 8: Cost of Rewriting History ---");
    let mut blockchain = create_test_blockchain(5);
    blockchain.set_difficulty(2); // Lower difficulty for faster demo

    println!("Created {}-block chain", blockchain.len());
    println!("All blocks are valid: {}", blockchain.is_valid());

    // Tamper with block 1
    println!("\nTampering with Block #1...");
    blockchain.chain[1].transactions[0].amount = 999.0;
    println!("Chain is valid: {}", blockchain.is_valid());

    // Calculate how much work to fix it
    println!("\nTo fix the chain, we must re-mine blocks 1, 2, 3, 4");
    println!("This demonstrates why old transactions are hard to change:");

    let start = Instant::now();
    match blockchain.remine_from(1) {
        Ok(blocks_remined) => {
            let duration = start.elapsed();
            println!("Re-mined {} blocks in {:?}", blocks_remined, duration);
            println!("Chain is valid after re-mining: {}", blockchain.is_valid());
        }
        Err(e) => println!("Error re-mining: {}", e),
    }
    print_separator();

    // Test 9: Difficulty as security parameter
    println!("--- Test 9: Difficulty as Security Parameter ---");

    // Create chain with difficulty 1
    let mut blockchain_easy = create_test_blockchain(2);
    blockchain_easy.set_difficulty(1);
    blockchain_easy.chain[1].hash = blockchain_easy.chain[1].calculate_hash();
    let start = Instant::now();
    blockchain_easy.chain[1].mine_block();
    let time_easy = start.elapsed();
    println!("Difficulty 1 - mining time: {:?}", time_easy);

    // Create chain with difficulty 2
    let mut blockchain_medium = create_test_blockchain(2);
    blockchain_medium.set_difficulty(2);
    blockchain_medium.chain[1].hash = blockchain_medium.chain[1].calculate_hash();
    let start = Instant::now();
    blockchain_medium.chain[1].mine_block();
    let time_medium = start.elapsed();
    println!("Difficulty 2 - mining time: {:?}", time_medium);

    // Create chain with difficulty 3
    let mut blockchain_hard = create_test_blockchain(2);
    blockchain_hard.set_difficulty(3);
    blockchain_hard.chain[1].hash = blockchain_hard.chain[1].calculate_hash();
    let start = Instant::now();
    blockchain_hard.chain[1].mine_block();
    let time_hard = start.elapsed();
    println!("Difficulty 3 - mining time: {:?}", time_hard);

    println!("\nConclusion: Higher difficulty = exponentially more work to attack");
    print_separator();

    // Test 10: Chain comparison
    println!("--- Test 10: Chain Comparison ---");
    let mut blockchain1 = create_test_blockchain(3);
    let mut blockchain2 = create_test_blockchain(3);

    println!("Comparing two identical chains...");
    let diff = blockchain1.compare_chains(&blockchain2);
    println!("Blocks different: {}", diff.blocks_different);
    println!("First divergence: {:?}", diff.first_divergence);

    println!("\nComparing after modifying blockchain2...");
    blockchain2.add_transaction(String::from("New"), String::from("User"), 1.0).unwrap();
    blockchain2.mine_block();

    let diff = blockchain1.compare_chains(&blockchain2);
    println!("Blocks different: {}", diff.blocks_different);
    println!("First divergence: {:?}", diff.first_divergence);
    print_separator();

    // Test 11: Chain replacement (reorganization)
    println!("--- Test 11: Chain Replacement (Reorganization) ---");
    let mut blockchain1 = create_test_blockchain(2);
    let mut blockchain2 = create_test_blockchain(4); // Longer chain

    println!("Chain 1 length: {}", blockchain1.len());
    println!("Chain 2 length: {}", blockchain2.len());
    println!("Chain 2 is longer: {}", blockchain2.is_longer_than(&blockchain1));

    println!("\nAttempting to replace Chain 1 with Chain 2...");
    match blockchain1.replace_chain(blockchain2) {
        Ok(_) => {
            println!("Success! Chain replaced");
            println!("New length: {}", blockchain1.len());
            println!("Chain is valid: {}", blockchain1.is_valid());
        }
        Err(e) => println!("Failed: {}", e),
    }
    print_separator();

    // Test 12: Comprehensive attack scenario
    println!("--- Test 12: Comprehensive Attack Scenario ---");
    println!("Scenario: Attacker wants to change an old transaction");

    let mut blockchain = create_test_blockchain(5);
    println!("\n1. Created blockchain with {} blocks", blockchain.len());
    println!("   Chain is valid: {}", blockchain.is_valid());

    println!("\n2. Attacker modifies Block #1 (early transaction)");
    println!("   Changes amount from 10.0 to 999999.0");
    blockchain.chain[1].transactions[0].amount = 999999.0;
    println!("   Chain is valid: {}", blockchain.is_valid());

    println!("\n3. Attacker realizes they need to re-mine all subsequent blocks");
    println!("   They must re-mine blocks 1, 2, 3, 4 (4 blocks total)");

    let start = Instant::now();
    match blockchain.remine_from(1) {
        Ok(blocks_remined) => {
            let duration = start.elapsed();
            println!("\n4. Attacker successfully re-mines {} blocks", blocks_remined);
            println!("   Time taken: {:?}", duration);
            println!("   Chain is valid: {}", blockchain.is_valid());

            println!("\n5. However:");
            println!("   - Honest network has added more blocks during this time");
            println!("   - Attacker's chain is now SHORTER than honest chain");
            println!("   - Network rejects attacker's chain (longest chain rule)");
            println!("   - Attack failed!");
        }
        Err(e) => println!("Error: {}", e),
    }
    print_separator();

    // Summary
    println!("=== Day 5 Complete: Validation & Attacks ===");
    println!("\nKey Insights:");
    println!("  1. Any tampering is mathematically detectable");
    println!("  2. Changing one block breaks ALL subsequent blocks (cascading failure)");
    println!("  3. Fixing the chain requires re-mining all affected blocks");
    println!("  4. The cost of rewriting history grows with chain depth");
    println!("  5. Higher difficulty = exponentially more expensive attacks");
    println!("  6. Longest chain rule protects against historical tampering");
    println!("  7. Proof-of-work makes blockchain history 'hard to change'");
    println!("\nWhy Bitcoin is Secure:");
    println!("  - To change a transaction from 1 hour ago: re-mine ~6 blocks");
    println!("  - To change a transaction from 1 day ago: re-mine ~144 blocks");
    println!("  - Attacker needs >50% of network hash power to succeed");
    println!("  - This is why blockchain creates 'trust through mathematics'");
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

/// Helper function to create a test blockchain with multiple blocks
fn create_test_blockchain(num_blocks: usize) -> Blockchain {
    let mut blockchain = Blockchain::new();
    blockchain.set_difficulty(2); // Lower difficulty for faster testing

    for i in 1..num_blocks {
        blockchain.add_transaction(
            String::from("Alice"),
            String::from(&format!("User{}", i)),
            10.0,
        ).unwrap();
        blockchain.mine_block();
    }

    blockchain
}

fn print_separator() {
    println!();
}
