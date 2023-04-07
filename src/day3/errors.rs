use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidCharacterError {
    details: String,
}

impl InvalidCharacterError {
    pub fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for InvalidCharacterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for InvalidCharacterError {
    fn description(&self) -> &str {
        &self.details
    }
}
