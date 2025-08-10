use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};

use crate::term_quiz_master::term_entry::Question;

#[derive(Serialize, Deserialize)]
pub struct Model {
    pub questions: Vec<Question>,
    pub weights: Vec<f32>, // Corresponds to the frequency/importance of each question
    // pub term_to_id: HashMap<String, usize>, // Optional: for faster lookup if needed
}

impl Model {
    pub fn new() -> Self {
        let mut model = Model {
            questions: Vec::new(),
            weights: Vec::new(),
            // term_to_id: HashMap::new(),
        };
        // Load existing embeddings if available
        if let Ok(data) = fs::read_to_string("term_embeddings.json") {
            if let Ok(loaded_model) = serde_json::from_str::<Model>(&data) {
                model.questions = loaded_model.questions;
                model.weights = loaded_model.weights;
                // for (id, question) in model.questions.iter().enumerate() {
                //     model.term_to_id.insert(question.text.clone(), id);
                // }
            }
        }
        model
    }

    pub fn save_embeddings(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&self)?;
        let mut file = fs::File::create("term_embeddings.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn get_question(&self) -> Option<&Question> {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.questions.choose(&mut rng)
    }

    pub fn update_embedding(&mut self, question_id: usize, new_embedding: Vec<f32>) {
        if let Some(question) = self.questions.get_mut(question_id) {
            question.embedding = new_embedding;
            self.save_embeddings().expect("Failed to save embeddings after update");
        }
    }

    pub fn update_weight(&mut self, question_id: usize, is_correct: bool) {
        if let Some(weight) = self.weights.get_mut(question_id) {
            if is_correct {
                *weight += 0.1; // Increase weight for correct answers
            } else {
                *weight = (*weight - 0.05).max(0.0); // Decrease weight for incorrect answers, but not below 0
            }
            self.save_embeddings().expect("Failed to save weights after update");
        }
    }

    pub fn update_term(&mut self, term_id: usize, new_embedding: Vec<f32>) -> Result<(), String> {
        if let Some(question) = self.questions.get_mut(term_id) {
            question.embedding = new_embedding;
            self.save_embeddings().map_err(|e| format!("Failed to save embeddings: {}", e))?;
            Ok(())
        } else {
            Err(format!("Term with ID {} not found.", term_id))
        }
    }

    pub fn delete_term(&mut self, term_id: usize) -> Result<(), String> {
        if term_id < self.questions.len() {
            self.questions.remove(term_id);
            self.weights.remove(term_id);
            // Re-assign IDs to maintain sequential order
            for (i, question) in self.questions.iter_mut().enumerate() {
                question.id = i;
            }
            self.save_embeddings().map_err(|e| format!("Failed to save embeddings: {}", e))?;
            Ok(())
        } else {
            Err(format!("Term with ID {} not found.", term_id))
        }
    }

    pub fn find_similar_embeddings(&self, query_term: &str) -> Vec<&Question> {
        let mut similarities: Vec<(&Question, f32)> = Vec::new();
        if let Some(query_question) = self.questions.iter().find(|q| q.text == query_term) {
            for question in self.questions.iter() {
                if question.id != query_question.id {
                    let distance = Question::calculate_distance(&query_question.embedding, &question.embedding);
                    similarities.push((question, distance));
                }
            }
        }
        similarities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        similarities.into_iter().map(|(q, _)| q).collect()
    }

    }
