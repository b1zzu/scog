extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::io::Read;
use std::path;
use std::process;
use std::str;

#[derive(Deserialize)]
pub struct Config {
    pub repository: String,
    pub files: Vec<File>,
}

#[derive(Deserialize)]
pub struct File {
    pub file: String,
    pub owner: String,
}

pub fn load(config: &path::Path) -> Config {

    // Test config file
    if !config.is_file() {
        println!("sync: error: The config file '{}' does not exists", config.to_string_lossy());
        process::exit(1);
    }

    // Open and read config file
    let mut config = fs::File::open(config).unwrap();
    let mut buffer = Vec::new();
    config.read_to_end(&mut buffer).unwrap();

    // Parse toml config file to Config struct
    return serde_yaml::from_str(str::from_utf8(&buffer).unwrap()).unwrap();
}