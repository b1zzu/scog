use std::env;
use std::path::Path;
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;

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

    let config: &Path = Path::new(&config);
    if !config.is_file() {
        println!("sync: error: config file '{}' does not exists", config.to_string_lossy())
    }

    let mut config: File = File::open(config).expect("file not found");
    let mut contents: String = String::new();
    config.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("contents:\n{}", contents);
}

fn help() {
    println!("Usage: sync [--config FILE]");
    exit(0);
}