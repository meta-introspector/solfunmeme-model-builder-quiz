use crate::term_quiz_master::quiz_logic::Model;
use crate::term_quiz_master::term_entry::Question;

const ADD_TERM_SUCCESS_MESSAGE: &str = "Successfully added new term \"{term}\" with ID {}.";

pub fn handle_add_vector_command(model: &mut Model, term: &str, embedding_str: &str) {
    let submitted_embedding = crate::cli::parse_embedding(embedding_str)
        .expect("Invalid embedding format");

    // Check if term already exists
    if model.questions.iter().any(|q| q.text == term) {
        println!("Term \"{term}\" already exists. Use 'answer' command to update its embedding.");
        return;
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

    //println!("{}", format_args!(ADD_TERM_SUCCESS_MESSAGE, term, new_id));
    println!("{}", format_args!("{} {} {}", ADD_TERM_SUCCESS_MESSAGE, term, new_id));
}
