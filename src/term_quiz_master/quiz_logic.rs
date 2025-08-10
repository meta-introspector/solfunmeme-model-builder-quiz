use std::collections::HashMap;
use std::fs;
use rand::prelude::*;
use crate::term_quiz_master::term_entry::Question;

#[derive(Clone)]
pub struct Model {
    pub questions: Vec<Question>,
    pub weights: Vec<f32>,
}

impl Model {
    pub fn new() -> Self {
        let embeddings_path = format!("{}/term_embeddings.json", env!("CARGO_MANIFEST_DIR"));
        let embeddings_str = fs::read_to_string(embeddings_path).expect("Could not read term_embeddings.json");
        let raw_embeddings: HashMap<String, Vec<f32>> = serde_json::from_str(&embeddings_str).expect("Could not parse term_embeddings.json");

        let questions: Vec<Question> = raw_embeddings.into_iter().enumerate().map(|(id, (text, embedding))| {
            Question { id, text, embedding }
        }).collect();

        let weights = vec![1.0; questions.len()];
        let model = Self { questions, weights };
        model.save_embeddings().expect("Failed to save initial embeddings"); // Save after loading
        model
    }

    pub fn get_question(&self) -> Option<Question> {
        let mut rng = thread_rng();
        self.questions.choose_weighted(&mut rng, |item| self.weights[item.id]).ok().cloned()
    }

    pub fn update_weight(&mut self, question_id: usize, correct: bool) {
        if correct {
            self.weights[question_id] *= 1.1;
        } else {
            self.weights[question_id] *= 0.9;
        }
    }

    pub fn update_embedding(&mut self, question_id: usize, submitted_embedding: Vec<f32>) {
        if let Some(question) = self.questions.get_mut(question_id) {
            // Simple averaging for now
            for i in 0..question.embedding.len() {
                question.embedding[i] = (question.embedding[i] + submitted_embedding[i]) / 2.0;
            }
        }
    }

    pub fn save_embeddings(&self) -> Result<(), std::io::Error> {
        let embeddings_path = format!("{}/term_embeddings.json", env!("CARGO_MANIFEST_DIR"));
        let mut raw_embeddings = HashMap::new();
        for q in &self.questions {
            raw_embeddings.insert(q.text.clone(), q.embedding.clone());
        }
        let json_str = serde_json::to_string_pretty(&raw_embeddings)?;
        fs::write(&embeddings_path, json_str)?;
        Ok(())
    }

    pub fn insert_term(&mut self, term: String, embedding: Vec<f32>) -> Result<(), String> {
        if self.questions.iter().any(|q| q.text == term) {
            return Err(format!("Term '''{}''' already exists.", term));
        }
        let new_id = self.questions.len();
        self.questions.push(Question { id: new_id, text: term, embedding });
        self.weights.push(1.0); // Default weight
        self.save_embeddings().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn update_term(&mut self, term_id: usize, new_embedding: Vec<f32>) -> Result<(), String> {
        if let Some(question) = self.questions.get_mut(term_id) {
            question.embedding = new_embedding;
            self.save_embeddings().map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err(format!("Term with ID {} not found.", term_id))
        }
    }

    pub fn delete_term(&mut self, term_id: usize) -> Result<(), String> {
        if term_id < self.questions.len() {
            self.questions.remove(term_id);
            // Re-assign IDs and weights to maintain consistency after deletion
            for (i, q) in self.questions.iter_mut().enumerate() {
                q.id = i;
            }
            self.weights.remove(term_id); // Remove corresponding weight
            self.save_embeddings().map_err(|e| e.to_string())?;
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

    pub fn find_similar_terms_by_vector(&self, target_embedding: &Vec<f32>, top_n: usize) -> Vec<(Question, f32)> {
        let mut similarities: Vec<(Question, f32)> = Vec::new();

        for question in self.questions.iter() {
            let distance = Question::calculate_distance(&question.embedding, target_embedding);
            // Store (Question, distance). We'll sort by distance, so lower is better.
            similarities.push((question.clone(), distance));
        }

        // Sort by distance (ascending)
        similarities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Take top_n results
        similarities.into_iter().take(top_n).collect()
    }
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}