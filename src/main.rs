use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::{collections::HashSet, fs, path::PathBuf};
use templates::{
    opts::{Action, Opts},
    setup::Setup,
    utils::{check_file, check_folder},
};

fn main() -> Result<()> {
    let setup = Setup::try_from(Opts::parse())?;
    let templates_path = &setup.config.templates_path;
    let path = &setup.path;
    match setup.action {
        Action::Set(path) => {
            println!("Setting templates path to {:?}", path.path);
            set_templates_path(&path.path, &setup.config.config_path)?;
        }
        Action::Use(val) => {
            let lib = &val.project;
            let pages = &val.pages;
            check_folder(&templates_path.join(lib))?;

            let templates = use_template_files(&templates_path.join(lib), &pages)?;
            for template in templates {
                println!(
                    "Copying {:?} to {:?}",
                    template.path,
                    path.join(&template.name)
                );
                fs::create_dir_all(&path).context("Folder not created")?;
                fs::copy(&template.path, &path.join(template.name)).context("File not copied")?;
            }
        }
        Action::Var(var) => {
            println!("Replacing variables in {:?}", var.path);
            let var_file: &PathBuf = &setup.config.templates_path.join(&var.project).join("var");
            check_file(var_file)?;
            replace_template_variables(&var.path, var_file)?;
        }
        Action::List => {
            list_templates(&templates_path)?;
        }
        Action::Config => {
            println!("{:?}", setup);
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Template {
    name: String,
    path: PathBuf,
}

/**
 * Set the templates path in the config file
 * @param path Path to the templates folder
 * @return Result
 */
fn set_templates_path(path: &PathBuf, config_path: &PathBuf) -> Result<()> {
    // templates path
    let templates_str = &path.to_str().context("Path not valid")?;
    let templates_json = to_string(&templates_str).context("Json not valid")?;

    // read config
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

/**
 * For every file inside path, find all the files that start with `[page]` and return the path with the page name
 * Files name must be in the format `[page]name`
 * @param path Path to the folder
 * @param pages Vector of pages to find
 * @return Paths of the files that are valid
 */
fn use_template_files(path: &PathBuf, pages: &Vec<String>) -> Result<Vec<Template>> {
    let mut templates = vec![];
    let mut files = fs::read_dir(path).context("Path not valid")?;

    let mut not_found: HashSet<String> = HashSet::new();

    while let Some(file) = files.next() {
        let file = file.context("File not valid")?;
        let file_path = file.path();
        let file_name = file_path.file_name().context("File not valid")?;
        let file_name = file_name.to_str().context("File not valid")?;

        for page in pages {
            let end = file_name.find(']').with_context(|| {
                format!("Filename not valid. Not found ']' in: {:?}", file_name)
            })?;
            let start = file_name.find('[').with_context(|| {
                format!("Filename not valid. Not found '[' in: {:?}", file_name)
            })?;
            let file_page = &file_name[start + 1..end];
            let new_name = file_name[end + 1..].to_string();
            if file_page == page {
                templates.push(Template {
                    name: new_name,
                    path: PathBuf::from(&file_path),
                });
            } else {
                not_found.insert(page.to_string());
            }
        }
    }
    not_found.iter().for_each(|page| {
        println!("Page {} not found", page);
    });
    return Ok(templates);
}

/**
 * Replace the variables in the requested file with the values in the template variables file
 * Variables in the template file must be in the format `VAR=VAL`
 * Variables in the requested file must be in the format `{{VAR}}`
 * @param path Path to the requested file
 * @param var_file Path to the variables file
 * @return Result
 */
fn replace_template_variables(path: &PathBuf, var_file: &PathBuf) -> Result<()> {
    let mut file = fs::read_to_string(&path).context(anyhow::anyhow!(
        "Requested file not found. Create it at {:?}",
        path
    ))?;
    let var_file = fs::read_to_string(&var_file).context(anyhow::anyhow!(
        "Variables file not found. Create it at {:?}",
        var_file
    ))?;

    // Split the file by lines
    let lines = var_file.split('\n').collect::<Vec<&str>>();
    for ele in lines {
        // Split the line by `=` to get the variable and the value
        let mut ele = ele.split('=').collect::<Vec<&str>>();
        if ele.len() == 2 {
            // Remove the spaces
            ele[0] = ele[0].trim();
            ele[1] = ele[1].trim();

            // Look for {{VAR}} in the file and replace it with the value
            if !file.find(&format!("{{{{{}}}}}", ele[0])).is_none() {
                file = file.replace(&format!("{{{{{}}}}}", ele[0]), ele[1]);
                println!("Replaced {{{{{}}}}} with {}", ele[0], ele[1]);
            }
        }
    }

    fs::write(&path, file).context("File not written")?;
    return Ok(());
}

/**
 * List all the templates in the templates folder
 * @param templates_path Path to the templates folder
 * @return Result
 */
fn list_templates(templates_path: &PathBuf) -> Result<()> {
    let mut files = fs::read_dir(templates_path).context("Templates path not valid")?;

    while let Some(file) = files.next() {
        let file = file.context("File not valid")?;
        let file_path = file.path();
        let file_name = file_path.file_name().context("File not valid")?;
        let file_name = file_name.to_str().context("File not valid")?;
        println!("{}", file_name);

        let is_dir = file.file_type().context("File not valid")?.is_dir();
        if is_dir {
            let mut sub_files = fs::read_dir(&file_path).context("Path not valid")?;
            while let Some(sub_file) = sub_files.next() {
                let sub_file = sub_file.context("File not valid")?;
                let sub_file_path = sub_file.path();
                let sub_file_name = sub_file_path.file_name().context("File not valid")?;
                let sub_file_name = sub_file_name.to_str().context("File not valid")?;
                println!("  {}", sub_file_name);
            }
        }
    }

    return Ok(());
}
