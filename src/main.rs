use clap::Parser;
use fsf::*;

pub fn main() {
    let cli = cli::Args::parse();

    let Some(cmd) = cli.cmd else {
        find_n_execute();
        return;
    };

    match cmd {
        cli::Commands::Edit => open_cmds_csv_in_fav_text_editor(),
        cli::Commands::Download { url } => download_from_url(url),
    }
}
