use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;
use crate::cli::parse_embedding;

pub async fn handle_insert_command(model_arc: Arc<Mutex<Model>>, term: String, embedding_str: String) -> Result<()> {
    println!("Inserting term: {} with embedding: {}", term, embedding_str);
    let submitted_embedding = parse_embedding(&embedding_str)
        .expect("Invalid embedding format");
    let mut model = model_arc.lock().unwrap();
    match model.insert_term(term.clone(), submitted_embedding) {
        Ok(_) => println!("Term inserted successfully."),
        Err(e) => eprintln!("Error inserting term: {}", e),
    }
    Ok(())
}
