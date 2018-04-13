use core::result::Result as CoreResult;
use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;
use std::process::Output;

pub type Result = CoreResult<Output, Output>;

pub struct Git {
    command: Command,
}

impl Git {
    pub fn new(repository: Option<&Path>) -> Git {
        let mut command = Command::new("git");

        match repository {
            Some(repository) => { command.current_dir(repository); }
            None => {}
        }

        Git { command }
    }

    pub fn arg<S: AsRef<OsStr>>(mut self, arg: S) -> Git {
        self.command.arg(arg);
        self
    }

    pub fn args<S: AsRef<OsStr>>(mut self, args: Vec<S>) -> Git {
        for arg in args {
            self = self.arg(arg)
        }
        self
    }

    pub fn clone<S: AsRef<OsStr>>(mut self, options: Vec<S>, repository: S, directory: &Path) -> Result {
        self.args(options).arg("--").arg(repository).arg(directory).execute()
    }

    pub fn execute(mut self) -> Result {
        let o = self.command.output().unwrap();
        match o.status.code().unwrap() {
            0 => Ok(o),
            _ => Err(o)
        }
    }
}

