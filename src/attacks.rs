//! Attack Simulation Module for RustChain
//!
//! This module provides educational attack simulations to demonstrate
//! blockchain security properties. All attacks are designed to FAIL
//! when detected by the validation system, teaching why blockchain
//! tampering is infeasible.
//!
//! # Educational Purpose
//! These attacks show:
//! - Why cryptographic linking prevents undetected tampering
//! - How proof-of-work makes rewriting history expensive
//! - Why cascading validation failures occur
//! - The computational cost of various attack attempts
//!
//! # Security Warning
//! These methods are for EDUCATIONAL PURPOSES ONLY.
//! In production blockchains, many of these capabilities would not exist.

use crate::blockchain::Blockchain;
use crate::transaction::Transaction;
use crate::validation::{self, ValidationError, ValidationResult};
use std::fmt;

/// Result of an attack simulation
#[derive(Debug, Clone)]
pub struct AttackResult {
    /// Name of the attack
    pub attack_name: String,
    /// Description of what the attack attempted
    pub description: String,
    /// Whether the attack was detected by validation
    pub detected: bool,
    /// Which validation check caught the attack (if any)
    pub detection_method: Option<String>,
    /// Educational explanation of why the attack failed
    pub explanation: String,
    /// Number of blocks affected by the attack
    pub blocks_affected: usize,
    /// The blockchain state after the attack (should be invalid)
    pub is_chain_valid: bool,
}

impl fmt::Display for AttackResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\n=== Attack: {} ===\n", self.attack_name)?;
        write!(f, "Description: {}\n", self.description)?;
        write!(f, "Detected: {}\n", if self.detected { "YES ✓" } else { "NO ✗" })?;

        if let Some(method) = &self.detection_method {
            write!(f, "Detection Method: {}\n", method)?;
        }

        write!(f, "Blocks Affected: {}\n", self.blocks_affected)?;
        write!(f, "Chain Valid After Attack: {}\n", if self.is_chain_valid { "Yes" } else { "No ✗" })?;
        write!(f, "\nEducational Note:\n  {}\n", self.explanation)?;

        Ok(())
    }
}

/// Available attack simulations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttackType {
    /// Modify transaction data in a block
    TransactionTampering,
    /// Replace block hash with fake valid-looking hash
    HashReplacement,
    /// Remove a block from the middle of the chain
    BlockRemoval,
    /// Insert a fake block into the chain
    BlockInsertion,
    /// Skip proof-of-work mining and use fake hash
    ProofOfWorkBypass,
    /// Modify the genesis block
    GenesisTampering,
    /// Modify block metadata only (not transactions)
    MetadataCorruption,
    /// Replace chain suffix with alternate valid chain
    ChainReplacement,
    /// Try to hide tampering by recalculating hashes
    HashRecalculation,
    /// Double spend attack simulation
    DoubleSpend,
}

impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AttackType::TransactionTampering => write!(f, "Transaction Tampering"),
            AttackType::HashReplacement => write!(f, "Hash Replacement"),
            AttackType::BlockRemoval => write!(f, "Block Removal"),
            AttackType::BlockInsertion => write!(f, "Block Insertion"),
            AttackType::ProofOfWorkBypass => write!(f, "Proof-of-Work Bypass"),
            AttackType::GenesisTampering => write!(f, "Genesis Tampering"),
            AttackType::MetadataCorruption => write!(f, "Metadata Corruption"),
            AttackType::ChainReplacement => write!(f, "Chain Replacement"),
            AttackType::HashRecalculation => write!(f, "Hash Recalculation"),
            AttackType::DoubleSpend => write!(f, "Double Spend"),
        }
    }
}

impl AttackType {
    /// Get all available attack types
    pub fn all() -> Vec<Self> {
        vec![
            Self::TransactionTampering,
            Self::HashReplacement,
            Self::BlockRemoval,
            Self::BlockInsertion,
            Self::ProofOfWorkBypass,
            Self::GenesisTampering,
            Self::MetadataCorruption,
            Self::ChainReplacement,
            Self::HashRecalculation,
            Self::DoubleSpend,
        ]
    }

