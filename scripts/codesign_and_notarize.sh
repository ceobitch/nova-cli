#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🔐 Code Signing and Notarizing Nova macOS Application${NC}"

# Configuration
APP_NAME="NovaApp"
APP_PATH="NovaApp/build/Release/${APP_NAME}.app"
HELPER_PATH="${APP_PATH}/Contents/Helpers/nova"
BUNDLE_ID="com.nova.app"

# Non-interactive credentials (env override supported). WARNING: storing secrets in source is insecure.
TEAM_ID="${TEAM_ID:-7CS3Y36YYN}"
APPLE_ID="${APPLE_ID:-miguelortegaaa@icloud.com}"
APP_SPECIFIC_PASSWORD="${APP_SPECIFIC_PASSWORD:-vovo-otxk-gdjh-oelg}"

# Optional: explicitly set a signing identity common name. If empty, we'll auto-detect.
SIGN_IDENTITY="${SIGN_IDENTITY:-}"

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}Error: App bundle not found at $APP_PATH${NC}"
    echo -e "${YELLOW}Please run build_nova_macos.sh first${NC}"
    exit 1
fi

# Resolve signing identity
if [ -z "$SIGN_IDENTITY" ]; then
    # Try to find a Developer ID Application identity installed in keychain
    SIGN_IDENTITY=$(security find-identity -v -p codesigning 2>/dev/null | grep -E '"Developer ID Application: ' | head -n1 | sed -E 's/.*"(.*)".*/\1/') || true
fi

DO_NOTARIZE=0
if [ -z "$SIGN_IDENTITY" ]; then
    echo -e "${YELLOW}No 'Developer ID Application' certificate found in your keychain. Using ad-hoc signing; notarization will be skipped.${NC}"
    SIGN_IDENTITY="-"
    DO_NOTARIZE=1
else
    echo -e "Using signing identity: ${GREEN}$SIGN_IDENTITY${NC}"
fi

# Step 1: Code sign the helper binary (sign helper first)
if [ -f "$HELPER_PATH" ]; then
  echo -e "${YELLOW}Step 1: Code signing helper binary...${NC}"
  codesign --force --timestamp --options runtime --sign "$SIGN_IDENTITY" \
      --entitlements NovaApp/NovaApp.entitlements \
      "$HELPER_PATH"
else
  echo -e "${YELLOW}Helper not found at ${HELPER_PATH}; skipping helper signing.${NC}"
fi

# Step 2: Code sign the app bundle
echo -e "${YELLOW}Step 2: Code signing app bundle...${NC}"
codesign --force --timestamp --options runtime --sign "$SIGN_IDENTITY" \
    --entitlements NovaApp/NovaApp.entitlements \
    --deep \
    "$APP_PATH"

# Step 3: Verify code signing
echo -e "${YELLOW}Step 3: Verifying code signing...${NC}"
codesign --verify --verbose=4 "$APP_PATH"
codesign --verify --verbose=4 "$HELPER_PATH"

# Step 4: Create DMG
echo -e "${YELLOW}Step 4: Creating DMG...${NC}"
DMG_NAME="Nova-1.0.0-universal.dmg"
DMG_PATH="dist/${DMG_NAME}"

mkdir -p dist

# Create DMG using create-dmg if available, otherwise use hdiutil
if command -v create-dmg &> /dev/null; then
    create-dmg \
        --volname "Nova" \
        --volicon "NovaApp/Assets.xcassets/AppIcon.appiconset/icon_512x512.png" \
        --window-pos 200 120 \
        --window-size 600 300 \
        --icon-size 100 \
        --icon "Nova.app" 175 120 \
        --hide-extension "Nova.app" \
        --app-drop-link 425 120 \
        "$DMG_PATH" \
        "$APP_PATH"
else
    echo -e "${YELLOW}create-dmg not found, using hdiutil...${NC}"
    hdiutil create -volname "Nova" -srcfolder "$APP_PATH" -ov -format UDZO "$DMG_PATH"
fi

# Step 5: Code sign the DMG
echo -e "${YELLOW}Step 5: Code signing DMG...${NC}"
codesign --force --options runtime --sign "$SIGN_IDENTITY" \
    "$DMG_PATH"

# Step 6: Notarize the DMG
if [ "$DO_NOTARIZE" -eq 0 ]; then
    echo -e "${YELLOW}Step 6: Notarizing DMG...${NC}"
    xcrun notarytool submit "$DMG_PATH" \
        --apple-id "$APPLE_ID" \
        --password "$APP_SPECIFIC_PASSWORD" \
        --team-id "$TEAM_ID" \
        --wait
else
    echo -e "${YELLOW}Skipping notarization (no Developer ID identity available).${NC}"
fi

# Step 7: Staple the notarization ticket
if [ "$DO_NOTARIZE" -eq 0 ]; then
    echo -e "${YELLOW}Step 7: Stapling notarization ticket...${NC}"
    xcrun stapler staple "$DMG_PATH"
fi

# Step 8: Verify notarization
if [ "$DO_NOTARIZE" -eq 0 ]; then
    echo -e "${YELLOW}Step 8: Verifying notarization...${NC}"
    xcrun stapler validate "$DMG_PATH"
fi

# Step 9: Generate checksum
echo -e "${YELLOW}Step 9: Generating SHA-256 checksum...${NC}"
CHECKSUM=$(shasum -a 256 "$DMG_PATH" | cut -d' ' -f1)
echo "$CHECKSUM" > "${DMG_PATH}.sha256"
echo "SHA-256: $CHECKSUM"

echo -e "${GREEN}✅ Code signing and notarization completed successfully!${NC}"
echo -e "${BLUE}DMG location: $DMG_PATH${NC}"
echo -e "${BLUE}Checksum file: ${DMG_PATH}.sha256${NC}"
echo -e "${YELLOW}You can now distribute the DMG file.${NC}"

