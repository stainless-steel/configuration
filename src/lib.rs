extern crate options;

#[cfg(feature = "toml")]
extern crate toml;

use std::{error, fmt};

macro_rules! ok(
    ($result:expr) => (match $result {
        Ok(result) => result,
        Err(error) => raise!(error),
    });
);

macro_rules! raise(
    ($message:expr) => (return Err(::Error($message.to_string())));
    ($($argument:tt)*) => (return Err(::Error(format!($($argument)*))));
);

/// An error.
pub struct Error(String);

/// A result.
pub type Result<T> = std::result::Result<T, Error>;

pub mod format;

mod node;
mod tree;

pub use node::Node;
pub use tree::Tree;

impl error::Error for Error {
    #[inline]
    fn description(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}
