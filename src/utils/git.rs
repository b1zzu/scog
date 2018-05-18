use git2::build::RepoBuilder;
use git2::Cred;
use git2::FetchOptions;
use git2::RemoteCallbacks;
use utils::error::Error;
use git2::Repository;
use git2::Branch;
use git2::BranchType;
use git2::ErrorCode;
use git2::Commit;
use git2::build::CheckoutBuilder;
use git2;
use std::path::Path;
use git2::Oid;
use git2::PushOptions;

pub struct Helper {
    repository: Repository,
}

impl Helper {
    pub fn new(work_dir: &Path) -> Result<Helper, Error> {
        let repository = match Repository::open(work_dir) {
            Ok(repository) => repository,
            Err(e) => return Err(e.into()),
        };
        let helper = Helper {
            repository,
        };
        Ok(helper)
    }

    /// Retrieve the Branch object of the passed branch
    /// Try to search it locally otherwise try to search it on each remote and return it
    /// on the first matched remote
    pub fn find_branch(&self, branch_name: &str) -> Result<Branch, Error> {
        match self.find_local_branch(branch_name) {
            Ok(branch) => Ok(branch),
            Err(error) => {
                match error.code() {
                    ErrorCode::NotFound => {
                        self.find_remote_branch(branch_name)
                    },
                    _ => Err(error.into()),
                }
            },
        }
    }

    fn find_local_branch(&self, branch_name: &str) -> Result<Branch, git2::Error> {
        self.repository.find_branch(branch_name, BranchType::Local)
    }

    fn find_remote_branch(&self, branch_name: &str) -> Result<Branch, Error> {
        let remotes = self.repository.remotes()?;
        for remote in remotes.iter() {
            match remote {
                Some(remote) => {
                    let branch = self.find_local_branch(format!("{}/{}", remote, branch_name).as_str());
                    match branch {
                        Ok(branch) => return Ok(branch),
                        Err(error) => {
                            match error.code() {
                                ErrorCode::NotFound => continue,
                                _ => return Err(error.into()),
                            }
                        },
                    }
                },
                None => {},
            }
        }
        Err(format!("can not locate remote or local branch '{}'", branch_name))?
    }

    /// Checkout a local or remote branch, this will behave like git checkout
    pub fn checkout_branch(&self, branch_name: &str) -> Result<(), Error> {
        let branch = self.find_branch(branch_name)?;

        let commit = branch.get().peel_to_commit()?;

        // If the branch is remote, create a new local branch with the same name
        if branch.get().is_remote() {
            self.repository.branch(branch_name, &commit, false)?;
        }

        // Checkout files (this will not move HEAD)
        self.checkout_commit(&commit)?;

        // Set the HEAD on the passed branch
        self.repository.set_head(branch.get().name().unwrap_or_default())?;

        Ok(())
    }

    /// Create a new branch and switch to it
    pub fn branch(&self, branch_name: &str) -> Result<Branch, Error> {
        let commit = self.repository.head()?.peel_to_commit()?;
        let branch = self.repository.branch(branch_name, &commit, false)?;
        self.repository.set_head(branch.get().name().unwrap_or_default())?;
        Ok(branch)
    }

    fn checkout_commit(&self, commit: &Commit) -> Result<(), git2::Error> {
        self.repository.checkout_tree(commit.as_object(), Some(CheckoutBuilder::new().safe()))
    }

    pub fn remote_callbacks<'a>() -> RemoteCallbacks<'a> {
        // Set Authentication method
        let mut remote_callbacks = RemoteCallbacks::new();
        remote_callbacks.credentials(|_, _, _| {
            // TODO: Handle different authentications type (username, password)
            Cred::ssh_key_from_agent("git")
        });
        remote_callbacks
    }

    pub fn fetch_options<'a>() -> FetchOptions<'a> {
        // Add Authentication call back to fetch options
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(Self::remote_callbacks());
        fetch_options
    }

