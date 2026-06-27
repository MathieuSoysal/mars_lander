#!/usr/bin/env bash
set -euo pipefail

# Build script for Cloudflare Pages' native Git build.
#
# Cloudflare's build image ships Node but not wasm-pack or the Rust wasm target,
# so install whatever is missing before running the normal build (wasm-pack + vite).

# 1. Rust toolchain (via rustup) if cargo isn't already on PATH.
if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi
# Make cargo/rustup available in this shell when rustup just installed them.
[ -f "$HOME/.cargo/env" ] && . "$HOME/.cargo/env"

# 2. wasm32 target (wasm-pack can auto-add it, but be explicit when rustup exists).
if command -v rustup >/dev/null 2>&1; then
  rustup target add wasm32-unknown-unknown
fi

# 3. wasm-pack (prebuilt binary install - fast, no compilation).
if ! command -v wasm-pack >/dev/null 2>&1; then
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# 4. Build the WASM package and bundle the site into ./dist.
npm run build
