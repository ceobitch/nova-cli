# Nova macOS Application Setup Guide

This guide will walk you through setting up your development environment to build the Nova macOS application.

## Prerequisites

### 1. macOS Requirements
- **macOS Version**: 13.0 (Ventura) or later
- **Architecture**: Intel (x86_64) or Apple Silicon (arm64)
- **Storage**: At least 10GB free space for development tools

### 2. Development Tools

#### Xcode
```bash
# Install Xcode from the Mac App Store, then install command line tools
xcode-select --install
```

**Version**: Xcode 15.0 or later is required for SwiftUI features and modern macOS APIs.

#### Rust Toolchain
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload shell environment
source ~/.zshrc  # or ~/.bash_profile

# Verify installation
rustc --version
cargo --version
```

#### Homebrew
```bash
# Install Homebrew
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install required tools
brew install create-dmg
```

### 3. Apple Developer Account

For code signing and distribution, you'll need:
- **Apple Developer Account**: $99/year
- **Developer ID Application Certificate**: For distribution outside the App Store
- **App-Specific Password**: For notarization

## Initial Setup

### 1. Clone and Navigate
```bash
# Clone the repository (if not already done)
git clone <your-repo-url>
cd nova-cli

# Make scripts executable
chmod +x build_macos.sh
chmod +x scripts/*.sh
```

### 2. Install Rust Targets
```bash
# Add targets for universal binary
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin

# Verify targets
rustup target list --installed
```

### 3. Configure Development Environment

#### Set Environment Variables
```bash
# Add to your shell profile (~/.zshrc or ~/.bash_profile)
export TEAM_ID="YOUR_TEAM_ID"
export APPLE_ID="your.apple.id@example.com"
export APP_SPECIFIC_PASSWORD="your-app-specific-password"

# Reload profile
source ~/.zshrc  # or ~/.bash_profile
```

#### Get Your Team ID
1. Go to [Apple Developer Portal](https://developer.apple.com/account/)
2. Navigate to "Certificates, Identifiers & Profiles"
3. Look for your Team ID in the top right corner

#### Create App-Specific Password
1. Go to [Apple ID](https://appleid.apple.com/)
2. Sign in with your Apple ID
3. Go to "Security" → "App-Specific Passwords"
4. Generate a new password for "Nova Development"

## Building the Application

### 1. Quick Build
```bash
# Build everything in one command
./build_macos.sh
```

This script will:
- Check prerequisites
- Build Rust binary for both architectures
- Create universal binary
- Build the macOS app bundle

### 2. Manual Build Steps

If you prefer to build step by step:

```bash
# Build Rust binaries
cargo build --release --target x86_64-apple-darwin
cargo build --release --target aarch64-apple-darwin

# Create universal binary
mkdir -p target/universal/release
lipo -create \
    target/x86_64-apple-darwin/release/bug-spray \
    target/aarch64-apple-darwin/release/bug-spray \
    -output target/universal/release/bug-spray

# Prepare app bundle
mkdir -p NovaApp/Contents/Helpers
cp target/universal/release/bug-spray NovaApp/Contents/Helpers/

# Build macOS app
cd NovaApp
xcodebuild -project NovaApp.xcodeproj -scheme NovaApp -configuration Release build
```

## Testing

### 1. Run Tests
```bash
# Test the built application
./scripts/test_nova_app.sh
```

### 2. Manual Testing
```bash
# Open the app
open NovaApp/build/Release/NovaApp.app

# Test functionality:
# - Terminal should display and be interactive
# - Esc key should quit the app
# - Window controls should work (minimize, zoom, close)
# - App should quit cleanly when terminal exits
```

## Code Signing and Notarization

### 1. Automatic Process
```bash
# Set credentials first
export TEAM_ID="YOUR_TEAM_ID"
export APPLE_ID="your.apple.id@example.com"
export APP_SPECIFIC_PASSWORD="your-app-specific-password"

# Run the signing and notarization script
./scripts/codesign_and_notarize.sh
```

### 2. Manual Process

#### Code Sign Helper Binary
```bash
codesign --force --options runtime \
    --sign "Developer ID Application: YOUR_TEAM_ID" \
    --entitlements NovaApp/NovaApp.entitlements \
    --timestamp \
    NovaApp/build/Release/NovaApp.app/Contents/Helpers/bug-spray
```

#### Code Sign App Bundle
```bash
codesign --force --options runtime \
    --sign "Developer ID Application: YOUR_TEAM_ID" \
    --entitlements NovaApp/NovaApp.entitlements \
    --timestamp \
    --deep \
    NovaApp/build/Release/NovaApp.app
```

#### Notarize
```bash
# Submit for notarization
xcrun notarytool submit Nova.dmg \
    --apple-id "your.apple.id@example.com" \
    --password "your-app-specific-password" \
    --team-id "YOUR_TEAM_ID" \
    --wait

# Staple the ticket
xcrun stapler staple Nova.dmg
```

## Distribution

### 1. Create DMG
```bash
# Using create-dmg (recommended)
create-dmg \
    --volname "Nova" \
    --window-pos 200 120 \
    --window-size 600 300 \
    --icon-size 100 \
    --icon "Nova.app" 175 120 \
    --hide-extension "Nova.app" \
    --app-drop-link 425 120 \
    Nova-1.0.0-universal.dmg \
    NovaApp/build/Release/NovaApp.app
```

### 2. Generate Checksum
```bash
shasum -a 256 Nova-1.0.0-universal.dmg > Nova-1.0.0-universal.dmg.sha256
```

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean and rebuild
cd NovaApp
xcodebuild clean
cd ..
./build_macos.sh
```

#### Code Signing Issues
```bash
# Verify certificates
security find-identity -v -p codesigning

# Check entitlements
codesign --display --entitlements - NovaApp/build/Release/NovaApp.app
```

#### Notarization Failures
```bash
# Check notarization status
xcrun notarytool info <submission-id> \
    --apple-id "your.apple.id@example.com" \
    --password "your-app-specific-password" \
    --team-id "YOUR_TEAM_ID"
```

### Debug Mode
```bash
# Enable debug logging
export DEBUG=1
./build_macos.sh
```

## Development Workflow

### 1. Making Changes
1. Edit Swift files in `NovaApp/`
2. Edit Rust code in `src/`
3. Run `./build_macos.sh` to rebuild
4. Test changes

### 2. Testing Changes
```bash
# Quick test
./scripts/test_nova_app.sh

# Manual test
open NovaApp/build/Release/NovaApp.app
```

### 3. Iterative Development
```bash
# Watch for changes and rebuild
fswatch -o . | xargs -n1 -I{} ./build_macos.sh
```

## Next Steps

After successful setup:

1. **Customize the App**: Modify `Info.plist`, app icon, and branding
2. **Configure Updates**: Set up Sparkle update feed
3. **Test Distribution**: Verify DMG works on other Macs
4. **Automate**: Set up CI/CD for automated builds
5. **Publish**: Host DMG and update feed on your website

## Support

- **GitHub Issues**: [Create an issue](https://github.com/nova/nova-cli/issues)
- **Documentation**: [nova.app/docs](https://nova.app/docs)
- **Community**: [Discord](https://discord.gg/nova)

## Resources

- [Apple Developer Documentation](https://developer.apple.com/documentation/)
- [SwiftUI Tutorials](https://developer.apple.com/tutorials/swiftui)
- [Code Signing Guide](https://developer.apple.com/support/code-signing/)
- [Notarization Guide](https://developer.apple.com/documentation/security/notarizing_macos_software_before_distribution)

