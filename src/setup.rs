use crate::{
    opts::{Action, Add, Opts, Set, Use},
    utils::{check_file, check_folder, get_config_path},
};
use anyhow::{Context, Result};
use std::{io::Write, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub templates_path: PathBuf,
}

#[derive(Debug)]
pub struct Setup {
    pub action: Action,
    pub config: Config,
    pub path: PathBuf,
}

impl Action {
    // TODO - dont use copy
    fn copy(&self) -> Self {
        match self {
            Action::Set(set) => Action::Set(Set {
                path: set.path.clone(),
            }),
            Action::Use(add) => Action::Use(Use {
                lib: add.lib.clone(),
                pages: add.pages.clone(),
                path: add.path.clone(),
            }),
            Action::Add(add) => Action::Add(Add {
                lib: add.lib.clone(),
                file: add.file.clone(),
                short: add.short.clone(),
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
            file.write_all(b"{\"templates_path\": \"~/tmp\"}")
                .context("Config file not written")?;
            return Ok(Config {
                templates_path: PathBuf::from("~/tmp"),
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

impl TryFrom<&Opts> for PathBuf {
    type Error = anyhow::Error;

    fn try_from(opts: &Opts) -> Result<Self> {
        match &opts.action {
            Action::Set(set) => Ok(set.path.to_owned()),
            Action::Use(add) => {
                if let Some(path) = &add.path {
                    Ok(path.to_owned())
                } else {
                    Ok(PathBuf::from("."))
                }
            }
            Action::Add(add) => Ok(add.file.to_owned()),
            Action::Print => Ok(PathBuf::from(".")),
        }
    }
}

impl TryFrom<Opts> for Setup {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> Result<Self> {
        // TODO - dont use copy
        let copy = opts.action.copy();
        let config = Config::create()?;
        let path = PathBuf::try_from(&opts)?;
        match opts.action {
            Action::Set(set) => {
                check_folder(&set.path)?;
                return Ok(Self {
                    action: copy,
                    config,
                    path,
                });
            }
            Action::Add(add) => {
                check_file(&add.file)?;
                return Ok(Self {
                    action: copy,
                    config,
                    path,
                });
            }
            Action::Use(val) => {
                let pages = val.pages;
                if pages.is_empty() {
                    return Err(anyhow::anyhow!("No pages provided"));
                }
                return Ok(Self {
                    action: copy,
                    config,
                    path,
                });
            }
            Action::Print => Ok(Self {
                action: copy,
                config,
                path,
            }),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Setup;
    use crate::opts::{Action, Opts, Set};
    use anyhow::Result;
    use std::path::PathBuf;

    #[test]
    fn setup_print() -> Result<()> {
        let setup: Setup = Opts {
            action: Action::Print,
        }
        .try_into()?;
        assert_eq!(setup.action, Action::Print);
        assert_eq!(setup.path, PathBuf::from("."));
        return Ok(());
    }

    #[test]
    fn setup_set() -> Result<()> {
        let template_path = PathBuf::from("/templates");
        let setup: Result<Setup> = Opts {
            action: Action::Set(Set {
                path: template_path.clone(),
            }),
        }
        .try_into();
        if template_path.is_dir() {
            assert!(setup.is_ok());
        } else {
            assert!(setup.is_err());
        }
        return Ok(());
    }
}
