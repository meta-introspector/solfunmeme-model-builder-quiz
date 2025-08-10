use anyhow::Result;
use clap::Parser;

// Module declarations
mod model_core;
mod api_types;
mod api_handlers;
mod server;
mod cli;
pub mod handlers;
mod cli_run;
mod cli_commands; // Added this line

// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse(); // Use Cli from cli.rs
    cli.run().await // Call the run method on the Cli instance
}