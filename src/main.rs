use clap::Parser;
use color_eyre::eyre::Result;
use fsf::{config::add_source, *};

pub fn main() -> Result<()> {
    let cli = cli::Args::parse();

    let Some(cmd) = cli.cmd else {
        find_n_execute();
        return Ok(());
    };

    match cmd {
        cli::Commands::Edit => open_cmds_toml_in_fav_text_editor(),
        cli::Commands::Add { url } => {
            add_source(url.as_str())?;
        }
        cli::Commands::Update {} => todo!(),
    };

    Ok(())
}
