use crate::cmd::LookupVec;
use color_eyre::{eyre::ErrReport, Result};
use directories::ProjectDirs;
use prettytable::{row, table};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self},
    path::PathBuf,
};
use tokio::task::JoinSet;
use url::Url;

static DEFAULT_SOURCE: &str =
    "https://raw.githubusercontent.com/WolfEYc/fsf/master/cmd/default.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    pub url: String,
    pub filepath: PathBuf,
}
#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub sources: Vec<Source>,
}

impl Source {
    pub async fn update_commands(self) -> Result<(Self, LookupVec)> {
        let res = reqwest::get(&self.url).await?;
        let toml_content: LookupVec = toml::from_str(&res.text().await?)?;
        toml_content.save(&self.filepath).await?;
        Ok((self, toml_content))
    }
}

impl Config {
    fn new() -> Result<Self> {
        let mut config = Config::default();
        config.add_source(DEFAULT_SOURCE)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<&Self> {
        let str_toml = toml::to_string_pretty(&self)?;
        let filepath = Config::get_filepath();
        println!("writing config to: {}", filepath.to_str().unwrap());
        fs::write(&filepath, str_toml.as_bytes())?;
        Ok(self)
    }

    pub fn load() -> Result<Self> {
        let toml_file = fs::read_to_string(Config::get_filepath());
        Ok(match toml_file {
            Ok(toml_str) => toml::from_str(&toml_str)?,
            Err(_) => Config::new()?,
        })
    }

    fn get_filepath() -> PathBuf {
        let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
        let config_dir = proj_dirs.config_local_dir();
        let config_filepath = config_dir.join("config.toml");
        fs::create_dir_all(&config_dir).unwrap();
        config_filepath
    }

    pub fn add_source(&mut self, url: &str) -> Result<Source> {
        let url = url.to_string();
        for source in &self.sources {
            if source.url == url {
                return Err(ErrReport::msg(format!("source already added! {}", url)));
            }
        }

        let binding = Url::parse(&url)?;
        let filename = binding.path_segments().unwrap().last().unwrap();
        let filepath = get_cmds_dir().join(filename);

        let source = Source {
            url: url.to_string(),
            filepath,
        };

        self.sources.push(source.clone());

        Ok(source)
    }

    pub fn remove_source(&mut self, url: &str) -> Result<Source> {
        let source_idx = self
            .sources
            .iter()
            .position(|src| src.url == url)
            .ok_or(ErrReport::msg("Source Not Found"))?;
        fs::remove_file(self.sources[source_idx].filepath.clone())?;
        Ok(self.sources.swap_remove(source_idx))
    }

    pub async fn load_commands(&self) -> Result<LookupVec> {
        let mut master_table = LookupVec::new();
        let mut set = JoinSet::new();

        for source in &self.sources {
            set.spawn(LookupVec::load(source.filepath.clone()));
        }

        while let Some(lookup_vec) = set.join_next().await {
            let Ok(Ok(lookup_vec)) = lookup_vec else {
                println!("failed to load a lookup_table {}", lookup_vec.unwrap_err());
                continue;
            };
            master_table.merge(lookup_vec);
        }

        Ok(master_table)
    }

    pub fn print_sources(&self) -> &Self {
        let mut table = table!(["url", "filepath"]);
        table.extend(
            self.sources
                .iter()
                .map(|x| row![x.url, x.filepath.to_str().unwrap()]),
        );

        table.printstd();
        self
    }

    pub async fn update_commands(&self) -> Result<()> {
        let mut set = JoinSet::new();
        for source in &self.sources {
            set.spawn(source.clone().update_commands());
        }

        while let Some(res) = set.join_next().await {
            let (source, lookup_vec) = res??;
            println!(
                "pulled: {} cmds from {}",
                lookup_vec.commands.len(),
                source.url
            )
        }
        Ok(())
    }
}

fn get_cmds_dir() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    let cmds_dir = proj_dirs.data_local_dir().join("cmds");
    fs::create_dir_all(&cmds_dir).unwrap();
    cmds_dir
}
