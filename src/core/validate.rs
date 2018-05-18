use utils::error::Error;
use utils::backup::is_backup;
use utils::git::Helper;

pub fn branch(branch_name: &str) -> Result<(), Error> {
    match is_backup(branch_name) {
        true => Err(format!("can not pull or push from backup branch: '{}'.", branch_name))?,
        false => Ok(()),
    }
}

pub fn repository(git: &Helper) -> Result<(), Error> {
    match git.is_dirty()? {
        true => Err("can not pull or push if repository is dirty".to_string())?,
        false => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::branch;

    #[test]
    fn test_validate_branch() {
        assert!(branch("_backup_test_bla").is_err());
        assert!(branch("master").is_ok());
    }
}