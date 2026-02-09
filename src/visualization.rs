//! Visualization Module for RustChain
//!
//! This module provides ASCII art and display helpers for visualizing
//! blockchain state, attack results, and chain structures.

use crate::blockchain::Blockchain;
use crate::validation::ValidationResult;

/// Colors for terminal output (using ANSI codes)
#[allow(dead_code)]
pub mod colors {
    pub const RESET: &str = "\x1b[0m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    pub const BOLD: &str = "\x1b[1m";

    /// Red bold text for invalid/bad
    pub fn error(text: &str) -> String {
        format!("{}{}{}{}", RED, BOLD, text, RESET)
    }

    /// Green bold text for valid/good
    pub fn success(text: &str) -> String {
        format!("{}{}{}{}", GREEN, BOLD, text, RESET)
    }

    /// Yellow bold text for warnings
    pub fn warning(text: &str) -> String {
        format!("{}{}{}{}", YELLOW, BOLD, text, RESET)
    }

    /// Blue bold text for info
    pub fn info(text: &str) -> String {
        format!("{}{}{}{}", BLUE, BOLD, text, RESET)
    }

    /// Cyan bold text for headers
    pub fn header(text: &str) -> String {
        format!("{}{}{}{}", CYAN, BOLD, text, RESET)
    }
}

/// Visual representation of blockchain structure
pub struct BlockchainVisualizer {
    /// Whether to use colors
    pub use_colors: bool,
}

impl BlockchainVisualizer {
    /// Create a new visualizer
    pub fn new() -> Self {
        BlockchainVisualizer {
            use_colors: true,
        }
    }

    /// Create a visualizer without colors
    pub fn without_colors() -> Self {
        BlockchainVisualizer {
            use_colors: false,
        }
    }

    /// Display blockchain as ASCII art
    pub fn display_chain(&self, blockchain: &Blockchain) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                    Blockchain View                     ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        for (i, block) in blockchain.chain.iter().enumerate() {
            let is_valid = block.hash == block.calculate_hash();
            let status = if is_valid { "✓" } else { "✗" };
            let status_color = if is_valid { colors::GREEN } else { colors::RED };

            println!("{} Block #{} {}{}", status_color, status, colors::RESET, colors::header(&format!("(Diff: {})", block.difficulty)));
            println!("┌──────────────────────────────────────────────────────┐");
            println!("│ Hash:       {}...│", &block.hash[..32.min(block.hash.len())]);
            println!("│ Previous:   {}...│", &block.previous_hash[..32.min(block.previous_hash.len())]);
            println!("│ Nonce:      {:>50}│", block.nonce);
            println!("│ Time:       {:>50}│", block.timestamp);
            println!("│ Txs:        {:>50}│", block.transaction_count());

            if !block.transactions.is_empty() {
                println!("├──────────────────────────────────────────────────────┤");
                for tx in &block.transactions {
                    println!("│ {} → {} : {:>38.2}│",
                        tx.sender,
                        tx.receiver,
                        tx.amount
                    );
                }
            }
            println!("└──────────────────────────────────────────────────────┘");

            // Show chain link to next block
            if i < blockchain.chain.len() - 1 {
                println!("                         │");
                println!("                         ▼");
                println!("              (previous_hash)");
            }
        }

        // Show chain validity
        let chain_valid = blockchain.is_valid();
        let status_text = if chain_valid {
            colors::success("CHAIN VALID ✓")
        } else {
            colors::error("CHAIN INVALID ✗")
        };

        println!("\n═════════════════════════════════════════════════════════");
        println!("Status: {}", status_text);
        println!("Blocks:  {} | Difficulty: {} | Pending: {}",
            blockchain.len(),
            blockchain.get_difficulty(),
            blockchain.pending_transaction_count()
        );
        println!("═════════════════════════════════════════════════════════\n");
    }