    /// Get description of the attack
    pub fn description(&self) -> &str {
        match self {
            Self::TransactionTampering => {
                "Modifies transaction data (amounts, addresses) in an existing block"
            }
            Self::HashReplacement => {
                "Replaces a block's hash with a fake valid-looking hash"
            }
            Self::BlockRemoval => {
                "Removes a block from the middle of the chain, breaking continuity"
            }
            Self::BlockInsertion => {
                "Inserts a fake block into the chain, disrupting index sequencing"
            }
            Self::ProofOfWorkBypass => {
                "Skips proof-of-work mining by manually setting a hash that appears valid"
            }
            Self::GenesisTampering => {
                "Modifies the genesis block, the foundation of the entire chain"
            }
            Self::MetadataCorruption => {
                "Modifies block metadata (index, timestamp, nonce) while leaving transactions unchanged"
            }
            Self::ChainReplacement => {
                "Tries to replace chain suffix with alternate valid chain"
            }
            Self::HashRecalculation => {
                "Attempts to hide tampering by recalculating hashes for modified blocks"
            }
            Self::DoubleSpend => {
                "Simulates spending the same coins twice by modifying historical transactions"
            }
        }
    }
}

/// Attack simulator that runs various attacks on a blockchain
pub struct AttackSimulator {
    /// Original blockchain before attacks (for comparison)
    original_chain: Option<Blockchain>,
    /// Results from attack runs
    pub results: Vec<AttackResult>,
}

impl AttackSimulator {
    /// Create a new attack simulator
    pub fn new() -> Self {
        AttackSimulator {
            original_chain: None,
            results: Vec::new(),
        }
    }

    /// Save the original chain for comparison
    pub fn save_original(&mut self, blockchain: &Blockchain) {
        self.original_chain = Some(blockchain.clone());
    }

    /// Run a specific attack on a blockchain copy
    pub fn run_attack(&mut self, attack_type: AttackType, blockchain: &Blockchain) -> AttackResult {
        // Create a copy to attack
        let mut attacked_chain = blockchain.clone();

        let result = match attack_type {
            AttackType::TransactionTampering => {
                self.attack_transaction_tampering(&mut attacked_chain)
            }
            AttackType::HashReplacement => {
                self.attack_hash_replacement(&mut attacked_chain)
            }
            AttackType::BlockRemoval => {
                self.attack_block_removal(&mut attacked_chain)
            }
            AttackType::BlockInsertion => {
                self.attack_block_insertion(&mut attacked_chain)
            }
            AttackType::ProofOfWorkBypass => {
                self.attack_pow_bypass(&mut attacked_chain)
            }
            AttackType::GenesisTampering => {
                self.attack_genesis_tampering(&mut attacked_chain)
            }
            AttackType::MetadataCorruption => {
                self.attack_metadata_corruption(&mut attacked_chain)
            }
            AttackType::ChainReplacement => {
                self.attack_chain_replacement(&mut attacked_chain)
            }
            AttackType::HashRecalculation => {
                self.attack_hash_recalculation(&mut attacked_chain)
            }
            AttackType::DoubleSpend => {
                self.attack_double_spend(&mut attacked_chain)
            }
        };

        self.results.push(result.clone());
        result
    }

