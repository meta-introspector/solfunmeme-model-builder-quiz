use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::cli::{Cli, Commands, parse_embedding};
use crate::model_core::Model;
use crate::handlers::{handle_quiz_command, handle_answer_command, handle_query_command, handle_add_vector_command};
use crate::server::run_server;

use reqwest; // For client HTTP calls in Start/Stop/Take

impl Cli {
    pub async fn run(&self) -> Result<()> {
        let model_arc = Arc::new(Mutex::new(Model::new())); // Create a single Model instance
        match &self.command {
            // command_cli.rs commands
            Commands::Start => {
                println!("Starting quiz server...");
                run_server(model_arc.clone()).await; // Pass the model to the server
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
                let mut model = model_arc.lock().unwrap();
                handle_quiz_command(&mut model);
            }

            // embedding_cli.rs commands
            Commands::Quiz => {
                println!("Starting embedding quiz...");
                let mut model = model_arc.lock().unwrap();
                handle_quiz_command(&mut model);
            }
            Commands::Answer { question_id, submitted_embedding_str } => {
                println!("Submitting answer...");
                let submitted_embedding = parse_embedding(&submitted_embedding_str)
                    .expect("Invalid embedding format");
                let mut model = model_arc.lock().unwrap();
                handle_answer_command(&mut model, *question_id, submitted_embedding);
            }

            // CRUD operations
            Commands::Insert { term, embedding_str } => {
                let mut model = model_arc.lock().unwrap();
                handle_add_vector_command(&mut model, term, embedding_str);
            }
            Commands::Update { term_id, embedding_str } => {
                println!("Updating term ID: {} with embedding: {}", term_id, embedding_str);
                let submitted_embedding = parse_embedding(embedding_str)
                    .expect("Invalid embedding format");
                let mut model = model_arc.lock().unwrap();
                match model.update_term(*term_id, submitted_embedding) {
                    Ok(_) => println!("Term updated successfully."),
                    Err(e) => eprintln!("Error updating term: {}", e),
                }
            }
            Commands::Delete { term_id } => {
                println!("Deleting term ID: {}", term_id);
                let mut model = model_arc.lock().unwrap();
                match model.delete_term(*term_id) {
                    Ok(_) => println!("Term deleted successfully."),
                    Err(e) => eprintln!("Error deleting term: {}", e),
                }
            }

            // Query operation
            Commands::Query { terms } => {
                let model = model_arc.lock().unwrap();
                handle_query_command(&model, terms);
            }

            // ListTerms operation
            Commands::ListTerms => {
                println!("Listing all terms and their embeddings:");
                let model = model_arc.lock().unwrap();
                for question in model.questions.iter() {
                    println!("ID: {}, Term: \"{}\", Embedding: {:?}", question.id, question.text, question.embedding);
                }
            }
        }
        Ok(())
    }
}
