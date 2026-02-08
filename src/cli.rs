//! CLI Module for RustChain
//! Provides command-line interface for interacting with the blockchain

use crate::blockchain::Blockchain;
use std::io::{self, Write};
use std::process;
use std::time::Instant;

/// CLI-specific errors
#[derive(Debug)]
pub enum CliError {
    InvalidCommand(String),
    MissingArgument(String),
    InvalidArgument(String),
    FileError(String),
    BlockchainError(String),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CliError::InvalidCommand(cmd) => write!(f, "Unknown command: '{}'. Type 'help' for available commands.", cmd),
            CliError::MissingArgument(arg) => write!(f, "Missing required argument: {}", arg),
            CliError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            CliError::FileError(msg) => write!(f, "File error: {}", msg),
            CliError::BlockchainError(msg) => write!(f, "Blockchain error: {}", msg),
        }
    }
}

/// CLI commands
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    /// Add a new transaction: add <sender> <receiver> <amount>
    AddTransaction { sender: String, receiver: String, amount: f64 },

    /// Mine a new block with pending transactions
    MineBlock,

    /// Display the blockchain
    ShowChain { full: bool, last_n: Option<usize>, block_n: Option<usize> },

    /// Validate blockchain integrity
    ValidateChain,

    /// Show pending transactions
    ShowPending,

    /// Show balance for an address
    ShowBalance { address: String },

    /// Set mining difficulty
    SetDifficulty { difficulty: u32 },

    /// Show blockchain statistics
    ShowStats,

    /// Save blockchain to file
    Save { path: String },

    /// Load blockchain from file
    Load { path: String },

    /// Display help information
    Help,

    /// Exit interactive mode
    Exit,
}

/// Command result
pub type CommandResult = Result<Option<String>, CliError>;

/// CLI interface
pub struct Cli {
    blockchain: Blockchain,
    command_history: Vec<String>,
    auto_save_path: Option<String>,
}

impl Cli {
    /// Create a new CLI instance
    pub fn new() -> Self {
        Cli {
            blockchain: Blockchain::new(),
            command_history: Vec::new(),
            auto_save_path: None,
        }
    }

    /// Create a new CLI instance with existing blockchain
    pub fn with_blockchain(blockchain: Blockchain) -> Self {
        Cli {
            blockchain,
            command_history: Vec::new(),
            auto_save_path: None,
        }
    }

    /// Parse command from string arguments
    pub fn parse_command(args: &[String]) -> Result<Command, CliError> {
        if args.is_empty() {
            return Err(CliError::InvalidCommand("".to_string()));
        }

        let command = &args[0].to_lowercase();

        match command.as_str() {
            "add" | "a" => {
                if args.len() < 4 {
                    return Err(CliError::MissingArgument(
                        "Usage: add <sender> <receiver> <amount>".to_string()
                    ));
                }
                let sender = args[1].clone();
                let receiver = args[2].clone();
                let amount = args[3].parse::<f64>()
                    .map_err(|_| CliError::InvalidArgument(
                        format!("Amount must be a valid number: {}", args[3])
                    ))?;

                if amount <= 0.0 {
                    return Err(CliError::InvalidArgument(
                        "Amount must be greater than zero".to_string()
                    ));
                }

                Ok(Command::AddTransaction { sender, receiver, amount })
            }

            "mine" | "m" => Ok(Command::MineBlock),

            "chain" | "c" => {
                let mut full = false;
                let mut last_n = None;
                let mut block_n = None;

                let mut i = 1;
                while i < args.len() {
                    match args[i].as_str() {
                        "--full" => full = true,
                        "--last" => {
                            if i + 1 >= args.len() {
                                return Err(CliError::MissingArgument(
                                    "--last requires a number".to_string()
                                ));
                            }
                            last_n = Some(args[i + 1].parse::<usize>()
                                .map_err(|_| CliError::InvalidArgument(
                                    format!("Invalid number for --last: {}", args[i + 1])
                                ))?);
                            i += 1;
                        }
                        "--block" => {
                            if i + 1 >= args.len() {
                                return Err(CliError::MissingArgument(
                                    "--block requires a number".to_string()
                                ));
                            }
                            block_n = Some(args[i + 1].parse::<usize>()
                                .map_err(|_| CliError::InvalidArgument(
                                    format!("Invalid number for --block: {}", args[i + 1])
                                ))?);
                            i += 1;
                        }
                        _ => {
                            return Err(CliError::InvalidArgument(
                                format!("Unknown flag: {}", args[i])
                            ));
                        }
                    }
                    i += 1;
                }

                Ok(Command::ShowChain { full, last_n, block_n })
            }

            "validate" | "v" => Ok(Command::ValidateChain),

            "pending" | "p" => Ok(Command::ShowPending),

            "balance" | "b" => {
                if args.len() < 2 {
                    return Err(CliError::MissingArgument(
                        "Usage: balance <address>".to_string()
                    ));
                }
                Ok(Command::ShowBalance { address: args[1].clone() })
            }

            "difficulty" | "diff" => {
                if args.len() < 2 {
                    return Err(CliError::MissingArgument(
                        "Usage: difficulty <N>".to_string()
                    ));
                }
                let difficulty = args[1].parse::<u32>()
                    .map_err(|_| CliError::InvalidArgument(
                        format!("Difficulty must be a number between 1-6: {}", args[1])
                    ))?;

                if difficulty < 1 || difficulty > 6 {
                    return Err(CliError::InvalidArgument(
                        "Difficulty must be between 1 and 6".to_string()
                    ));
                }

                Ok(Command::SetDifficulty { difficulty })
            }

            "stats" => Ok(Command::ShowStats),

            "save" => {
                if args.len() < 2 {
                    return Err(CliError::MissingArgument(
                        "Usage: save <path>".to_string()
                    ));
                }
                Ok(Command::Save { path: args[1].clone() })
            }

            "load" => {
                if args.len() < 2 {
                    return Err(CliError::MissingArgument(
                        "Usage: load <path>".to_string()
                    ));
                }
                Ok(Command::Load { path: args[1].clone() })
            }

            "help" | "h" | "?" => Ok(Command::Help),

            "exit" | "quit" | "q" => Ok(Command::Exit),

            _ => Err(CliError::InvalidCommand(command.clone())),
        }
    }

