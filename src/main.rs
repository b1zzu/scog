extern crate chrono;
extern crate core;
extern crate regex;
#[macro_use]
extern crate serde_derive;

use app::App;
use app::Result;
use options::Options;
use std::env;
use std::process;

mod options;
mod config;
mod git;
mod repository;
mod app;

fn main() {
    let options = options().unwrap();

    let repository = env::home_dir().unwrap().join(".scog/");
    let repository = repository.as_path();

    if *options.get_help() {
        help();
        process::exit(0);
    }

    let app = App::new(repository);

    let result: Result = app.route(options);

    match result {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            error(e);
        }
    }
}

fn options() -> Option<Options> {
    let result = Options::parse(env::args().collect());
    match result {
        Ok(options) => {
            Some(options)
        }
        Err(e) => {
            error(e);
            None
        }
    }
}

fn error(e: String) {
    println!("scog: {}", e);
    process::exit(1);
}

fn help() {
    println!("Usage: scog COMMAND [ARGS]");
    println!(" ");
    println!("Command:");
    println!("  clone           ...");
    println!("  checkout        ...");
    println!("  pull            ...");
    println!("  push            ...");
}
