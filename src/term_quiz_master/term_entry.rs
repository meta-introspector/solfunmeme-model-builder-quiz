use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Question {
    pub id: usize,
    pub text: String,
    pub embedding: Vec<f32>,
}

impl Question {
    pub fn calculate_distance(embedding1: &[f32], embedding2: &[f32]) -> f32 {
        embedding1.iter().zip(embedding2.iter()).map(|(a, b)| (a - b).powi(2)).sum::<f32>().sqrt()
    }
}
