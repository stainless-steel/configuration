features := serde serde_json

.PHONY: all
all: check test

.PHONY: check
check: $(addprefix check-,$(features))
	cargo clippy --no-default-features -- -D warnings
	cargo clippy --all-features -- -D warnings
	cargo fmt --all -- --check

.PHONY: $(addprefix check-,$(features))
$(addprefix check-,$(features)): check-%:
	cargo clippy --features $* -- -D warnings

.PHONY: test
test: $(addprefix test-,$(features))
	cargo test --no-default-features
	cargo test --all-features

.PHONY: $(addprefix test-,$(features))
$(addprefix test-,$(features)): test-%:
	cargo build --features $*
