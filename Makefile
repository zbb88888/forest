.PHONY: run run-headless build test clean

# Default run (Graphics mode)
run:
	cargo run

# Headless mode (for cloud/server environment)
run-headless:
	HEADLESS=1 cargo run

# Build release
build:
	cargo build --release

# Run tests
test:
	cargo test

# Clean build artifacts
clean:
	cargo clean
