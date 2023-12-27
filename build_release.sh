#!/bin/bash

# Set option to exit on any error
set -e

# Get Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Source the cargo env
source $HOME/.cargo/env

# Add wasm target
rustup target add wasm32-unknown-unknown

# Get trunk
cargo install --locked trunk

# Get current directory
ROOT_DIR=$(pwd)

# First build the submodules
cd $ROOT_DIR/fractal_rust
trunk build --release --public-url "/public/project_code/fractal_rust/"
cp -r $ROOT_DIR/fractal_rust/dist/* $ROOT_DIR/src/public/project_code/fractal_rust/

# Then build the main project
cd $ROOT_DIR
trunk build --release
