SHELL:=/usr/bin/env bash

.PHONY: all build build-multiple-arch run clean
all: build run

build:
	cargo build
build-multiple-arch:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	lipo -create -output "target/release/alfred-eudic" "target/aarch64-apple-darwin/release/alfred-eudic" "target/x86_64-apple-darwin/release/alfred-eudic"

run:
	cargo run -- search example
clean:
	@rm -rf rs/target
a:
	@echo "a is $$0"
b:
	@echo "b is $$0"
