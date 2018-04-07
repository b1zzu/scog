use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::process::exit;
use std::process::Output;

pub struct Git<'a> {
    command: Command,
    repository: Option<&'a Path>,
}

impl<'a> Git<'a> {
    pub fn new(repository: Option<&'a Path>) -> Git<'a> {
        Git {
            command: Command::new("git"),
            repository,
        }
    }

    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Git<'a> {
        self.command.arg(arg);
        self
    }

    pub fn execute(self) -> Result<Output, Output> {
        let mut command = self.command;

        if self.repository.is_some() {
            command.current_dir(self.repository.unwrap());
        }

        let output = command.output().unwrap();

        if output.status.code().unwrap() != 0 {
            Err(output)
        } else {
            Ok(output)
        }
    }
}
