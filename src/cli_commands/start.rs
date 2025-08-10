use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::model_core::Model;
use crate::server::run_server;

pub async fn handle_start_command(model_arc: Arc<Mutex<Model>>) -> Result<()> {
    println!("Starting quiz server...");
    run_server(model_arc.clone()).await;
    Ok(())
}
