# Configuration [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides a malleable tree structure.

## [Documentation][doc]

## Example

```rust
let tree = configuration::format::toml::parse(r#"
    message = "one"

    [foo.bar]
    message = "two"

    [foo.baz]
    answer = 42
"#).unwrap();

assert_eq!(tree.get::<String>("message").unwrap(), "one");
assert_eq!(tree.get::<String>("foo.bar.message").unwrap(), "two");
assert_eq!(tree.get::<String>("foo.baz.message").unwrap(), "one");

let tree = tree.branch("foo.baz").unwrap();

assert_eq!(tree.get::<i64>("answer").unwrap(), &42);
```

## Contributing

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/configuration.svg
[version-url]: https://crates.io/crates/configuration
[status-img]: https://travis-ci.org/stainless-steel/configuration.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/configuration
[doc]: https://stainless-steel.github.io/configuration
