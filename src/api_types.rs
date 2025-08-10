use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use crate::model_core::Model;

// Application State
#[derive(Clone)]
pub struct AppState {
    pub shutdown_tx: Arc<Mutex<Option<oneshot::Sender<()>>>>,
    pub model: Arc<Mutex<Model>>,
}

// API Data Structures
#[derive(Serialize, Deserialize)]
pub struct QuizQuestion {
    pub id: usize,
    pub text: String,
}

#[derive(Deserialize)]
pub struct Answer {
    pub question_id: usize,
    pub submitted_embedding: Vec<f32>,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerResponse {
    pub correct: bool,
}

#[derive(Serialize)]
pub struct StatusResponse {
    pub status: String,
}
