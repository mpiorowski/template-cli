use anyhow::{Context, Result};
use std::{
    env::var,
    fs::{metadata, read_to_string, File},
    io::Write,
    path::PathBuf,
};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub config_path: PathBuf,
    pub templates_path: PathBuf,
}

impl Config {
    pub fn create() -> Result<Self> {
        // Get config path
        let config = var("XDG_CONFIG_HOME")
            .or_else(|_| var("HOME").map(|v| v + "/.config"))
            .context("Config not set")?;
        let mut config = PathBuf::from(config);
        config.push("templates-cli.json");

        // If config file doesn't exist, create it
        if metadata(&config).is_err() {
            let mut file = File::create(&config).context("Config file not created")?;
            file.write_all(b"{\"templates_path\": \"~/tmp\"}")
                .context("Config file not written")?;
            return Ok(Config {
                config_path: config,
                templates_path: PathBuf::from("~/tmp"),
            });
        }
        // If config file exists, read it
        let config_str = read_to_string(&config).context("Config not found")?;
        let config_json: serde_json::Value =
            serde_json::from_str(&config_str).context("Config not valid")?;
        let templates_path = config_json
            .get("templates_path")
            .and_then(|v| v.as_str())
            .map(|v| PathBuf::from(v))
            .context("Templates folder path not found in config")?;
        return Ok(Config {
            config_path: config,
            templates_path,
        });
    }
}
