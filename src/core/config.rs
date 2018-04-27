use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::str;
use serde_yaml;
use utils::error::Error;

#[derive(Deserialize)]
pub struct Config {
    sections: Vec<Section>,
}

#[derive(Deserialize)]
pub struct Section {
    path: String,
}

impl Config {
    pub fn new(config: &PathBuf) -> Result<Config, Error> {

        // Test config file
        if !config.is_file() {
            return Err(format!("the config file '{}' does not exists.", config.to_string_lossy()))?;
        }

        // Open and read config file
        let mut config = File::open(config)?;
        let mut buffer = Vec::new();
        config.read_to_end(&mut buffer)?;

        // Parse toml config file to Config struct
        let content = str::from_utf8(&buffer)?;
        Ok(serde_yaml::from_str(content)?)
    }

    pub fn sections(&self) -> &Vec<Section> {
        &self.sections
    }
}

impl Section {
    pub fn path(&self) -> PathBuf {
        PathBuf::from(&self.path)
    }
}

impl Clone for Config {
    fn clone(&self) -> Self {
        let mut sections: Vec<Section> =  vec![];
        for section in &self.sections {
            sections.push(section.clone());
        }
        Config { sections }
    }
}

impl Clone for Section {
    fn clone(&self) -> Self {
        Section { path: self.path.clone() }
    }
}