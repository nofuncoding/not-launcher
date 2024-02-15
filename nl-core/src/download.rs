// TODO: Make a custom downloader using reqwest

use std::path::PathBuf;
use trauma::{download::Download, downloader::DownloaderBuilder};
use anyhow::Result;

#[tokio::main]
pub async fn download() -> Result<()> {
    let meta = "https://piston-meta.mojang.com/mc/game/version_manifest.json";
    let download = Download::try_from(meta)?;
    let downloader = DownloaderBuilder::new()
        .directory(PathBuf::from("versions"))
        .build();
    downloader.download(&
        [download]).await;
    Ok(())
}