    /// Display chain in compact format
    pub fn display_compact_chain(&self, blockchain: &Blockchain) {
        println!("\n┌─ Blockchain ({} blocks, difficulty {}) ──────────────┐",
            blockchain.len(),
            blockchain.get_difficulty()
        );

        for block in &blockchain.chain {
            let status = if block.hash == block.calculate_hash() { "✓" } else { "✗" };
            let hash_preview = &block.hash[..12.min(block.hash.len())];

            println!("│ {} #{} {}... [{} txs, nonce: {}] │",
                status,
                block.index,
                hash_preview,
                block.transaction_count(),
                block.nonce
            );
        }

        let valid = if blockchain.is_valid() { colors::success("Valid") } else { colors::error("Invalid") };
        println!("└────────────────────────────────────────────────────────┘");
        println!("Status: {} | Pending: {}\n", valid, blockchain.pending_transaction_count());
    }

    /// Display validation result with details
    pub fn display_validation_result(&self, result: &ValidationResult) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                 Validation Result                      ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        let status = if result.is_valid {
            colors::success("✓ CHAIN VALID")
        } else {
            colors::error("✗ CHAIN INVALID")
        };

        println!("Status: {}\n", status);

        if result.is_valid {
            println!("All blockchain validation checks passed:");
            println!("  ✓ Block hashes are correct");
            println!("  ✓ Chain links are intact");
            println!("  ✓ Proof-of-work is valid\n");
        } else {
            println!("Validation errors detected:\n");

            for (i, error) in result.errors.iter().enumerate() {
                let error_type = match error {
                    crate::validation::ValidationError::InvalidHash { .. } => "Hash Mismatch",
                    crate::validation::ValidationError::BrokenLink { .. } => "Broken Link",
                    crate::validation::ValidationError::InvalidProofOfWork { .. } => "Invalid PoW",
                    crate::validation::ValidationError::InvalidIndex { .. } => "Index Error",
                    crate::validation::ValidationError::InvalidGenesis { .. } => "Genesis Error",
                };

                println!("  {}. {}:", i + 1, colors::error(error_type));
                println!("     {}", error);
            }

            println!("\n{} {}\n",
                colors::warning("⚠ WARNING:"),
                "The blockchain has been tampered with or is corrupted."
            );
        }
    }

    /// Display attack comparison (before vs after)
    pub fn display_attack_comparison(
        &self,
        before: &Blockchain,
        after: &Blockchain,
        attack_name: &str,
    ) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║   Attack Simulation: {:34}║", attack_name);
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("┌─ BEFORE Attack ─────────────────────────────────────────┐");
        println!("│ Valid: {} │ Blocks: {} │ Hash: {}... │",
            if before.is_valid() { "✓" } else { "✗" },
            before.len(),
            &before.get_latest_block().hash[..12]
        );
        println!("└────────────────────────────────────────────────────────┘");

        println!("\n            │");
        println!("            ▼");
        println!("      ⚠ {} ⚠", attack_name);
        println!("            │");
        println!("            ▼\n");

        println!("┌─ AFTER Attack ──────────────────────────────────────────┐");
        println!("│ Valid: {} │ Blocks: {} │ Hash: {}... │",
            if after.is_valid() { "✓" } else { "✗" },
            after.len(),
            &after.get_latest_block().hash[..12.min(after.get_latest_block().hash.len())]
        );
        println!("└────────────────────────────────────────────────────────┘\n");

        // Find differences
        if before.len() == after.len() {
            let mut differences = Vec::new();
            for i in 0..before.len() {
                let b1 = before.get_block(i).unwrap();
                let b2 = after.get_block(i).unwrap();

                if b1.hash != b2.hash {
                    differences.push((i, "Hash changed"));
                }
                if b1.transactions != b2.transactions {
                    differences.push((i, "Transactions modified"));
                }
            }

            if !differences.is_empty() {
                println!("Changes detected:");
                for (block_num, change) in differences {
                    println!("  • Block #{}: {}", block_num, change);
                }
                println!();
            }
        }
    }

    /// Display cascading failure diagram
    pub fn display_cascading_failure(&self, tamper_block: usize, chain_len: usize) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║           Cascading Failure Visualization              ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Scenario: Block #{} has been tampered with\n", tamper_block);

        for i in 0..chain_len {
            if i == tamper_block {
                println!("  Block #{} {} TAMPERED ✗",
                    colors::error(&format!("#{}", i)),
                    colors::error("→")
                );
                println!("           ↓");
                println!("           (invalid hash)");
                println!("           ↓");
            } else if i > tamper_block {
                println!("  Block #{} {} INVALID ✗",
                    colors::error(&format!("#{}", i)),
                    colors::error("→")
                );
                println!("           ↓");
                println!("           (previous_hash mismatch)");
                if i < chain_len - 1 {
                    println!("           ↓");
                }
            } else {
                println!("  Block #{} {} Valid ✓",
                    colors::success(&format!("#{}", i)),
                    colors::success("→")
                );
                if i < tamper_block {
                    println!("           ↓");
                    println!("           (valid link)");
                    println!("           ↓");
                }
            }
        }

        println!("\nResult: {} blocks affected ({} out of {} total)\n",
            chain_len - tamper_block,
            chain_len - tamper_block,
            chain_len
        );

        println!("Why this happens:");
        println!("  1. Block #{} is modified → hash changes", tamper_block);
        println!("  2. Block #{}'s previous_hash still points to old block #{} hash",
            tamper_block + 1, tamper_block);
        println!("  3. This creates a mismatch → invalid chain");
        println!("  4. All subsequent blocks inherit this invalidity\n");
    }

    /// Display proof-of-work visualization
    pub fn display_pow_visualization(&self, block_index: u64, difficulty: u32, nonce: u64, hash: &str) {
        let target_zeros = "0".repeat(difficulty as usize);
        let hash_start = &hash[..(difficulty as usize).min(hash.len())];

        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║            Proof-of-Work Visualization                ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Block #{} - Difficulty: {} ({} leading zeros required)",
            block_index,
            difficulty,
            difficulty
        );

        println!("\nMining Process:");
        println!("  Target: Hash must start with '{}'\n", target_zeros);

        println!("  Attempted nonces: 0 → {} ({} attempts)", nonce, nonce + 1);

        let matches = if hash_start == target_zeros {
            colors::success("✓ MATCHES")
        } else {
            colors::error("✗ NO MATCH")
        };

        println!("\n  Result: {} {}...\n", matches, &hash[..32]);

        println!("What this means:");
        println!("  • The miner tried {} different nonces", nonce + 1);
        println!("  • Each attempt calculated a new hash");
        println!("  • Found a hash meeting the difficulty requirement");
        println!("  • This proves computational work was done\n");

        println!("Security Implication:");
        println!("  • To rewrite this block, you must redo all this work");
        println!("  • Higher difficulty = exponentially more work required");
        println!("  • This makes rewriting history prohibitively expensive\n");
    }

    /// Display difficulty comparison table
    pub fn display_difficulty_table(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║         Difficulty Level Comparison                    ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("┌──────────┬──────────────┬──────────────┬────────────┐");
        println!("│ Difficulty│  Zeros Req'd │ Avg Attempts │ Security   │");
        println!("├──────────┼──────────────┼──────────────┼────────────┤");

        let difficulties = [(0, "~1"), (1, "~16"), (2, "~256"), (3, "~4,096"),
            (4, "~65,536"), (5, "~1,048,576"), (6, "~16,777,216")];

        for (diff, attempts) in difficulties {
            let security = if diff == 0 { "None" }
            else if diff <= 2 { "Low" }
            else if diff <= 4 { "Medium" }
            else { "High" };

            println!("│    {:2}    │    {:2}        │ {:>12} │ {:>10} │",
                diff, diff, attempts, security
            );
        }

        println!("└──────────┴──────────────┴──────────────┴────────────┘\n");

        println!("Key Points:");
        println!("  • Each additional zero multiplies difficulty by ~16");
        println!("  • Difficulty 4 = ~65K attempts per block (reasonable)");
        println!("  • Difficulty 6 = ~17M attempts per block (secure)");
        println!("  • Bitcoin uses much higher difficulty (~70+ zeros equivalent)\n");
    }

    /// Display double spend diagram
    pub fn display_double_spend_scenario(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║            Double Spend Attack Scenario               ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("Scenario: Alice wants to double-spend 10 BTC\n");

        println!("Step 1: Alice → Bob (10 BTC)");
        println!("         │");
        println!("         ▼");
        println!("  [Block #100] ✓ Mined");
        println!("         │");
        println!("         ▼");
        println!("  [Block #101] ✓ Mined");
        println!("         │");
        println!("         ▼");
        println!("  [Block #102] ✓ Mined");
        println!("\n         Bob accepts payment (3 confirmations)\n");

        println!("─────────────────────────────────────────────────────────\n");

        println!("Step 2: Alice secretly creates fork");
        println!("         │");
        println!("         ├─ Original chain: ... → Block #100 → Block #101 → Block #102");
        println!("         │");
        println!("         └─ Fork chain:     ... → Block #100' (Alice→Carol)");
        println!("                                            │");
        println!("                                            ▼");
        println!("                                     Block #101'");
        println!("                                            │");
        println!("                                            ▼");
        println!("                                     Block #103'");
        println!("                                     Block #104'");
        println!("                                     Block #105'  ← Longer!");
        println!("\n         Network accepts longer chain (6 > 3 blocks)");
        println!("         Bob's transaction is replaced ✗\n");

        println!("─────────────────────────────────────────────────────────\n");

        println!("Why This Attack Fails in Practice:");
        println!("  1. Creating longer chain requires >50% network hashrate");
        println!("  2. Each block requires proof-of-work (expensive)");
        println!("  3. More confirmations = exponentially harder to reverse");
        println!("  4. Bitcoin network hashrate: ~600 exahashes/second");
        println!("  5. Cost to rewrite 6 blocks: billions of dollars\n");

        println!("Mitigation:");
        println!("  • Wait for more confirmations (6+ for large payments)");
        println!("  • Monitor for orphaned blocks");
        println!("  • Use payment channels with timelocks");
        println!("  • Accept finality after sufficient depth\n");
    }

    /// Display transaction lifecycle
    pub fn display_transaction_lifecycle(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║          Transaction Lifecycle                        ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("1. Creation");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ Alice creates transaction           │");
        println!("   │   → Sender: Alice                   │");
        println!("   │   → Receiver: Bob                   │");
        println!("   │   → Amount: 10.0                    │");
        println!("   └─────────────────────────────────────┘");
        println!("                  │");
        println!("                  ▼\n");

        println!("2. Broadcasting");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ Transaction broadcast to network    │");
        println!("   │ Added to mempool (pending)          │");
        println!("   │ Status: Unconfirmed                 │");
        println!("   └─────────────────────────────────────┘");
        println!("                  │");
        println!("                  ▼\n");

        println!("3. Mining");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ Miner picks up transaction          │");
        println!("   │ Adds to block candidate             │");
        println!("   │ Runs proof-of-work                  │");
        println!("   │ Finds valid nonce                   │");
        println!("   └─────────────────────────────────────┘");
        println!("                  │");
        println!("                  ▼\n");

        println!("4. Confirmation");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ Block broadcast to network          │");
        println!("   │ Other miners verify block           │");
        println!("   │ Block added to chain                │");
        println!("   │ Status: 1 Confirmation              │");
        println!("   └─────────────────────────────────────┘");
        println!("                  │");
        println!("                  ▼\n");

        println!("5. Finality (after more blocks)");
        println!("   ┌─────────────────────────────────────┐");
        println!("   │ 6+ blocks mined on top              │");
        println!("   │ Transaction deeply buried           │");
        println!("   │ Cost to reverse: very high          │");
        println!("   │ Status: Confirmed (Final)           │");
        println!("   └─────────────────────────────────────┘\n");

        println!("Risks at Each Stage:");
        println!("  Stage 1: No risk (transaction not yet public)");
        println!("  Stage 2: Double-spend possible (transaction unconfirmed)");
        println!("  Stage 3: Orphan risk (block might not become part of longest chain)");
        println!("  Stage 4: Low risk (1 confirmation, but chain could reorg)");
        println!("  Stage 5: Minimal risk (6+ confirmations = economic finality)\n");
    }

    /// Display comprehensive blockchain education summary
    pub fn display_education_summary(&self) {
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                                                           ║");
        println!("║        Blockchain Security: Key Learnings                ║");
        println!("║                                                           ║");
        println!("╚════════════════════════════════════════════════════════╝\n");

        println!("🔐 Core Security Properties:\n");
        println!("  1. Immutable Ledger");
        println!("     • Once written, history cannot be changed");
        println!("     • Any modification breaks cryptographic hashes");
        println!("     • Detectable through validation checks\n");

        println!("  2. Cryptographic Integrity");
        println!("     • SHA-256 hashes provide tamper evidence");
        println!("     • Avalanche effect: small changes → completely different hash");
        println!("     • Each block contains fingerprint of all previous blocks\n");

        println!("  3. Proof-of-Work");
        println!("     • Mining requires computational work");
        println!("     • Rewriting history requires redoing all work");
        println!("     • Higher difficulty = exponentially more expensive\n");

        println!("  4. Distributed Consensus");
        println!("     • Longest chain rule prevents forks");
        println!("     • 51% attack is only theoretical weakness");
        println!("     • Economic incentives align honest behavior\n");

        println!("─────────────────────────────────────────────────────────\n");

        println!("⚔️  Why Attacks Fail:\n");
        println!("  • Transaction Tampering: Hash mismatch detected");
        println!("  • Block Removal: Chain link break detected");
        println!("  • Hash Replacement: Computed hash doesn't match");
        println!("  • PoW Bypass: Validation recalcures hashes");
        println!("  • Genesis Modification: Entire chain invalidated\n");

        println!("─────────────────────────────────────────────────────────\n");

        println!("💡 Key Insights:\n");
        println!("  • Security comes from structure, not secrets");
        println!("  • Trust emerges from math, not authority");
        println!("  • Cost to attack >> potential gain");
        println!("  • Depth = Finality (confirmations matter)");
        println!("  • Blockchain is a 'Truth Engine'\n");

        println!("─────────────────────────────────────────────────────────\n");

        println!("📊 Difficulty vs Security:\n");
        self.display_difficulty_table();

        println!("═════════════════════════════════════════════════════════");
        println!("  'Blockchain makes history hard to change'             ");
        println!("           This is why it's revolutionary                ");
        println!("═════════════════════════════════════════════════════════\n");
    }
}

impl Default for BlockchainVisualizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visualizer_creation() {
        let viz = BlockchainVisualizer::new();
        assert!(viz.use_colors);

        let viz_no_color = BlockchainVisualizer::without_colors();
        assert!(!viz_no_color.use_colors);
    }

    #[test]
    fn test_visualizer_default() {
        let viz = BlockchainVisualizer::default();
        assert!(viz.use_colors);
    }

    #[test]
    fn test_format_colors() {
        assert!(colors::success("test").contains("32")); // Green
        assert!(colors::error("test").contains("31")); // Red
        assert!(colors::warning("test").contains("33")); // Yellow
    }
}
