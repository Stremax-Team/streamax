#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

echo -e "${BLUE}Installing Stremax...${NC}"

# Detect OS and architecture
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
ARCH=$(uname -m)

# Convert architecture names
case $ARCH in
    x86_64)
        ARCH="amd64"
        ;;
    aarch64)
        ARCH="arm64"
        ;;
    *)
        echo -e "${RED}Unsupported architecture: $ARCH${NC}"
        exit 1
        ;;
esac

# Determine latest version
VERSION=$(curl -s https://api.github.com/repos/stremax/stremax/releases/latest | grep '"tag_name":' | cut -d'"' -f4)

# Create installation directory
INSTALL_DIR="/usr/local/bin"
sudo mkdir -p "$INSTALL_DIR"

# Download and install binary
BINARY_URL="https://github.com/stremax/stremax/releases/download/${VERSION}/stremax-${OS}-${ARCH}"
echo -e "${BLUE}Downloading Stremax ${VERSION} for ${OS}-${ARCH}...${NC}"
sudo curl -L "$BINARY_URL" -o "$INSTALL_DIR/strm"
sudo chmod +x "$INSTALL_DIR/strm"

# Verify installation
if command -v strm >/dev/null; then
    echo -e "${GREEN}Stremax has been successfully installed!${NC}"
    echo -e "${BLUE}Version: ${NC}$(strm --version)"
    echo -e "${BLUE}Location: ${NC}$(which strm)"
    echo -e "\nGet started with: strm new my_project"
else
    echo -e "${RED}Installation failed. Please try again or install manually.${NC}"
    exit 1
fi 