use directories::ProjectDirs;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, File},
};

#[derive(Serialize, Deserialize)]
struct CommandLookup {
    keyword: String,
    command: Command,
}

struct Command {
    command: String,
    params: Vec<String>,
}

impl From<String> for Command {
    fn from(command: String) -> Self {
        let re = Regex::new(r"\{(\w+)\}").unwrap();
        let params = re
            .captures_iter(&command)
            .map(|capture| capture[1].to_string())
            .collect();
        Command { command, params }
    }
}

pub fn deserialize(file: File) -> HashMap<String, String> {
    let mut map = HashMap::new();

    let mut rdr = csv::Reader::from_reader(file);

    for row in rdr.deserialize() {
        let cmd: SerializedCommandRow = row.unwrap();
        map[&cmd.keyword] = cmd.command;
    }

    map
}

pub fn get_commands_from_local_data() -> HashMap<String, String> {
    let proj_dirs = ProjectDirs::from("com", "wolfey", "ecli").unwrap();
    let local_data = proj_dirs.data_local_dir();
    let cmds_filepath = local_data.join("ecli_cmds.csv");
    let cmds_file = fs::File::open(cmds_filepath).unwrap();
    deserialize(cmds_file)
}
