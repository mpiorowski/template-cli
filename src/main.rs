use anyhow::{Context, Result};
use clap::Parser;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::{collections::HashSet, fs, path::PathBuf};
use templates::{
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
            println!("Setting templates path to {:?}", set.path);
            set_templates_path(&set.path)?;
        }
        Action::Use(val) => {
            let lib = &val.lib;
            let pages = &val.pages;
            check_folder(&templates_path.join(lib))?;

            let templates = find_templates(&templates_path.join(lib), &pages)?;
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
        Action::Add(add) => {
            let lib = &add.lib;
            let short = &add.short;
            let file = &add.file;
            add_file_to_templates(&file, &lib, &short, &templates_path)?;
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

/**
 * For every file inside path, find all the files that start with `[page]` and return the path with the page name
 * Files name must be in the format `[page]name`
 * @param path Path to the folder
 * @param pages Vector of pages to find
 * @return Paths of the files that are valid
 */
fn find_templates(path: &PathBuf, pages: &Vec<String>) -> Result<Vec<Template>> {
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
 * Add a file to the templates folder
 * @param path Path to the file
 * @param lib Library name
 * @param short Shortcut to be used
 * @param templates_path Path to the templates folder
 * @return Result
 */
fn add_file_to_templates(
    path: &PathBuf,
    lib: &String,
    short: &String,
    templates_path: &PathBuf,
) -> Result<()> {
    check_folder(&templates_path.join(lib)).with_context(|| {
        format!(
            "Folder not found: {:?}",
            templates_path.join(lib).to_str().unwrap()
        )
    })?;

    let filename = path.file_name().context("File not valid")?;

    fs::copy(
        &path,
        &templates_path.join(lib).join(format!(
            "[{}]{}",
            short,
            filename.to_str().context("File not valid")?
        )),
    )
    .with_context(|| format!("File not copied to {:?}", templates_path))?;

    println!(
        "File {:?} copied to {:?}",
        path,
        templates_path
            .join(lib)
            .join(format!("[{}]{}", short, filename.to_str().unwrap()))
    );

    return Ok(());
}

/**
 * List all the templates in the templates folder
 * @param templates_path Path to the templates folder
 * @return Result
 */
fn list_templates(templates_path: &PathBuf) -> Result<()> {
    let mut files = fs::read_dir(templates_path).context("Path not valid")?;

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
