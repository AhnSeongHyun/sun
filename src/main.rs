mod config;
mod misc;
extern crate clap;
use clap::{App, ArgMatches};
use std::string::String;

type HostFilePath = String;
type Input = String;

fn main() {
    let matches = App::new("sun")
        .version("1.0")
        .author("ash84 <me@ash84.io>")
        .about("Run shell command to multiple hosts")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a config.yaml file'
            <INPUT>            'Sets the input file to use'
            ",
        )
        .get_matches();

    let (host_file_path, input_command) = parse_arg(matches);

    if host_file_path.is_empty() {
        panic!("Invali Argument");
    }

    let config = config::Config {
        host_file_path: host_file_path,
    };

    // let file_path: String = misc::convert_resource_path(String::from("host.yaml"));
    // let content = misc::load_yaml(file_path);
    println!("{:#?}", input_command);
    println!("{:#?}", config);
}

fn parse_arg(matches: ArgMatches) -> (HostFilePath, Input) {
    let host_file_path = matches.value_of("config").unwrap_or("");
    let input_command = matches.value_of("INPUT").unwrap_or("");
    return (host_file_path.to_string(), input_command.to_string());
}