    /// Execute a command
    pub fn execute_command(&mut self, command: Command) -> CommandResult {
        match command {
            Command::AddTransaction { sender, receiver, amount } => {
                self.execute_add_transaction(sender, receiver, amount)
            }

            Command::MineBlock => {
                self.execute_mine_block()
            }

            Command::ShowChain { full, last_n, block_n } => {
                self.execute_show_chain(full, last_n, block_n)
            }

            Command::ValidateChain => {
                self.execute_validate_chain()
            }

            Command::ShowPending => {
                self.execute_show_pending()
            }

            Command::ShowBalance { address } => {
                self.execute_show_balance(address)
            }

            Command::SetDifficulty { difficulty } => {
                self.execute_set_difficulty(difficulty)
            }

            Command::ShowStats => {
                self.execute_show_stats()
            }

            Command::Save { path } => {
                self.execute_save(path)
            }

            Command::Load { path } => {
                self.execute_load(path)
            }

            Command::Help => {
                Ok(Some(Self::display_help()))
            }

            Command::Exit => {
                Ok(Some("Goodbye!".to_string()))
            }
        }
    }

    /// Execute add transaction command
    fn execute_add_transaction(&mut self, sender: String, receiver: String, amount: f64) -> CommandResult {
        // Validate inputs
        if sender.trim().is_empty() {
            return Err(CliError::InvalidArgument("Sender cannot be empty".to_string()));
        }
        if receiver.trim().is_empty() {
            return Err(CliError::InvalidArgument("Receiver cannot be empty".to_string()));
        }

        // Add transaction to blockchain
        self.blockchain.add_transaction(sender.clone(), receiver.clone(), amount)
            .map_err(|e| CliError::BlockchainError(e))?;

        let message = format!(
            "Transaction added: {} -> {} ({:.4})\nPending transactions: {}",
            sender,
            receiver,
            amount,
            self.blockchain.pending_transaction_count()
        );

        Ok(Some(message))
    }

    /// Execute mine block command
    fn execute_mine_block(&mut self) -> CommandResult {
        let pending_count = self.blockchain.pending_transaction_count();

        if pending_count == 0 {
            println!("Warning: No pending transactions. Mining empty block...");
        }

        println!("Mining block #{} with {} transaction(s)...",
            self.blockchain.len(),
            pending_count
        );

        let start = Instant::now();
        self.blockchain.mine_block();
        let duration = start.elapsed();

        let block = self.blockchain.get_latest_block();

        let message = format!(
            "Block #{} mined successfully!\n  Hash: {}...\n  Nonce: {}\n  Transactions: {}\n  Time: {:?}",
            block.index,
            &block.hash[..16.min(block.hash.len())],
            block.nonce,
            block.transaction_count(),
            duration
        );

        Ok(Some(message))
    }

