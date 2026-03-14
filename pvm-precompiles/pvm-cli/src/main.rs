//! PVM Precompiles CLI
//!
//! A developer utility for interacting with cryptographic precompiles
//! implemented for the PVM runtime.
//!
//! # Structure
//!
//! - `commands/` - Command handlers for each cryptographic primitive
//!   - `schnorr` - Schnorr signature operations
//!   - (future: `bls` - BLS12-381 operations)
//! - `utils` - Shared utility functions
//!
//! # Usage
//!
//! ```bash
//! pvmcli schnorr sign -s <SECRET_KEY> -m "message"
//! pvmcli schnorr verify -p <PUBKEY> -n <NONCE> -s <SIG> -m "message"
//! pvmcli schnorr test-data --secret-key <SECRET_KEY> --nonce <NONCE_SEED>
//! ```

use clap::{Parser, Subcommand};

mod commands;
mod utils;

use commands::schnorr::SchnorrCommands;

/// PVM Precompiles CLI - Developer utility for cryptographic precompiles.
#[derive(Parser)]
#[command(name = "pvmcli")]
#[command(about = "PVM Precompiles CLI - Developer utility for cryptographic precompiles")]
#[command(version = "0.1.0")]
#[command(author = "Jamal Jones")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

/// Top-level commands organized by cryptographic primitive.
#[derive(Subcommand)]
enum Commands {
    /// Schnorr signature operations (sign, verify, test-data)
    Schnorr {
        #[command(subcommand)]
        action: SchnorrCommands,
    },

    // Future commands:
    // /// BLS12-381 elliptic curve operations
    // Bls {
    //     #[command(subcommand)]
    //     action: BLSCommands,
    // },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Schnorr { action } => commands::schnorr::handle(action),
        // Future:
        // Commands::Bls { action } => commands::bls::handle(action),
    }
}
