.PHONY: help build run test clean install dev check fmt clippy doc

help:
	@echo "Gemini MCP Proxy - Development Commands"
	@echo ""
	@echo "  make build      - Build release binary"
	@echo "  make run        - Run with example config"
	@echo "  make dev        - Run in development mode with hot-reload"
	@echo "  make test       - Run all tests"
	@echo "  make check      - Run cargo check"
	@echo "  make fmt        - Format code"
	@echo "  make clippy     - Run clippy linter"
	@echo "  make clean      - Clean build artifacts"
	@echo "  make install    - Install to ~/.local/bin"
	@echo "  make doc        - Generate documentation"

build:
	cargo build --release

run:
	cargo run --release

dev:
	cargo watch -x run

test:
	cargo test --all-features

check:
	cargo check --all-features

fmt:
	cargo fmt --all

clippy:
	cargo clippy --all-features -- -D warnings

clean:
	cargo clean
	rm -f config.toml

install: build
	mkdir -p ~/.local/bin
	cp target/release/gemini-mcp-proxy ~/.local/bin/
	@echo "Installed to ~/.local/bin/gemini-mcp-proxy"
	@echo "Make sure ~/.local/bin is in your PATH"

doc:
	cargo doc --no-deps --open

# Setup development environment
setup:
	rustup component add rustfmt clippy
	cargo install cargo-watch
	@echo "Development tools installed!"
	@echo "Copy config.example.toml to config.toml and customize"

# Quick test with minimal config
quick-test:
	@echo "Creating minimal test config..."
	@cp config.example.toml /tmp/test-config.toml
	cargo run --release -- --config /tmp/test-config.toml