    /// Execute show chain command
    fn execute_show_chain(&self, full: bool, last_n: Option<usize>, block_n: Option<usize>) -> CommandResult {
        if let Some(n) = block_n {
            // Show specific block
            if let Some(block) = self.blockchain.get_block(n) {
                let output = if full {
                    format!(
                        "Block #{}\n  Index: {}\n  Hash: {}\n  Previous: {}\n  Nonce: {}\n  Transactions: {}",
                        block.index,
                        block.index,
                        block.hash,
                        block.previous_hash,
                        block.nonce,
                        block.transaction_count()
                    )
                } else {
                    format!(
                        "Block #{} | Hash: {}... | Txs: {}",
                        block.index,
                        &block.hash[..16.min(block.hash.len())],
                        block.transaction_count()
                    )
                };
                return Ok(Some(output));
            } else {
                return Err(CliError::InvalidArgument(format!("Block #{} does not exist", n)));
            }
        }

        let blocks_to_show: Vec<_> = if let Some(n) = last_n {
            self.blockchain.chain.iter()
                .rev()
                .take(n)
                .collect()
        } else {
            self.blockchain.chain.iter().collect()
        };

        let mut output = format!("\n=== Blockchain ===\nTotal blocks: {}\nDifficulty: {}\nChain valid: {}\n\n",
            self.blockchain.len(),
            self.blockchain.get_difficulty(),
            self.blockchain.is_valid()
        );

        for block in blocks_to_show.into_iter().rev() {
            if full {
                output.push_str(&format!(
                    "Block #{}\n  Hash: {}\n  Previous: {}\n  Nonce: {}\n  Transactions: {}\n",
                    block.index,
                    block.hash,
                    block.previous_hash,
                    block.nonce,
                    block.transaction_count()
                ));

                for tx in &block.transactions {
                    output.push_str(&format!("    {}\n", tx));
                }
                output.push('\n');
            } else {
                output.push_str(&format!(
                    "Block #{} | Hash: {}... | Txs: {}\n",
                    block.index,
                    &block.hash[..16.min(block.hash.len())],
                    block.transaction_count()
                ));
            }
        }

        Ok(Some(output))
    }

    /// Execute validate chain command
    fn execute_validate_chain(&self) -> CommandResult {
        let is_valid = self.blockchain.is_valid();

        if is_valid {
            Ok(Some("Chain is VALID ✓\nAll blocks have valid hashes, links, and proof-of-work.".to_string()))
        } else {
            Ok(Some("Chain is INVALID ✗\nOne or more blocks have been tampered with.".to_string()))
        }
    }

    /// Execute show pending command
    fn execute_show_pending(&self) -> CommandResult {
        let pending = self.blockchain.get_pending_transactions();

        if pending.is_empty() {
            Ok(Some("No pending transactions".to_string()))
        } else {
            let mut output = format!("\n=== Pending Transactions ({}) ===\n", pending.len());
            for (i, tx) in pending.iter().enumerate() {
                output.push_str(&format!("  {}. {}\n", i + 1, tx));
            }
            Ok(Some(output))
        }
    }

    /// Execute show balance command
    fn execute_show_balance(&self, address: String) -> CommandResult {
        let balance = self.calculate_balance(&address);

        Ok(Some(format!(
            "Balance for '{}': {:.4}",
            address,
            balance
        )))
    }

    /// Execute set difficulty command
    fn execute_set_difficulty(&mut self, difficulty: u32) -> CommandResult {
        self.blockchain.set_difficulty(difficulty);

        Ok(Some(format!(
            "Mining difficulty set to {} (requires {} leading zeros)",
            difficulty,
            difficulty
        )))
    }

    /// Execute show stats command
    fn execute_show_stats(&self) -> CommandResult {
        let stats = format!(
            "\n=== Blockchain Statistics ===\n\
             Total blocks:           {}\n\
             Latest block:           #{}\n\
             Latest hash:            {}...\n\
             Pending transactions:   {}\n\
             Current difficulty:     {}\n\
             Chain valid:            {}",
            self.blockchain.len(),
            self.blockchain.get_latest_block().index,
            &self.blockchain.get_latest_block().hash[..16.min(self.blockchain.get_latest_block().hash.len())],
            self.blockchain.pending_transaction_count(),
            self.blockchain.get_difficulty(),
            if self.blockchain.is_valid() { "Yes ✓" } else { "No ✗" }
        );

        Ok(Some(stats))
    }

    /// Execute save command
    fn execute_save(&self, path: String) -> CommandResult {
        // Serialize blockchain to JSON
        let json = serde_json::to_string_pretty(&self.blockchain)
            .map_err(|e| CliError::FileError(format!("Serialization failed: {}", e)))?;

        // Write to file
        std::fs::write(&path, json)
            .map_err(|e| CliError::FileError(format!("Failed to write to '{}': {}", path, e)))?;

        Ok(Some(format!("Blockchain saved to '{}'", path)))
    }

