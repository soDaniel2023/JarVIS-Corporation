//! The shared error type.
//!
//! One error type for the crate, so early modules don't each define
//! their own before there's a real need to distinguish error kinds.

use std::fmt;

#[derive(Debug)]
pub enum CoreError {
    Other(String),
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::Other(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for CoreError {}
