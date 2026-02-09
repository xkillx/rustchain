//! Security Experiments Module for RustChain
//!
//! This module provides experiments to understand blockchain security properties,
//! difficulty relationships, and the computational cost of various attacks.

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use std::time::{Duration, Instant};
use std::thread;

/// Result of a mining experiment
#[derive(Debug, Clone)]
pub struct MiningExperimentResult {
    /// Difficulty level tested
    pub difficulty: u32,
    /// Number of blocks mined
    pub blocks_mined: usize,
    /// Total time taken for all blocks
    pub total_time: Duration,
    /// Average time per block
    pub avg_time_per_block: Duration,
    /// Total nonces across all blocks
    pub total_nonces: u64,
    /// Average nonce per block
    pub avg_nonce: u64,
    /// Estimated hashes per second
    pub hashes_per_second: f64,
}

/// Result of a security cost calculation
#[derive(Debug, Clone)]
pub struct SecurityCostResult {
    /// Number of blocks to rewrite
    pub blocks_to_rewrite: usize,
    /// Current difficulty
    pub difficulty: u32,
    /// Estimated hashes needed per block
    pub estimated_hashes_per_block: u64,
    /// Total estimated hashes
    pub total_hashes: u64,
    /// Estimated time at given hashrate
    pub estimated_time: Duration,
    /// Estimated cost at given electricity rate
    pub estimated_cost: f64,
}

/// Result of a difficulty comparison experiment
#[derive(Debug, Clone)]
pub struct DifficultyComparisonResult {
    /// Difficulty levels tested
    pub difficulties: Vec<u32>,
    /// Average mining time for each difficulty
    pub avg_times: Vec<Duration>,
    /// Average nonces for each difficulty
    pub avg_nonces: Vec<u64>,
    /// Time increase factor (relative to lowest difficulty)
    pub time_increase_factor: f64,
    /// Security increase factor (exponential)
    pub security_increase_factor: f64,
}

/// Security experiment runner
pub struct SecurityExperiments {
    /// Test blockchain for experiments
    blockchain: Option<Blockchain>,
}

impl SecurityExperiments {
    /// Create a new security experiment runner
    pub fn new() -> Self {
        SecurityExperiments {
            blockchain: None,
        }
    }

    /// Create a test blockchain for experiments
    pub fn create_test_blockchain(&mut self, difficulty: u32, blocks: usize) -> &Blockchain {
        let mut blockchain = Blockchain::new();
        blockchain.set_difficulty(difficulty);

        for i in 0..blocks {
            blockchain.add_transaction(
                format!("User{}", i),
                format!("User{}", i + 1),
                10.0,
            ).unwrap();
            blockchain.mine_block();
        }

        self.blockchain = Some(blockchain);
        self.blockchain.as_ref().unwrap()
    }

    /// Experiment 1: Difficulty vs Mining Time
    /// Measure how difficulty affects mining time
    pub fn experiment_difficulty_vs_time(
        &self,
        max_difficulty: u32,
        blocks_per_difficulty: usize,
    ) -> DifficultyComparisonResult {
        let mut difficulties = Vec::new();
        let mut avg_times = Vec::new();
        let mut avg_nonces = Vec::new();

        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     Experiment: Difficulty vs Mining Time              ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        for difficulty in 1..=max_difficulty {
            let mut total_nonce = 0u64;
            let mut total_time = Duration::from_secs(0);

            println!("Testing difficulty {}...", difficulty);

            for block_num in 0..blocks_per_difficulty {
                let mut blockchain = Blockchain::new();
                blockchain.set_difficulty(difficulty);
                blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0).unwrap();

                let start = Instant::now();
                blockchain.mine_block();
                let duration = start.elapsed();

                let block = blockchain.get_latest_block();
                total_nonce += block.nonce;
                total_time += duration;

                println!("  Block {}: {}ms, nonce: {}",
                    block_num + 1,
                    duration.as_millis(),
                    block.nonce
                );
            }

            let avg_time = total_time / blocks_per_difficulty as u32;
            let avg_nonce = total_nonce / blocks_per_difficulty as u64;

            difficulties.push(difficulty);
            avg_times.push(avg_time);
            avg_nonces.push(avg_nonce);

            println!("  Average: {}ms, nonce: {}\n", avg_time.as_millis(), avg_nonce);
        }

