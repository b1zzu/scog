extern crate chrono;
extern crate git2;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_yaml;

use commands::help;
use core::context::Context;
use std::env;
use std::process;
use utils::error::Error;

mod commands;
mod core;
mod utils;

fn main() {
    match exec() {
        Ok(_) => {
            process::exit(0);
        }
        Err(e) => {
            println!("scog: {}", e.error());
            process::exit(1);
        }
    }
}

fn exec() -> Result<(), Error> {
    let mut context = Context::new();
    let mut args: Vec<String> = env::args().collect();
    // First args is the name of the program
    args.remove(0);
    if args.len() > 0 {
        match args.remove(0).as_str() {
            "--help" => help::exec(&mut context, &mut args),
            cmd => {
                match commands::exec(cmd) {
                    Some(f) => f(&mut context, &mut args),
                    None => Err(format!("'{}' is not a valid COMMAND.", cmd))?,
                }
            }
        }
    } else {
        Err(format!("no COMMAND defined."))?
    }
}