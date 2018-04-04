#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate regex;
extern crate toml;

use std::env;
use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

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
        match args[i].as_str() {
            "--help" => help(),
            "--config" => {
                if (i + 1) < args.len() {
                    i = i + 1;
                    config = args[i].clone();
                }
            }
            &_ => {
                println!("sync: error: options: '{}' is not valid", args[i]);
                exit(1);
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
    let mut config: File = File::open(config).unwrap();
    let mut contents: String = String::new();
    config.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // Parse toml config file to Config struct
    let config: Config = toml::from_str(contents.as_str()).unwrap();

    // If repository already exists and is clean pull it, otherwise should be fixed manually
    if Path::new("/tmp/sync").is_dir() {

        // Check if the repository is up to date
        let output = Command::new("git")
            .arg("pull")
            .arg("--ff-only")
            .current_dir("/tmp/sync")
            .output()
            .unwrap();

        if output.status.code().unwrap() != 0 {
            println!("sync: error: failed to git pull dir: '{}'", "/tmp/sync");
            println!("  you need to fix this problem manually");
            exit(1);
        }

    } else {

        // Simple repository clone
        let output = Command::new("git")
            .arg("clone")
            .arg(config.main.repository.as_str())
            .arg("sync")
            .current_dir("/tmp")
            .output()
            .unwrap();

        if output.status.code().unwrap() != 0 {
            println!("sync: error: failed to clone repository: '{}'", config.main.repository);
            exit(1);
        }
    }

    // Loop on configured files and copy them to the repository

    // Commit the repository

    println!("Config:\n{:?}", config);
}

fn help() {
    println!("Usage: sync [--config FILE]");
    exit(0);
}