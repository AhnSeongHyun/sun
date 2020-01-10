#[cfg(test)]
mod test {
    use clap::ArgMatches;
    use std::collections::HashMap;
    use std::string::String;
    use sun::arg::parse_arg;

    #[test]
    fn test_parse_arg_empty_case() {
        let mut args = HashMap::new();
        let empty_matches = ArgMatches {
            args: args,
            subcommand: None,
            usage: None,
        };
        assert_eq!(parse_arg(matches), (String::from(""), String::from("")));
    }

    #[test]
    fn test_parse_arg() {
        let mut args = HashMap::new();
        let empty_matches = ArgMatches {
            args: args,
            subcommand: None,
            usage: None,
        };
        assert_eq!(parse_arg(matches), (String::from(""), String::from("")));
    }

}
