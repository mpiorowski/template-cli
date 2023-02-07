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

#[derive(Subcommand, Debug)]
pub enum Action {
    #[command(about = "Set a templates folder")]
    Set(Set),
    #[command(about = "Add a new page")]
    Add(Add),
    #[command(about = "Print the current configuration")]
    Print,
}

#[derive(Args, Debug)]
pub struct Set {
    pub path: PathBuf,
}

#[derive(Args, Debug)]
pub struct Add {

    pub lib: String,

    pub pages: Vec<String>,

    #[arg(short = 'p', long = "path")]
    pub path: Option<PathBuf>,
}
