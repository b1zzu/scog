use core::result::Result as CoreResult;
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

    pub fn arg(mut self, arg: &str) -> Git {
        self.command.arg(arg);
        self
    }

    pub fn args(mut self, args: Vec<&str>) -> Git {
        for arg in args {
            self = self.arg(arg)
        }
        self
    }

    pub fn execute(mut self) -> Result {
        let o = self.command.output().unwrap();
        match o.status.code().unwrap() {
            0 => Ok(o),
            _ => Err(o)
        }
    }
}

