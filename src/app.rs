use chrono::DateTime;
use chrono::Local;
use config::Config;
use core::result::Result as CoreResult;
use options::Command;
use options::Options;
use regex::Regex;
use repository::Repository;
use std::fs;
use std::path::Path;
use controller::ver;
use controller::with;

pub type Result<'a> = CoreResult<App<'a>, String>;

pub struct App<'b> {
    destination: &'b Path,
    repository: Repository<'b>,
}

impl<'c> App<'c> {
    pub fn new(repository: &'c Path) -> App<'c> {
        let destination = repository;
        let repository = Repository::new(destination);
        App { repository, destination }
    }

    pub fn route(self, options: Options) -> Result<'c> {
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
                ver(App::is_clean, ver(App::is_not_backup, with(App::config, App::box_pull())))(self)
            }

            Command::Push => {
                ver(App::is_clean, ver(App::is_not_backup, with(App::config, App::box_push())))(self)
            }
        }
    }

    fn clone(self, remote: &String) -> Result<'c> {
        self.repository.clone(remote).unwrap();
        Ok(self)
    }

    fn checkout(self, branch: &String) -> Result<'c> {
        let result = self.repository.checkout(branch);
        if result.is_err() {
            self.repository.checkout_new(branch).unwrap();
        }
        Ok(self)
    }

    fn config(app: &App) -> CoreResult<Config, String> {
        let config = Config::load(app.destination.join("config.yaml").as_path()).unwrap();
        Ok(config)
    }


    fn is_clean(app: App) -> Result {
        match app.repository.is_clean() {
            true => Ok(app),
            false => Err(String::from("repository is not clean")),
        }
    }

    fn is_not_backup(app: App) -> Result {
        let branch = app.repository.get_current_branch();
        match Regex::new(r"^_backup").unwrap().is_match(branch.as_str()) {
            false => Ok(app),
            true => Err(format!("can not pull from backup branch: '{}'.", branch)),
        }
    }

    fn box_pull() -> Box<Fn(App, Config) -> Result> {
        Box::new(|a: App, c: Config| -> Result {
            a.pull(c)
        })
    }

    fn pull(self, config: Config) -> Result<'c> {
        let branch = self.repository.get_current_branch();

        let now: DateTime<Local> = Local::now();
        let backup_branch = format!("_backup_{}_{}", branch, now.format("%F_%H-%M-%S_%f"));
        let backup_branch = backup_branch.as_str();
        self.repository.checkout_new(&backup_branch).unwrap();

        for file in config.get_files() {
            // TODO: Handle dirs
            let source = Path::new(file.get_file());
            let destination = self.destination.join(&source.to_owned().strip_prefix("/").unwrap());
            let destination = destination.as_path();

            if !destination.parent().unwrap().exists() {
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
            }

            fs::copy(source, &destination).unwrap();

            self.repository.add(destination).unwrap();
        }

        let mut delete = false;
        if !self.repository.is_clean() {
            let now: DateTime<Local> = Local::now();
            self.repository.commit(now.to_string().as_str()).unwrap();
            self.repository.push_new_branch().unwrap();
        } else {
            delete = true;
        }

        self.repository.checkout(branch.as_str()).unwrap();
        if delete {
            self.repository.branch_delete(&backup_branch).unwrap();
        }

        for file in config.get_files() {
            // TODO: Handle dirs
            let destination = Path::new(file.get_file());
            let source = self.destination.join(&destination.to_owned().strip_prefix("/").unwrap());
            let source = source.as_path();

            if !destination.parent().unwrap().exists() {
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
            }

            fs::copy(source, &destination).unwrap();
        }

        Ok(self)
    }

    fn box_push() -> Box<Fn(App, Config) -> Result> {
        Box::new(|app: App, config: Config| -> Result {
            app.push(config)
        })
    }

    fn push(self, config: Config) -> Result<'c> {
        for file in config.get_files() {
            // TODO: Handle dirs
            let source = Path::new(file.get_file());
            let destination = self.destination.join(&source.to_owned().strip_prefix("/").unwrap());
            let destination = destination.as_path();

            if !destination.parent().unwrap().exists() {
                fs::create_dir_all(destination.parent().unwrap()).unwrap();
            }

            fs::copy(source, &destination).unwrap();

            self.repository.add(destination).unwrap();
        }

        if !self.repository.is_clean() {
            let now: DateTime<Local> = Local::now();
            self.repository.commit(now.to_string().as_str()).unwrap();
        }

        match self.pull(config) {
            Ok(a) => {
                a.repository.push().unwrap();
                Ok(a)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}