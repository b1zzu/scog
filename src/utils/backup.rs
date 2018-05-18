use regex::Regex;
use utils::time;

pub fn is_backup(branch_name: &str) -> bool {
    Regex::new(r"^_backup_").unwrap().is_match(branch_name)
}

pub fn backup_branch_name(from_branch_name: &str) -> String {
    format!("_backup_{}_{}", from_branch_name, time::now_to_string())
}

#[cfg(test)]
mod tests {
    use super::backup_branch_name;
    use super::is_backup;
    use regex::Regex;

    #[test]
    fn test_backup_branch_name() {
        assert!(Regex::new(r"^_backup_test_").unwrap().is_match(backup_branch_name("test").as_str()));
        assert!(is_backup(backup_branch_name("test").as_str()));
    }

    #[test]
    fn test_is_backup() {
        assert!(is_backup(backup_branch_name("test").as_str()));
        assert!(!is_backup("master"));
        assert!(is_backup("_backup_test"));
    }
}
