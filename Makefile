.PHONY: build test fmt lint clean audit help

help:
	@echo "ShadowPay - Privacy-Preserving Compliance Layer"
	@echo ""
	@echo "Available targets:"
	@echo "  build       - Build WASM contract"
	@echo "  test        - Run all tests"
	@echo "  fmt         - Format code"
	@echo "  lint        - Run clippy linter"
	@echo "  audit       - Check dependencies for vulnerabilities"
	@echo "  clean       - Clean build artifacts"

build:
	cd ShadowPay && cargo build --target wasm32-unknown-unknown --release

test:
	cd ShadowPay && cargo test

fmt:
	cd ShadowPay && cargo fmt

lint:
	cd ShadowPay && cargo clippy -- -D warnings

audit:
	cd ShadowPay && cargo audit

clean:
	cd ShadowPay && cargo clean
