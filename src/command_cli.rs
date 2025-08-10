use clap::{Parser, Subcommand};
use anyhow::Result;
use reqwest;
use tokio::process::Command; // For spawning background process

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the quiz server in the background
    Start,
    /// Stops the running quiz server
    Stop,
    /// Takes a quiz
    Take,
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Start => {
                println!("Starting quiz server...");
                // Spawn the quiz_server binary in the background
                Command::new("cargo")
                    .arg("run")
                    .arg("--package")
                    .arg("quiz_server")
                    .spawn()?; // Use spawn to run in background

                println!("Quiz server running.");
            }
            Commands::Stop => {
                println!("Stopping quiz server...");
                let client = reqwest::Client::new();
                match client.get("http://127.0.0.1:3000/stop").send().await {
                    Ok(response) => {
                        println!("Server response: {:?}", response.text().await?);
                    }
                    Err(e) => {
                        eprintln!("Error stopping server: {}", e);
                    }
                }
                println!("Quiz server stop signal sent.");
            }
            Commands::Take => {
                println!("Taking quiz...");
                let client = reqwest::Client::new();
                match client.get("http://127.0.0.1:3000/status").send().await {
                    Ok(response) => {
                        let status_text = response.text().await?;
                        println!("Quiz server status: {}", status_text);
                        if status_text.contains("running") {
                            println!("Server is running. Quiz logic will go here.");
                        } else {
                            println!("Server is not running. Please start it first.");
                        }
                    }
                    Err(e) => {
                        eprintln!("Error checking server status: {}", e);
                    }
                }
                println!("Quiz interaction finished.");
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}