extern crate chrono;
extern crate core;
extern crate regex;
#[macro_use]
extern crate serde_derive;

use chrono::DateTime;
use chrono::offset::Local;
use core::result::Result as CoreResult;
use options::Options;
use regex::Regex;
use repository::Repository;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

mod options;
mod config;
mod git;
mod repository;

type Result = CoreResult<(), String>;

fn main() {
    let options = options().unwrap();

    let repository = env::home_dir().unwrap().join(".scog/");
    let repository = repository.as_path();

    if *options.get_help() {
        help();
        process::exit(0);
    }

    let result: Result = match *options.get_command() {
        options::Command::None => {
            help();
            Err(String::from("no command defined"))
        }

        options::Command::Clone => {
            clone(repository, options.get_repo())
        }

        options::Command::Checkout => {
            checkout(repository, options.get_branch())
        }

        options::Command::Pull => {
            pull(repository)
        }

        options::Command::Push => {
            push(repository)
        }
    };

    match result {
        Ok(()) => {
            process::exit(0)
        }
        Err(e) => {
            error(e)
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
    process::exit(1)
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

fn clone(repository: &Path, remote: &String) -> Result {
    Repository::new(repository).clone(remote).unwrap();
    Ok(())
}

fn checkout(repository: &Path, branch: &String) -> Result {
    let r = Repository::new(repository);
    let o = r.checkout(branch);
    if o.is_err() {
        r.checkout_new(branch).unwrap();
    }
    Ok(())
}

fn pull(repository: &Path) -> Result {
    let r = Repository::new(repository);

    if !r.is_clean() {
        return Err(String::from("repository is not clean"));
    }

    let current_branch = r.get_current_branch();

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        return Err(format!("can not pull from backup branch: '{}'.", current_branch));
    }

    let config = match config::load(repository.join("config.yaml").as_path()) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    let now: DateTime<Local> = Local::now();
    let backup_branch = format!("_backup_{}_{}", current_branch, now.format("%F_%H-%M-%S_%f"));
    let backup_branch = backup_branch.as_str();

    r.checkout_new(&backup_branch).unwrap();

    for file in config.get_files() {
        // TODO: Handle dirs
        let source = Path::new(file.get_file());
        let destination = repository.join(&source.to_owned().strip_prefix("/").unwrap());
        let destination = destination.as_path();

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        fs::copy(source, &destination).unwrap();

        r.add(destination).unwrap();
    }

    let mut delete = false;
    if !r.is_clean() {
        let now: DateTime<Local> = Local::now();
        r.commit(now.to_string().as_str()).unwrap();
        r.push_new_branch().unwrap();
    } else {
        delete = true;
    }

    r.checkout(current_branch.as_str()).unwrap();
    if delete {
        r.branch_delete(&backup_branch).unwrap();
    }

    for file in config.get_files() {
        // TODO: Handle dirs
        let destination = Path::new(file.get_file());
        let source = repository.join(&destination.to_owned().strip_prefix("/").unwrap());
        let source = source.as_path();

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        fs::copy(source, &destination).unwrap();
    }

    Ok(())
}

fn push(repository: &Path) -> Result {
    let r = Repository::new(repository);

    if !r.is_clean() {
        return Err(String::from("repository is not clean"));
    }

    let current_branch = r.get_current_branch();

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        return Err(format!("can not pull from backup branch: '{}'.", current_branch));
    }

    let config = match config::load(repository.join("config.yaml").as_path()) {
        Ok(c) => c,
        Err(e) => return Err(e),
    };

    for file in config.get_files() {
        // TODO: Handle dirs
        let source = Path::new(file.get_file());
        let destination = repository.join(&source.to_owned().strip_prefix("/").unwrap());
        let destination = destination.as_path();

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        fs::copy(source, &destination).unwrap();

        r.add(destination).unwrap();
    }

    if !r.is_clean() {
        let now: DateTime<Local> = Local::now();
        r.commit(now.to_string().as_str()).unwrap();
    }

    let result = pull(repository);
    if result.is_err() {
        return result;
    }

    r.push().unwrap();

    Ok(())
}