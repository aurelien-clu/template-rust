SHELL := /bin/bash
.PHONY : all

all:
	cargo fmt
	cargo test
	cargo build --release --bin server
	cargo build --release --bin client
	./target/release/server
	RUST_LOG=TRACE ./target/release/client
