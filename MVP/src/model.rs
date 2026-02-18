use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use anyhow::{Context, Result};
use colored::*;

const MODEL_URL: &str = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main/ggml-base.en.bin";
const MODEL_FILENAME: &str = "ggml-base.en.bin";

pub fn ensure_model_exists() -> Result<PathBuf> {
    let mut path = std::env::current_exe()?;
    path.pop(); // Remove executable name
    path.push(MODEL_FILENAME);

    // If running from cargo run (target/debug/...), we might want to put it in project root for cache?
    // But for simplicity, let's look in the same folder as the exe first.
    // Actually, let's use a dedicated data folder if possible, or just local.
    // For MVP, keep it simple: next to binary.
    
    if path.exists() {
        println!("{} Model found at: {:?}", "✓".green(), path);
        return Ok(path);
    }

    println!("{} Model not found. Downloading {}...", "⬇".blue(), MODEL_FILENAME);
    println!("  URL: {}", MODEL_URL);
    println!("  (This is ~142MB, happens once)");

    download_model(&path)?;

    println!("{} Model downloaded successfully!", "✓".green());
    Ok(path)
}

fn download_model(path: &Path) -> Result<()> {
    // Increase timeout for large download
    let client = reqwest::blocking::Client::builder()
        .timeout(std::time::Duration::from_secs(600)) // 10 minutes
        .build()?;

    let response = client.get(MODEL_URL)
        .send()
        .context("Failed to connect to model URL")?;

    if !response.status().is_success() {
        anyhow::bail!("Failed to download model: Status {}", response.status());
    }

    let content = response.bytes()?;
    let mut file = fs::File::create(path).context("Failed to create model file")?;
    file.write_all(&content).context("Failed to write model file")?;

    Ok(())
}
