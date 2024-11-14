.PHONY: all
all: check test

.PHONY: check
check:
	cargo clippy --no-default-features -- -D warnings
	cargo clippy --all-features -- -D warnings
	cargo fmt --all -- --check

.PHONY: test
test:
	cargo test --no-default-features
	cargo test --all-features
