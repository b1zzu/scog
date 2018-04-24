use git::Git;
use git::GitResult;
use std::path::Path;
use std::str;

pub struct Repository<'a> {
    repository: &'a Path,
}

impl<'a> Repository<'a> {
    pub fn new(repository: &'a Path) -> Repository<'a> {
        return Repository { repository };
    }

    fn git(&self) -> Git {
        Git::new(Some(self.repository))
    }

    pub fn clone(&self, url: &str) -> GitResult {
        Git::new(None)
            .clone(Vec::new(), url, self.repository.to_str().unwrap())
    }

    pub fn checkout(&self, branch: &str) -> GitResult {
        self.git()
            .checkout(vec![], branch)
    }

    pub fn checkout_new(&self, branch: &str) -> GitResult {
        self.git()
            .checkout(vec!["-b"], branch)
    }

    pub fn status_porcelain(&self) -> GitResult {
        self.git()
            .status(vec!["--porcelain"], vec![])
    }

    pub fn is_clean(&self) -> bool {
        let output = self.status_porcelain().unwrap();
        if output.stdout.len() != 0 {
            false
        } else {
            true
        }
    }

    pub fn get_current_branch(&self) -> String {
        let branch = self.git()
            .rev_parse("--abbrev-ref", vec!["HEAD"]).unwrap().stdout;
        String::from_utf8(branch).unwrap()
    }

    pub fn add(&self, pathspec: &Path) -> GitResult {
        self.git().add(vec![], vec![pathspec.to_str().unwrap()])
    }

    pub fn commit(&self, message: &str) -> GitResult {
        self.git().commit(vec!["-m", message], vec![])
    }

    pub fn push(&self) -> GitResult {
        self.git().push(vec![])
    }

    pub fn pull(&self) -> GitResult {
        self.git().pull(vec!["--ff-only"], None, vec![])
    }

    pub fn push_new_branch(&self) -> GitResult {
        self.git().push(vec!["-u", "origin", "HEAD"])
    }

    pub fn branch_delete(&self, branch: &str) -> GitResult {
        self.git().branch(vec!["-d", branch])
    }
}