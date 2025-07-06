use crate::config;
use clap::Command;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

pub fn read_config_file(path: &str) -> Result<String, Box<dyn Error>> {
    let path = shellexpand::tilde(path).to_string();
    fs::read_to_string(&path).map_err(Into::into)
}

pub fn read_config(path: &str) -> Result<Config, Box<dyn Error>> {
    let config_content = read_config_file(path)?;
    let config: Config = serde_yml::from_str(&config_content)?;
    Ok(config)
}

pub fn get_config_path() -> PathBuf {
    env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            env::home_dir()
                .map(|home| home.join(".config"))
                .unwrap_or_else(|| PathBuf::from(".config"))
        })
        .join("file-rustler")
        .join("config.yaml")
}

pub fn build_command(cmd: Command) -> Command {
    cmd.author("Gerald Lee Yurek III, g3@yurek.me")
        .version("0.0.1")
        .about("A program that organizes files based on a YAML config file")
        .subcommand(Command::new("run").about("Organize files"))
        .subcommand(Command::new("sim").about("Simulate organizing files"))
        .subcommand(Command::new("new").about("Create a YAML template config file"))
        .subcommand(Command::new("edit").about("Edit the YAML config file with $EDITOR"))
        .subcommand(Command::new("check").about("Check validity of Config file"))
        .subcommand(Command::new("debug").about("Shows the raw config parsing steps"))
        .subcommand(Command::new("show").about("Print the config file to stdout"))
}

pub fn write_default_config() {
    let default_config = r#"
# Name / Explanation of rule
#--------------------------------------------------------
rule: "Lorem Ipsum"
locations: 
      - ~/Downloads

"#;
}
