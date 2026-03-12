.PHONY: run test fmt clippy check

run:
	cargo run

test:
	cargo test

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-targets --all-features -- -D warnings

check:
	cargo check
