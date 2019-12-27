use std::env::current_dir;
use std::fs;
use std::string::String;

pub fn convert_resource_path(file_name: String) -> String {
    let p = current_dir();
    let mut path_buf = match p {
        Ok(d) => d,
        Err(error) => {
            panic!("{:?}", error);
        }
    };
    path_buf.push(String::from("resource"));
    path_buf.push(file_name);
    return path_buf.to_str().unwrap().to_string();
}

pub fn load_yaml(file_name: String) -> String {
    let contents = fs::read_to_string(file_name).expect("no file");
    return contents;
}
