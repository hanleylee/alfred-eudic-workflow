SHELL:=/usr/bin/env bash
ALFRED_EUDIC_WORKFLOW="/Users/hanley/Library/Mobile Documents/com~apple~CloudDocs/ihanley/config/Alfred/Alfred.alfredpreferences/workflows/user.workflow.800F5D55-E73C-4C91-B86B-0A6D37216D19"

.PHONY: all build install run clean
all: build run

build:
	# swift build -c release --build-path ".build" --target alfred-qsirch
	swift build -c release --build-path ".build"
install:
	@install -D -m 755 .build/release/alfred-eudic $(ALFRED_EUDIC_WORKFLOW)/bin/alfred-eudic
run:
	swift run --build-path .build alfred-qsirch search example
clean:
	@rm -rf .build .swiftpm
a:
	@echo "a is $$0"
b:
	@echo "b is $$0"
