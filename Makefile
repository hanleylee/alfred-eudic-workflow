SHELL:=/usr/bin/env bash
BINARY_NAME:=alfred-eudic

.PHONY: all build build-multi-arch test run clean
all: build run

build:
	cargo build --release
build-multi-arch:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	lipo -create -output "target/release/$(BINARY_NAME)" "target/aarch64-apple-darwin/release/$(BINARY_NAME)" "target/x86_64-apple-darwin/release/$(BINARY_NAME)"

test:
	cargo test

run:
	cargo run -- search example
clean:
	@rm -rf rs/target
