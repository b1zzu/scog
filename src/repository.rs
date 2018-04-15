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

    fn git(&self) -> Git {
        Git::new(Some(self.repository))
    }

    pub fn clone(&self, url: &str) -> Result {
        Git::new(None)
            .clone(Vec::new(), url, self.repository.to_str().unwrap())
    }

    pub fn checkout(&self, branch: &str) -> Result {
        self.git()
            .checkout(vec![], branch)
    }

    pub fn checkout_new(&self, branch: &str) -> Result {
        self.git()
            .checkout(vec!["-b"], branch)
    }

    pub fn status_porcelain(&self) -> Result {
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

    pub fn add(&self, pathspec: &Path) -> Result {
        self.git().add(vec![], vec![pathspec.to_str().unwrap()])
    }

    pub fn commit(&self, message: &str) -> Result {
        self.git().commit(vec!["-m", message], vec![])
    }

    pub fn push(&self) -> Result {
        self.git().push(vec![])
    }

    pub fn push_new_branch(&self) -> Result {
        self.git().push(vec!["-u", "origin", "HEAD"])
    }

    pub fn branch_delete(&self, branch: &str) -> Result {
        self.git().branch(vec!["-d", branch])
    }
}