extern crate clap;
use clap::App;

fn main() {
    let matches = App::new("sun")
        .version("0.0.1")
        .about("Utils for me")
        .author("ash84")
        .get_matches();
}
