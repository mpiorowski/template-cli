use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::{fs, path::PathBuf};
use templates_cli::{
    opts::{Action, Opts},
    setup::Setup,
    utils::get_config_path,
};

fn main() -> Result<()> {
    let setup = Setup::try_from(Opts::parse())?;
    match setup.action {
        Action::Set(set) => {
            set_templates_path(&set.path)?;
        }
        Action::Add(add) => {
            println!("Add");
            println!("{:?}", add);
        }
        Action::Print => {
            println!("{:?}", setup);
        }
    }
    Ok(())
}

fn set_templates_path(path: &PathBuf) -> Result<()> {
    // templates path
    let templates_str = &path.to_str().context("Path not valid")?;
    let templates_json = to_string(&templates_str).context("Json not valid")?;

    // read config
    let config_path = get_config_path()?;
    let mut config_string = std::fs::read_to_string(&config_path).context("Config not found")?;
    let mut config_json: Value = from_str(&config_string).context("Config not valid")?;

    // write config
    config_json["templates_path"] = from_str(&templates_json)
        .with_context(|| format!("Json not valid: {:?}", templates_json))?;
    config_string = to_string_pretty(&config_json)
        .with_context(|| format!("Config not valid {:?}", config_string))?;

    // save config
    fs::write(&config_path, &config_string)
        .with_context(|| format!("Config not written to {:?}", config_path))?;

    return Ok(());
}
