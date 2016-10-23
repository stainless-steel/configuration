# Configuration [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a malleable tree structure.

## [Documentation][documentation]

## Example

```rust
let tree = configuration::format::TOML::parse(r#"
    message = "one"

    [foo.bar]
    message = "two"

    [foo.baz]
    answer = 42
"#).unwrap();

assert_eq!(tree.get::<String>("message").unwrap(), "one");
assert_eq!(tree.get::<String>("foo.message").unwrap(), "one");
assert_eq!(tree.get::<String>("foo.bar.message").unwrap(), "two");
assert_eq!(tree.get::<String>("foo.baz.message").unwrap(), "one");

let tree = tree.branch("foo.baz").unwrap();

assert_eq!(tree.get::<i64>("answer").unwrap(), &42);
```

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[documentation]: https://docs.rs/configuration
[status-img]: https://travis-ci.org/stainless-steel/configuration.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/configuration
[version-img]: https://img.shields.io/crates/v/configuration.svg
[version-url]: https://crates.io/crates/configuration
