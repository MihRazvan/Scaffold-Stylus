# scripts/deploy.sh
#!/bin/bash

# Load environment variables
source .env

# Build the contract
echo "Building contract..."
cargo build --release

# Deploy the contract
echo "Deploying contract..."
cargo run --bin deploy

# Verify the contract
echo "Verifying contract..."
cargo run --bin verify