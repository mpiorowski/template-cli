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
    Set(Path),
    #[command(about = "Replace the template variables")]
    Var(Var),
    #[command(about = "Copy the template file")]
    Copy(Copy),
    #[command(about = "Use the template files")]
    Use(Use),
    #[command(about = "List all templates")]
    List,
    #[command(about = "Print the current configuration")]
    Config,
}

#[derive(Args, Debug, PartialEq)]
pub struct Path {
    pub path: PathBuf,
}

#[derive(Args, Debug, PartialEq)]
pub struct Var {
    pub project: String,

    pub path: PathBuf,
}

#[derive(Args, Debug, PartialEq)]
pub struct Copy {
    pub project: String,

    pub page: String,
}

#[derive(Args, Debug, PartialEq)]
pub struct Use {
    pub project: String,

    pub pages: Vec<String>,

    #[arg(short = 'p', long = "path")]
    pub path: Option<PathBuf>,
}
