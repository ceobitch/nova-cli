# 🪪 Nova CLI & Desktop — AI Cybersecurity Companion (Mac, Windows, Linux)

**Protect your machine from malware, crypto theft, and developer-targeted attacks with intelligent AI assistance.**

Nova is a specialized cybersecurity companion built on a modern Codex fork, with a beautiful desktop app for macOS and cross-platform packaging via Tauri for Windows/Linux. The core CLI remains fully usable as `nova`.

## 🚀 Key Features

- 🛡️ **Advanced Mac Threat Detection** - AtomicStealer, RustBucket, KandyKorn, XCSSET protection
- 🧠 **AI-Powered Analysis** - GPT-4 powered security consultation and threat explanation  
- 💎 **Crypto Wallet Protection** - Clipboard hijack prevention and wallet security
- 🔧 **Developer Security** - Xcode, NPM, and supply chain attack protection
- 🎨 **Beautiful Interface** - Custom terminal UI with animations and colors
- 🔐 **Privacy-Focused** - Local processing with minimal data sharing

## 🏃‍♂️ Quick Start (CLI)

1. **Set up OpenAI API key:**
```bash
export OPENAI_API_KEY="your-api-key-here"
```

2. **Build and run:**
   ```bash
   cargo build --release
   ./target/release/nova
   ```

3. **Start protecting your Mac!**

## 🎮 Usage Examples (CLI)

```bash
# Interactive mode with beautiful TUI
nova

# Ask AI about security
nova "Is my Mac safe from crypto malware?"

# Quick security scan
nova --scan

# Offline mode (no AI)
nova --offline

## 🖥️ Desktop App

- macOS: native SwiftUI shell with glass dark window and PTY integration
- Windows/Linux: Tauri app shell bundling the `nova` sidecar

### Build macOS app

```bash
./scripts/build_nova_macos.sh
open NovaApp/build/Release/NovaApp.app
```

### Build Windows/Linux app (Tauri)

```bash
cargo install tauri-cli
cd app/src-tauri
cargo tauri build
```
```

## 🛡️ What Bug Spray Protects Against

- **AtomicStealer**: Mac malware targeting crypto wallets and browser data
- **RustBucket**: Lazarus Group attacks on Mac developers
- **KandyKorn**: Fake Discord/trading app updates
- **XCSSET**: Xcode project supply chain attacks
- **Clipboard Hijackers**: Crypto address replacement
- **Fake Apps**: Trojanized cryptocurrency and developer tools

Bug Spray combines advanced threat detection with AI-powered analysis to keep your Mac secure!