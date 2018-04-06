use std::collections;
use std::fs;
use std::io::Read;
use std::path;
use std::process;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub main: Main,
    pub files: collections::HashMap<String, File>,
}

#[derive(Deserialize)]
pub struct Main {
    pub repository: String,
}

#[derive(Deserialize)]
pub struct File {
    pub file: String,
    pub owner: String,
}

pub fn load(file: &String) -> Config {

    // Test config file
    let config = path::Path::new(file);
    if !config.is_file() {
        println!("sync: error: config file: '{}' does not exists", config.to_string_lossy());
        process::exit(1);
    }

    // Open and read config file
    let mut config = fs::File::open(config).unwrap();
    let mut contents = String::new();
    config.read_to_string(&mut contents).unwrap();

    // Parse toml config file to Config struct
    return toml::from_str(contents.as_str()).unwrap();
}