extern crate serde;
extern crate serde_yaml;

use std::fs;
use std::io::Read;
use std::path;
use std::str;

#[derive(Deserialize)]
pub struct Config {
    // TODO:
    // repository: String,
    files: Vec<File>,
}

#[derive(Deserialize)]
pub struct File {
    file: String,
    // TODO:
    // owner: String,
}

impl Config {
    pub fn get_files(&self) -> &Vec<File> {
        &self.files
    }

    pub fn load(config: &path::Path) -> Result<Config, String> {

        // Test config file
        if !config.is_file() {
            return Err(format!("the config file '{}' does not exists.", config.to_string_lossy()));
        }

        // Open and read config file
        let mut config = fs::File::open(config).unwrap();
        let mut buffer = Vec::new();
        config.read_to_end(&mut buffer).unwrap();

        // Parse toml config file to Config struct
        Ok(serde_yaml::from_str(str::from_utf8(&buffer).unwrap()).unwrap())
    }
}

impl File {
    pub fn get_file(&self) -> &String {
        &self.file
    }
}

