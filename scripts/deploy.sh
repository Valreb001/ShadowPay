#!/bin/bash

# ShadowPay Deployment Script
# Usage: ./scripts/deploy.sh [testnet|mainnet]

set -e

NETWORK=${1:-testnet}

if [ "$NETWORK" != "testnet" ] && [ "$NETWORK" != "mainnet" ]; then
    echo "Usage: ./scripts/deploy.sh [testnet|mainnet]"
    exit 1
fi

# Load environment variables
if [ ! -f .env ]; then
    echo "Error: .env file not found. Copy .env.example to .env and fill in values."
    exit 1
fi

source .env

echo "Deploying to $NETWORK..."

# Step 1: Build WASM
echo "Step 1: Building WASM contract..."
cd ShadowPay
cargo build --target wasm32-unknown-unknown --release
cd ..

WASM_PATH="ShadowPay/target/wasm32-unknown-unknown/release/shadow_pay.wasm"

# Step 2: Deploy contract
echo "Step 2: Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
  --wasm "$WASM_PATH" \
  --network "$NETWORK" \
  --source "$DEPLOYER_SECRET_KEY" \
  | grep -oP 'Contract ID: \K[^"]*' || echo "")

if [ -z "$CONTRACT_ID" ]; then
    echo "Error: Failed to deploy contract"
    exit 1
fi

echo "Contract deployed: $CONTRACT_ID"

# Step 3: Initialize contract
echo "Step 3: Initializing contract..."
stellar contract invoke \
  --id "$CONTRACT_ID" \
  --fn initialize \
  --network "$NETWORK" \
  --source "$DEPLOYER_SECRET_KEY" \
  -- \
  --deployer "$DEPLOYER_SECRET_KEY" \
  --admin "$ADMIN_ADDRESS" \
  --token "$TOKEN_CONTRACT"

echo "Deployment complete!"
echo "Contract ID: $CONTRACT_ID"
echo "Update .env with: CONTRACT_ID=$CONTRACT_ID"
