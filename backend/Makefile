# Makefile for IPA Pronunciation Coach Backend

.PHONY: fmt fmt-check build test run clean

# Format code with 2-space indentation
fmt:
	cargo +nightly fmt -- --config indent_style=Block,tab_spaces=2

# Check formatting with 2-space indentation
fmt-check:
	cargo +nightly fmt -- --check --config indent_style=Block,tab_spaces=2

# Build the project
build:
	cargo build

# Run tests
test:
	cargo test

# Run the application
run:
	cargo run

# Clean build artifacts
clean:
	cargo clean

# Default target
all: fmt-check build test
