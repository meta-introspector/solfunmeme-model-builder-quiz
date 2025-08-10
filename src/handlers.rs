use crate::model_core::{Model, Question};

pub fn handle_quiz_command(model: &mut Model) {
    if let Some(question) = model.get_question() {
        println!("Question ID: {}", question.id);
        println!("Question Text: {}", question.text);
        println!("Current Embedding: {:?}", question.embedding);
    } else {
        println!("No questions available.");
    }
}

pub fn handle_answer_command(model: &mut Model, question_id: usize, submitted_embedding: Vec<f32>) {
    if let Some(question) = model.questions.get_mut(question_id) {
        let distance = Model::calculate_distance(&question.embedding, &submitted_embedding);
        let is_correct = distance < 0.1; // Threshold for correctness

        if !is_correct {
            model.update_embedding(question_id, submitted_embedding.clone());
        }
        model.update_weight(question_id, is_correct);
        println!("Answer submitted for Question ID: {}", question_id);
        println!("Correct: {}", is_correct);
        if !is_correct {
            println!("Embedding updated.");
        }
    } else {
        println!("Question ID {} not found.", question_id);
    }
}

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

                let similar_embeddings = model.find_similar_embeddings(question);
                if !similar_embeddings.is_empty() {
                    println!("Most Similar Embeddings:");
                    for (sim_q, distance) in similar_embeddings {
                        println!("  - ID: {}, Text: {}, Distance: {:.4}, Embedding: {:?}\n", sim_q.id, sim_q.text, distance, sim_q.embedding);
                    }
                } else {
                    println!("No similar embeddings found for \"{}\".\n", term_str);
                }
            } else {
                println!("Term \"{}\" not found in embeddings. Skipping individual query.\n", term_str);
            }
        }
        println!("--- End Individual Term Queries ---\n+");
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
        is_missing_embedding: false,
    };

    println!("--- Combined Query ---");
    println!("Combined Embedding for input terms: {:?}", combined_question.embedding);

    let similar_embeddings = model.find_similar_embeddings(&combined_question);
    if !similar_embeddings.is_empty() {
        println!("\nMost Similar Embeddings to combined query:");
        for (sim_q, distance) in similar_embeddings {
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

pub fn handle_add_vector_command(model: &mut Model, term: &str, embedding_str: &str) {
    let submitted_embedding = crate::cli::parse_embedding(embedding_str)
        .expect("Invalid embedding format");

    // Check if term already exists
    if model.questions.iter().any(|q| q.text == term) {
        println!("Term \"{}\" already exists. Use 'answer' command to update its embedding.", term);
        return;
    }

    let new_id = model.questions.len(); // Simple sequential ID
    let new_question = Question {
        id: new_id,
        text: term.to_string(),
        embedding: submitted_embedding,
        is_missing_embedding: false, // It's explicitly added, so not missing
    };

    model.questions.push(new_question);
    model.weights.push(1.0); // Give it a default weight
    model.save();

    println!("Successfully added new term \"{}\" with ID {}.", term, new_id);
}
