mod misc;
use std::string::String;

fn main() {
    let file_path: String = misc::convert_resource_path(String::from("host.yaml"));
    let content = misc::load_yaml(file_path);
    println!("{}", content); 
}
