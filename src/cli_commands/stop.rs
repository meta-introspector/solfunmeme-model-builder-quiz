use anyhow::Result;
use reqwest;

pub async fn handle_stop_command() -> Result<()> {
    println!("Stopping quiz server...");
    let client = reqwest::Client::new();
    match client.get("http://127.0.0.1:3000/stop").send().await {
        Ok(response) => {
            println!("Server response: {:?}", response.text().await?);
        }
        Err(e) => {
            eprintln!("Error stopping server: {}", e);
        }
    }
    println!("Quiz server stop signal sent.");
    Ok(())
}