    pub fn push_options<'a>() -> PushOptions<'a> {
        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(Self::remote_callbacks());
        push_options
    }

    /// Fetch all the branches of all remotes
    pub fn fetch(&self) -> Result<(), Error> {
        let mut fetch_options = Self::fetch_options();

        let remotes = self.repository.remotes()?;
        for remote in remotes.iter() {
            match remote {
                Some(remote) => {
                    let mut remote = self.repository.find_remote(remote)?;
                    // Fetch all branches
                    remote.fetch(&[], Some(&mut fetch_options), None)?;
                }
                None => {}
            }
        }

        Ok(())
    }

    /// Create a new repository
    pub fn clone(repo: &str, work_dir: &Path) -> Result<(), Error> {
        let fetch_options = Self::fetch_options();

        let mut repo_builder = RepoBuilder::new();
        repo_builder.fetch_options(fetch_options);

        repo_builder.clone(repo, work_dir)?;

        Ok(())
    }

    /// Update the local branch with the remote one, only fast forward
    pub fn pull(&self, branch_name: &str) -> Result<(), Error> {
        self.fetch()?;

        let branch = self.find_local_branch(branch_name)?;
        let branch_oid = branch.get().peel_to_commit()?.id();

        let upstream = branch.upstream()?.get().peel_to_commit()?;
        let upstream_oid = upstream.id();

        let base = self.repository.merge_base(branch_oid, upstream_oid)?;

        // If the common commit between local and remote branch is the commit to which te remote
        // branch points than the local branch is ahead the remote branch and there is nothing
        // to pull
        if base == upstream_oid {
            return Ok(());
        }

        // Check if it is possible to fast forward
        if base != branch_oid {
            Err(format!("can not fast forward branch: {}, fix this manually", branch_name))?;
        }

        // Checkout files of commit
        self.checkout_commit(&upstream)?;

        // Move reference of branch to new commit
        let mut reference = self.repository.find_reference(branch.get().name().unwrap_or_default())?;
        reference.set_target(upstream_oid, "pull: Fast-forward")?;

        Ok(())
    }

    pub fn get_current_branch_name(&self) -> Result<String, Error> {
        let head = self.repository.head()?;
        match head.is_branch() {
            true => Ok(head.shorthand().unwrap_or_default().to_owned()),
            false => Err("HEAD is not attached to a branch".to_string())?,
        }
    }

    /// Stage passed file
    pub fn add(&self, path: &Path) -> Result<(), Error> {
        let mut index = self.repository.index()?;
        index.add_path(path)?;
        index.write()?;
        Ok(())
    }

    /// Commit all staged files with the passed message and the local user
    pub fn commit(&self, message: &str) -> Result<Oid, Error> {
        let tree = self.repository.index()?.write_tree()?;
        let tree = self.repository.find_tree(tree)?;
        let signature = self.repository.signature()?;
        let parent = self.repository.head()?.peel_to_commit()?;
        let commit = self.repository.commit(Some("HEAD"), &signature, &signature, message, &tree, &[&parent])?;
        Ok(commit)
    }

    /// Check if the current work dir is dirty
    pub fn is_dirty(&self) -> Result<bool, Error> {
        let tree = self.repository.head()?.peel_to_tree()?;
        let diff = self.repository.diff_tree_to_workdir_with_index(Some(&tree), None)?;
        Ok(diff.deltas().len() > 0)
    }

    /// Push the passed branch to all remotes
    pub fn push(&self, branch_name: &str) -> Result<(), Error> {
        let branch = self.find_branch(branch_name)?;

        let mut push_options = Self::push_options();

        let remotes = self.repository.remotes()?;
        for remote in remotes.iter() {
            match remote {
                Some(remote) => {
                    let mut remote = self.repository.find_remote(remote)?;
                    // Push only the passed branch all remotes
                    remote.push(&[branch.get().name().unwrap_or_default()], Some(&mut push_options))?;
                }
                None => {}
            }
        }

        Ok(())
    }
}
