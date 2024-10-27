use std::error::Error;
use std::fmt::Display;

/// Error type for the storage module.
/// 
/// This error type is used to represent all possible errors that can occur
/// in the storage module.
/// 

#[derive(Debug)]
pub enum WriteError {
    /// Placeholder for a generic error.
    TeaPot,
    NormalizationError(String),
}

impl Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "WriteError: {:?}", self)
    }
}

impl Error for WriteError {}

#[derive(Debug)]
pub enum DocumentError {
    /// Placeholder for a generic error.
    TeaPot,
    NormalizationError(String),
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "DocumentError: {:?}", self)
    }
}

impl Error for DocumentError {}