//! Malleable tree structure.
//!
//! ```
//! let tree = configuration::format::TOML::parse(r#"
//!     message = "one"
//!
//!     [foo.bar]
//!     message = "two"
//!
//!     [foo.baz]
//!     answer = 42
//! "#).unwrap();
//!
//! assert_eq!(tree.get::<String>("message").unwrap(), "one");
//! assert_eq!(tree.get::<String>("foo.message").unwrap(), "one");
//! assert_eq!(tree.get::<String>("foo.bar.message").unwrap(), "two");
//! assert_eq!(tree.get::<String>("foo.baz.message").unwrap(), "one");
//!
//! let tree = tree.branch("foo.baz").unwrap();
//!
//! assert_eq!(tree.get::<i64>("answer").unwrap(), &42);
//! ```

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
