use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::{fs, path::PathBuf};
use templates_cli::{
    opts::{Action, Opts},
    setup::Setup,
    utils::{check_folder, get_config_path},
};

fn main() -> Result<()> {
    let setup = Setup::try_from(Opts::parse())?;
    let templates_path = &setup.config.templates_path;
    let path = &setup.path;
    match setup.action {
        Action::Set(set) => {
            set_templates_path(&set.path)?;
        }
        Action::Add(add) => {
            let lib = &add.lib;
            let pages = &add.pages;
            check_folder(&templates_path.join(lib))?;

            let templates = find_templates(&templates_path.join(lib), &pages)?;
            for template in templates {
                let template_name = template.file_name().context("File not valid")?;
                fs::copy(&template, &path.join(template_name)).context("File not copied")?;
            }
        }
        Action::Print => {
            println!("{:?}", setup);
        }
    }
    Ok(())
}

/**
 * For every file inside path, find all the files that start with # page_name
 * @param path Path to the folder
 * @param pages Vector of pages to find
 * @return Paths of the files that are valid
 */
fn find_templates(path: &PathBuf, pages: &Vec<String>) -> Result<Vec<PathBuf>> {
    let mut templates = vec![];
    let mut files = fs::read_dir(path).context("Path not valid")?;
    while let Some(file) = files.next() {
        let file = file.context("File not valid")?;
        let file_path = file.path();
        let file_content = fs::read_to_string(&file_path).context("File not valid")?;
        let file_content = file_content.lines().next().context("File not valid")?;
        let file_content = file_content.trim();

        for page in pages {
            if file_content.starts_with("# ") && file_content[2..] == page.to_string() {
                templates.push(PathBuf::from(&file_path));
            } else {
                println!("No templates found matching \"# {}\"", page);
            }
        }
    }
    return Ok(templates);
}

/**
 * Set the templates path in the config file
 * @param path Path to the templates folder
 * @return Result
 */
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
