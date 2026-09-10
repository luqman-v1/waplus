.PHONY: all dev run build check test clean install help

# Default target
all: help

# Run Waplus desktop app in development mode
run: dev

dev:
	bun run tauri dev

# Build production desktop application
build:
	bun run tauri build

# Install dependencies via Bun
install:
	bun install

# Run automated E2E tests
test:
	bun test

# Run frontend typecheck and Rust compiler check
check:
	bun run check
	cargo check --manifest-path src-tauri/Cargo.toml
# Clean build artifacts
clean:
	cargo clean --manifest-path src-tauri/Cargo.toml

help:
	@echo Waplus Makefile commands:
	@echo   make run      - Run desktop app in development mode
	@echo   make dev      - Alias for make run
	@echo   make build    - Build production desktop binary
	@echo   make check    - Run Svelte and Cargo checks
	@echo   make test     - Run automated E2E tests (Bun)
	@echo   make install  - Install dependencies via Bun
	@echo   make clean    - Clean Rust build cache
