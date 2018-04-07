extern crate chrono;
#[macro_use]
extern crate serde_derive;

use chrono::DateTime;
use chrono::offset::Local;
use git::Git;
use std::env;
use std::env::home_dir;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::time::SystemTime;

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
            checkout()
        },

        options::Command::Pull => {
            pull()
        },

        options::Command::Push => {
            push()
        },
    }

    exit(0);

//    let config = config::load(dir.join("config.yaml").as_path());
//
//    // If repository already exists and is clean pull it, otherwise should be fixed manually
//    if Path::new("/tmp/sync").is_dir() {
//
//        // Check if the repository is up to date
//        git(Command::new("git")
//            .arg("pull")
//            .arg("--ff-only"));
//    } else {
//
//        // Simple repository clone
//        let output = Command::new("git")
//            .arg("clone")
//            .arg(config.repository.as_str())
//            .arg("sync")
//            .current_dir("/tmp")
//            .output()
//            .unwrap();
//
//        if output.status.code().unwrap() != 0 {
//            println!("sync: error: failed to clone repository: '{}'", config.repository);
//            exit(1);
//        }
//    }
//
//    // Loop on configured files and copy them to the repository
//    for file in config.files {
//        // TODO: Handle dirs
//        let source = Path::new(&file.file);
//        if !source.is_file() {
//            println!("sync: error: file: '{}' does not exists", source.to_string_lossy());
//            exit(1);
//        }
//
//        let destination = Path::new("/tmp/sync").join(&source.to_owned().strip_prefix("/").unwrap());
//
//        if !destination.parent().unwrap().exists() {
//            fs::create_dir_all(destination.parent().unwrap()).unwrap();
//        }
//
//        println!("sync: {}: copy '{}' to '{}'", file.file, source.to_string_lossy(), destination.to_string_lossy());
//
//        fs::copy(source, &destination).unwrap();
//
//        // Stash the copied file
//        git(Command::new("git")
//            .arg("add")
//            .arg(destination.to_str().unwrap()));
//    }
//
//    let output = git(Command::new("git").arg("status").arg("--porcelain"));
//    // TODO: Check this with a multi-line regex only for line starting with A
//    if output.stdout.len() == 0 {
//        println!("Nothing to commit");
//        exit(0);
//    }
//
//    // Commit the repository
//    let now: DateTime<Local> = SystemTime::now().into();
//    git(Command::new("git")
//        .arg("commit")
//        .arg("-m")
//        .arg(now.to_string().as_str()));
//
//    // Push the repository
//    git(Command::new("git")
//        .arg("push"));
}

fn git(command: &mut Command) -> std::process::Output {
    let output = command.current_dir("/tmp/sync")
        .output()
        .unwrap();

    if output.status.code().unwrap() != 0 {
        panic!("don't have the time")
    }

    output
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

fn checkout() {}

fn pull() {}

fn push() {}
