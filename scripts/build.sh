#!/bin/bash

# ShadowPay Build Script

set -e

echo "Building ShadowPay WASM contract..."

cd ShadowPay

cargo build --target wasm32-unknown-unknown --release

WASM_PATH="target/wasm32-unknown-unknown/release/shadow_pay.wasm"

if [ -f "$WASM_PATH" ]; then
    SIZE=$(du -h "$WASM_PATH" | cut -f1)
    echo "✓ Build successful!"
    echo "  WASM: $WASM_PATH ($SIZE)"
else
    echo "✗ Build failed"
    exit 1
fi
