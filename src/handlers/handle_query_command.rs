use crate::term_quiz_master::quiz_logic::Model;
use crate::term_quiz_master::term_entry::Question;

pub fn handle_query_command(model: &Model, terms: &[String]) {
    if terms.is_empty() {
        println!("Please provide at least one term to query.");
        return;
    }

    // Query for each individual term
    if terms.len() > 1 {
        println!("--- Individual Term Queries ---");
        for term_str in terms {
            if let Some(question) = model.questions.iter().find(|q| q.text == *term_str) {
                println!("\nQuerying for individual term: \"{}\"", term_str);
                println!("Current Embedding: {:?}\n", question.embedding);

                let similar_embeddings = model.find_similar_embeddings(&question.text);
                if !similar_embeddings.is_empty() {
                    println!("Most Similar Embeddings:");
                    for sim_q in similar_embeddings {
                        let distance = Question::calculate_distance(&question.embedding, &sim_q.embedding);
                        println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}\n", sim_q.id, sim_q.text, distance, sim_q.embedding);
                    }
                } else {
                    println!("No similar embeddings found for \"{}\".\n", term_str);
                }
            }
        }
        println!("--- End Individual Term Queries ---\n");
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
        return;
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
    println!("Combined Embedding for input terms: {:?}\n", combined_question.embedding);

    let similar_embeddings = model.find_similar_embeddings(&combined_question.text);
    if !similar_embeddings.is_empty() {
        println!("\nMost Similar Embeddings to combined query:");
        for sim_q in similar_embeddings {
            let distance = Question::calculate_distance(&combined_question.embedding, &sim_q.embedding);
            println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}\n", sim_q.id, sim_q.text, distance, sim_q.embedding);
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