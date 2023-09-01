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
    #[command(about = "Set a templates folder")]
    Set(Set),
    #[command(about = "Use a template")]
    Use(Use),
    #[command(about = "Add a new page to templates")]
    Add(Add),
    #[command(about = "Print the current configuration")]
    Config,
}

#[derive(Args, Debug, PartialEq)]
pub struct Set {
    pub path: PathBuf,
}

#[derive(Args, Debug, PartialEq)]
pub struct Use {
    pub lib: String,

    pub pages: Vec<String>,

    #[arg(short = 'p', long = "path")]
    pub path: Option<PathBuf>,
}

#[derive(Args, Debug, PartialEq)]
pub struct Add {
    pub file: PathBuf,

    pub lib: String,

    pub short: String,
}
