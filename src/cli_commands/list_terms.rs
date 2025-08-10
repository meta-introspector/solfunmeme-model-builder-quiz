use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;

pub async fn handle_list_terms_command(model_arc: Arc<Mutex<Model>>) -> Result<()> {
    println!("Listing all terms and their embeddings:");
    let model = model_arc.lock().unwrap();
    for question in model.questions.iter() {
        println!("ID: {}, Term: \"{}\", Embedding: {:?}", question.id, question.text, question.embedding);
    }
    Ok(())
}

