use core::result::Result;
use std::path::Path;
use std::process::Command;
use std::process::Output;

pub type GitResult = Result<Output, Output>;

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

    pub fn pathspec(self, pathspecs: Vec<&str>) -> Git {
        if pathspecs.len() != 0 {
            self.arg("--").args(pathspecs)
        } else {
            self
        }
    }

    pub fn optional_arg(self, arg: Option<&str>) -> Git {
        match arg {
            Some(arg) => self.arg(arg),
            None => self
        }
    }

    pub fn clone(self, options: Vec<&str>, repository: &str, directory: &str) -> GitResult {
        self.arg("clone").args(options).arg("--").arg(repository).arg(directory).execute()
    }

    pub fn checkout(self, options: Vec<&str>, branch: &str) -> GitResult {
        self.arg("checkout").args(options).arg(branch).execute()
    }

    pub fn status(self, options: Vec<&str>, pathspecs: Vec<&str>) -> GitResult {
        self.arg("status").args(options).pathspec(pathspecs).execute()
    }

    pub fn rev_parse(self, option: &str, args: Vec<&str>) -> GitResult {
        self.arg("rev-parse").arg(option).args(args).execute()
    }

    pub fn add(self, options: Vec<&str>, pathspecs: Vec<&str>) -> GitResult {
        self.arg("add").args(options).pathspec(pathspecs).execute()
    }

    pub fn commit(self, options: Vec<&str>, files: Vec<&str>) -> GitResult {
        self.arg("commit").args(options).pathspec(files).execute()
    }

    pub fn push(self, options: Vec<&str>) -> GitResult {
        self.arg("push").args(options).execute()
    }

    pub fn pull(self, options: Vec<&str>, repository: Option<&str>, refspec: Vec<&str>) -> GitResult {
        self.arg("pull").args(options).optional_arg(repository).args(refspec).execute()
    }

    pub fn branch(self, options: Vec<&str>) -> GitResult {
        self.arg("branch").args(options).execute()
    }

    pub fn execute(mut self) -> GitResult {
        let o = self.command.output().unwrap();
        match o.status.code().unwrap() {
            0 => Ok(o),
            _ => Err(o)
        }
    }
}

