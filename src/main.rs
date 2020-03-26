extern crate java_props;

pub mod world;
pub mod util;
pub mod settings;

use std::env;
use std::path::PathBuf;
use settings::Settings;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    let mut settings = Settings::load(path);

    println!("Easy: {:?}", settings.difficulty());
}
