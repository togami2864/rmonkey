use std::fmt;

use rmonkey_token::Token;

#[derive(Debug)]
pub enum RMonkeyError {
    UnexpectedToken { expected: Token, got: Token },
    InvalidPrefix { got: Token },
    Custom(String),
}

impl fmt::Display for RMonkeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RMonkeyError::UnexpectedToken { expected, got } => write!(
                f,
                "unexpected token error: expected {}, but got {}",
                expected, got
            ),
            RMonkeyError::InvalidPrefix { got } => {
                write!(f, "invalid prefix: {} is invalid for prefix", got)
            }
            RMonkeyError::Custom(msg) => write!(f, "custom error: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, RMonkeyError>;
