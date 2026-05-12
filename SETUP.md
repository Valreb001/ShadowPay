# Development Setup

## Prerequisites

### 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

### 2. Install Stellar CLI

```bash
cargo install --locked stellar-cli
stellar --version
```

### 3. Verify Installation

```bash
rustc --version
cargo --version
stellar --version
```

## Build & Test

```bash
# Build WASM contract
make build

# Run tests
make test

# Format code
make fmt

# Lint code
make lint

# Check dependencies
make audit
```

## Project Structure

```
ShadowPay/
├── ShadowPay/
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs          # Main contract
│       ├── errors.rs       # Error types
│       └── storage.rs      # Storage keys
├── .github/workflows/
│   └── ci.yml              # CI/CD pipeline
├── Cargo.toml              # Workspace config
├── Makefile                # Build targets
├── README.md               # Project documentation
├── CONTRIBUTING.md         # Contribution guidelines
├── SECURITY.md             # Security policy
└── LICENSE                 # MIT License
```

## Next Steps

1. **Week 1**: Enhance ZK-proof verification with actual cryptographic validation
2. **Week 2**: Integrate Stellar Anchor API for settlement
3. **Week 3**: Build Privacy Shield frontend toggle

## Troubleshooting

**Cargo not found**: Ensure Rust is installed and `~/.cargo/bin` is in your PATH.

**WASM target missing**: Run `rustup target add wasm32-unknown-unknown`

**Tests failing**: Ensure you're using Soroban SDK 21.0 or later.
