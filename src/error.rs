use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    message: String
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error { message: value.to_string() }
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error { message: value }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
}
