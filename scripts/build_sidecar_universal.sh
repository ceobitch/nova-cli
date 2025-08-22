#!/bin/bash

set -euo pipefail

echo "Building universal 'nova' sidecar (macOS)"

if [[ "${OSTYPE}" != darwin* ]]; then
  echo "This script is macOS-only."
  exit 1
fi

rustup target add x86_64-apple-darwin >/dev/null 2>&1 || true
rustup target add aarch64-apple-darwin >/dev/null 2>&1 || true

cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

mkdir -p target/universal/release
lipo -create \
  target/x86_64-apple-darwin/release/nova \
  target/aarch64-apple-darwin/release/nova \
  -output target/universal/release/nova

file target/universal/release/nova
lipo -info target/universal/release/nova

echo "Universal 'nova' built at target/universal/release/nova"

#!/bin/bash
set -e
cd "./.."
rustup target add aarch64-apple-darwin x86_64-apple-darwin >/dev/null || true
cargo build --release --target aarch64-apple-darwin
cargo build --release --target x86_64-apple-darwin
mkdir -p app/src-tauri/bin
lipo -create target/aarch64-apple-darwin/release/bug-spray target/x86_64-apple-darwin/release/bug-spray -output app/src-tauri/bin/nova
chmod +x app/src-tauri/bin/nova
