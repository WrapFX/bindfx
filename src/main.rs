// Crates
extern crate bindgen;
extern crate toml;
extern crate serde;
extern crate serde_derive;

// Imports
use std::fs;
use std::io::Write;
use std::path::Path;

// Constants
const DEFAULT_CONFIG: &str = include_str!("default_config.toml");

fn main() {
    if Path::new("./config.toml").exists() {
        create_bindings();
    } else {
        create_default_config();
    }
}

fn create_bindings() {
    // Get config from config file
    let config: Config = toml::from_str(fs::read_to_string("./config.toml").expect("Couldn't read config file").as_str())
        .expect("Couldn't parse config file");

    // Set bindgen config
    println!("cargo:rustc-link-search={}", config.input.path);
    println!("cargo:rustc-link-lib={}", config.input.name);
    println!("cargo:rerun-if-changed={}", config.input.wrapper);

    let bindings = bindgen::Builder::default()
        .header(format!("{}/{}", config.input.path, config.input.wrapper))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let contents = bindings.to_string();
    let mut file = fs::File::create(format!("{}/{}.rs", config.output.path, config.output.name))
        .expect("Couldn't create output file");
    file.write_all(contents.as_bytes()).expect("Couldn't write to output file");
}

fn create_default_config() {
    let mut file = fs::File::create("./config.toml").expect("Couldn't create config file");
    file.write_all(DEFAULT_CONFIG.as_bytes()).expect("Couldn't write default config to file");
    println!("Succesfully created config file");
}

// Config
#[derive(Debug, serde_derive::Deserialize)]
struct Config {
    input: InputConfig,
    output: OutputConfig,
}

#[derive(Debug, serde_derive::Deserialize)]
struct InputConfig {
    path: String,
    name: String,
    wrapper: String,
}

#[derive(Debug, serde_derive::Deserialize)]
struct OutputConfig {
    path: String,
    name: String,
}