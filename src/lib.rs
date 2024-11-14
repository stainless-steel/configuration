//! Means of configuration.

/// Define a configuration.
///
/// ```
/// use std::net::SocketAddr;
/// use std::path::{Path, PathBuf};
///
/// configuration::define! {
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
///                     config_foo_buz() { "buz".into() }
///                 ],
///                 /// The qux.
///                 qux: usize = [
///                     CONFIG_FOO_QUX,
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
///                     config_buz_qux() { "0.0.0.0:80".parse().unwrap() }
///                 ],
///             },
///         /// The qux.
///         qux: String = [CONFIG_QUX],
///     }
/// }
///
/// std::env::set_var("CONFIG_FOO_BAR", "bar");
/// std::env::set_var("CONFIG_BAR_BUZ", "buz");
/// std::env::set_var("CONFIG_BUZ_QUX", "127.0.0.1:80");
/// std::env::set_var("CONFIG_QUX", "qux");
///
/// let config = Config::default();
/// assert_eq!(config.foo.bar, "");
/// assert_eq!(config.buz.qux, "0.0.0.0:80".parse().unwrap());
/// assert_eq!(config.qux, "");
///
/// let config = Config::try_default().unwrap();
/// assert_eq!(config.foo.bar, "bar");
/// assert_eq!(config.foo.buz, Path::new("buz"));
/// assert_eq!(config.foo.qux, 42);
/// assert_eq!(config.bar.buz, "buz");
/// assert_eq!(config.buz.qux, "127.0.0.1:80".parse().unwrap());
/// assert_eq!(config.qux, "qux");
/// ```
#[macro_export]
macro_rules! define {
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
                    $default_variable:ident $(,)?
                    $(
                        $default_function:ident ()
                        $default_block:block
                    )?
                ])?,
            )+
        }
    ) => (
        $(#[$struct_attribute])*
        #[derive(Clone, Debug)]
        pub struct $struct {
            $(
                $(#[$field_attribute])*
                pub $field: $type,
            )+
        }

        impl $struct {
            /// Create an instance from the environment.
            pub fn try_default() -> std::io::Result<Self> {
                Ok(Self {
                    $(
                        $field: $crate::define!(
                            @try_default
                            $type [$($inner)?] $($default_variable $($default_function())?)?
                        ),
                    )+
                })
            }
        }

        impl Default for $struct {
            fn default() -> Self {
                Self {
                    $(
                        $field: $crate::define!(
                            @default
                            $type [$($inner)?] $($default_variable $($default_function())?)?
                        ),
                    )+
                }
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
                $crate::define! {
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
        <$type>::default()
    );
    (@default $type:ty [$($inner:ident)?] $variable:ident) => (
        <$type>::default()
    );
    (@default $type:ty [$($inner:ident)?] $variable:ident $function:ident ()) => (
        $function()
    );
    (@default $type:ty [$($inner:ident)?] $function:ident ()) => (
        $function()
    );
    (@try_default $type:ty []) => (
        <$type>::default()
    );
    (@try_default $type:ty [$inner:ident]) => (
        <$type>::try_default()?
    );
    (@try_default $type:ty [$($inner:ident)?] $variable:ident) => (
        match std::env::var(stringify!($variable)) {
            Ok(value) => std::str::FromStr::from_str(&value).map_err(|error| std::io::Error::other(error))?,
            _ => <$type>::default(),
        }
    );
    (@try_default $type:ty [$($inner:ident)?] $variable:ident $function:ident ()) => (
        match std::env::var(stringify!($variable)) {
            Ok(value) => std::str::FromStr::from_str(&value).map_err(|error| std::io::Error::other(error))?,
            _ => $function(),
        }
    );
    (@try_default $type:ty [$($inner:ident)?] $function:ident ()) => (
        $function()
    );
}
