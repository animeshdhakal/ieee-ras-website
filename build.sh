#!/bin/bash
set -e

echo "=== Preparing Build Environment ==="

# Ensure Rust & wasm32 target
if ! command -v rustc &> /dev/null; then
  echo "Installing Rust toolchain..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --target wasm32-unknown-unknown
  source "$HOME/.cargo/env"
else
  rustup target add wasm32-unknown-unknown
fi

# Ensure Trunk
if ! command -v trunk &> /dev/null; then
  echo "Fetching prebuilt Trunk binary..."
  mkdir -p "$HOME/.bin"
  # Vercel containers run x86_64 Linux
  curl -sL "https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-gnu.tar.gz" | tar -xz -C "$HOME/.bin"
  export PATH="$HOME/.bin:$PATH"
fi

echo "Building with $(trunk --version)..."
trunk build --release
echo "=== Build Completed Successfully ==="
