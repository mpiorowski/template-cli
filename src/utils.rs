use anyhow::Result;
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
