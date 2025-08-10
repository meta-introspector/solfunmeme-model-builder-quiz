use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;

pub async fn handle_delete_command(model_arc: Arc<Mutex<Model>>, term_id: usize) -> Result<()> {
    println!("Deleting term ID: {}", term_id);
    let mut model = model_arc.lock().unwrap();
    match model.delete_term(term_id) {
        Ok(_) => println!("Term deleted successfully."),
        Err(e) => eprintln!("Error deleting term: {}", e),
    }
    Ok(())
}
