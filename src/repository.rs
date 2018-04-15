use git::Git;
use git::Result;
use std::path::Path;
use std::str;

pub struct Repository<'a> {
    repository: &'a Path,
}

impl<'a> Repository<'a> {
    pub fn new(repository: &'a Path) -> Repository<'a> {
        return Repository { repository };
    }

    pub fn clone(&self, url: &str) -> Result {
        Git::new(None).clone(Vec::new(), url, self.repository)
    }

    fn git(&self, command: &str) -> Git {
        Git::new(Some(self.repository)).arg(command)
    }


    fn _checkout(&self, options: Vec<&str>, branch: &str) -> Result {
        self.git("checkout").args(options).arg(branch).execute()
    }

    fn _status(&self, options: Vec<&str>) -> Result {
        self.git("status").args(options).execute()
    }

    fn _rev_parse(&self, option: &str, args: Vec<&str>) -> Result {
        self.git("rev-parse").arg(option).args(args).execute()
    }

    fn _add(&self, options: Vec<&str>, pathspec: Vec<&str>) -> Result {
        self.git("add").args(options).args(pathspec).execute()
    }

    pub fn checkout(&self, branch: &str) -> Result {
        self._checkout(vec![], branch)
    }

    pub fn checkout_new(&self, branch: &str) -> Result {
        self._checkout(vec!["-b"], branch)
    }

    pub fn status_porcelain(&self) -> Result {
        self._status(vec!["--porcelain"])
    }

    pub fn is_clean(&self) -> bool {
        let o = self.status_porcelain().unwrap();
        if o.stdout.len() != 0 {
            false
        } else {
            true
        }
    }

    pub fn get_current_branch(&self) -> String {
        String::from_utf8(self._rev_parse("--abbrev-ref", vec!["HEAD"]).unwrap().stdout).unwrap()
    }

    pub fn add(&self, pathspec: &Path) -> Result {
        self._add(vec![], vec![pathspec.to_str().unwrap()])
    }

    pub fn commit(&self, message: &str) -> Result {
        self.git("commit").args(vec!["-m", message]).execute()
    }

    pub fn _push(&self, options: Vec<&str>) -> Result {
        self.git("push").args(options).execute()
    }

    pub fn push(&self) -> Result {
        self._push(vec![])
    }

    pub fn push_new_branch(&self) -> Result {
        self._push(vec!["-u", "origin", "HEAD"])
    }

    pub fn branch_delete(&self, branch: &str) -> Result {
        self.git("branch").args(vec!["-d", branch]).execute()
    }
}