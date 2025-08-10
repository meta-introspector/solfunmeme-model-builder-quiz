use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;
use crate::cli::parse_embedding;

pub async fn handle_update_command(model_arc: Arc<Mutex<Model>>, term_id: usize, embedding_str: String) -> Result<()> {
    println!("Updating term ID: {} with embedding: {}", term_id, embedding_str);
    let submitted_embedding = parse_embedding(&embedding_str)
        .expect("Invalid embedding format");
    let mut model = model_arc.lock().unwrap();
    match model.update_term(term_id, submitted_embedding) {
        Ok(_) => println!("Term updated successfully."),
        Err(e) => eprintln!("Error updating term: {}", e),
    }
    Ok(())
}
