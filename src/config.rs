use std::fs::File;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct Rule {
    name: String,
    enabled: bool,
    targets: String,
    locations: Vec<Location>,
    subfolders: bool,
    filter_mode: String,
    filters: Vec<Filter>,
    actions: Vec<Action>,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    path: String,
    min_depth: u32,
    max_depth: u32,
    search: String,
    exclude_files: Vec<String>,
    exclude_dirs: Vec<String>,
    system_exclude_files: Vec<String>,
    system_exclude_dirs: Vec<String>,
    ignore_errors: bool,
    filter: Vec<String>,
    filter_dirs: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub enum Filter {
    created(Time),
    duplicate,
    empty,
    exif,
    extension(Vec<String>),
    filecontent(String),
    hash(String),
    lastmodified(Time),
    mimetype(Vec<String>),
    name(NameFilterStruct),
    regex(String),
    size(Vec<String>),
}

#[derive(Deserialize, Debug)]
pub struct Time {
    years: u32,
    months: u32,
    weeks: f32,
    days: f32,
    hours: f32,
    seconds: f32,
    mode: String,
}

#[derive(Deserialize, Debug)]
pub struct NameFilterStruct {
    matches: Vec<String>,
    startswith: Vec<String>,
    containst: Vec<String>,
    endswith: Vec<String>,
    case_sensitive: bool,
}

#[derive(Deserialize, Debug)]
pub enum Action {
    confirm(String),
    copy(FileActionStruct),
    delete,
    echo(String),
    hardlink(FileActionStruct),
    mv(FileActionStruct),
    rename(FileActionStruct),
    shell(ShellActionStruct),
    symlink(FileActionStruct),
    trash,
    write(WriteActionStruct),
}
#[derive(Deserialize, Debug)]
pub struct FileActionStruct {
    dest: String,
    on_conflict: String,
    rename_template: String,
    autodetect_folder: bool,
    continue_with: String,
}

#[derive(Deserialize, Debug)]
pub struct ShellActionStruct {
    cmd: String,
    run_in_simulation: bool,
    ignore_errors: bool,
    simulation_output: String,
    simulation_resturncode: u32,
}

#[derive(Deserialize, Debug)]

pub struct WriteActionStruct {
    text: String,
    outfile: String,
    mode: String,
    encoding: String,
    newline: String,
    clear_before_first_write: bool,
}
