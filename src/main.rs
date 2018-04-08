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

    let dir = home_dir().unwrap().join(".bog/");
    let dir = dir.as_path();

    if options.get_help() {
        help();
        exit(0);
    }

    match options.get_command() {
        options::Command::None => {
            help();
            exit(1)
        },

        options::Command::Clone => {
            clone(options.get_repo(), dir)
        },

        options::Command::Checkout => {
            checkout(dir, options.get_branch())
        },

        options::Command::Pull => {
            pull(dir)
        },

        options::Command::Push => {
            push(dir)
        },
    }

    exit(0);
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

fn clone(repo: String, dir: &Path) {
    Git::new(None).arg("clone").arg(repo).arg(dir).execute().unwrap();
}

fn checkout(repository: &Path, branch: String) {
    let result = Git::new(Option::from(repository)).arg("checkout").arg(&branch).execute();

    if result.is_err() {
        Git::new(Option::from(repository)).arg("checkout").arg("-b").arg(branch).execute().unwrap();
    }
}

fn pull(repository: &Path) {
    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        println!("bog: '{}' is not clean. Clean it manually.", repository.to_string_lossy());
        exit(1);
    }

    let output = Git::new(Option::from(repository)).arg("rev-parse").arg("--abbrev-ref").arg("HEAD").execute().unwrap();
    let current_branch = String::from_utf8(output.stdout).unwrap();
    let current_branch = String::from(current_branch.trim());

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        println!("bog: can not pull from backup branch: '{}'", current_branch);
        exit(1);
    }

    let result = Git::new(Option::from(repository)).arg("pull").arg("--ff-only").execute();
    if result.is_err() {
        println!("bog: can not merge remote changes with local changes.");
        println!("  Fix it manually in: '{}'.", repository.to_string_lossy());
        println!("  Then 'bog pull' to update local files.");
        exit(1)
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
}

fn push(repository: &Path) {
    let output = Git::new(Option::from(repository)).arg("status").arg("--porcelain").execute().unwrap();
    if output.stdout.len() != 0 {
        println!("bog: '{}' is not clean. Clean it manually.", repository.to_string_lossy());
        exit(1);
    }

    let output = Git::new(Option::from(repository)).arg("rev-parse").arg("--abbrev-ref").arg("HEAD").execute().unwrap();
    let current_branch = String::from_utf8(output.stdout).unwrap();
    let current_branch = String::from(current_branch.trim());

    if Regex::new(r"^_backup").unwrap().is_match(current_branch.as_str()) {
        println!("bog: can not push to backup branch '{}'", current_branch);
        exit(1);
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

    pull(repository);

    Git::new(Option::from(repository)).arg("push").execute().unwrap();
}
