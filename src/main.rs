use anyhow::{Context, Ok, Result};
use clap::Parser;
use serde_json::{from_str, to_string, to_string_pretty, Value};
use std::io::Write;
use std::{fs, path::PathBuf};
use templates::{
    config::Config,
    opts::{Action, Opts},
    utils::{check_file, check_folder},
};

fn main() -> Result<()> {
    let config = Config::create()?;
    let opts = Opts::parse();
    match opts.action {
        Action::Set(val) => {
            println!("Setting templates path to {:?}", val.path);
            println!("Setting clipboard command to {:?}", val.clipboard);
            check_folder(&val.path)?;
            set_config(&val.path, &val.clipboard, &config.config_path)?;
        }
        Action::Show(val) => {
            let page = &val.page;
            let project = &val.project.unwrap_or("".to_string());

            let template = find_template_file(&config.templates_path.join(project), page)?
                    .context(format!("Template not found. Create it in the templates folder with the format [{page}]filename", page = page))?;
            let template_path = &template.path;
            let template_content = fs::read_to_string(&template_path).context("File not found")?;

            println!("{}", template_content);
        }
        Action::Copy(val) => {
            let page = &val.page;
            let project = &val.project.unwrap_or("".to_string());

            let template = find_template_file(&config.templates_path.join(project), page)?
                    .context(format!("Template not found. Create it in the templates folder with the format [{page}]filename", page = page))?;
            let template_path = &template.path;
            let template_content = fs::read_to_string(&template_path).context("File not found")?;

            let mut child = std::process::Command::new(config.clipboard_command)
                .stdin(std::process::Stdio::piped())
                .spawn()
                .context("Clipboard not found")?;
            child
                .stdin
                .as_mut()
                .unwrap()
                .write_all(template_content.as_bytes())
                .context("Clipboard not written")?;
        }
        Action::Var(val) => {
            let project_path = PathBuf::from(val.project.unwrap_or("".to_string()));
            let variable_file_path = config.templates_path.join(project_path).join("var");
            check_file(&variable_file_path).context(format!(
                "Variables file not found. Create it at {:?}",
                variable_file_path
            ))?;
            show_variables(&variable_file_path)?;
        }
        Action::Config => {
            println!("{:?}", config);
            list_templates(&config.templates_path)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Template {
    path: PathBuf,
}

/**
 * Set the templates path in the config file
 * @param path Path to the templates folder
 * @return Result
 */
fn set_config(path: &PathBuf, clipboard: &str, config_path: &PathBuf) -> Result<()> {
    // templates path
    let templates_json: serde_json::Value =
        from_str(&to_string(path).context("Json not valid")?)
            .with_context(|| format!("Json not valid: {:?}", path))?;
    let clipboard_json: serde_json::Value =
        from_str(&to_string(clipboard).context("Json not valid")?)
            .with_context(|| format!("Json not valid: {:?}", clipboard))?;

    // read config
    let mut config_string = std::fs::read_to_string(&config_path).context("Config not found")?;
    let mut config_json: Value = from_str(&config_string).context("Config not valid")?;

    // write config
    config_json["templates_path"] = templates_json;
    config_json["clipboard_command"] = clipboard_json;
    config_string = to_string_pretty(&config_json)
        .with_context(|| format!("Config not valid {:?}", config_string))?;

    // save config
    fs::write(&config_path, &config_string)
        .with_context(|| format!("Config not written to {:?}", config_path))?;

    Ok(())
}

/**
 * Find the template files in the templates folder
 * Files name must be in the format `[page]name`
 * @param project_path Path to the project folder
 * @param pages Vector of pages to find
 * @return Paths of the files that are valid
 */
fn find_template_file(project_path: &PathBuf, page: &str) -> Result<Option<Template>> {
    let mut project_dir = fs::read_dir(project_path).context("Project path not valid")?;

    while let Some(file) = project_dir.next() {
        let file = file.context("File not valid")?;
        let file_path = file.path();
        let file_name = file_path.file_name().context("File not valid")?;
        let file_name = file_name.to_str().context("File not valid")?;

        if file_name.starts_with(&format!("[{}]", page)) {
            return Ok(Some(Template {
                path: PathBuf::from(&file_path),
            }));
        }
    }
    Ok(None)
}

fn show_variables(file_path: &PathBuf) -> Result<()> {
    let var_file = fs::read_to_string(&file_path).context("Variables file not found")?;
    let lines = var_file.split('\n').collect::<Vec<&str>>();
    for ele in lines {
        let mut ele = ele.split('=').collect::<Vec<&str>>();
        if ele.len() == 2 {
            ele[0] = ele[0].trim();
            ele[1] = ele[1].trim();
            println!(r"    {}={} \", ele[0], ele[1]);
        }
    }
    Ok(())
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
                if sub_file_name == "var" {
                    show_variables(&sub_file_path)?;
                }
            }
        }
    }

    Ok(())
}
