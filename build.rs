use std::env;
use std::path::PathBuf;
use std::fs;

fn main() {
    let out = PathBuf::from(env::var("OUT_DIR").unwrap());
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let assets = root.join("assets");

    build_sounds(&assets, &out);
}

fn build_sounds(assets: &PathBuf, out: &PathBuf) {
    let mut sounds: Vec<serde_json::Value> = serde_json::from_str(include_str!("assets/sounds.json")).unwrap();
    let sounds = sounds.iter().map(|sound| format!(
        "sounds_map.insert(\"{0}\", sounds_registry.register(\"{1}\", Sound::new(\"{1}\")));",
        &sound["key"].as_str().unwrap(),
        &sound["location"].as_str().unwrap(),
    )).collect::<Vec<String>>().join("\n");

    let out = out.join("sounds").with_extension("rs");
    fs::write(&out, format!("{{ {} }}", sounds)).unwrap();

    println!("cargo:rerun-if-changed=assets/sounds.json");
}