use crate::term_quiz_master::quiz_logic::Model;
use crate::term_quiz_master::term_entry::Question;

pub fn handle_answer_command(model: &mut Model, question_id: usize, submitted_embedding: Vec<f32>) {
    if let Some(question) = model.questions.get_mut(question_id) {
        let distance = Question::calculate_distance(&question.embedding, &submitted_embedding);
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
