# RustChain

A minimal educational blockchain implemented in Rust to understand core blockchain concepts through hands-on implementation.

> **"Blockchain is not 'crypto'. A blockchain is a system for making history hard to change."**

## Purpose

RustChain demonstrates core blockchain concepts by building a functional blockchain from first principles. Through 7 days of progressive implementation, you'll learn:

- How cryptographic hashing creates tamper-evident ledgers
- Why proof-of-work makes rewriting history computationally expensive
- How validation ensures chain integrity
- Why attacks fail and what makes blockchains secure
- The relationship between difficulty, mining, and security

## Features

| Day | Feature | Description |
|-----|---------|-------------|
| 1 | **Block & Hash** | SHA-256 hashing, block structure, cryptographic fingerprints |
| 2 | **Blockchain Core** | Genesis block, chain structure, block linking |
| 3 | **Transactions** | Transaction validation, mempool, pending transactions |
| 4 | **Proof-of-Work** | Mining with adjustable difficulty, nonce discovery |
| 5 | **Validation** | Chain integrity verification, tamper detection |
| 6 | **CLI Interface** | Interactive command-line interface for blockchain operations |
| 7 | **Attack Simulation** | 10 attack types, security experiments, educational visualizations |

## Getting Started

### Prerequisites

- Rust 1.70+ edition 2024
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/your-username/rustchain.git
cd rustchain

# Build the project
cargo build --release

# Run the CLI
cargo run --release
```

## Usage

### Interactive Mode

```bash
cargo run
```

### Single Commands

```bash
# Add a transaction
cargo run -- add Alice Bob 10.5

# Mine a block
cargo run -- mine

# Display the blockchain
cargo run -- chain --full

# Validate chain integrity
cargo run -- validate
```

### CLI Commands

#### Transaction Commands
```bash
add <sender> <receiver> <amount>   Add a new transaction
pending                              Show pending transactions
balance <address>                   Show balance for address
```

#### Mining Commands
```bash
mine                                 Mine a new block
difficulty <N>                       Set mining difficulty (1-6)
```

#### Display Commands
```bash
chain [--full] [--last N]            Display blockchain
validate                              Validate chain integrity
visualize                             Display blockchain visualization
stats                                 Show blockchain statistics
```

#### Day 7: Attack Simulation
```bash
attack list                           List available attacks
attack run <name>                     Run a specific attack
attack all                            Run all attack simulations
attack report                         Show attack results summary
```

#### Day 7: Security Experiments
```bash
experiment <type>                     Run security experiment
  Types: difficulty, cost, cascade, finality, longest, all

learn [topic]                         Educational content
  Topics: difficulty, double-spend, lifecycle, pow
```

#### Storage Commands
```bash
save <path>                           Save blockchain to file
load <path>                           Load blockchain from file
```

#### Other
```bash
help                                  Show help message
exit                                  Exit interactive mode
```

### Aliases

```
a = add     m = mine     c = chain     v = validate
p = pending b = balance  h = help      q = exit
atk = attack   exp = experiment   viz = visualize
```

## Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_attack_transaction_tampering
```

**Test Coverage:** 81 tests passing

## Architecture

### Core Components

```
src/
├── main.rs           # Entry point, module declarations
├── blockchain.rs     # Blockchain struct, chain management
├── block.rs          # Block struct, hash calculation, mining
├── transaction.rs    # Transaction struct, validation
├── crypto.rs         # SHA-256 hashing utilities
├── validation.rs     # Chain validation, attack detection
├── cli.rs            # Command-line interface
├── attacks.rs        # Day 7: Attack simulations (10 types)
├── experiments.rs    # Day 7: Security experiments
└── visualization.rs  # Day 7: Educational visualizations
```

### Data Structures

#### Block
```rust
pub struct Block {
    pub index: u64,
    pub timestamp: u128,
    pub transactions: Vec<Transaction>,
    pub previous_hash: String,
    pub nonce: u64,
    pub difficulty: u32,
    pub hash: String,
}
```

#### Transaction
```rust
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: f64,
}
```

#### Blockchain
```rust
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub difficulty: u32,
    pub pending_transactions: Vec<Transaction>,
}
```

## Security Properties

### Why Blockchain Is Secure

1. **Cryptographic Hashing**
   - SHA-256 provides tamper-evident integrity
   - Avalanche effect: small changes completely different hash
   - Each block contains fingerprint of all previous blocks

2. **Proof-of-Work**
   - Mining requires computational work
   - Rewriting history requires redoing all work
   - Higher difficulty = exponentially more expensive

3. **Chain Linking**
   - Each block references previous block's hash
   - Tampering breaks all subsequent blocks
   - Cascading validation failures

4. **Economic Security**
   - Cost to attack >> potential gain
   - 51% attack requires majority of network power
   - Confirmations provide probabilistic finality

### Attack Simulations (Day 7)

The project includes 10 attack simulations that **all fail** when detected:

1. **Transaction Tampering** - Modifying transaction amounts
2. **Hash Replacement** - Faking block hashes
3. **Block Removal** - Deleting blocks from chain
4. **Block Insertion** - Adding fake blocks
5. **Proof-of-Work Bypass** - Skipping mining
6. **Genesis Tampering** - Modifying the foundation
7. **Metadata Corruption** - Changing block metadata
8. **Chain Replacement** - Trying to replace chain suffix
9. **Hash Recalculation** - Hiding tampering
10. **Double Spend** - Spending coins twice

