mod block;
mod crypto;

use block::Block;

fn main() {
    println!("=== RustChain Day 1: Block & Hash ===\n");

    // Test 1: Create a simple block
    println!("--- Test 1: Creating a Block ---");
    let block = Block::new(
        1,
        1738867200,
        String::from("Alice sent 10 BTC to Bob"),
        String::from("0000"),
    );
    print_block(&block);

    // Test 2: Create genesis block
    println!("\n--- Test 2: Genesis Block ---");
    let genesis = Block::genesis();
    print_block(&genesis);

    // Test 3: Hash Determinism
    println!("\n--- Test 3: Hash Determinism ---");
    println!("Creating two identical blocks...");
    let block_a = Block::new(1, 1000, String::from("Same data"), String::from("prev"));
    let block_b = Block::new(1, 1000, String::from("Same data"), String::from("prev"));

    println!("Block A hash: {}", block_a.hash);
    println!("Block B hash: {}", block_b.hash);
    println!("Hashes match: {}", block_a.hash == block_b.hash);

    // Test 4: Avalanche Effect
    println!("\n--- Test 4: Avalanche Effect ---");
    println!("Creating two blocks with 1 character difference...");
    let block_x = Block::new(1, 1000, String::from("Hello"), String::from("prev"));
    let block_y = Block::new(1, 1000, String::from("Hello!"), String::from("prev"));

    println!("Block X data: '{}', hash: {}", block_x.data, block_x.hash);
    println!("Block Y data: '{}', hash: {}", block_y.data, block_y.hash);
    println!("Hashes differ: {}", block_x.hash != block_y.hash);

    // Test 5: Hash Properties
    println!("\n--- Test 5: Hash Properties ---");
    println!("Hash length: {} characters (256 bits)", block.hash.len());
    println!("Hash format: SHA-256 = 64 hex characters");

    println!("\n=== Day 1 Complete: Block & Hash ===");
    println!("Key observations:");
    println!("  Deterministic: Same input produces same hash");
    println!("  Avalanche: Tiny change creates completely different hash");
    println!("  Fixed output: Always 64 hex characters");
    println!("  Foundation: Hashes enable tamper detection");
}

fn print_block(block: &Block) {
    println!("Block #{}", block.index);
    println!("  Timestamp: {}", block.timestamp);
    println!("  Data:      {}", block.data);
    println!("  Prev Hash: {}", block.previous_hash);
    println!("  Nonce:     {}", block.nonce);
    println!("  Hash:      {}", block.hash);
}
