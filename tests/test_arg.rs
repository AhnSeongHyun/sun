#[cfg(test)]
mod test {
    use clap::{App, Arg, ArgMatches, Values};
    use std::collections::HashMap;
    use std::string::String;
    use sun::arg::parse_arg;

    #[test]
    fn test_parse_arg_empty_case() {
        let matches = App::new("empty test").get_matches();
        assert_eq!(parse_arg(matches), (String::from(""), String::from("")));
    }

    #[test]
    #[ignore]
    fn test_parse_arg() {
        // TODO : https://github.com/clap-rs/clap/issues/1616
    }

}
