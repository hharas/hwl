#!/bin/bash

set -e

echo "Building Hwl..."
echo "Running ($ cargo build --release)..."
cargo build --release

if [ $? -ne 0 ]; then
  echo "Failed to build Hwl."
  exit 1
fi

if command -v upx &> /dev/null; then
  echo "Compressing Hwl's Executable..."
  echo "Running ($ upx --best --lzma target/release/hwl)..."
  upx --best --lzma target/release/hwl
fi

echo "Copying Hwl to the /usr/local/bin/ directory..."
echo "Running (# cp ./target/release/hwl /usr/local/bin/hwl)..."
sudo cp ./target/release/hwl /usr/local/bin/hwl

if [ $? -ne 0 ]; then
  echo "Failed to copy Hwl to /usr/local/bin/."
  exit 1
fi

echo "Alhamdulillah! Exiting... "
