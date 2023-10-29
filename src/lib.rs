use cmd::{
    execute_shell_command, get_commands_filepath, get_commands_from_local_data,
    write_commands_to_local_data,
};
use fzf::lookup;

pub mod cli;
pub mod cmd;
pub mod fzf;

pub fn find_n_execute() {
    let cmds = get_commands_from_local_data();
    let selected_cmd = lookup(cmds);
    let rendered_cmd = selected_cmd.command.render_command();
    execute_shell_command(&rendered_cmd)
}

pub fn download_from_url(url: String) {
    let res = reqwest::blocking::get(url).unwrap();

    write_commands_to_local_data(res.bytes().unwrap().as_ref())
}

pub fn open_cmds_csv_in_fav_text_editor() {
    let cmds_filepath = get_commands_filepath();
    open::that(cmds_filepath).unwrap();
}
