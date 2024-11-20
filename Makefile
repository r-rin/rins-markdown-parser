build:
	cargo fmt
	cargo test
	cargo build

run-tests:
	cargo test

lint:
	cargo fmt

clippy:
	cargo clippy
	cargo clippy --tests

run:
	cargo fmt
	cargo run $(args)

publish:
	cargo fmt
	cargo test
	cargo publish