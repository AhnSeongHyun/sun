pub mod config {
    use crate::entity::entity::Host;
    use std::collections::LinkedList;
    use std::env::current_dir;
    use std::fs;
    use std::string::String;

    extern crate yaml_rust;

    use yaml_rust::YamlLoader;

    #[derive(Debug)]
    pub struct Config {
        pub host_file_path: String,
    }

    pub fn read_config_file(file_name: String) -> String {
        let contents = fs::read_to_string(file_name).expect("no file");
        return contents;
    }

    pub fn load_hosts(content: String) -> LinkedList<Host> {
        let docs = YamlLoader::load_from_str(&content).unwrap();
        if docs.len() != 1 {
            panic!("invalid format");
        }
        let doc = &docs[0];
        println!("{:?}", doc["hosts"]);
        return LinkedList::new();
    }
}
