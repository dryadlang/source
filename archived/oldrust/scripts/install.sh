#!/bin/bash
set -e

# Dryad Automatic Installer (Unix-like)
# Usage: curl -fsSL https://dryadlang.org/install.sh | bash

DRYAD_HOME="$HOME/.dryad"
DRYAD_BIN="$DRYAD_HOME/bin"
BASE_URL="https://github.com/dryad-lang/source/releases/latest/download"

echo "Installing Dryad..."

# Create directory structure
mkdir -p "$DRYAD_BIN"

# Detect OS and Architecture
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"

case "$OS" in
    linux*)     PLATFORM="linux" ;;
    darwin*)    PLATFORM="macos" ;;
    *)          echo "Unsupported OS: $OS"; exit 1 ;;
esac

case "$ARCH" in
    x86_64)     ARCH_NAME="x86_64" ;;
    aarch64|arm64) ARCH_NAME="arm64" ;;
    *)          echo "Unsupported architecture: $ARCH"; exit 1 ;;
esac

echo "Detected: $PLATFORM ($ARCH_NAME)"

# Download binaries
echo "Downloading dryad..."
curl -L "$BASE_URL/dryad-$PLATFORM-$ARCH_NAME" -o "$DRYAD_BIN/dryad"
chmod +x "$DRYAD_BIN/dryad"

echo "Downloading oak..."
curl -L "$BASE_URL/oak-$PLATFORM-$ARCH_NAME" -o "$DRYAD_BIN/oak"
chmod +x "$DRYAD_BIN/oak"

echo "--------------------------------------------------------"
echo "Dryad has been installed to $DRYAD_BIN"
echo ""
echo "Add the following to your shell profile (.bashrc, .zshrc, or .profile):"
echo "  export PATH=\"\$HOME/.dryad/bin:\$PATH\""
echo "--------------------------------------------------------"
echo "Run 'dryad --version' to verify the installation."
