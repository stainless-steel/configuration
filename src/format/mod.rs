//! File formats.

#[cfg(feature = "toml")]
mod toml;

#[cfg(feature = "toml")]
pub use self::toml::TOML;
