#!/bin/bash
set -e

# Detect architecture
ARCH=$(uname -m)

if [ "$ARCH" = "aarch64" ]; then
    echo "Detected ARM64 architecture (likely Raspberry Pi 5). Using Box64..."
    # Execute with box64
    exec box64 ./TerrariaServer.bin.x86_64 "$@"
elif [ "$ARCH" = "x86_64" ]; then
    echo "Detected x86_64 architecture. Running natively..."
    # Execute natively
    exec ./TerrariaServer.bin.x86_64 "$@"
else
    echo "Unsupported architecture: $ARCH"
    exit 1
fi
