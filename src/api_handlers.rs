use axum::{ 
    Json, 
    extract::State, 
}; 
use serde_json; 
 
use std::io::Write; // Added this line
 
use crate::model_core::Model; // Assuming Model is in model_core.rs 
use crate::api_types::{AppState, QuizQuestion, Answer, AnswerResponse, StatusResponse}; // Assuming API types are in api_types.rs 
 
pub async fn status_handler() -> Json<StatusResponse> { 
    Json(StatusResponse { 
        status: "running".to_string(), 
    }) 
} 
 
pub async fn stop_handler(State(state): State<AppState>) -> &'static str { 
    if let Some(tx) = state.shutdown_tx.lock().unwrap().take() { 
        let _ = tx.send(()); 
        "Shutting down..." 
    } else { 
        "Shutdown signal already sent or not available." 
    } 
} 
 
pub async fn quiz_handler(State(state): State<AppState>) -> Json<Option<QuizQuestion>> { 
    let model = state.model.lock().unwrap(); 
    let question = model.get_question().map(|q| QuizQuestion { id: q.id, text: q.text }); 
    Json(question) 
} 
 
pub async fn answer_handler(State(state): State<AppState>, payload: Json<serde_json::Value>) -> Json<AnswerResponse> { 
    println!("Received raw JSON payload: {:?}", payload); 
 
    let answer: Answer = serde_json::from_value(payload.0).expect("Failed to deserialize Answer"); 
 
    let mut model = state.model.lock().unwrap(); 
    let correct = if let Some(question) = model.questions.get(answer.question_id) { 
        let distance = Model::calculate_distance(&question.embedding, &answer.submitted_embedding); 
        let is_correct = distance < 0.1; // Threshold for correctness 
        if !is_correct { 
            model.update_embedding(answer.question_id, answer.submitted_embedding); 
        } 
        model.update_weight(answer.question_id, is_correct); 
        is_correct 
    } else { 
        false 
    }; 
    Json(AnswerResponse { correct }) 
} 
 
pub async fn debug_answer_handler(payload: Json<serde_json::Value>) -> &'static str { 
    let log_path = format!("{}/tmp/debug_payload.log", env!("CARGO_MANIFEST_DIR")); 
    let mut file = std::fs::OpenOptions::new() // Use std::fs explicitly 
        .create(true) 
        .append(true) 
        .open(&log_path) 
        .expect("Could not open debug_payload.log"); 
 
    writeln!(&mut file, "Received raw JSON payload on debug endpoint: {:?}", payload) // Use writeln! macro 
        .expect("Could not write to debug_payload.log"); 
 
    "Debug endpoint reached! Check debug_payload.log" 
} 
