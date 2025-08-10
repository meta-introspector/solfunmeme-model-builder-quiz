use axum::{ 
    routing::{get, post}, 
    Router, 
}; 
use tokio::signal; 
use tokio::sync::oneshot; 
use std::sync::{Arc, Mutex}; 
 
use crate::term_quiz_master::quiz_logic::Model; 
//use crate::term_quiz_master::term_entry::Question; 
use crate::api_types::AppState; 
use crate::api_handlers::{status_handler, stop_handler, quiz_handler, answer_handler, debug_answer_handler}; 
 
pub async fn run_server(model_arc: Arc<Mutex<Model>>) { 
    let (shutdown_tx, shutdown_rx) = oneshot::channel(); 
    let app_state = AppState { 
        shutdown_tx: Arc::new(Mutex::new(Some(shutdown_tx))), 
        model: model_arc, // Use the passed model_arc 
    }; 
 
    let app = Router::new() 
        .route("/", get(|| async { "Hello, Quiz Server!" })) 
        .route("/status", get(status_handler)) 
        .route("/stop", get(stop_handler)) 
        .route("/quiz", get(quiz_handler)) 
        .route("/answer", post(answer_handler)) 
        .route("/debug_answer", post(debug_answer_handler)) 
        .with_state(app_state); 
 
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000") 
        .await 
        .unwrap(); 
 
    println!("Quiz server listening on http://127.0.0.1:3000"); 
 
    axum::serve(listener, app) 
        .with_graceful_shutdown(async { 
            signal::ctrl_c().await.unwrap(); 
            println!("Ctrl-C received, shutting down gracefully."); 
            // Also listen for the oneshot signal from the /stop endpoint 
            let _ = shutdown_rx.await; 
            println!("Shutdown signal received, shutting down."); 
            std::process::exit(0); 
        }) 
        .await 
        .unwrap(); 
} 