    /// Attack 1: Transaction Tampering
    /// Modify transaction amounts in an existing block
    fn attack_transaction_tampering(&self, blockchain: &mut Blockchain) -> AttackResult {
        // Need at least 2 blocks (genesis + 1 mined)
        if blockchain.len() < 2 {
            return AttackResult {
                attack_name: AttackType::TransactionTampering.to_string(),
                description: AttackType::TransactionTampering.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - chain too short".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        // Store original state
        let original_amount = blockchain.get_block(1)
            .and_then(|b| b.transactions.first())
            .map(|tx| tx.amount);

        // Tamper with transaction in block 1
        if let Some(block) = blockchain.get_block_mut(1) {
            if !block.transactions.is_empty() {
                block.transactions[0].amount = 999999.0;
            }
        }

        // Run validation to detect the attack
        let validation_result = validation::validate_chain(blockchain);
        let detected = !validation_result.is_valid;

        let detection_method = if detected {
            Some(validation_result.get_first_error().map(|e| {
                match e {
                    ValidationError::InvalidHash { .. } => "Hash Validation".to_string(),
                    _ => "Chain Validation".to_string(),
                }
            }).unwrap_or_else(|| "Chain Validation".to_string()))
        } else {
            None
        };

        AttackResult {
            attack_name: AttackType::TransactionTampering.to_string(),
            description: format!("Changed transaction amount from {:.2} to 999999.0 in block #1",
                original_amount.unwrap_or(0.0)),
            detected,
            detection_method,
            explanation: "When transaction data changes, the block's hash changes. \
                         Since the hash is stored in the block, validation detects the mismatch. \
                         This demonstrates how cryptographic linking makes data tampering detectable.".to_string(),
            blocks_affected: blockchain.len() - 1, // All blocks after tampered block
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 2: Hash Replacement
    /// Try to replace a block's hash with a fake one
    fn attack_hash_replacement(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 2 {
            return AttackResult {
                attack_name: AttackType::HashReplacement.to_string(),
                description: AttackType::HashReplacement.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - chain too short".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        let original_hash = blockchain.get_block(1).map(|b| b.hash.clone()).unwrap_or_default();

        // Replace with fake hash that looks valid (starts with zeros)
        blockchain.tamper_with_hash(1, "0000000000000000000000000000000000000000000000000000000000000000".to_string());

        let validation_result = validation::validate_chain(blockchain);
        let detected = !validation_result.is_valid;

        AttackResult {
            attack_name: AttackType::HashReplacement.to_string(),
            description: format!("Replaced block #1's hash with fake all-zeros hash\nOriginal: {}...", &original_hash[..16.min(original_hash.len())]),
            detected,
            detection_method: Some("Hash Validation - stored hash doesn't match computed hash".to_string()),
            explanation: "The stored hash must match the hash computed from the block's data. \
                         You cannot simply replace a hash - it's a cryptographic fingerprint of the block's contents. \
                         This demonstrates why hashes provide integrity guarantees.".to_string(),
            blocks_affected: 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 3: Block Removal
    /// Remove a block from the middle of the chain
    fn attack_block_removal(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 3 {
            return AttackResult {
                attack_name: AttackType::BlockRemoval.to_string(),
                description: AttackType::BlockRemoval.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - need at least 3 blocks".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        let removed_hash = blockchain.get_block(1).map(|b| b.hash.clone()).unwrap_or_default();
        let chain_len_before = blockchain.len();

        // Get genesis hash before mutable borrow
        let genesis_hash = blockchain.chain[0].hash.clone();

        // Remove block 1
        blockchain.chain.remove(1);

        // Try to fix by updating next block's previous_hash
        if blockchain.len() > 1 {
            if let Some(block) = blockchain.get_block_mut(1) {
                // This was block 2, now block 1 - try to point to genesis
                block.previous_hash = genesis_hash;
                block.index = 1;
            }
        }

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::BlockRemoval.to_string(),
            description: format!("Removed block #1 (hash: {}...) from chain of {} blocks",
                &removed_hash[..16.min(removed_hash.len())], chain_len_before),
            detected,
            detection_method: Some("Chain Link Validation - broken reference chain".to_string()),
            explanation: "Removing a block breaks the cryptographic chain. Each block contains \
                         the hash of the previous block. Removing one block invalidates all subsequent \
                         blocks because their previous_hash references become invalid. Even if you try \
                         to update references, the modified block's hash won't match.".to_string(),
            blocks_affected: chain_len_before - 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 4: Block Insertion
    /// Insert a fake block into the chain
    fn attack_block_insertion(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 2 {
            return AttackResult {
                attack_name: AttackType::BlockInsertion.to_string(),
                description: AttackType::BlockInsertion.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - chain too short".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        // Create a fake block
        let fake_block = crate::block::Block::new(
            1, // Will cause index conflict
            1234567890,
            vec![Transaction::new("Attacker".to_string(), "Victim".to_string(), 1000.0).unwrap()],
            blockchain.chain[0].hash.clone(),
            blockchain.difficulty,
        );

        let chain_len_before = blockchain.len();

        // Insert at position 1
        blockchain.chain.insert(1, fake_block);

        // Update indices of subsequent blocks (try to hide the attack)
        for i in 2..blockchain.chain.len() {
            if let Some(block) = blockchain.get_block_mut(i) {
                block.index = i as u64;
            }
        }

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::BlockInsertion.to_string(),
            description: format!("Inserted fake block at position 1 into chain of {} blocks", chain_len_before),
            detected,
            detection_method: Some("Hash Validation - chain link and index mismatches".to_string()),
            explanation: "Inserting a block shifts all subsequent block indices, breaking their \
                         hash calculations (index is part of the hash input). Even if you update \
                         indices, the hashes change, which breaks all subsequent chain links. \
                         This demonstrates why blockchains are append-only.".to_string(),
            blocks_affected: chain_len_before,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 5: Proof-of-Work Bypass
    /// Skip mining and set a hash that looks valid
    fn attack_pow_bypass(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 2 {
            return AttackResult {
                attack_name: AttackType::ProofOfWorkBypass.to_string(),
                description: AttackType::ProofOfWorkBypass.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - chain too short".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        let difficulty = blockchain.get_block(1).map(|b| b.difficulty).unwrap_or(0);

        // Create a hash that meets difficulty requirement but wasn't mined
        let fake_hash = "0".repeat(difficulty as usize) +
            &"a".repeat(64 - difficulty as usize);

        blockchain.tamper_with_hash(1, fake_hash.clone());

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::ProofOfWorkBypass.to_string(),
            description: format!("Set block #1's hash to {} (difficulty: {}) instead of mining",
                &fake_hash, difficulty),
            detected,
            detection_method: Some("Hash Validation - stored hash doesn't match computed hash".to_string()),
            explanation: "You cannot choose a hash - it must be computed from the block's data. \
                         The proof-of-work requires finding a nonce that produces a valid hash. \
                         Simply setting a hash that 'looks' valid doesn't work because validation \
                         recalculates the hash from the block data. This demonstrates why PoW makes \
                         rewriting history expensive - you must actually do the work.".to_string(),
            blocks_affected: 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 6: Genesis Tampering
    /// Try to modify the genesis block
    fn attack_genesis_tampering(&self, blockchain: &mut Blockchain) -> AttackResult {
        let original_hash = blockchain.get_block(0).map(|b| b.hash.clone()).unwrap_or_default();

        // Tamper with genesis block
        if let Some(block) = blockchain.get_block_mut(0) {
            block.timestamp = 999999999999;
        }

        let validation_result = validation::validate_chain(blockchain);
        let detected = !validation_result.is_valid;

        AttackResult {
            attack_name: AttackType::GenesisTampering.to_string(),
            description: format!("Modified genesis block timestamp (original hash: {}...)",
                &original_hash[..16.min(original_hash.len())]),
            detected,
            detection_method: Some("Hash Validation and Cascading Failure".to_string()),
            explanation: "The genesis block is the foundation of the entire chain. EVERY subsequent \
                         block contains a hash chain that leads back to genesis. Modifying genesis \
                         invalidates the entire chain because block 1's previous_hash no longer matches. \
                         This demonstrates why the genesis block is immutable - changing it requires \
                         recalculating the entire chain.".to_string(),
            blocks_affected: blockchain.len(), // Entire chain
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 7: Metadata Corruption
    /// Modify only block metadata, not transactions
    fn attack_metadata_corruption(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 2 {
            return AttackResult {
                attack_name: AttackType::MetadataCorruption.to_string(),
                description: AttackType::MetadataCorruption.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - chain too short".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        // Modify metadata only
        if let Some(block) = blockchain.get_block_mut(1) {
            let original_timestamp = block.timestamp;
            let original_nonce = block.nonce;

            block.timestamp = original_timestamp + 1000000;
            block.nonce = original_nonce + 9999;
        }

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::MetadataCorruption.to_string(),
            description: "Modified block #1's timestamp and nonce (left transactions unchanged)".to_string(),
            detected,
            detection_method: Some("Hash Validation - metadata changes affect hash".to_string()),
            explanation: "The block hash includes ALL block data: index, timestamp, transactions, \
                         previous_hash, and nonce. Changing ANY of these changes the hash. Even if \
                         you only modify metadata, the hash changes, and validation detects the mismatch. \
                         This demonstrates the avalanche effect - small input changes cause completely \
                         different outputs.".to_string(),
            blocks_affected: blockchain.len() - 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 8: Chain Replacement
    /// Try to replace chain suffix with alternate chain
    fn attack_chain_replacement(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 3 {
            return AttackResult {
                attack_name: AttackType::ChainReplacement.to_string(),
                description: AttackType::ChainReplacement.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - need at least 3 blocks".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        // Create alternate chain starting from block 0
        let mut alternate_chain = Blockchain::new();
        alternate_chain.set_difficulty(blockchain.difficulty);

        // Add different transaction
        alternate_chain.add_transaction("Alice".to_string(), "Eve".to_string(), 99999.0).unwrap();
        alternate_chain.mine_block();
        alternate_chain.add_transaction("Eve".to_string(), "Mallory".to_string(), 88888.0).unwrap();
        alternate_chain.mine_block();

        let original_len = blockchain.len();

        // Try to replace suffix starting from block 1
        blockchain.chain[1] = alternate_chain.chain[1].clone();
        blockchain.chain[1].previous_hash = blockchain.chain[0].hash.clone();

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::ChainReplacement.to_string(),
            description: format!("Replaced chain suffix with {} different blocks", alternate_chain.len() - 1),
            detected,
            detection_method: Some("Chain Link Validation - broken hash chain".to_string()),
            explanation: "To replace part of a chain, you need to maintain valid hash links. \
                         Since block 2 still points to the old block 1's hash, replacing block 1 \
                         breaks the link. You would need to recalculate ALL subsequent blocks \
                         with proof-of-work. This demonstrates the cost of rewriting history - \
                         you must re-mine everything after the change.".to_string(),
            blocks_affected: original_len - 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 9: Hash Recalculation
    /// Try to hide tampering by recalculating hashes
    fn attack_hash_recalculation(&self, blockchain: &mut Blockchain) -> AttackResult {
        if blockchain.len() < 3 {
            return AttackResult {
                attack_name: AttackType::HashRecalculation.to_string(),
                description: AttackType::HashRecalculation.description().to_string(),
                detected: false,
                detection_method: None,
                explanation: "Cannot run attack - need at least 3 blocks".to_string(),
                blocks_affected: 0,
                is_chain_valid: true,
            };
        }

        // Tamper with block 1
        if let Some(block) = blockchain.get_block_mut(1) {
            if !block.transactions.is_empty() {
                block.transactions[0].amount = 55555.0;
            }
            // Recalculate hash for THIS block only
            block.hash = block.calculate_hash();
        }

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::HashRecalculation.to_string(),
            description: "Modified block #1 transaction and recalculated its hash (but didn't update subsequent blocks)".to_string(),
            detected,
            detection_method: Some("Chain Link Validation - subsequent blocks still reference old hash".to_string()),
            explanation: "Recalculating the modified block's hash makes that block valid, but BLOCK 2 \
                         still has the old hash in its previous_hash field. You would need to update \
                         block 2's previous_hash, which changes block 2's hash, which block 3 references, \
                         and so on. This creates a cascading requirement to re-mine the entire chain \
                         after any modification. This is why 'going back in time' is computationally \
                         infeasible.".to_string(),
            blocks_affected: blockchain.len() - 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Attack 10: Double Spend
    /// Simulate spending the same coins twice
    fn attack_double_spend(&self, blockchain: &mut Blockchain) -> AttackResult {
        // Create a blockchain with a transaction
        blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0).unwrap();
        blockchain.mine_block();

        let original_tx_hash = blockchain.get_block(1)
            .and_then(|b| b.transactions.first())
            .map(|tx| format!("{}->{}:{:.2}", tx.sender, tx.receiver, tx.amount))
            .unwrap_or_default();

        // Now try to change the past to make Alice give to Carol instead
        if let Some(block) = blockchain.get_block_mut(1) {
            if !block.transactions.is_empty() {
                block.transactions[0].receiver = "Carol".to_string();
            }
        }

        let detected = !blockchain.is_valid();

        AttackResult {
            attack_name: AttackType::DoubleSpend.to_string(),
            description: format!("Double spend: Alice->Bob (10.0) changed to Alice->Carol (10.0)\nOriginal tx: {}", original_tx_hash),
            detected,
            detection_method: Some("Hash Validation - transaction data change detected".to_string()),
            explanation: "In a real blockchain network, a double spend requires creating an \
                         alternate fork of the chain. You would need to mine a competing chain \
                         that's longer than the current one. With sufficient proof-of-work difficulty, \
                         this becomes prohibitively expensive. This demonstrates why Bitcoin requires \
                         '6 confirmations' - waiting for 6 blocks makes double spends extremely \
                         expensive to attempt.".to_string(),
            blocks_affected: 1,
            is_chain_valid: blockchain.is_valid(),
        }
    }

    /// Run all attacks and return results
    pub fn run_all_attacks(&mut self, blockchain: &Blockchain) -> Vec<AttackResult> {
        let mut results = Vec::new();

        for attack_type in AttackType::all() {
            // Create fresh copy for each attack
            let mut chain_copy = blockchain.clone();

            let result = match attack_type {
                AttackType::TransactionTampering => {
                    self.attack_transaction_tampering(&mut chain_copy)
                }
                AttackType::HashReplacement => {
                    self.attack_hash_replacement(&mut chain_copy)
                }
                AttackType::BlockRemoval => {
                    self.attack_block_removal(&mut chain_copy)
                }
                AttackType::BlockInsertion => {
                    self.attack_block_insertion(&mut chain_copy)
                }
                AttackType::ProofOfWorkBypass => {
                    self.attack_pow_bypass(&mut chain_copy)
                }
                AttackType::GenesisTampering => {
                    self.attack_genesis_tampering(&mut chain_copy)
                }
                AttackType::MetadataCorruption => {
                    self.attack_metadata_corruption(&mut chain_copy)
                }
                AttackType::ChainReplacement => {
                    self.attack_chain_replacement(&mut chain_copy)
                }
                AttackType::HashRecalculation => {
                    self.attack_hash_recalculation(&mut chain_copy)
                }
                AttackType::DoubleSpend => {
                    self.attack_double_spend(&mut chain_copy)
                }
            };

            println!("{}", result);
            results.push(result);
        }

        self.results = results.clone();
        results
    }

    /// Generate summary report of all attacks
    pub fn generate_summary(&self) -> String {
        if self.results.is_empty() {
            return "No attack results available. Run attacks first.".to_string();
        }

        let detected_count = self.results.iter().filter(|r| r.detected).count();
        let total_count = self.results.len();

        let mut report = format!("\n╔════════════════════════════════════════════════════════╗\n");
        report.push_str(&format!("║           Attack Simulation Summary Report              ║\n"));
        report.push_str(&format!("╚════════════════════════════════════════════════════════╝\n\n"));
        report.push_str(&format!("Total Attacks Run:     {}\n", total_count));
        report.push_str(&format!("Attacks Detected:     {} / {} ({:.0}%)\n",
            detected_count, total_count,
            (detected_count as f64 / total_count as f64) * 100.0));

        if detected_count == total_count {
            report.push_str("\n✓ ALL ATTACKS SUCCESSFULLY DETECTED!\n");
            report.push_str("The blockchain validation system is working correctly.\n");
        } else {
            report.push_str("\n✗ WARNING: Some attacks were not detected!\n");
            report.push_str("This may indicate a security vulnerability.\n");
        }

        report.push_str("\n─────────────────────────────────────────────────────────────\n");
        report.push_str("Individual Attack Results:\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");

        for (i, result) in self.results.iter().enumerate() {
            report.push_str(&format!("\n{}. {}\n", i + 1, result.attack_name));
            report.push_str(&format!("   Detected: {}\n", if result.detected { "✓ YES" } else { "✗ NO" }));
            report.push_str(&format!("   Blocks Affected: {}\n", result.blocks_affected));
        }

        report.push_str("\n─────────────────────────────────────────────────────────────\n");
        report.push_str("Key Takeaways:\n");
        report.push_str("─────────────────────────────────────────────────────────────\n");
        report.push_str("• Cryptographic hashing makes any data change detectable\n");
        report.push_str("• Chain linking creates cascading validation failures\n");
        report.push_str("• Proof-of-work makes rewriting history computationally expensive\n");
        report.push_str("• Recent blocks are easier to attack than older blocks\n");
        report.push_str("• The cost of an attack grows exponentially with chain depth\n");

        report
    }
}

impl Default for AttackSimulator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_blockchain() -> Blockchain {
        let mut blockchain = Blockchain::new();
        blockchain.add_transaction("Alice".to_string(), "Bob".to_string(), 10.0).unwrap();
        blockchain.mine_block();
        blockchain.add_transaction("Bob".to_string(), "Charlie".to_string(), 5.0).unwrap();
        blockchain.mine_block();
        blockchain
    }

    #[test]
    fn test_attack_transaction_tampering() {
        let blockchain = create_test_blockchain();
        let simulator = AttackSimulator::new();
        let result = simulator.attack_transaction_tampering(&mut blockchain.clone());

        assert!(result.detected);
        assert!(!result.is_chain_valid);
    }

    #[test]
    fn test_attack_hash_replacement() {
        let blockchain = create_test_blockchain();
        let simulator = AttackSimulator::new();
        let result = simulator.attack_hash_replacement(&mut blockchain.clone());

        assert!(result.detected);
        assert!(!result.is_chain_valid);
    }

    #[test]
    fn test_attack_genesis_tampering() {
        let blockchain = create_test_blockchain();
        let simulator = AttackSimulator::new();
        let result = simulator.attack_genesis_tampering(&mut blockchain.clone());

        assert!(result.detected);
        assert_eq!(result.blocks_affected, blockchain.len()); // Entire chain
    }

    #[test]
    fn test_all_attacks_detected() {
        let blockchain = create_test_blockchain();
        let mut simulator = AttackSimulator::new();
        let results = simulator.run_all_attacks(&blockchain);

        // All attacks should be detected
        let all_detected = results.iter().all(|r| r.detected);
        assert!(all_detected, "Not all attacks were detected");
    }

    #[test]
    fn test_attack_type_display() {
        assert_eq!(AttackType::TransactionTampering.to_string(), "Transaction Tampering");
        assert_eq!(AttackType::DoubleSpend.to_string(), "Double Spend");
    }

    #[test]
    fn test_attack_type_all() {
        let all = AttackType::all();
        assert_eq!(all.len(), 10);
    }
}
