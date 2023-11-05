use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List,
    Add { url: String },
    Remove { url: String },
    Update {},
}
