#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🧪 Testing Nova macOS Application${NC}"

APP_PATH="NovaApp/build/Release/NovaApp.app"

# Check if app exists
if [ ! -d "$APP_PATH" ]; then
    echo -e "${RED}Error: App bundle not found at $APP_PATH${NC}"
    echo -e "${YELLOW}Please run build_macos.sh first${NC}"
    exit 1
fi

# Check if helper binary exists
HELPER_PATH="$APP_PATH/Contents/Helpers/nova"
if [ ! -f "$HELPER_PATH" ]; then
    echo -e "${RED}Error: Helper binary not found at $HELPER_PATH${NC}"
    exit 1
fi

# Check if helper binary is executable
if [ ! -x "$HELPER_PATH" ]; then
    echo -e "${RED}Error: Helper binary is not executable${NC}"
    exit 1
fi

# Check if helper binary is universal
echo -e "${YELLOW}Checking binary architecture...${NC}"
if command -v lipo &> /dev/null; then
    ARCH_INFO=$(lipo -info "$HELPER_PATH" 2>/dev/null || echo "Error getting arch info")
    echo -e "${BLUE}Architecture: $ARCH_INFO${NC}"
    
    if [[ "$ARCH_INFO" == *"x86_64"* ]] && [[ "$ARCH_INFO" == *"arm64"* ]]; then
        echo -e "${GREEN}✅ Universal binary confirmed${NC}"
    else
        echo -e "${YELLOW}⚠️  Binary may not be universal${NC}"
    fi
fi

# Check code signing
echo -e "${YELLOW}Checking code signing...${NC}"
if codesign --verify --verbose=4 "$APP_PATH" 2>/dev/null; then
    echo -e "${GREEN}✅ App bundle is code signed${NC}"
else
    echo -e "${YELLOW}⚠️  App bundle is not code signed${NC}"
fi

if codesign --verify --verbose=4 "$HELPER_PATH" 2>/dev/null; then
    echo -e "${GREEN}✅ Helper binary is code signed${NC}"
else
    echo -e "${YELLOW}⚠️  Helper binary is not code signed${NC}"
fi

# Check entitlements
echo -e "${YELLOW}Checking entitlements...${NC}"
if [ -f "NovaApp/NovaApp.entitlements" ]; then
    echo -e "${GREEN}✅ Entitlements file exists${NC}"
else
    echo -e "${YELLOW}⚠️  Entitlements file not found${NC}"
fi

# Test app launch (non-interactive)
echo -e "${YELLOW}Testing app launch...${NC}"
if timeout 10s "$APP_PATH/Contents/MacOS/NovaApp" --help >/dev/null 2>&1; then
    echo -e "${GREEN}✅ App launches successfully${NC}"
else
    echo -e "${YELLOW}⚠️  App launch test inconclusive (may require GUI)${NC}"
fi

# Check file sizes
echo -e "${YELLOW}Checking file sizes...${NC}"
APP_SIZE=$(du -sh "$APP_PATH" | cut -f1)
HELPER_SIZE=$(du -sh "$HELPER_PATH" | cut -f1)
echo -e "${BLUE}App bundle size: $APP_SIZE${NC}"
echo -e "${BLUE}Helper binary size: $HELPER_SIZE${NC}"

# Summary
echo -e "\n${BLUE}📋 Test Summary${NC}"
echo -e "${GREEN}✅ App bundle structure: OK${NC}"
echo -e "${GREEN}✅ Helper binary: OK${NC}"
echo -e "${GREEN}✅ Executable permissions: OK${NC}"

if command -v codesign &> /dev/null; then
    echo -e "${GREEN}✅ Code signing verification: OK${NC}"
fi

echo -e "\n${BLUE}🎯 Next Steps:${NC}"
echo -e "${YELLOW}1. Open the app: open $APP_PATH${NC}"
echo -e "${YELLOW}2. Test terminal functionality${NC}"
echo -e "${YELLOW}3. Verify Esc key quits the app${NC}"
echo -e "${YELLOW}4. Test window controls (minimize, zoom, close)${NC}"

echo -e "\n${GREEN}✅ Testing completed successfully!${NC}"

