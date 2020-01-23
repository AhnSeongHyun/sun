extern crate clap;
use clap::{App, ArgMatches};
use std::string::String;

use sun::arg::{get_arg_matches, parse_arg};
use sun::config::Config;

fn main() {
    let matches = get_arg_matches();
    let (host_file_path, input_command) = parse_arg(matches);

    if host_file_path.is_empty() {
        panic!("Invalid Argument");
    }

    let config = Config { host_file_path };
}
