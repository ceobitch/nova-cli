#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Building Nova macOS Application${NC}"

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}Error: This script must be run on macOS${NC}"
    exit 1
fi

# Check if required tools are installed
if ! command -v xcodebuild &> /dev/null; then
    echo -e "${RED}Error: Xcode command line tools not found. Please install them first.${NC}"
    echo -e "${YELLOW}Run: xcode-select --install${NC}"
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust not found. Please install Rust first.${NC}"
    echo -e "${YELLOW}Run: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh${NC}"
    exit 1
fi

# Make scripts executable
chmod +x scripts/build_nova_macos.sh
chmod +x scripts/codesign_and_notarize.sh

# Run the build script
echo -e "${YELLOW}Starting build process...${NC}"
./scripts/build_nova_macos.sh

echo -e "${GREEN}✅ Build completed successfully!${NC}"
echo -e "${BLUE}Next steps:${NC}"
echo -e "${YELLOW}1. Code sign and notarize: ./scripts/codesign_and_notarize.sh${NC}"
echo -e "${YELLOW}2. Test the app: open NovaApp/build/Release/NovaApp.app${NC}"
echo -e "${YELLOW}3. Create DMG for distribution${NC}"

