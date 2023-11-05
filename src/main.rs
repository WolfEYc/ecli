use clap::Parser;
use color_eyre::eyre::Result;
use fsf::{add_source, cli, find_n_execute, list_sources, remove_source, update_commands};
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Args::parse();

    let Some(cmd) = cli.cmd else {
        return find_n_execute().await;
    };

    match cmd {
        cli::Commands::List => list_sources(),
        cli::Commands::Add { url } => add_source(url.as_str()).await,
        cli::Commands::Remove { url } => remove_source(url),
        cli::Commands::Update {} => update_commands().await,
    }
}