Run `cargo run -- attack all` to see all attacks get detected!

## Learning Outcomes

After completing this project, you will understand:

- How cryptographic hashing enables data integrity
- Why proof-of-work creates economic security
- How chain linking prevents undetected tampering
- The computational cost of rewriting history
- Why recent blocks are easier to attack than old blocks
- How difficulty affects security and mining time
- The concept of finality through confirmations
- Why Bitcoin requires 6 confirmations for large payments
- The longest chain rule and blockchain reorganization
- Trust emerging from math, not authority

## Development Journey

### Day 1: Block & Hash
- Created `Block` struct with index, timestamp, transactions, previous_hash, nonce, hash
- Implemented SHA-256 hashing
- Learned about avalanche effect and hash determinism

### Day 2: Blockchain Core
- Created `Blockchain` struct managing blocks
- Implemented genesis block creation
- Added blocks to chain

### Day 3: Transactions
- Created `Transaction` struct with validation
- Implemented mempool (pending transactions)
- Added transactions to blocks during mining

### Day 4: Proof-of-Work
- Implemented mining loop with nonce incrementation
- Added adjustable difficulty (leading zeros requirement)
- Demonstrated computational work for security

### Day 5: Validation
- Implemented chain validation (hash, link, PoW checks)
- Added tamper detection
- Created detailed validation error reporting

### Day 6: CLI Interface
- Built interactive command-line interface
- Added commands for transactions, mining, display
- Implemented save/load functionality

### Day 7: Attack Simulation
- Implemented 10 attack types with detailed results
- Created security experiments
- Added educational visualizations
- All 81 tests passing

## Educational Value

> "We are not 'learning blockchain'. We are **re-creating Bitcoin's core idea from nothing**."

This project teaches that:
- Blockchain is cleverly chained cryptography, not magic
- Security comes from structure, not secrets
- Trust emerges from making cheating expensive
- This is **truth engineering** at its purest

## Configuration

### Difficulty Levels

| Difficulty | Leading Zeros | Avg Attempts | Security Level |
|-----------|---------------|--------------|----------------|
| 1 | 1 | ~16 | Low |
| 2 | 2 | ~256 | Low |
| 3 | 3 | ~4,096 | Medium |
| 4 | 4 | ~65,536 | Medium (default) |
| 5 | 5 | ~1,048,576 | High |
| 6 | 6 | ~16,777,216 | High |

## Example Session

```bash
$ cargo run
=== RustChain Day 7: Attack Simulation & Security ===
Type 'help' for available commands

rustchain> add Alice Bob 10.0
Transaction added: Alice -> Bob (10.0000)
Pending transactions: 1

rustchain> mine
Mining block #1 with 1 transaction(s)...
Block #1 mined successfully!
  Hash: 00003a8f2c1b9d4a...
  Nonce: 48231
  Transactions: 1
  Time: 12.5ms

rustchain> chain --last 2
=== Blockchain ===
Total blocks: 2
Difficulty: 4
Chain valid: Yes

Block #0 | Hash: 00005a2f... | Txs: 0
Block #1 | Hash: 00003a8f... | Txs: 1

rustchain> validate
Chain is VALID
All blocks have valid hashes, links, and proof-of-work.

rustchain> attack run "Transaction Tampering"
=== Attack: Transaction Tampering ===
Description: Changed transaction amount from 10.00 to 999999.0 in block #1
Detected: YES
Detection Method: Hash Validation
Blocks Affected: 1
Chain Valid After Attack: No

Educational Note:
  When transaction data changes, the block's hash changes. Since the hash
  is stored in the block, validation detects the mismatch. This demonstrates
  how cryptographic linking makes data tampering detectable.

rustchain> exit
Goodbye!
```

## Experiment Examples

### Running All Attacks

```bash
rustchain> attack all
Total Attacks Run:     10
Attacks Detected:     10 / 10 (100%)

ALL ATTACKS SUCCESSFULLY DETECTED!
The blockchain validation system is working correctly.
```

### Security Experiments

```bash
rustchain> experiment cascade
Modifying block #1 (changing transaction amount from 10.0 to 999.0)

Result: 4 out of 5 blocks are invalid

Explanation:
  Block #1: Invalid because data changed but hash wasn't recalculated
  Blocks #2-4: Invalid because their previous_hash references old block #1 hash
  This demonstrates the cascading effect...
```

## Contributing

This is an educational project. Feel free to:
- Report issues
- Suggest improvements
- Submit pull requests
- Share your learning journey

## License

This project is open source and available under the MIT License.

## Acknowledgments

Built with:
- **Rust** - Systems programming language
- **sha2** - SHA-256 implementation
- **serde** - Serialization framework
- **hex** - Hex encoding/decoding

## Further Learning

- [Bitcoin Whitepaper](https://bitcoin.org/bitcoin.pdf)
- [Mastering Bitcoin](https://github.com/bitcoinbook/bitcoinbook) by Andreas Antonopoulos
- [Rust Blockchain Projects](https://github.com/search?q=rust+blockchain)

---

**"Blockchain makes history hard to change."**

*Built for learning*
