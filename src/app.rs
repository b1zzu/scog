use chrono::DateTime;
use chrono::Local;
use config::Config;
use core::result::Result;
use options::Command;
use options::Options;
use regex::Regex;
use repository::Repository;
use std::fs;
use std::path::Path;
use controller::step;
use controller::end;
use std::path::PathBuf;

pub type AppResult<'a> = Result<App<'a>, String>;

pub struct App<'b> {
    repository_path: &'b Path,
    repository: Repository<'b>,
}

impl<'c> App<'c> {
    pub fn new(repository: &'c Path) -> App<'c> {
        let destination = repository;
        let repository = Repository::new(destination);
        App { repository, repository_path: destination }
    }

    pub fn route(self, options: Options) -> AppResult<'c> {
        match *options.get_command() {
            Command::None => {
                Err(String::from("no command defined"))
            }

            Command::Clone => {
                self.clone(options.get_repo())
            }

            Command::Checkout => {
                self.checkout(options.get_branch())
            }

            Command::Pull => {
                step(Self::is_clean, step(Self::is_not_backup, end(Self::pull)))(self)
            }

            Command::Push => {
                step(Self::is_clean, step(Self::is_not_backup, end(Self::push))
                )(self)
            }
        }
    }

    fn clone(self, remote: &String) -> AppResult<'c> {
        self.repository.clone(remote).unwrap();
        Ok(self)
    }

    fn checkout(self, branch: &String) -> AppResult<'c> {
        let result = self.repository.checkout(branch);
        if result.is_err() {
            self.repository.checkout_new(branch).unwrap();
        }
        Ok(self)
    }

    fn load_config(&self) -> Config {
        Config::load(self.repository_path.join("config.yaml").as_path()).unwrap()
    }

    fn is_clean(app: App) -> AppResult {
        match app.repository.is_clean() {
            true => Ok(app),
            false => Err(String::from("repository is not clean")),
        }
    }

    fn is_not_backup(app: App) -> AppResult {
        let branch = app.repository.get_current_branch();
        match Regex::new(r"^_backup").unwrap().is_match(branch.as_str()) {
            false => Ok(app),
            true => Err(format!("can not pull from backup branch: '{}'.", branch)),
        }
    }

    fn pull(self) -> AppResult<'c> {
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

    fn push(self) -> AppResult<'c> {
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

    fn get_now() -> DateTime<Local> {
        Local::now()
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

    fn copy_to_repo(&self) {
        let config = self.load_config();
        for file in config.get_files() {
            let source = file.get_file();
            let destination = self.repository_path.join(source.strip_prefix("/").unwrap());
            let result = Self::copy(source, destination).unwrap();
            for destination in result {
                self.repository.add(destination.as_path()).unwrap();
            }
        }
    }

    fn copy_to_disk(&self) {
        let config = self.load_config();
        for file in config.get_files() {
            let destination = file.get_file();
            let source = self.repository_path.join(destination.strip_prefix("/").unwrap());
            Self::copy(source, destination).unwrap();
        }
    }

    fn copy(source: PathBuf, destination: PathBuf) -> Result<Vec<PathBuf>, ()> {
        if source.exists() {
            if source.is_file() {
                if destination.is_dir() {
                    panic!("destination is a directory")
                }
                if !destination.parent().unwrap().exists() {
                    fs::create_dir_all(destination.parent().unwrap()).unwrap();
                }
                fs::copy(&source, &destination).unwrap();
                Ok(vec![destination])
            } else if source.is_dir() {
                let mut result: Vec<PathBuf> = vec![];
                for _source in fs::read_dir(&source).unwrap() {
                    let _source: PathBuf = _source.unwrap().path();
                    let _destination: PathBuf = destination.join(_source.strip_prefix(&source).unwrap());
                    let mut _result = Self::copy(_source, _destination).unwrap();
                    result.append(&mut _result);
                }
                Ok(result)
            } else {
                panic!("destination is not a file and not a dir")
            }
        } else {
            Ok(vec![])
        }
    }
}