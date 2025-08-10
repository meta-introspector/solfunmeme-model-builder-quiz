use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;

pub async fn handle_query_command(model_arc: Arc<Mutex<Model>>, terms: Vec<String>) -> Result<()> {
    println!("Querying terms: {:?}", terms);
    let model = model_arc.lock().unwrap();
    // Individual term matches
    println!("\n--- Individual Term Matches ---");
    for term_str in terms.iter() {
        if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
            println!("Term: \"{}\"", question.text); // Original line
            // Find similar embeddings for individual terms (if Model had find_similar_embeddings)
            // For now, just display the term itself
        } else {
            println!("Term: \"{}\" not found.", term_str);
        }
    }

    // Group match
    println!("\n--- Group Match ---");
    let mut combined_embedding: Vec<f32> = vec![0.0; 8]; // Assuming 8-dimensional embeddings
    let mut found_terms_count = 0;

    for term_str in terms.iter() {
        if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
            for i in 0..8 {
                combined_embedding[i] += question.embedding[i];
            }
            found_terms_count += 1;
        }
    }

    if found_terms_count > 0 {
        // Average the combined embedding
        for i in 0..8 {
            combined_embedding[i] /= found_terms_count as f32;
        }
        println!("Combined Embedding for group: {:?}", combined_embedding);

        // Find similar embeddings for the combined embedding
        // This requires a find_similar_embeddings method in Model
        // For now, just display the combined embedding
    } else {
        println!("No known terms found for group match.");
    }
    Ok(())
}