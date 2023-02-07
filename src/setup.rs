use crate::{
    opts::{Action, Add, Opts, Set},
    utils::{check_folder, get_config_path},
};
use anyhow::{Context, Result};
use std::{io::Write, path::PathBuf};

#[derive(Debug)]
pub struct Config {
    pub templates_path: PathBuf,
}

#[derive(Debug)]
pub struct Setup {
    pub action: Action,
    pub config: Config,
    pub path: Option<PathBuf>,
}

impl Action {
    fn copy(&self) -> Self {
        match self {
            Action::Set(set) => Action::Set(Set {
                path: set.path.clone(),
            }),
            Action::Add(add) => Action::Add(Add {
                lib: add.lib.clone(),
                pages: add.pages.clone(),
                path: add.path.clone(),
            }),
            Action::Print => Action::Print,
        }
    }
}

impl Config {
    fn create() -> Result<Self> {
        let config_path = get_config_path()?;
        if std::fs::metadata(&config_path).is_err() {
            let mut file =
                std::fs::File::create(&config_path).context("Config file not created")?;
            file.write_all(b"{\"templates_path\": \"/tmp\"}")
                .context("Config file not written")?;
            return Ok(Config {
                templates_path: PathBuf::from("/tmp"),
            });
        }

        let config = std::fs::read_to_string(&config_path).context("Config not found")?;
        let config: serde_json::Value =
            serde_json::from_str(&config).context("Config not valid")?;
        let templates_path = config
            .get("templates_path")
            .and_then(|v| v.as_str())
            .map(|v| PathBuf::from(v))
            .context("Templates folder path not found in config")?;
        return Ok(Config { templates_path });
    }
}

impl TryFrom<Opts> for Setup {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> Result<Self> {
        let action = opts.action;
        let copy = action.copy();
        let config = Config::create()?;
        match action {
            Action::Set(set) => {
                check_folder(&set.path)?;
                return Ok(Self {
                    action: copy,
                    config,
                    path: Some(set.path),
                });
            }
            Action::Add(add) => {
                let pages = add.pages;
                if pages.is_empty() {
                    return Err(anyhow::anyhow!("No pages provided"));
                }
                return Ok(Self {
                    action: copy,
                    config,
                    path: add.path,
                });
            }
            Action::Print => Ok(Self {
                action: copy,
                config,
                path: None,
            }),
        }
    }
}
