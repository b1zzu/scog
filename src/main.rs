extern crate chrono;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate toml;

use chrono::DateTime;
use chrono::offset::Local;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::time::SystemTime;

mod options;
mod config;

fn main() {
    let options = options::parse();

    let config = config::from(options.get_config());

    // If repository already exists and is clean pull it, otherwise should be fixed manually
    if Path::new("/tmp/sync").is_dir() {

        // Check if the repository is up to date
        git(Command::new("git")
            .arg("pull")
            .arg("--ff-only"));
    } else {

        // Simple repository clone
        let output = Command::new("git")
            .arg("clone")
            .arg(config.main.repository.as_str())
            .arg("sync")
            .current_dir("/tmp")
            .output()
            .unwrap();

        if output.status.code().unwrap() != 0 {
            println!("sync: error: failed to clone repository: '{}'", config.main.repository);
            exit(1);
        }
    }

    // Loop on configured files and copy them to the repository
    for (key, file) in config.files {
        // TODO: Handle dirs
        let source = Path::new(&file.file);
        if !source.is_file() {
            println!("sync: error: file: '{}' does not exists", source.to_string_lossy());
            exit(1);
        }

        let destination = Path::new("/tmp/sync").join(&source.to_owned().strip_prefix("/").unwrap());

        if !destination.parent().unwrap().exists() {
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
        }

        println!("sync: {}: copy '{}' to '{}'", key, source.to_string_lossy(), destination.to_string_lossy());

        fs::copy(source, &destination).unwrap();

        // Stash the copied file
        git(Command::new("git")
            .arg("add")
            .arg(destination.to_str().unwrap()));
    }

    let output = git(Command::new("git").arg("status").arg("--porcelain"));
    // TODO: Check this with a multi-line regex only for line starting with A
    if output.stdout.len() == 0 {
        println!("Nothing to commit");
        exit(0);
    }

    // Commit the repository
    let now: DateTime<Local> = SystemTime::now().into();
    git(Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(now.to_string().as_str()));

    // Push the repository
    git(Command::new("git")
        .arg("push"));
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
