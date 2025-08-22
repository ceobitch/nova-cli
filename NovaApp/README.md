# Nova macOS Application

A native macOS application wrapper for Nova (formerly Bug Spray), the AI-powered cybersecurity companion. This app provides a full terminal experience within a native macOS application, making it easy for users to download and run Nova without dealing with command-line installation.

## Features

- 🖥️ **Native macOS Experience**: Built with SwiftUI and AppKit for seamless integration
- 🌫️ **Glass Dark Shell**: Modern translucent HUD-style window with native mac traffic lights
- 🐛 **Integrated Terminal**: Full PTY terminal for an authentic shell experience
- 🔄 **Auto-Updates**: Integrated Sparkle 2 for seamless in-app updates
- 🛡️ **Hardened Runtime**: Enhanced security with Apple's Hardened Runtime
- 🔐 **Code Signed**: Properly signed and notarized for Gatekeeper compatibility
- 📱 **Universal Binary**: Supports both Intel (x86_64) and Apple Silicon (arm64) Macs

## Prerequisites

- macOS 13.0 or later
- Xcode 15.0 or later
- Rust toolchain (for building the binary)
- Apple Developer Account (for code signing and notarization)
- Homebrew (for additional tools)

## Quick Start

### 1. Install Dependencies

```bash
# Install required tools
brew install create-dmg

# Install Rust toolchains for universal build
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

### 2. Build the Application

```bash
# Build Rust binary and macOS app (embeds helper as Contents/Helpers/nova)
./scripts/build_nova_macos.sh
```

### 3. Code Sign and Notarize

```bash
# Set your credentials
export TEAM_ID="YOUR_TEAM_ID"
export APPLE_ID="your.apple.id@example.com"
export APP_SPECIFIC_PASSWORD="your-app-specific-password"

# Sign and notarize
./scripts/codesign_and_notarize.sh
```

## Project Structure

```
NovaApp/
├── NovaApp.xcodeproj/          # Xcode project file
├── NovaApp.swift               # Main app entry point
├── ContentView.swift           # Main content view with custom title bar
├── TerminalView.swift          # SwiftTerm integration and PTY handling
├── Info.plist                  # App configuration
├── NovaApp.entitlements       # Hardened Runtime entitlements
├── Package.swift               # Swift Package Manager configuration
├── exportOptions.plist         # Archive export options
└── Contents/
    └── Helpers/
        └── nova                # Embedded Rust binary (universal)
```

## Building from Source

### Manual Build Process

1. **Build Rust Binary**
   ```bash
   # Build for both architectures
   cargo build --release --target x86_64-apple-darwin
   cargo build --release --target aarch64-apple-darwin
   
   # Create universal binary
   lipo -create \
       target/x86_64-apple-darwin/release/bug-spray \
       target/aarch64-apple-darwin/release/bug-spray \
       -output target/universal/release/bug-spray
   ```

2. **Prepare App Bundle**
   ```bash
   mkdir -p NovaApp/Contents/Helpers
   cp target/universal/release/bug-spray NovaApp/Contents/Helpers/
   ```

3. **Build macOS App**
   ```bash
   cd NovaApp
   xcodebuild -project NovaApp.xcodeproj -scheme NovaApp -configuration Release build
   ```

### Using Swift Package Manager

```bash
cd NovaApp
swift package resolve
swift build
```

## Code Signing

### Requirements

- Apple Developer Account
- Developer ID Application certificate
- App-specific password for notarization

### Process

1. **Sign Helper Binary**
   ```bash
   codesign --force --options runtime \
       --sign "Developer ID Application: YOUR_TEAM_ID" \
       --entitlements NovaApp.entitlements \
       --timestamp \
       NovaApp.app/Contents/Helpers/nova
   ```

2. **Sign App Bundle**
   ```bash
   codesign --force --options runtime \
       --sign "Developer ID Application: YOUR_TEAM_ID" \
       --entitlements NovaApp.entitlements \
       --timestamp \
       --deep \
       NovaApp.app
   ```

3. **Verify Signing**
   ```bash
   codesign --verify --verbose=4 NovaApp.app
   ```

## Notarization

### Submit for Notarization

```bash
xcrun notarytool submit Nova.dmg \
    --apple-id "your.apple.id@example.com" \
    --password "your-app-specific-password" \
    --team-id "YOUR_TEAM_ID" \
    --wait
```

### Staple Notarization Ticket

```bash
xcrun stapler staple Nova.dmg
xcrun stapler validate Nova.dmg
```

## Distribution

### Creating DMG

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
    NovaApp.app

# Using hdiutil (fallback)
hdiutil create -volname "Nova" -srcfolder NovaApp.app -ov -format UDZO Nova.dmg
```

### Generating Checksums

```bash
shasum -a 256 Nova-1.0.0-universal.dmg > Nova-1.0.0-universal.dmg.sha256
```

## Configuration

### Sparkle Updates

Update the following in `Info.plist`:

```xml
<key>SUFeedURL</key>
<string>https://your-domain.com/updates/appcast.xml</string>
<key>SUPublicEDKey</key>
<string>YOUR_SPARKLE_PUBLIC_KEY</string>
```

### App Bundle ID

Update `PRODUCT_BUNDLE_IDENTIFIER` in the Xcode project settings and `exportOptions.plist`.

## Troubleshooting

### Common Issues

1. **"App can't be opened because it is from an unidentified developer"**
   - Ensure the app is properly code signed
   - Verify notarization was successful
   - Check that the notarization ticket is stapled

2. **Terminal not displaying properly**
   - Verify SwiftTerm framework is linked correctly
   - Check PTY creation in TerminalView.swift
   - Ensure the helper binary is in the correct location

3. **Build failures**
   - Verify Xcode version compatibility
   - Check Swift Package Manager dependencies
   - Ensure Rust toolchains are installed

### Debug Mode

Enable debug logging by setting the `DEBUG` environment variable:

```bash
export DEBUG=1
./scripts/build_nova_macos.sh
```

## Security Considerations

- **Hardened Runtime**: Enabled by default for enhanced security
- **Entitlements**: Configured for necessary system access while maintaining security
- **Code Signing**: Ensures authenticity and integrity
- **Notarization**: Prevents Gatekeeper blocking

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly on both Intel and Apple Silicon Macs
5. Submit a pull request

## License

Apache 2.0 - See LICENSE file for details.

## Support

For issues and questions:
- GitHub Issues: [Create an issue](https://github.com/nova/nova-cli/issues)
- Documentation: [nova.app/docs](https://nova.app/docs)
- Community: [Discord](https://discord.gg/nova)

