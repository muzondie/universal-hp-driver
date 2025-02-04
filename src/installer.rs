use reqwest::Client;
use tokio::fs::{File, create_dir_all};
use tokio::io::AsyncWriteExt;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InstallError {
    #[error("Download failed")]
    DownloadFailed,
    #[error("Invalid checksum")]
    ChecksumMismatch,
    #[error("Installation timeout")]
    Timeout,
}

pub async fn install_driver(url: &str, target_dir: PathBuf) -> Result<(), InstallError> {
    let client = Client::new();
    let response = client.get(url)
        .send()
        .await
        .map_err(|_| InstallError::DownloadFailed)?;

    let temp_path = target_dir.join("_temp_driver.zip");
    let mut file = File::create(&temp_path)
        .await
        .map_err(|_| InstallError::DownloadFailed)?;

    let content = response.bytes().await.map_err(|_| InstallError::DownloadFailed)?;
    file.write_all(&content).await.map_err(|_| InstallError::DownloadFailed)?;

    extract_archive(&temp_path, &target_dir).await?;
    Ok(())
}

async fn extract_archive(src: &PathBuf, dest: &PathBuf) -> Result<(), InstallError> {
    let file = std::fs::File::open(src).map_err(|_| InstallError::DownloadFailed)?;
    let mut archive = zip::ZipArchive::new(file).map_err(|_| InstallError::DownloadFailed)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|_| InstallError::DownloadFailed)?;
        let outpath = dest.join(file.name());
        
        if file.name().ends_with('/') {
            create_dir_all(&outpath).await.map_err(|_| InstallError::DownloadFailed)?;
        } else {
            let mut outfile = File::create(&outpath).await.map_err(|_| InstallError::DownloadFailed)?;
            tokio::io::copy(&mut file, &mut outfile).await.map_err(|_| InstallError::DownloadFailed)?;
        }
    }
    Ok(())
}