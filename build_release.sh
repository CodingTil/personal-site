#!/bin/bash

# Set option to exit on any error
set -e

# Get Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the cargo env
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Get cargo binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Get trunk
cargo binstall trunk

# Get wasm-opt
cargo install wasm-opt --locked

# Clean the project
trunk clean
cargo clean

# Get current directory
ROOT_DIR=$(pwd)

# First build the submodules
cd $ROOT_DIR/fractal_rust
trunk build --release --public-url "/public/project_code/fractal_rust/"
cp -r $ROOT_DIR/fractal_rust/dist/* $ROOT_DIR/src/public/project_code/fractal_rust/

# Then build the main project
cd $ROOT_DIR
trunk build --release

# Find all .wasm files in the current directory and subdirectories
find dist/ -name "*.wasm" | while read wasm_file; do
    # Execute wasm-opt command on each file
    wasm-opt -Oz -o "$wasm_file" "$wasm_file"
done

