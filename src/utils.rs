use std::time::Duration;
use reqwest::Client;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Connection timeout")]
    Timeout,
    #[error("Server error")]
    ServerError,
}

pub async fn check_online() -> Result<bool, NetworkError> {
    let client = Client::new()
        .timeout(Duration::from_secs(3));

    match client.get("https://hp.com/ping").send().await {
        Ok(resp) => Ok(resp.status().is_success()),
        Err(_) => Ok(false),
    }
}

pub fn compare_versions(current: &str, new: &str) -> bool {
    current != new
}

pub async fn cleanup_temp_files() -> std::io::Result<()> {
    tokio::fs::remove_dir_all("temp_drivers").await?;
    Ok(())
}