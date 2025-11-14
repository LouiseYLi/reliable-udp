#!/bin/bash
# Exit immediately if any command fails
set -e

# BUILD CLIENT
echo "===BUILD CLIENT==="

echo "Running rustfmt..."
cd client/
cargo fmt

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -Dwarnings

echo "Building client..."
cargo build

echo "Client successfully built."

# BUILD SERVER
echo "===BUILD SERVER==="

echo "Running rustfmt..."
cd ../server/
cargo fmt

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -Dwarnings

echo "Building server..."
cargo build

echo "Server successfully built."

# BUILD PROXY
echo "===BUILD PROXY==="

echo "Running rustfmt..."
cd ../proxy/
cargo fmt

echo "Running clippy..."
cargo clippy --all-targets --all-features -- -Dwarnings

echo "Building proxy..."
cargo build

echo "Proxy successfully built."
