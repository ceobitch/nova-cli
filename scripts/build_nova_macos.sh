#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🐛 Building Nova macOS Application${NC}"

# Check if we're on macOS
if [[ "$OSTYPE" != "darwin"* ]]; then
    echo -e "${RED}Error: This script must be run on macOS${NC}"
    exit 1
fi

# Check if Xcode command line tools are installed
if ! command -v xcodebuild &> /dev/null; then
    echo -e "${RED}Error: Xcode command line tools not found. Please install them first.${NC}"
    exit 1
fi

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust not found. Please install Rust first.${NC}"
    exit 1
fi

# Set up Rust toolchain for universal build
echo -e "${YELLOW}Setting up Rust toolchain for universal build...${NC}"
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Build for both architectures
echo -e "${YELLOW}Building for x86_64...${NC}"
cargo build --release --target x86_64-apple-darwin

echo -e "${YELLOW}Building for aarch64...${NC}"
cargo build --release --target aarch64-apple-darwin

# Create universal binary named 'nova' (helper embedded into app bundle)
echo -e "${YELLOW}Creating universal binary...${NC}"
mkdir -p target/universal/release
lipo -create \
    target/x86_64-apple-darwin/release/bug-spray \
    target/aarch64-apple-darwin/release/bug-spray \
    -output target/universal/release/nova

# Verify universal binary
echo -e "${YELLOW}Verifying universal binary...${NC}"
file target/universal/release/nova
lipo -info target/universal/release/nova

# Build the macOS app (do not embed helper before build; we'll copy it after)
echo -e "${YELLOW}Building macOS app...${NC}"
cd NovaApp
# Build and place products under NovaApp/build
xcodebuild -project NovaApp.xcodeproj -scheme NovaApp -configuration Release \
  SYMROOT=$(PWD)/build OBJROOT=$(PWD)/build \
  CODE_SIGNING_ALLOWED=NO \
  CODE_SIGN_ALLOW_ENTITLEMENTS_MODIFICATION=YES \
  build

# Ensure helper binary is embedded in the built app bundle (post-build)
APP_NAME="NovaApp"
APP_BUNDLE="build/Release/${APP_NAME}.app"
HELPERS_DIR="$APP_BUNDLE/Contents/Helpers"
mkdir -p "$HELPERS_DIR"
cp -f ../target/universal/release/nova "$HELPERS_DIR/nova" || true
chmod +x "$HELPERS_DIR/nova" || true

# Embed application icon if icon.png exists at repo root
if [ -f "../icon.png" ]; then
  echo -e "${YELLOW}Embedding app icon...${NC}"
  ICONS_DIR="$APP_BUNDLE/Contents/Resources"
  mkdir -p "$ICONS_DIR"
  # Build an .iconset and generate .icns
  TMP_ICONSET="$(mktemp -d)"/icon.iconset
  mkdir -p "$TMP_ICONSET"
  # Normalize to PNG
  cp -f ../icon.png ../.icon-src.png
  sips -s format png ../.icon-src.png --out ../.icon-src.png >/dev/null 2>&1 || true
  for sz in 16 32 64 128 256 512; do
    sips -z $sz $sz ../.icon-src.png --out "$TMP_ICONSET/icon_${sz}x${sz}.png" >/dev/null 2>&1 || true
    db=$((sz*2))
    if [ $db -le 1024 ]; then sips -z $db $db ../.icon-src.png --out "$TMP_ICONSET/icon_${sz}x${sz}@2x.png" >/dev/null 2>&1 || true; fi
  done
  iconutil -c icns "$TMP_ICONSET" -o "$ICONS_DIR/icon.icns" >/dev/null 2>&1 || true
  rm -rf "$TMP_ICONSET" ../.icon-src.png
  # Point Info.plist to icon.icns
  /usr/libexec/PlistBuddy -c "Set :CFBundleIconFile icon.icns" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || \
  /usr/libexec/PlistBuddy -c "Add :CFBundleIconFile string icon.icns" "$APP_BUNDLE/Contents/Info.plist" 2>/dev/null || true
fi

# Note: The SwiftUI shell implements a glass dark UI with native traffic lights
# and launches the bundled 'nova' helper via PTY.

echo -e "${GREEN}✅ Nova macOS app built successfully!${NC}"
echo -e "${BLUE}App bundle location: NovaApp/build/Release/NovaApp.app${NC}"

