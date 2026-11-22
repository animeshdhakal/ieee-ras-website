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
  echo "Fetching prebuilt Trunk static binary (musl)..."
  mkdir -p "$HOME/.bin"
  # Statically linked musl binary has zero glibc dependencies for Vercel Amazon Linux 2
  curl -sL "https://github.com/trunk-rs/trunk/releases/latest/download/trunk-x86_64-unknown-linux-musl.tar.gz" | tar -xz -C "$HOME/.bin"
fi

export PATH="$HOME/.bin:$HOME/.cargo/bin:$PATH"

echo "Building with $(trunk --version)..."
trunk build --release
echo "=== Build Completed Successfully ==="
