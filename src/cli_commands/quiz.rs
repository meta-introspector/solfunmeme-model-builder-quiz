use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;
use crate::handlers::handle_quiz_command;

pub async fn handle_quiz_command_cli(model_arc: Arc<Mutex<Model>>) -> Result<()> {
    println!("Starting embedding quiz...");
    let mut model = model_arc.lock().unwrap();
    handle_quiz_command(&mut model);
    Ok(())
}
