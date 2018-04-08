extern crate chrono;
extern crate regex;
#[macro_use]
extern crate serde_derive;

use chrono::DateTime;
use chrono::offset::Local;
use git::Git;
use regex::Regex;
use std::env;
use std::env::home_dir;
use std::fs;
use std::path::Path;
use std::process::exit;

mod options;
mod config;
mod git;

fn main() {
    let args = env::args().collect();
    let options = options::parse(&args);

    let repository = home_dir().unwrap().join(".bog/");
    let repository = repository.as_path();

    if *options.get_help() {
        help();
        exit(0);
    }

    let result: Result<(), String> = match *options.get_command() {
        options::Command::None => {
            help();
            Err(String::new())
        },

        options::Command::Clone => {
            clone(options.get_repo(), repository)
        },

        options::Command::Checkout => {
            checkout(repository, options.get_branch())
        },

        options::Command::Pull => {
            pull(repository)
        },

        options::Command::Push => {
            push(repository)
        },
    };

    match result {
        Ok(()) => {
            exit(0)
        }
        Err(e) => {
            println!("bog: {}", e);
            exit(1)
        }
    }
}

fn help() {
    println!("Usage: bog COMMAND [ARGS]");
    println!(" ");
    println!("Command:");
    println!("  clone           ...");
    println!("  checkout        ...");
    println!("  pull            ...");
    println!("  push            ...");
}

fn clone(repo: &String, dir: &Path) -> Result<(), String> {
    Git::new(None).arg("clone").arg(repo).arg(dir).execute().unwrap();
    Ok(())
}

fn checkout(repository: &Path, branch: &String) -> Result<(), String> {
    let result = Git::new(Option::from(repository)).arg("checkout").arg(&branch).execute();
    if result.is_err() {
        Git::new(Option::from(repository)).arg("checkout").arg("-b").arg(branch).execute().unwrap();
    }
    Ok(())
}

fn pull(repository: &Path) -> Result<(), String> {
    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        return Err(format!("'{}' is not clean. Clean it manually.", repository.to_string_lossy()));
    }

    let output = Git::new(Option::from(repository)).arg("rev-parse").arg("--abbrev-ref").arg("HEAD").execute().unwrap();
    let current_branch = String::from_utf8(output.stdout).unwrap();
    let current_branch = String::from(current_branch.trim());

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        return Err(format!("can not pull from backup branch: '{}'.", current_branch));
    }

    let result = Git::new(Option::from(repository)).arg("pull").arg("--ff-only").execute();
    if result.is_err() {
        return Err(format!("can not merge remote changes with local changes.\n  Fix it manually in: '{}'.\n  Then 'bog pull' to update local files.", repository.to_string_lossy()))
    }

    let now: DateTime<Local> = Local::now();
    let config = config::load(repository.join("config.yaml").as_path());

    // TODO: The backup branch should be called with the same name of the current branch
    let backup_branch = format!("_backup_{}_{}", current_branch, now.format("%F_%H-%M-%S_%f"));

    Git::new(Option::from(repository)).arg("checkout").arg("-b").arg(&backup_branch).execute().unwrap();

    for file in config.get_files() {
        // TODO: Handle dirs
        let source = Path::new(file.get_file());
        let destination = repository.join(&source.to_owned().strip_prefix("/").unwrap());
        let destination = destination.as_path();

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        fs::copy(source, &destination).unwrap();

        Git::new(Option::from(repository)).arg("add").arg(destination).execute().unwrap();
    }

    let mut delete = false;
    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        let now: DateTime<Local> = Local::now();
        Git::new(Option::from(repository)).arg("commit").arg("-m").arg(now.to_string()).execute().unwrap();

        Git::new(Option::from(repository)).arg("push").arg("-u").arg("origin").arg("HEAD").execute().unwrap();
    } else {
        delete = true;
    }

    Git::new(Option::from(repository)).arg("checkout").arg(current_branch).execute().unwrap();

    if delete {
        Git::new(Option::from(repository)).arg("branch").arg("-d").arg(&backup_branch).execute().unwrap();
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

fn push(repository: &Path) -> Result<(), String> {
    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        return Err(format!("'{}' is not clean. Clean it manually.", repository.to_string_lossy()))
    }

    let output = Git::new(Option::from(repository)).arg("rev-parse").arg("--abbrev-ref").arg("HEAD").execute().unwrap();
    let current_branch = String::from_utf8(output.stdout).unwrap();
    let current_branch = String::from(current_branch.trim());

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        return Err(format!("can not push to backup branch '{}'", current_branch));
    }

    let config = config::load(repository.join("config.yaml").as_path());

    for file in config.get_files() {
        // TODO: Handle dirs
        let source = Path::new(file.get_file());
        let destination = repository.join(&source.to_owned().strip_prefix("/").unwrap());
        let destination = destination.as_path();

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        fs::copy(source, &destination).unwrap();

        Git::new(Option::from(repository)).arg("add").arg(destination).execute().unwrap();
    }

    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        let now: DateTime<Local> = Local::now();
        Git::new(Option::from(repository)).arg("commit").arg("-m").arg(now.to_string()).execute().unwrap();
    }

    let result = pull(repository);
    if result.is_err() {
        return result
    }

    Git::new(Option::from(repository)).arg("push").execute().unwrap();

    Ok(())
}
