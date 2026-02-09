mod attacks;
mod block;
mod blockchain;
mod cli;
mod crypto;
mod experiments;
mod transaction;
mod validation;
mod visualization;

use cli::Cli;
use std::env;

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Create CLI instance
    let mut cli = Cli::new();

    // Check if we're in interactive mode (no arguments) or single-command mode
    if args.len() <= 1 {
        // Interactive mode
        cli.run_interactive();
    } else {
        // Single command mode - skip the program name (args[0])
        cli.run_single_command(&args[1..]);
    }
}
