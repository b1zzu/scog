use core::repository::Repository;

pub struct Context {
    repository: Option<Repository>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            repository: None,
        }
    }

    pub fn repository(&mut self) -> &mut Repository {
        if self.repository.is_none() {
            self.repository = Some(Repository::new());
        }
        self.repository.as_mut().unwrap()
    }
}