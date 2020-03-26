extern crate java_props;

pub mod world;
pub mod util;
pub mod settings;

use std::env;
use settings::Settings;

fn main() {
    let mut path = env::current_dir().unwrap();
    path.push("server.properties");

    let settings = Settings::load(path);

    println!("{:?}", settings);
}