    /// Execute load command
    fn execute_load(&mut self, path: String) -> CommandResult {
        // Read from file
        let json = std::fs::read_to_string(&path)
            .map_err(|e| CliError::FileError(format!("Failed to read from '{}': {}", path, e)))?;

        // Deserialize blockchain
        let blockchain: Blockchain = serde_json::from_str(&json)
            .map_err(|e| CliError::FileError(format!("Deserialization failed: {}", e)))?;

        // Validate loaded chain
        if !blockchain.is_valid() {
            return Err(CliError::FileError(
                "Loaded blockchain is invalid and cannot be used".to_string()
            ));
        }

        self.blockchain = blockchain;

        Ok(Some(format!("Blockchain loaded from '{}'", path)))
    }

    /// Calculate balance for an address
    fn calculate_balance(&self, address: &str) -> f64 {
        let mut balance = 0.0;

        for block in &self.blockchain.chain {
            for tx in &block.transactions {
                if tx.sender == address {
                    balance -= tx.amount;
                }
                if tx.receiver == address {
                    balance += tx.amount;
                }
            }
        }

        balance
    }

    /// Display help information
    fn display_help() -> String {
        format!(
            "\n=== RustChain CLI Help ===\n\n\
             Commands:\n\
             \n  Transaction Commands:\n\
                add <sender> <receiver> <amount>   Add a new transaction\n\
                pending                            Show pending transactions\n\
                balance <address>                  Show balance for address\n\
             \n  Mining Commands:\n\
                mine                               Mine a new block\n\
                difficulty <N>                     Set mining difficulty (1-6)\n\
             \n  Display Commands:\n\
                chain [--full] [--last N]          Display blockchain\n\
                          [--block N]                \n\
                stats                              Show blockchain statistics\n\
                validate                           Validate chain integrity\n\
             \n  Storage Commands:\n\
                save <path>                        Save blockchain to file\n\
                load <path>                        Load blockchain from file\n\
             \n  Other:\n\
                help                               Show this help message\n\
                exit                               Exit interactive mode\n\
             \n  Aliases:\n\
                a = add     m = mine     c = chain     v = validate\n\
                p = pending b = balance  h = help      q = exit\n\
             \nExamples:\n\
                add Alice Bob 10.5\n\
                mine\n\
                chain --full\n\
                chain --last 3\n\
                balance Alice\n\
             \n"
        )
    }

    /// Run interactive mode
    pub fn run_interactive(&mut self) {
        println!("\n=== RustChain Day 6: CLI Interface ===");
        println!("Type 'help' for available commands\n");

        loop {
            print!("rustchain> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Ok(0) => {
                    // EOF (Ctrl+D)
                    println!("\nGoodbye!");
                    break;
                }
                Ok(_) => {
                    let input = input.trim();
                    if input.is_empty() {
                        continue;
                    }

                    // Add to history
                    self.command_history.push(input.to_string());

                    // Parse command
                    let args: Vec<String> = input
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect();

                    match Self::parse_command(&args) {
                        Ok(command) => {
                            if command == Command::Exit {
                                println!("Goodbye!");
                                break;
                            }

                            match self.execute_command(command) {
                                Ok(Some(message)) => println!("{}", message),
                                Ok(None) => {}
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Error reading input: {}", e);
                    break;
                }
            }
        }
    }

    /// Run single command mode
    pub fn run_single_command(&mut self, args: &[String]) {
        match Self::parse_command(args) {
            Ok(command) => {
                match self.execute_command(command) {
                    Ok(Some(message)) => println!("{}", message),
                    Ok(None) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }

    /// Show command history
    pub fn show_history(&self) {
        println!("\n=== Command History ===");
        for (i, cmd) in self.command_history.iter().enumerate() {
            println!("  {}  {}", i + 1, cmd);
        }
    }

    /// Get reference to blockchain
    pub fn blockchain(&self) -> &Blockchain {
        &self.blockchain
    }

    /// Get mutable reference to blockchain
    pub fn blockchain_mut(&mut self) -> &mut Blockchain {
        &mut self.blockchain
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse command from arguments (skipping program name)
pub fn parse_args(args: &[String]) -> Result<Command, CliError> {
    if args.len() <= 1 {
        // No arguments provided
        return Err(CliError::InvalidCommand("".to_string()));
    }
    Cli::parse_command(&args[1..])
}
