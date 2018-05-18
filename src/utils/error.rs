use std;
use git2;
use serde_yaml;

pub struct Error {
    error: String,
}

impl Error {
    fn new(error: String) -> Error {
        Error { error }
    }

    pub fn error(&self) -> &str {
        self.error.as_str()
    }
}


impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error::new(format!("std::io::Error: {}", (&error as &std::error::Error).description().to_string()))
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(error: serde_yaml::Error) -> Self {
        Error::new(format!("serde_yaml::Error: {}", (&error as &std::error::Error).description().to_string()))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::new(format!("std::str::Utf8Error: {}", (&error as &std::error::Error).description().to_string()))
    }
}

impl From<std::path::StripPrefixError> for Error {
    fn from(error: std::path::StripPrefixError) -> Self {
        Error::new(format!("std::path::StripPrefixError: {}", (&error as &std::error::Error).description().to_string()))
    }
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Self {
        Error::new(format!("git2::Error: {}", (&error as &std::error::Error).description().to_string()))
    }
}

impl From<String> for Error {
    fn from(error: String) -> Self {
        Error::new(error)
    }
}