type HostFilePath = String;
type Input = String;
use clap::{App, ArgMatches};

pub fn parse_arg(matches: ArgMatches) -> (HostFilePath, Input) {
    let host_file_path = matches.value_of("config").unwrap_or("");
    let input_command = matches.value_of("INPUT").unwrap_or("");
    return (host_file_path.to_string(), input_command.to_string());
}

pub fn get_arg_matches() -> ArgMatches<'static> {
    return App::new("sun")
        .version("1.0")
        .author("ash84 <me@ash84.io>")
        .about("Run shell command to multiple hosts")
        .args_from_usage(
            "-c, --config=[FILE] 'Sets a config.yaml file'
            <INPUT>            'Sets the input file to use'
            ",
        )
        .get_matches();
}
