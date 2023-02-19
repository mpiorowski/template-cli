use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn check_folder(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("Folder does not exist: {:?}", path));
    }
    if !path.is_dir() {
        return Err(anyhow::anyhow!("Path is not a folder: {:?}", path));
    }
    Ok(())
}

pub fn check_file(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        return Err(anyhow::anyhow!("File does not exist: {:?}", path));
    }
    if !path.is_file() {
        return Err(anyhow::anyhow!("Path is not a file: {:?}", path));
    }
    Ok(())
}

pub fn get_config_path() -> Result<PathBuf> {
    let loc = std::env::var("XDG_CONFIG_HOME")
        .or_else(|_| std::env::var("HOME").map(|v| v + "/.config"))
        .context("Config not set")?;
    let mut loc = PathBuf::from(loc);
    loc.push("templates-cli.json");
    return Ok(loc);
}
