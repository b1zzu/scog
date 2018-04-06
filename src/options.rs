use std;

pub struct Options {
    config: String
}

impl Options {
    fn new() -> Options {
        return Options {
            config: String::from("config.toml")
        };
    }

    pub fn get_config(&self) -> &String {
        return &self.config;
    }
}

pub fn parse() -> Options {
    let args: Vec<String> = std::env::args().collect();
    let mut o = Options::new();

    // first ( 0 ) arguments is the name of the program
    let mut i: usize = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" => help(),
            "--config" => {
                if (i + 1) < args.len() {
                    i = i + 1;
                    o.config = args.get(i).unwrap().clone();
                }
            }
            &_ => {
                println!("sync: error: options: '{}' is not valid", args[i]);
                std::process::exit(1);
            }
        }
        i = i + 1;
    }

    return o;
}

fn help() {
    println!("Usage: sync [--config FILE]");
    std::process::exit(0)
}
