use clap::{Parser, Subcommand};

pub fn parse_embedding(s: &str) -> Result<Vec<f32>, String> {
    s.split(',')
        .map(|s| s.trim().parse::<f32>().map_err(|e| format!("Invalid float: {}", e)))
        .collect()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
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
    /// Get a quiz question
    Quiz,
    /// Submit an answer to a quiz question
    Answer {
        question_id: usize,
        submitted_embedding_str: String,
    },
}
