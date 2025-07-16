#!/bin/bash

# Exit immediately if a command exits with a non-zero status.
set -e

echo "Installing system dependencies..."
sudo apt-get update
sudo apt-get install -y build-essential cmake qtbase5-dev libqt5svg5-dev

echo "Installing Rust toolchain if not already installed..."
if ! command -v rustup &> /dev/null
then
    echo "rustup not found, installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust is already installed."
fi

echo "Building Rust backend..."
cargo build --release --manifest-path rust_backend/Cargo.toml

echo "Building C++ frontend..."
mkdir -p build
cd build
cmake ..
make
cd ..

echo "Build complete!"