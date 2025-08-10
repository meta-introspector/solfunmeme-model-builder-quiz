use crate::term_quiz_master::quiz_logic::Model;
//use crate::term_quiz_master::term_entry::Question;

pub fn handle_quiz_command(model: &mut Model) {
    if let Some(question) = model.get_question() {
        println!("Question ID: {}", question.id);
        println!("Question Text: {}", question.text);
        println!("Current Embedding: {:?}", question.embedding);
    } else {
        println!("No questions available.");
    }
}
