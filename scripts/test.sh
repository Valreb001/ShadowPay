#!/bin/bash

# ShadowPay Test Script

set -e

echo "Running ShadowPay tests..."

cd ShadowPay

# Run all tests
cargo test --lib

# Run with output
echo ""
echo "Running tests with output..."
cargo test --lib -- --nocapture

echo ""
echo "All tests passed!"
