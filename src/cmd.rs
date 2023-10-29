use directories::ProjectDirs;
use handlebars::Handlebars;
use regex::Regex;
use serde::Deserialize;
use skim::SkimItem;
use std::{
    borrow::Cow,
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{self, Write},
    path::PathBuf,
    process::Command,
};

use crate::download_from_url;

#[derive(Deserialize, Clone)]
pub struct CommandLookup {
    pub keyword: String,
    pub command: Cmd,
}

#[derive(Clone)]
pub struct Cmd {
    command: String,
    params: Vec<String>,
}

impl Cmd {
    pub fn render_command(&self) -> String {
        let mut handlebars = Handlebars::new();
        handlebars.register_escape_fn(handlebars::no_escape);
        let mut context = BTreeMap::new();
        for param in self.params.iter() {
            context.insert(param, input_prompt(format!("{param}:")));
        }

        handlebars.render_template(&self.command, &context).unwrap()
    }
}

impl SkimItem for CommandLookup {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(self.keyword.as_str())
    }
}

impl From<String> for Cmd {
    fn from(command: String) -> Self {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let params = re
            .captures_iter(&command)
            .map(|capture| capture[1].to_string())
            .collect();
        Cmd { command, params }
    }
}

impl<'de> Deserialize<'de> for Cmd {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(String::deserialize(deserializer)?.into())
    }
}

fn deserialize(file: String) -> Vec<CommandLookup> {
    toml::from_str(file.as_str()).unwrap()
}

pub fn get_commands_filepath() -> PathBuf {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "fsf").unwrap();
    let local_data = proj_dirs.data_local_dir();
    let cmds_filepath = local_data.join("ecli_cmds.toml");
    cmds_filepath
}

pub fn get_commands_from_local_data() -> Vec<CommandLookup> {
    let cmds_path = get_commands_filepath();
    let Ok(toml_content) = fs::read_to_string(cmds_path) else {
        download_from_url("https://github.com/WolfEYc/fsf/blob/master/cmd/default.toml");
        return get_commands_from_local_data();
    };
    deserialize(toml_content)
}

pub fn write_commands_to_local_data(cmds_toml: &[u8]) {
    let cmds_path = get_commands_filepath();
    let mut file = OpenOptions::new().append(true).open(cmds_path).unwrap();
    file.write_all(b"\n").unwrap();
    file.write_all(cmds_toml).unwrap();
}

pub fn execute_shell_command(command: &str) {
    Command::new(command).spawn().unwrap();
}

pub fn input_prompt(prompt: String) -> String {
    println!("{}", prompt);
    input()
}

pub fn input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
