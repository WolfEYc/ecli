use color_eyre::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};
use url::Url;

use crate::cmd::LookupVec;

#[derive(Serialize, Deserialize)]
struct Config {
    sources: Vec<String>,
}

impl Config {
    fn save(&self) -> Result<()> {
        let str_toml = toml::to_string_pretty(&self)?;
        let filepath = get_config_filepath();
        fs::create_dir_all(&filepath)?;
        fs::write(&filepath, str_toml.as_bytes())?;
        Ok(())
    }

    fn load() -> Result<Config> {
        let toml_content = fs::read_to_string(get_config_filepath())?;
        Ok(toml::from_str(&toml_content)?)
    }
}

fn get_config_filepath() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    let configfile = proj_dirs.config_local_dir().join("config");
    fs::create_dir_all(&configfile).unwrap();
    configfile
}

fn save_source_url(url: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.sources.push(url.to_string());
    config.save()?;
    Ok(())
}

pub fn add_source(url: &str) -> Result<LookupVec> {
    let lookup_vec = update_from_source(url)?;
    save_source_url(url)?;
    Ok(lookup_vec)
}

pub fn update_from_source(url: &str) -> Result<LookupVec> {
    let binding = Url::parse(&url)?;
    let filename = binding.path_segments().unwrap().last().unwrap();
    let filepath = get_commands_dir().join(filename);

    let res = reqwest::blocking::get(url)?;
    let toml_contet: LookupVec = res.text()?.try_into()?;

    println!("Downloaded {} commands", toml_contet.commands.len());
    println!("Saving to local disk ...");
    let toml_contet = toml_contet.save(&filepath)?;
    println!("Saved!");
    Ok(toml_contet)
}

pub fn get_commands_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    let cmds_dir = proj_dirs.data_local_dir().join("cmds");
    fs::create_dir_all(&cmds_dir).unwrap();
    cmds_dir.to_path_buf()
}

pub fn load_saved_commands() -> LookupVec {
    let cmds_path = get_commands_dir();
    let cmd_files = read_files_in_dir(&cmds_path);

    let mut master_table = LookupVec::new();

    if cmd_files.is_empty() {
        println!("No commands found, downloading defaults...");
        let defaut_table =
            add_source("https://raw.githubusercontent.com/WolfEYc/fsf/master/cmd/default.toml")
                .unwrap();

        master_table.merge(defaut_table);
    }

    for cmd_filepath in cmd_files {
        let Ok(lookup_table) = cmd_filepath.try_into() else {
            continue;
        };

        master_table.merge(lookup_table);
    }

    master_table
}

pub fn read_files_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).unwrap();
    entries.map(|e| e.unwrap().path()).collect()
}
