use std::env;
use std::process::exit;

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

    println!("config: {}", config);
}

fn help() {
    println!("Usage: sync [--config FILE]");
    exit(0);
}