        // Calculate increase factors
        let base_time = avg_times.first().map(|d| d.as_secs_f64()).unwrap_or(1.0);
        let time_increase = avg_times.last().map(|d| d.as_secs_f64() / base_time).unwrap_or(1.0);

        // Security increases exponentially with difficulty (16^diff)
        let security_increase = if max_difficulty > 1 {
            16_f64.powi(max_difficulty as i32 - 1) / 16_f64.powi(0)
        } else {
            1.0
        };

        println!("═════════════════════════════════════════════════════════");
        println!("Results Summary:");
        println!("  Time increase factor: {:.2}x", time_increase);
        println!("  Security increase factor: {:.0}x", security_increase);
        println!("  Each additional zero multiplies difficulty by ~16");
        println!("═════════════════════════════════════════════════════════\n");

        DifficultyComparisonResult {
            difficulties,
            avg_times,
            avg_nonces,
            time_increase_factor: time_increase,
            security_increase_factor: security_increase,
        }
    }

    /// Experiment 2: Calculate Attack Cost
    /// Estimate the computational cost of rewriting N blocks
    pub fn calculate_attack_cost(
        &self,
        blocks_to_rewrite: usize,
        difficulty: u32,
        hashrate_hashes_per_second: u64,
        electricity_rate_per_kwh: f64,
        power_consumption_watts: f64,
    ) -> SecurityCostResult {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     Attack Cost Calculation                            ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Parameters:");
        println!("  Blocks to rewrite:      {}", blocks_to_rewrite);
        println!("  Difficulty:             {} leading zeros", difficulty);
        println!("  Attacker hashrate:      {} hashes/second", hashrate_hashes_per_second);
        println!("  Electricity cost:       ${}/kWh", electricity_rate_per_kwh);
        println!("  Power consumption:      {} watts\n", power_consumption_watts);

        // Estimate hashes needed per block
        // On average, need to try 16^difficulty hashes
        let estimated_hashes_per_block = 16_u64.pow(difficulty);

        // For safety margin, multiply by 2 (could get lucky or unlucky)
        let estimated_hashes_per_block = estimated_hashes_per_block * 2;

        println!("Calculations:");
        println!("  Estimated hashes/block:  {}", format_number(estimated_hashes_per_block));
        println!("  Total hashes needed:     {}", format_number(estimated_hashes_per_block * blocks_to_rewrite as u64));

        // Calculate time
        let total_hashes = estimated_hashes_per_block * blocks_to_rewrite as u64;
        let estimated_seconds = total_hashes as f64 / hashrate_hashes_per_second as f64;
        let estimated_time = Duration::from_secs_f64(estimated_seconds);

        println!("  Estimated time:         {}", format_duration(estimated_time));

        // Calculate electricity cost
        let kilowatt_hours = (estimated_seconds / 3600.0) * (power_consumption_watts / 1000.0);
        let estimated_cost = kilowatt_hours * electricity_rate_per_kwh;

        println!("  Energy consumption:     {:.2} kWh", kilowatt_hours);
        println!("  Estimated cost:         ${:.2}\n", estimated_cost);

        // Compare with Bitcoin network
        println!("Real-world Context:");
        println!("  Bitcoin network hashrate: ~600 EH/s (600,000,000,000,000,000,000 hashes/sec)");
        println!("  Bitcoin difficulty:      Much higher than this simulation");
        println!("  Estimated cost to rewrite 6 recent blocks: $Billions");
        println!("\nThis is why Bitcoin is secure - the attack cost far exceeds potential gain!");

        println!("═════════════════════════════════════════════════════════\n");

        SecurityCostResult {
            blocks_to_rewrite,
            difficulty,
            estimated_hashes_per_block,
            total_hashes,
            estimated_time,
            estimated_cost,
        }
    }

    /// Experiment 3: Cascading Failure Demonstration
    /// Show how modifying one block affects all subsequent blocks
    pub fn demonstrate_cascading_failure(&self, chain_depth: usize) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     Experiment: Cascading Failure Demonstration       ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        let mut blockchain = Blockchain::new();
        blockchain.set_difficulty(2); // Low difficulty for faster demo

        // Create a chain
        println!("Creating blockchain with {} blocks...", chain_depth);
        for i in 0..chain_depth {
            blockchain.add_transaction(
                format!("User{}", i),
                format!("User{}", i + 1),
                10.0,
            ).unwrap();
            blockchain.mine_block();
        }

        println!("Blockchain created with {} blocks\n", blockchain.len());

        // Show initial state
        println!("Initial validation:");
        println!("  Chain valid: {}\n", blockchain.is_valid());

        // Modify block 1
        println!("Modifying block #1 (changing transaction amount from 10.0 to 999.0)...");
        if let Some(block) = blockchain.get_block_mut(1) {
            if !block.transactions.is_empty() {
                block.transactions[0].amount = 999.0;
            }
        }

        // Check each block
        println!("\nChecking each block's validity:");
        let mut invalid_count = 0;

        for i in 0..blockchain.len() {
            let is_valid = if i == 0 {
                // Genesis block
                if let Some(block) = blockchain.get_block(i) {
                    block.hash == block.calculate_hash()
                } else {
                    false
                }
            } else {
                // Other blocks
                if let Some(current) = blockchain.get_block(i) {
                    let hash_valid = current.hash == current.calculate_hash();

                    let prev = blockchain.get_block(i - 1).unwrap();
                    let link_valid = current.previous_hash == prev.hash;

                    hash_valid && link_valid
                } else {
                    false
                }
            };

            let status = if is_valid { "✓ Valid" } else { "✗ Invalid" };
            println!("  Block #{}: {}", i, status);

            if !is_valid {
                invalid_count += 1;
            }
        }

        println!("\nResult: {} out of {} blocks are invalid", invalid_count, blockchain.len());
        println!("\nExplanation:");
        println!("  • Block #1: Invalid because data changed but hash wasn't recalculated");
        println!("  • Blocks #2-{}: Invalid because their previous_hash references old block #1 hash",
            chain_depth);
        println!("  • This demonstrates the cascading effect - tampering with one block");
        println!("    breaks all subsequent blocks due to cryptographic linking.");

        println!("\nTo fix this, you would need to:");
        println!("  1. Recalculate block #1's hash");
        println!("  2. Update block #2's previous_hash");
        println!("  3. Recalculate block #2's hash");
        println!("  4. Update block #3's previous_hash");
        println!("  5. ... repeat for all {} blocks", chain_depth);
        println!("  6. Re-mine ALL blocks with proof-of-work");

        let mut test_chain = blockchain.clone();
        let remining_result = test_chain.remine_from(1);

        if let Ok(blocks_remined) = remining_result {
            println!("\nDemonstrating re-mining from block #1...");
            println!("  Blocks re-mined: {}", blocks_remined);
            println!("  Chain now valid: {}", test_chain.is_valid());
        }

        println!("═════════════════════════════════════════════════════════\n");
    }

    /// Experiment 4: Finality and Confirmations
    /// Demonstrate why transactions become more secure over time
    pub fn demonstrate_finality(&self, confirmations: usize) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     Experiment: Transaction Finality                 ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Understanding why Bitcoin waits for 6 confirmations...\n");

        let mut blockchain = Blockchain::new();
        blockchain.set_difficulty(2);

        // Add a transaction
        println!("1. Adding transaction: Alice -> Bob (10.0)");
        blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0).unwrap();
        blockchain.mine_block();

        let tx_block = blockchain.get_latest_block().index;
        println!("   Transaction included in block #{}\n", tx_block);

        // Add more blocks
        println!("2. Adding {} more blocks (confirmations)...", confirmations);
        for i in 0..confirmations {
            blockchain.add_transaction(
                format!("Miner{}", i),
                format!("Receiver{}", i),
                1.0,
            ).unwrap();
            blockchain.mine_block();
        }

        println!("   Current chain height: #{}\n", blockchain.get_latest_block().index);

        // Calculate attack cost at different depths
        println!("3. Attack cost analysis (rewriting blocks to double-spend):");

        let difficulty = blockchain.get_difficulty();
        let hashrate = 1_000_000_000.0; // 1 GH/s for calculation

        for depth in [0, 1, 3, 6, 10].iter() {
            if *depth <= tx_block as usize {
                let blocks_to_rewrite = (tx_block as usize - *depth) + 1;
                let hashes_per_block = 16_u64.pow(difficulty) as f64;
                let total_hashes = hashes_per_block * blocks_to_rewrite as f64;
                let seconds = total_hashes / hashrate;

                println!("   {} confirmation(s):  Rewrite {} blocks  (~{} with 1 GH/s)",
                    depth,
                    blocks_to_rewrite,
                    format_duration(Duration::from_secs_f64(seconds))
                );
            }
        }

        println!("\nKey Insights:");
        println!("  • 0 confirmations: Transaction in mempool (not yet in block)");
        println!("  • 1 confirmation:  Transaction in latest block (easy to attack)");
        println!("  • 6 confirmations: Transaction 6 blocks deep (requires 51% hashrate)");
        println!("  • More confirmations = exponentially more expensive to reverse");

        println!("\nThis is why merchants wait for confirmations:");
        println!("  • Low-value items:    0-1 confirmations (coffee, fast food)");
        println!("  • Medium-value items: 3-6 confirmations (electronics, online orders)");
        println!("  • High-value items:   6+ confirmations (cars, real estate)");

        println!("═════════════════════════════════════════════════════════\n");
    }

    /// Experiment 5: Longest Chain Rule
    /// Demonstrate chain reorganization
    pub fn demonstrate_longest_chain_rule(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     Experiment: Longest Chain Rule                    ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Understanding blockchain consensus through chain reorganization...\n");

        // Create main chain
        let mut main_chain = Blockchain::new();
        main_chain.set_difficulty(1);

        println!("Creating main chain:");
        for i in 0..5 {
            main_chain.add_transaction(
                format!("MainTx{}", i),
                format!("MainRx{}", i),
                10.0,
            ).unwrap();
            main_chain.mine_block();
            println!("  Mined block #{}: MainTx{}", i + 1, i);
        }

        println!("\nMain chain: {} blocks", main_chain.len());
        println!("Latest hash: {}...\n", &main_chain.get_latest_block().hash[..16]);

        // Create competing fork
        println!("Creating competing fork (attacker's chain):");
        let mut fork_chain = Blockchain::new();
        fork_chain.set_difficulty(1);

        // Fork starts from same genesis
        for i in 0..7 {
            fork_chain.add_transaction(
                format!("ForkTx{}", i),
                format!("ForkRx{}", i),
                10.0,
            ).unwrap();
            fork_chain.mine_block();
            println!("  Mined block #{}: ForkTx{}", i + 1, i);
        }

        println!("\nFork chain: {} blocks", fork_chain.len());
        println!("Latest hash: {}...\n", &fork_chain.get_latest_block().hash[..16]);

        // Apply longest chain rule
        println!("Applying longest chain rule:");
        println!("  Main chain length: {}", main_chain.len());
        println!("  Fork chain length:  {}", fork_chain.len());
        println!("  Winner: Fork chain (longer)\n");

        let before_replace = main_chain.get_latest_block().index;
        match main_chain.replace_chain(fork_chain) {
            Ok(_) => {
                println!("✓ Chain reorganized!");
                println!("  Before: chain ending at block #{}", before_replace);
                println!("  After:  chain ending at block #{}", main_chain.get_latest_block().index);
            }
            Err(e) => {
                println!("✗ Reorganization failed: {}", e);
            }
        }

        println!("\nReal-world implications:");
        println!("  • Miners always extend the longest valid chain");
        println!("  • To double-spend, you need to create a longer chain");
        println!("  • With >50% network hashrate, you can outpace honest miners");
        println!("  • This is the '51% attack' scenario");
        println!("  • Bitcoin mitigates this through distributed mining");

        println!("═════════════════════════════════════════════════════════\n");
    }

    /// Run all experiments
    pub fn run_all_experiments(&mut self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                                                           ║");
        println!("║     RustChain Security Experiments Suite               ║");
        println!("║                                                           ║");
        println!("╚════════════════════════════════════════════════════════╝");

        // Experiment 1: Difficulty vs Time
        self.experiment_difficulty_vs_time(4, 3);

        // Experiment 2: Attack Cost
        self.calculate_attack_cost(
            6,                           // Rewrite 6 blocks
            4,                           // Difficulty 4
            1_000_000_000,               // 1 GH/s (fast for demo)
            0.10,                        // $0.10 per kWh
            1000.0,                      // 1000 watts
        );

        // Experiment 3: Cascading Failure
        self.demonstrate_cascading_failure(5);

        // Experiment 4: Finality
        self.demonstrate_finality(6);

        // Experiment 5: Longest Chain
        self.demonstrate_longest_chain_rule();

        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║     All Experiments Complete!                          ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Key Takeaways:");
        println!("  1. Difficulty exponentially increases mining time");
        println!("  2. Attack cost grows with chain depth and difficulty");
        println!("  3. Tampering with any block breaks all subsequent blocks");
        println!("  4. Confirmations provide probabilistic finality");
        println!("  5. Longest chain rule enables consensus");
        println!("\nBlockchain security comes from:");
        println!("  • Cryptographic linking (integrity)");
        println!("  • Proof-of-work (cost to rewrite)");
        println!("  • Distributed consensus (no single point of trust)");
    }
}

