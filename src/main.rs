extern crate clap;

mod arg;
mod config;
mod entity;

use crate::arg::arg::{get_arg_matches, parse_arg};
use crate::config::config::{load_hosts, read_config_file, Config};

fn main() {
    let matches = get_arg_matches();
    let (host_file_path, input_command) = parse_arg(matches);

    if host_file_path.is_empty() {
        panic!("Invalid Argument");
    }

    let config = Config { host_file_path };
    let config_content = read_config_file(config.host_file_path);
    let hosts = load_hosts(config_content);
}
