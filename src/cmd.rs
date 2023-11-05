use color_eyre::Result;
use regex::Regex;
use serde::{Deserialize, Serialize};
use skim::SkimItem;
use std::{
    borrow::Cow,
    fs,
    io::{self},
    path::PathBuf,
    process::Command,
};
use tera::{Context, Tera};

#[derive(Debug, Serialize, Deserialize)]
pub struct LookupVec {
    pub commands: Vec<CommandLookup>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandLookup {
    pub keyword: String,
    pub command: Cmd,
}

#[derive(Clone, Debug)]
pub struct Cmd {
    command: String,
    params: Vec<String>,
}

impl Cmd {
    pub fn render(&self) -> String {
        let mut tera = Tera::default();
        println!("Rendering command: {}", self.command);
        println!("Reading params: {:?}", self.params);
        let mut context = Context::new();
        for param in self.params.iter() {
            println!("bruh");

            context.insert(param, &input_prompt(format!("{param}:")));
        }

        tera.render_str(&self.command, &context).unwrap()
    }

    pub fn execute(&self) {
        let command = self.render();
        Command::new("sh").arg("-c").arg(command).spawn().unwrap();
    }
}

impl SkimItem for CommandLookup {
    fn text(&self) -> Cow<str> {
        Cow::Borrowed(self.keyword.as_str())
    }
}

impl From<String> for Cmd {
    fn from(command: String) -> Self {
        let re = Regex::new(r"\{\{(\s*\w+\s*)\}\}").unwrap();
        let params = re
            .captures_iter(&command)
            .map(|capture| capture[1].trim().to_string())
            .collect();

        Cmd { command, params }
    }
}

impl From<Cmd> for String {
    fn from(command: Cmd) -> Self {
        command.command
    }
}

impl Serialize for Cmd {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bytes(String::from(self.to_owned()).as_bytes())
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

impl TryFrom<String> for LookupVec {
    type Error = toml::de::Error;

    fn try_from(file: String) -> Result<Self, Self::Error> {
        toml::from_str(file.as_str())
    }
}

impl TryFrom<PathBuf> for LookupVec {
    type Error = color_eyre::Report;

    fn try_from(value: PathBuf) -> Result<Self, Self::Error> {
        let file_str = fs::read_to_string(value)?;
        Ok(toml::from_str(&file_str)?)
    }
}

impl LookupVec {
    pub fn new() -> Self {
        LookupVec {
            commands: Vec::new(),
        }
    }
    pub fn save(self, filepath: &PathBuf) -> Result<Self> {
        let toml_str = toml::to_string_pretty(&self)?;
        fs::write(filepath, toml_str.as_bytes())?;
        Ok(self)
    }
    pub fn merge(&mut self, mut other: LookupVec) -> &Self {
        self.commands.append(&mut other.commands);
        self
    }
}

fn read_line() -> String {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("!");
    str
}

fn input_prompt(prompt: String) -> String {
    println!("{}", prompt);
    read_line()
}
