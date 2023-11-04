use config::{get_commands_dir, load_saved_commands};
use fzf::lookup;

pub mod cli;
pub mod cmd;
pub mod config;
pub mod fzf;

pub fn find_n_execute() {
    let cmds = load_saved_commands();
    let selected_cmd = lookup(cmds);
    selected_cmd.command.execute();
}

pub fn open_cmds_toml_in_fav_text_editor() {
    let cmds_filepath = get_commands_dir();
    open::that(cmds_filepath).unwrap();
}
