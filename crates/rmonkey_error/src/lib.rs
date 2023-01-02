use std::fmt;

#[derive(Debug)]
pub enum RMonkeyError {
    UnexpectedTokenError,
}

impl fmt::Display for RMonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RMonkeyError::UnexpectedTokenError => write!(f, "unexpected error"),
        }
    }
}

pub type Result<T> = std::result::Result<T, RMonkeyError>;
