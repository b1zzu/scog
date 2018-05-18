use chrono::DateTime;
use chrono::Local;
use config::Config;
use core::result::Result;
use deprecated::end;
use deprecated::step;
use git::repository::Repository;
use options::Command;
use options::DeprecatedOptions;
use regex::Regex;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct Options {
    repository: PathBuf
}

impl Options {
    pub fn new(repository: PathBuf) -> Options {
        Options { repository }
    }
}

pub struct App {
    git: Option<Repository>,
    config: Option<Config>,
    options: Options,
}

impl App {
    pub fn new(options: Options) -> App {
        App {
            git: None,
            config: None,
            options
        }
    }

    pub fn load_git(&mut self) -> &App {}

    pub fn get_git(&mut self) -> &Repository {
        match &self.git {
            None => { self.load_git().get_git() }
            Some(git) => git
        }
    }
}

pub type DeprecatedAppResult<'a> = Result<DeprecatedApp<'a>, String>;

pub type BaseResult<T> = Result<T, String>;

pub struct DeprecatedApp<'b> {
    repository_path: PathBuf,
    repository: Repository,
}

impl<'c> DeprecatedApp<'c> {
    pub fn new(repository: PathBuf) -> DeprecatedApp<'c> {
        let destination = repository;
        let repository = Repository::new(destination);
        DeprecatedApp { repository, repository_path: destination }
    }

    pub fn route(self, options: DeprecatedOptions) -> DeprecatedAppResult<'c> {
        match *options.get_command() {
            Command::None => {
                Err(String::from("no command defined"))
            }

            Command::Add => {
                match self.add(options.get_file()) {
                    Ok(_) => Ok(self),
                    Err(e) => Err(e),
                }
            }

            Command::Clone => {
                self.clone(options.get_repo())
            }

            Command::Checkout => {
                self.checkout(options.get_branch())
            }

            Command::Pull => {
                step(Self::deprecated_is_clean, step(Self::deprecated_is_not_backup, end(Self::pull)))(self)
            }

            Command::Push => {
                step(Self::deprecated_is_clean, step(Self::deprecated_is_not_backup, end(Self::push))
                )(self)
            }
        }
    }

    fn add(&self, _file: &String) -> BaseResult<()> {
        self.is_clean()?;
        self.is_not_backup()?;
        Ok(())
    }

    fn clone(self, remote: &String) -> DeprecatedAppResult<'c> {
        self.repository.clone(remote).unwrap();
        Ok(self)
    }

    fn checkout(self, branch: &String) -> DeprecatedAppResult<'c> {
        let result = self.repository.checkout(branch);
        if result.is_err() {
            self.repository.checkout_new(branch).unwrap();
        }
        Ok(self)
    }

    fn load_config(&self) -> Config {
        Config::load(self.repository_path.join("config.yaml").as_path()).unwrap()
    }

    fn deprecated_is_clean(app: DeprecatedApp) -> DeprecatedAppResult {
        match app.repository.is_clean() {
            true => Ok(app),
            false => Err(String::from("repository is not clean")),
        }
    }

    fn is_clean(&self) -> BaseResult<()> {
        match self.repository.is_clean() {
            true => Ok(()),
            false => Err(String::from("repository is not clean")),
        }
    }

    fn deprecated_is_not_backup(app: DeprecatedApp) -> DeprecatedAppResult {
        let branch = app.repository.get_current_branch();
        match Regex::new(r"^_backup").unwrap().is_match(branch.as_str()) {
            false => Ok(app),
            true => Err(format!("can not pull from backup branch: '{}'.", branch)),
        }
    }

    fn is_not_backup(&self) -> BaseResult<()> {
        let branch = self.repository.get_current_branch();
        match Regex::new(r"^_backup").unwrap().is_match(branch.as_str()) {
            false => Ok(()),
            true => Err(format!("can not pull from backup branch: '{}'.", branch)),
        }
    }

    fn pull(self) -> DeprecatedAppResult<'c> {
        let branch = self.repository.get_current_branch();
        let branch = branch.as_str();

        self.repository.pull().unwrap();

        let now = Self::get_now();
        let backup_branch = format!("_backup_{}_{}", branch, now.format("%F_%H-%M-%S_%f"));
        let backup_branch = backup_branch.as_str();
        self.repository.checkout_new(&backup_branch).unwrap();

        self.copy_to_repo();

        let committed = self.commit();

        if committed {
            self.repository.push_new_branch().unwrap();
        }

        self.repository.checkout(branch).unwrap();

        if !committed {
            self.repository.branch_delete(&backup_branch).unwrap();
        }

        self.copy_to_disk();

        Ok(self)
    }

    fn push(self) -> DeprecatedAppResult<'c> {
        self.copy_to_repo();

        self.commit();

        match self.pull() {
            Ok(a) => {
                a.repository.push().unwrap();
                Ok(a)
            }
            Err(e) => {
                Err(e)
            }
        }
    }


    fn commit(&self) -> bool {
        if !self.repository.is_clean() {
            let now = Self::get_now();
            self.repository.commit(now.to_string().as_str()).unwrap();
            true
        } else {
            false
        }
    }

}