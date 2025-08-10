use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::cli::{Cli, Commands, parse_embedding};
use crate::term_quiz_master::quiz_logic::Model;
use crate::term_quiz_master::term_entry::Question;
use crate::server::run_server;

use reqwest; // For client HTTP calls in Start/Stop/Take

use crate::handlers::match_prime_pattern; // NEW LINE

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
                let model = model_arc.lock().unwrap();
                if let Some(question) = model.get_question() {
                    println!("Question ID: {}", question.id);
                    println!("Question Text: {}", question.text);
                    println!("Current Embedding: {:?}
", question.embedding); // Added \n for consistency
                } else {
                    println!("No questions available.");
                }
            }

            // embedding_cli.rs commands
            Commands::Quiz => {
                println!("Starting embedding quiz...");
                let model = model_arc.lock().unwrap();
                if let Some(question) = model.get_question() {
                    println!("Question ID: {}", question.id);
                    println!("Question Text: {}", question.text);
                    println!("Current Embedding: {:?}
", question.embedding); // Added \n for consistency
                } else {
                    println!("No questions available.");
                }
            }
            Commands::Answer { question_id, submitted_embedding_str } => {
                println!("Submitting answer...");
                let submitted_embedding = parse_embedding(&submitted_embedding_str)
                    .expect("Invalid embedding format");
                let mut model = model_arc.lock().unwrap();
                if let Some(question) = model.questions.get_mut(*question_id) {
                    let distance = Question::calculate_distance(&question.embedding, &submitted_embedding);
                    let is_correct = distance < 0.1; // Threshold for correctness

                    if !is_correct {
                        model.update_embedding(*question_id, submitted_embedding.clone());
                    }
                    model.update_weight(*question_id, is_correct);
                    println!("Answer submitted for Question ID: {}", question_id);
                    println!("Correct: {}", is_correct);
                    if !is_correct {
                        println!("Embedding updated.");
                    }
                } else {
                    println!("Question ID {} not found.", question_id);
                }
            }

            // CRUD operations
            Commands::Insert { term, embedding_str } => {
                let submitted_embedding = parse_embedding(embedding_str)
                    .expect("Invalid embedding format");

                let mut model = model_arc.lock().unwrap();
                // Check if term already exists
                if model.questions.iter().any(|q| q.text == *term) {
                    println!("Term \"{}\" already exists. Use 'answer' command to update its embedding.", term);
                    return Ok(()); // FIXED
                }

                let new_id = model.questions.len(); // Simple sequential ID
                let new_question = Question {
                    id: new_id,
                    text: term.to_string(),
                    embedding: submitted_embedding,
                };

                model.questions.push(new_question);
                model.weights.push(1.0); // Give it a default weight
                model.save_embeddings().expect("Failed to save embeddings");

                println!("Successfully added new term \"{}\" with ID {}.", term, new_id);
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
                if terms.is_empty() {
                    println!("Please provide at least one term to query.");
                    return Ok(()); // FIXED
                }

                // Query for each individual term
                if terms.len() > 1 {
                    println!("--- Individual Term Queries ---");
                    for term_str in terms {
                        if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
                            println!("\nQuerying for individual term: \"{}\"", term_str);
                            println!("Current Embedding: {:?}
", question.embedding);

                            let similar_embeddings = model.find_similar_embeddings(&question.text);
                            if !similar_embeddings.is_empty() {
                                println!("Most Similar Embeddings:");
                                for sim_q in similar_embeddings {
                                    let distance = Question::calculate_distance(&question.embedding, &sim_q.embedding);
                                    println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}
", sim_q.id, sim_q.text, distance, sim_q.embedding);
                                }
                            } else {
                                println!("No similar embeddings found for \"{}\"\n", term_str);
                            }
                        }
                    }
                    println!("--- End Individual Term Queries ---
");
                }


                // Combined query (always perform if at least one term is found)
                let mut combined_embedding: Vec<f32> = vec![0.0; 8]; // Assuming 8-dimensional embeddings
                let mut found_terms_count = 0;

                for term_str in terms {
                    if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
                        for i in 0..8 {
                            combined_embedding[i] += question.embedding[i];
                        }
                        found_terms_count += 1;
                    }
                }

                if found_terms_count == 0 {
                    println!("No known terms found among the input. Cannot perform combined query.");
                    return Ok(()); // FIXED
                }

                // Average the combined embedding
                for i in 0..8 {
                    combined_embedding[i] /= found_terms_count as f32;
                }

                // Create a dummy question for the combined embedding to use find_similar_embeddings
                let combined_question = Question {
                    id: usize::MAX, // Use a dummy ID that won't conflict
                    text: format!("Combined({})", terms.join(", ")),
                    embedding: combined_embedding,
                };

                println!("--- Combined Query ---");
                println!("Combined Embedding for input terms: {:?}
", combined_question.embedding);

                let similar_embeddings = model.find_similar_embeddings(&combined_question.text);
                if !similar_embeddings.is_empty() {
                    println!("\nMost Similar Embeddings to combined query:");
                    for sim_q in similar_embeddings {
                        let distance = Question::calculate_distance(&combined_question.embedding, &sim_q.embedding);
                        println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}
", sim_q.id, sim_q.text, distance, sim_q.embedding);
                    }
                } else {
                    println!("No similar embeddings found for combined query.");
                }

                // Placeholder for RDF integration
                println!("\n--- RDF Information (Placeholder) ---");
                println!("(Would query RDF graph for information related to \"{}\")", terms.join(", "));

                // Placeholder for Documentation integration
                println!("\n--- Documentation Information (Placeholder) ---");
                println!("(Would search documentation for information related to \"{}\")", terms.join(", "));
            }

            // ListTerms operation
            Commands::ListTerms => {
                println!("Listing all terms and their embeddings:");
                let model = model_arc.lock().unwrap();
                for question in model.questions.iter() {
                    println!("ID: {}, Term: \"{}\", Embedding: {:?}
", question.id, question.text, question.embedding);
                }
            }
            Commands::QueryByEmbedding { embedding_str, top_n } => {
                println!("Querying for terms similar to: {}", embedding_str);
                let target_embedding = parse_embedding(embedding_str)
                    .expect("Invalid embedding format for query-by-embedding");

                let model = model_arc.lock().unwrap();
                let similar_terms = (*model).find_similar_terms_by_vector(&target_embedding, *top_n);

                if similar_terms.is_empty() {
                    println!("No similar terms found.");
                } else {
                    println!("Top {} Similar Terms:", top_n);
                    for (question, distance) in similar_terms {
                        println!("  - ID: {}, Term: \"{}\", Distance: {:.4}, Embedding: {:?}
", question.id, question.text, distance, question.embedding);
                    }
                }
            }
            Commands::MatchPrimePattern { table } => {
                match_prime_pattern::run_match_prime_pattern_command(model_arc.clone(), *table);
            }
        }
        Ok(())
    }
}
