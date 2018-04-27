use core::config::Config;
use utils::git::Helper;
use utils::error::Error;
use std::env;
use std::path::PathBuf;
use core::validate;
use utils::backup::backup_branch_name;
use utils::copy::copy;
use utils::time::now_to_string;
use std::path::Path;

pub struct Repository {
    home_dir: PathBuf,
    repository_dir: PathBuf,
    config_file: PathBuf,
    config: Option<Config>,
    git: Option<Helper>,
}

impl Repository {
    pub fn new() -> Repository {
        let home_dir = env::home_dir().unwrap();
        let repository_dir = home_dir.join(".scog/");
        let config_file = repository_dir.join("config.yaml");

        Repository {
            home_dir,
            repository_dir,
            config_file,
            config: None,
            git: None,
        }
    }

    fn free_config(&mut self) {
        self.config = None;
    }

    fn config(&mut self) -> Result<&Config, Error> {
        if self.config.is_none() {
            self.config = Some(Config::new(&self.config_file)?);
        }
        Ok(self.config.as_ref().unwrap())
    }

    fn git(&mut self) -> Result<&Helper, Error> {
        if self.git.is_none() {
            self.git = Some(Helper::new(&self.repository_dir)?);
        }
        Ok(self.git.as_ref().unwrap())
    }

    /// Copy files listed in config from home_dir to repository_dir
    fn copy_to_repository(&mut self) -> Result<Vec<PathBuf>, Error> {
        let repository_dir = self.repository_dir.clone();
        let home_dir = self.home_dir.clone();

        let mut copied: Vec<PathBuf> = vec![];
        for section in self.config()?.sections() {
            let source = home_dir.join(section.path());
            let destination = repository_dir.join(section.path());
            let mut _copied = copy(source.as_path(), destination.as_path())?;
            copied.append(&mut _copied)
        }

        Ok(copied)
    }

    /// Copy files listed in config from repository_dir to home_dir
    fn copy_to_local(&mut self) -> Result<(), Error> {
        let repository_dir = self.repository_dir.clone();
        let home_dir = self.home_dir.clone();
        for section in self.config()?.sections() {
            let source = repository_dir.join(section.path());
            let destination = home_dir.join(section.path());
            copy(source.as_path(), destination.as_path())?;
        }
        Ok(())
    }

    /// Stage all the passed files (add to index)
    fn stage_files(&mut self, files: Vec<&Path>) -> Result<(), Error> {
        let repository_dir = &self.repository_dir.clone();
        for file in files {
            self.git()?.add(file.strip_prefix(repository_dir)?)?;
        }

        Ok(())
    }

    /// Copy files from local disk to repository and stage all files
    /// return than true if repository is dirty or false if there si nothing to commit
    fn copy_to_repository_and_stage_files(&mut self) -> Result<bool, Error> {
        let copied = self.copy_to_repository()?;
        self.stage_files(copied.iter().map(|path| path.as_path()).collect())?;
        Ok(self.git()?.is_dirty()?)
    }


    /// Copy local file the repository and if there are changes it commit them to
    /// the backup branch and clean up again the repository
    fn backup_local_files(&mut self, branch_name: &str) -> Result<(), Error> {
        if self.copy_to_repository_and_stage_files()? {
            self.git()?.branch(backup_branch_name(branch_name).as_str())?;
            self.git()?.commit(now_to_string().as_str())?;
            self.git()?.checkout_branch(branch_name)?;
        }
        Ok(())
    }

    pub fn clone(&self, repo: &str) -> Result<(), Error> {
        Helper::clone(repo, &self.repository_dir)
    }

    pub fn checkout(&mut self, branch_name: &str) -> Result<(), Error> {
        // Fetch remote data
        self.git()?.fetch()?;

        // Checkout the branch
        self.git()?.checkout_branch(branch_name)?;

        Ok(())
    }

    pub fn pull(&mut self) -> Result<(), Error> {

        // Get current branch name
        let branch_name = self.git()?.get_current_branch_name()?;

        // Check if branch is not a backup
        validate::branch(branch_name.as_str())?;

        // Check if repository is not dirty
        validate::repository(self.git()?)?;

        // Fast forward branch
        self.git()?.pull(branch_name.as_str())?;

        // Config must be reloaded
        self.free_config();

        // Backup local files
        self.backup_local_files(branch_name.as_str())?;

        // Copy files form repository to local
        self.copy_to_local()?;

        Ok(())
    }

    pub fn push(&mut self) -> Result<(), Error> {

        // Get current branch name
        let branch_name = self.git()?.get_current_branch_name()?;

        // Check if branch is not a backup
        validate::branch(branch_name.as_str())?;

        // Check if repository is not dirty
        validate::repository(self.git()?)?;

        // Fetch
        self.git()?.fetch()?;

        // Update repository and commit changes
        if self.copy_to_repository_and_stage_files()? {
            self.git()?.commit(now_to_string().as_str())?;
        }

        // Pull new changes
        self.pull()?;

        // Push the working branch
        self.git()?.push(branch_name.as_str())?;

        Ok(())
    }
}