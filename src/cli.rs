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

    /// Takes a quiz
    Quiz,
    /// Answers a quiz question
    Answer { question_id: usize, submitted_embedding_str: String },

    /// Inserts a new term and its embedding
    Insert { term: String, embedding_str: String },
    /// Updates an existing term's embedding
    Update { term_id: usize, embedding_str: String },
    /// Deletes a term
    Delete { term_id: usize },

    /// Queries the model for terms, providing individual and group matches
    Query { #[arg(num_args = 1..)] terms: Vec<String> },

    /// Lists all terms and their embeddings
    ListTerms,

    /// Queries the model for terms similar to a provided embedding vector
    QueryByEmbedding {
        embedding_str: String,
        #[arg(long, default_value_t = 5)]
        top_n: usize,
    },

    /// Finds terms whose embeddings match specific prime reciprocal patterns
    MatchPrimePattern {
        #[arg(long)]
        table: bool,
    },
}