impl Default for SecurityExperiments {
    fn default() -> Self {
        Self::new()
    }
}

/// Format a large number with commas
fn format_number(n: u64) -> String {
    if n >= 1_000_000_000 {
        format!("{:.2} billion", n as f64 / 1_000_000_000.0)
    } else if n >= 1_000_000 {
        format!("{:.2} million", n as f64 / 1_000_000.0)
    } else if n >= 1_000 {
        format!("{:.2} thousand", n as f64 / 1_000.0)
    } else {
        format!("{}", n)
    }
}

/// Format a duration in human-readable form
fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs_f64();

    if secs >= 86400.0 {
        format!("{:.2} days", secs / 86400.0)
    } else if secs >= 3600.0 {
        format!("{:.2} hours", secs / 3600.0)
    } else if secs >= 60.0 {
        format!("{:.2} minutes", secs / 60.0)
    } else if secs >= 1.0 {
        format!("{:.2} seconds", secs)
    } else if secs >= 0.001 {
        format!("{:.2} milliseconds", secs * 1000.0)
    } else {
        format!("{:.2} microseconds", secs * 1_000_000.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(100), "100");
        assert!(format_number(1_500).contains("thousand"));
        assert!(format_number(1_000_000).contains("million"));
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_secs(90)), "1.50 minutes");
        assert_eq!(format_duration(Duration::from_secs(3600)), "1.00 hours");
    }

    #[test]
    fn test_create_test_blockchain() {
        let mut experiments = SecurityExperiments::new();
        let blockchain = experiments.create_test_blockchain(2, 3);

        assert_eq!(blockchain.len(), 4); // Genesis + 3 blocks
        assert_eq!(blockchain.get_difficulty(), 2);
    }

    #[test]
    fn test_difficulty_experiment() {
        let experiments = SecurityExperiments::new();
        let result = experiments.experiment_difficulty_vs_time(2, 2);

        assert_eq!(result.difficulties.len(), 2);
        assert_eq!(result.avg_times.len(), 2);
    }

    #[test]
    fn test_attack_cost_calculation() {
        let experiments = SecurityExperiments::new();
        let result = experiments.calculate_attack_cost(3, 2, 1_000_000, 0.10, 1000.0);

        assert_eq!(result.blocks_to_rewrite, 3);
        assert_eq!(result.difficulty, 2);
        assert!(result.estimated_hashes_per_block > 0);
    }

    #[test]
    fn test_security_experiments_default() {
        let experiments = SecurityExperiments::default();
        assert!(experiments.blockchain.is_none());
    }
}
