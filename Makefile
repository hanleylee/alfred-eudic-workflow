SHELL:=/usr/bin/env bash
ALFRED_EUDIC_WORKFLOW="/Users/hanley/Library/Mobile Documents/com~apple~CloudDocs/ihanley/config/Alfred/Alfred.alfredpreferences/workflows/user.workflow.800F5D55-E73C-4C91-B86B-0A6D37216D19"

.PHONY: all build build-rs install install-rs run clean
all: build run

# Swift (legacy)
build:
	cargo build
build-multiple-arch:
	cargo build --release --target aarch64-apple-darwin
	cargo build --release --target x86_64-apple-darwin
	lipo -create -output "target/release/alfred-eudic" "target/aarch64-apple-darwin/release/alfred-eudic" "target/x86_64-apple-darwin/release/alfred-eudic"
install:
	@install -D -m 755 .build/release/alfred-eudic $(ALFRED_EUDIC_WORKFLOW)/bin/alfred-eudic

# Rust (alfred-eudic-workflow/rs)
build-rs:
install-rs:
	@install -D -m 755 rs/target/release/alfred-eudic $(ALFRED_EUDIC_WORKFLOW)/bin/alfred-eudic

run:
	swift run --build-path .build alfred-qsirch search example
run-rs:
	cd rs && cargo run -- search example
clean:
	@rm -rf .build .swiftpm
	@rm -rf rs/target
a:
	@echo "a is $$0"
b:
	@echo "b is $$0"
