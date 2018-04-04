use std::env;
use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

extern crate toml;

#[macro_use]
extern crate serde_derive;

extern crate serde;

#[derive(Deserialize, Debug)]
struct Config {
    main: Main,
}

#[derive(Deserialize, Debug)]
struct Main {
    repository: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut config: String = String::from("config.toml");

    // first ( 0 ) arguments is the name of the program
    let mut i: usize = 1;
    while i < args.len() {
        if args[i] == String::from("--help") {
            help();
        } else if args[i] == String::from("--config") {
            if (i + 1) < args.len() {
                i = i + 1;
                config = args[i].clone();
            }
        }
        i = i + 1;
    }

    // Test config file
    let config: &Path = Path::new(&config);
    if !config.is_file() {
        println!("sync: error: config file: '{}' does not exists", config.to_string_lossy());
        exit(1);
    }

    // Open and read config file
    let mut config: File = File::open(config).expect("file not found");
    let mut contents: String = String::new();
    config.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // Parse toml config file to Config struct
    let config: Config = toml::from_str(contents.as_str()).unwrap();

    // Simple repository clone
    let output = Command::new("git")
        .arg("clone")
        .arg(config.main.repository.as_str())
        .arg("sync")
        .current_dir("/tmp")
        .output()
        .expect("command failed");

    if output.status.code().expect("no status") != 0 {
        println!("sync: error: failed to clone repository: '{}'", config.main.repository);
        exit(1);
    }

    println!("Config:\n{:?}", config);
}

fn help() {
    println!("Usage: sync [--config FILE]");
    exit(0);
}