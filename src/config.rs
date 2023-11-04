use color_eyre::{Report, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, OpenOptions},
    io::Write,
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
        fs::create_dir_all(filepath)?;
        fs::write(filepath, str_toml.as_bytes())?;
        Ok(())
    }

    fn load() -> Result<Config> {
        let toml_content = fs::read_to_string(get_config_filepath())?;
        Ok(toml::from_str(&toml_content)?)
    }
}

fn get_config_filepath() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    proj_dirs.config_local_dir().join("config")
}

fn remember_source_url(url: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.sources.push(url.to_string());
    config.save()?;
    Ok(())
}

pub fn add_source(url: &str) -> Result<PathBuf> {
    let binding = Url::parse(&url)?;
    let filename = binding.path_segments().unwrap().last().unwrap();

    let res = reqwest::blocking::get(url)?;
    let toml_contet: LookupVec = res.text()?.try_into()?;
    toml_contet.
    save_cmd_file(filename, &toml_contet)

    remember_source_url(url)?;
    
}

pub fn get_commands_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    let cmds_dir = proj_dirs.data_local_dir().join("cmds");
    cmds_dir.to_path_buf()
}

pub fn load_saved_commands() -> LookupVec {
    let cmds_path = get_commands_dir();
    let cmd_files = read_files_in_dir(&cmds_path);

    if cmd_files.is_empty() {
        println!("No commands found, downloading defaults...");
        add_source("https://raw.githubusercontent.com/WolfEYc/fsf/master/cmd/default.toml");
    }

    toml_content.try_into().unwrap()
}

pub fn read_files_in_dir(dir: &PathBuf) -> Vec<PathBuf> {
    let entries = fs::read_dir(dir).unwrap();
    entries.map(|e| e.unwrap().path()).collect()
}
