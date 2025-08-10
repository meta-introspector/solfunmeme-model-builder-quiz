use anyhow::Result;
use clap::Parser;

// Module declarations
mod api_types;
mod api_handlers;
mod server;
mod cli;
pub mod handlers;
mod cli_run;
pub mod term_quiz_master;

// Main entry point
#[tokio::main]
async fn main() -> Result<()> {
    println!("CARGO_MANIFEST_DIR: {}", env!("CARGO_MANIFEST_DIR"));
    let cli = cli::Cli::parse();
    cli.run().await
}