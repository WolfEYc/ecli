use color_eyre::Result;
use config::Config;
use fzf::lookup;
pub mod cli;
pub mod cmd;
pub mod config;
pub mod fzf;

pub async fn find_n_execute() -> Result<()> {
    let config = Config::load()?;
    let cmds = config.load_commands().await?;
    let selected_cmd = lookup(cmds);
    selected_cmd.command.execute()?;
    Ok(())
}

pub fn list_sources() -> Result<()> {
    let config = Config::load()?;
    config.print_sources();
    Ok(())
}

pub async fn add_source(url: &str) -> Result<()> {
    let mut config = Config::load()?;
    let source = config.add_source(url)?;
    let (source, lookup_tbl) = source.update_commands().await?;
    println!(
        "installed {} commands\nto {}\nfrom {}",
        lookup_tbl.commands.len(),
        source.filepath.to_str().unwrap(),
        url
    );
    config.save()?;
    Ok(())
}

pub fn remove_source(url: String) -> Result<()> {
    let mut config = Config::load()?;
    let src = config.remove_source(&url)?;
    config.save()?;
    println!("removed {} from sources", src.url);
    Ok(())
}

pub async fn update_commands() -> Result<()> {
    let config = Config::load()?;
    config.update_commands().await?;
    Ok(())
}
