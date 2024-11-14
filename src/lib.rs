//! Means of configuration.

#[cfg(feature = "serde")]
#[doc(hidden)]
pub extern crate serde;

#[cfg(feature = "serde_json")]
#[doc(hidden)]
pub extern crate serde_json;

/// Define a configuration.
///
/// ```
/// use std::net::SocketAddr;
/// use std::path::{Path, PathBuf};
///
/// configuration::config! {
///     /// A config.
///     Config {
///         /// The foo config.
///         foo: Foo
///         where
///             /// A foo config.
///             Foo {
///                 /// The bar.
///                 bar: String = [CONFIG_FOO_BAR],
///                 /// The buz.
///                 buz: PathBuf = [
///                     CONFIG_FOO_BUZ,
///                     "config_foo_buz",
///                     config_foo_buz() { "buz".into() }
///                 ],
///                 /// The qux.
///                 qux: usize = [
///                     CONFIG_FOO_QUX,
///                     "config_foo_qux",
///                     config_foo_qux() { 42 }
///                 ],
///             },
///         /// The bar config.
///         bar: Bar
///         where
///             /// A bar config.
///             Bar {
///                 /// The buz.
///                 buz: String = [CONFIG_BAR_BUZ],
///             },
///         /// The buz config.
///         buz: Buz
///         where
///             /// A buz config.
///             Buz {
///                 /// The qux.
///                 qux: SocketAddr = [
///                     CONFIG_BUZ_QUX,
///                     "config_buz_qux",
///                     config_buz_qux() { "0.0.0.0:80".parse().unwrap() }
///                 ],
///             },
///         /// The qux.
///         qux: String = [CONFIG_QUX],
///     }
/// }
///
/// std::env::set_var("CONFIG_BAR_BUZ", "buz");
/// std::env::set_var("CONFIG_QUX", "qux");
///
/// let config = Config::default().unwrap();
///
/// assert_eq!(config.foo.bar, "");
/// assert_eq!(config.foo.buz, Path::new("buz"));
/// assert_eq!(config.foo.qux, 42);
/// assert_eq!(config.bar.buz, "buz");
/// assert_eq!(config.buz.qux, "0.0.0.0:80".parse().unwrap());
/// assert_eq!(config.qux, "qux");
/// ```
#[macro_export]
macro_rules! config {
    (
        $(#[$struct_attribute:meta])*
        $struct:ident {
            $(
                $(#[$field_attribute:meta])*
                $field:ident:
                $type:ty
                $(
                    where
                    $(#[$type_attribute:meta])*
                    $inner:ident {
                        $($nested:tt)+
                    }
                )?
                $(= [
                    $(
                        $default_variable:ident $(,)?
                    )?
                    $(
                        $default_literal:literal,
                        $default_function:ident ()
                        $default_block:block
                    )?
                ])?,
            )+
        }
    ) => (
        $(#[$struct_attribute])*
        #[derive(Clone, Debug)]
        #[cfg_attr(feature = "serde", derive($crate::serde::Deserialize, $crate::serde::Serialize))]
        pub struct $struct {
            $(
                $(#[$field_attribute])*
                $($(#[cfg_attr(feature = "serde", serde(default = $default_literal))])?)?
                pub $field: $type,
            )+
        }

        impl $struct {
            /// Create an instance from a JSON file.
            #[cfg(feature = "serde_json")]
            #[inline]
            pub fn new<T: AsRef<std::path::Path>>(path: T) -> std::io::Result<Self> {
                let file = std::fs::File::open(path)?;
                Ok($crate::serde_json::from_reader(file)?)
            }

            /// Create an instance from the environment.
            #[allow(clippy::should_implement_trait)]
            pub fn default() -> std::io::Result<Self> {
                Ok(Self {
                    $(
                        $field: $crate::config!(
                            @default
                            $type [$($inner)?] $($($default_variable)? $($default_function())?)?
                        ),
                    )+
                })
            }
        }

        $(
            $(
                $(
                    #[inline]
                    fn $default_function() -> $type $default_block
                )?
            )?
        )+

        $(
            $(
                $crate::config! {
                    $(#[$type_attribute])*
                    $inner {
                        $($nested)+
                    }
                }
            )?
        )+
    );
    (@default $type:ty []) => (
        <$type>::default()
    );
    (@default $type:ty [$inner:ident]) => (
        <$type>::default()?
    );
    (@default $type:ty [$($inner:ident)?] $variable:ident) => (
        match std::env::var(stringify!($variable)) {
            Ok(value) => match std::str::FromStr::from_str(&value) {
                Ok(value) => value,
                Err(error) => return Err(std::io::Error::other(error)),
            },
            _ => <$type>::default(),
        }
    );
    (@default $type:ty [$($inner:ident)?] $variable:ident $function:ident ()) => (
        match std::env::var(stringify!($variable)) {
            Ok(value) => match std::str::FromStr::from_str(&value) {
                Ok(value) => value,
                Err(error) => return Err(std::io::Error::other(error)),
            },
            _ => $function(),
        }
    );
    (@default $type:ty [$($inner:ident)?] $function:ident ()) => (
        $function()
    );
}
