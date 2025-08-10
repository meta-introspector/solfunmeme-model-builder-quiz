use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;
use crate::handlers::handle_answer_command;
use crate::cli::parse_embedding;

pub async fn handle_answer_command_cli(model_arc: Arc<Mutex<Model>>, question_id: usize, submitted_embedding_str: String) -> Result<()> {
    println!("Submitting answer...");
    let submitted_embedding = parse_embedding(&submitted_embedding_str)
        .expect("Invalid embedding format");
    let mut model = model_arc.lock().unwrap();
    handle_answer_command(&mut model, question_id, submitted_embedding);
    Ok(())
}
