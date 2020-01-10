#[cfg(test)]
mod test {
    use std::string::String;
    use sun::misc::load_yaml;

    #[test]
    #[should_panic]
    fn test_load_yaml_abnormal_path() {
        load_yaml(String::from("test"));
    }

}
