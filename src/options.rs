use std::env;
use std::path;
use std::process;

pub struct Options<'a> {
    config: &'a path::Path
}

impl<'a> Options<'a> {
    fn new() -> Options<'a> {
        return Options {
            config: path::Path::new("config.yaml")
        };
    }

    pub fn get_config(&self) -> &'a path::Path {
        return &self.config;
    }
}

pub fn parse<'a>(args: &'a Vec<String>) -> Options<'a> {
    let mut o = Options::new();

    // first ( 0 ) arguments is the name of the program
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => help(),
            "--config" => {
                if (i + 1) < args.len() {
                    i = i + 1;
                    o.config = path::Path::new(args.get(i).unwrap());
                }
            }
            &_ => {
                println!("sync: error: options: '{}' is not valid", args[i]);
                process::exit(1);
            }
        }
        i = i + 1;
    }

    return o;
}

fn help() {
    println!("Usage: sync [--config FILE]");
    process::exit(0)
}
