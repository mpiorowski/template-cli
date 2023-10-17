use crate::{
    config::Config,
    opts::{Action, Opts},
    utils::{check_file, check_folder},
};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Setup {
    pub action: Action,
    pub config: Config,
    pub path: PathBuf,
}

impl TryFrom<Opts> for Setup {
    type Error = anyhow::Error;

    fn try_from(opts: Opts) -> Result<Self> {
        let action = opts.action;
        let config = Config::create()?;
        match action {
            Action::Set(_) => {
                let path = PathBuf::try_from(&action)?;
                check_folder(&path)?;
                return Ok(Self {
                    action,
                    config,
                    path,
                });
            }
            Action::Var(_) => {
                let path = PathBuf::try_from(&action)?;
                check_file(&path)?;
                return Ok(Self {
                    action,
                    config,
                    path,
                });
            }
            Action::Copy(_) => {
                let path = PathBuf::try_from(&action)?;
                return Ok(Self {
                    action,
                    config,
                    path,
                });
            }
            Action::Use(ref val) => {
                let path = PathBuf::try_from(&action)?;
                let pages = &val.pages;
                if pages.is_empty() {
                    return Err(anyhow::anyhow!("No pages provided"));
                }
                return Ok(Self {
                    action,
                    config,
                    path,
                });
            }
            Action::List => {
                let path = PathBuf::try_from(&action)?;
                check_folder(&path)?;
                return Ok(Self {
                    action,
                    config,
                    path,
                });
            }
            Action::Config => {
                let path = PathBuf::try_from(&action)?;
                Ok(Self {
                    action,
                    config,
                    path,
                })
            }
        }
    }
}

impl TryFrom<&Action> for PathBuf {
    type Error = anyhow::Error;

    fn try_from(action: &Action) -> Result<Self> {
        match &action {
            Action::Set(path) => Ok(path.path.to_owned()),
            Action::Copy(_) => Ok(PathBuf::from(".")),
            Action::Var(path) => Ok(path.path.to_owned()),
            Action::Use(add) => {
                if let Some(path) = &add.path {
                    Ok(path.to_owned())
                } else {
                    Ok(PathBuf::from("."))
                }
            }
            Action::List => Ok(PathBuf::from(".")),
            Action::Config => Ok(PathBuf::from(".")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Setup;
    use crate::opts::{Action, Opts, Path};
    use anyhow::Result;
    use std::path::PathBuf;

    #[test]
    fn setup_print() -> Result<()> {
        let setup: Setup = Opts {
            action: Action::Config,
        }
        .try_into()?;
        assert_eq!(setup.action, Action::Config);
        assert_eq!(setup.path, PathBuf::from("."));
        return Ok(());
    }

    #[test]
    fn setup_set() -> Result<()> {
        let template_path = PathBuf::from("/templates");
        let setup: Result<Setup> = Opts {
            action: Action::Set(Path {
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
