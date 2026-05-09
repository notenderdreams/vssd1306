use std::fmt;

#[derive(Debug)]
pub enum DisplayError {
    NotInitialized,
}

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DisplayError::NotInitialized => {
                write!(f, "DisplayError: Not initialized")
            }
        }
    }
}

impl std::error::Error for DisplayError {}