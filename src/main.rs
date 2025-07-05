mod util;
use clap::{Arg, Command, Subcommand, ValueEnum};
use serde;
use serde_yml;
use util::{Config, build_command, get_config_path};

use crate::util::{read_config, write_default_config};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let command_struct = build_command(Command::new("file-rustler"));

    let command_matches = command_struct.get_matches();

    let config_dir = get_config_path();
    let config: Config = read_config(config_dir.to_str().unwrap())?;

    match command_matches.subcommand() {
        Some(("run", sub_matches)) => {}
        Some(("sim", sub_matches)) => {}
        Some(("new", sub_matches)) => {
            write_default_config();
        }
        Some(("edit", sub_matches)) => {
            open::that(config_dir)?;
        }
        Some(("check", sub_matches)) => {}
        Some(("debug", sub_matches)) => {}
        Some(("show", sub_matches)) => {}
        _ => {}
    }
    Ok(())
}
