use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version)]
#[command(
    about = "Templates CLI",
    long_about = "Templates CLI is a command line tool to manage templates for your projects."
)]
pub struct Opts {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug, PartialEq)]
pub enum Action {
    #[command(about = "Set a templates main folder")]
    Set(Set),
    #[command(about = "Show the template file")]
    Show(Show),
    #[command(about = "Copy the template files")]
    Copy(Copy),
    #[command(about = "Show variables")]
    Var(Var),
    #[command(about = "Print the current configuration and all templates")]
    Config,
}

#[derive(Args, Debug, PartialEq)]
pub struct Set {
    pub path: PathBuf,
    pub clipboard: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Show {
    pub page: String,
    pub project: Option<String>,
}

#[derive(Args, Debug, PartialEq)]
pub struct Copy {
    pub page: String,
    pub project: Option<String>,
}

#[derive(Args, Debug, PartialEq)]
pub struct Var {
    #[arg(last = true)]
    pub project: Option<String>,
}
