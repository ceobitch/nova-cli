# 🛡️ Nova Shield - AI Cybersecurity Assistant

**Nova Shield** is your AI-powered cybersecurity expert that scans, detects, and eliminates malware while preventing attacks. Built as an enhanced coding agent that runs locally on your computer.

## Features

- 🚀 Enhanced performance and stability
- 🔧 Extended customization options
- 🛡️ Improved security and sandboxing
- 🎯 Better error handling and debugging
- 📦 Simplified installation and setup

## Persona

Nova is an AI cybersecurity coding assistant. If asked who she is or what she does, she will answer: a cybersecurity expert who can scan a computer, detect and help eliminate malware, and help prevent attacks — while also providing coding assistance.

## 🚀 Quick Installation

```bash
# One-line install (recommended)
curl -fsSL https://raw.githubusercontent.com/ceobitch/nova-cli/main/install.sh | bash

# Or via NPM (after cloning)
git clone https://github.com/ceobitch/nova-cli.git
cd nova-cli/codex-cli
npm install -g .
```

**[📖 Complete Installation Guide](INSTALLATION_GUIDE.md)** with all methods and troubleshooting.

## Installation Options

### macOS DMG Installer (Easiest for Mac users)

1. Download `Nova-CLI-0.1.0.dmg` from releases
2. Double-click to mount the DMG
3. Double-click "Install Nova CLI.command"
4. Follow the prompts

### One-Line Install (Cross-platform)

```bash
curl -fsSL https://raw.githubusercontent.com/ceobitch/nova-cli/main/install.sh | bash
```

### Alternative: Manual Install

**Option 1: Simple npm install**
```bash
git clone https://github.com/ceobitch/nova-cli.git
cd nova-cli/codex-cli
npm install -g .
```

**Option 2: Using the install script**
```bash
git clone https://github.com/ceobitch/nova-cli.git
cd nova-cli
chmod +x install.sh
./install.sh
```

**Option 3: From npm package**
```bash
git clone https://github.com/ceobitch/nova-cli.git
cd nova-cli/codex-cli
npm pack
npm install -g nova-cli-0.1.0.tgz
```

### Requirements
- **Node.js 20+** (required)
- **Git** (for cloning the repository)
- **Rust** (optional, only needed if building from source)

### From Source (Development)

```bash
# Clone the repository
git clone https://github.com/ceobitch/nova-cli.git
cd nova-cli

# Build the Rust components
cd codex-rs
cargo build --release

# Install the CLI globally
cd ../codex-cli
npm install -g .
```

## Quick Start

Once installed, simply run:

```bash
nova
```

Or with a direct prompt:

```bash
nova "explain this codebase to me"
```

### Uninstall

**Easy way (recommended):**
```bash
nova --uninstall
```

**Manual removal:**
```bash
# Stop running processes (ignore errors if none are running)
pkill -f 'nova|codex-tui' 2>/dev/null || true

# Uninstall global npm package (if present)
npm uninstall -g nova-cli 2>/dev/null || sudo npm uninstall -g nova-cli 2>/dev/null || true

# Remove install directories and common symlinks
rm -rf "$HOME/.nova" "$HOME/.codex"
rm -f "$HOME/.local/bin/nova" /usr/local/bin/nova /opt/homebrew/bin/nova

# Remove PATH lines added by installer
# macOS (zsh/bash):
sed -i '' '/# Nova CLI/,+1d' "$HOME/.zshrc" 2>/dev/null || true
sed -i '' '/# Nova CLI/,+1d' "$HOME/.bash_profile" 2>/dev/null || true
sed -i '' '/# Nova CLI/,+1d' "$HOME/.bashrc" 2>/dev/null || true

# Linux (bash): uncomment if needed
# sed -i '/# Nova CLI/,+1d' "$HOME/.bashrc" 2>/dev/null || true
# sed -i '/# Nova CLI/,+1d' "$HOME/.bash_profile" 2>/dev/null || true

# Reload your shell (macOS zsh)
exec zsh -l
```

## Configuration

Nova CLI supports rich configuration options. By default, it loads configuration from `~/.nova/config.toml`.

## System Requirements

- **Operating Systems:** macOS 12+, Ubuntu 20.04+/Debian 10+, or Windows 11 via WSL2
- **Git:** 2.23+ (optional, recommended)
- **RAM:** 4GB minimum (8GB recommended)
- **Node.js:** 20+ (for npm installation)
- **Rust:** Latest stable (for building from source)

## What's Different from Nova CLI

This fork includes:

- Enhanced error handling and recovery
- Improved configuration management
- Better performance optimizations
- Extended plugin system
- Enhanced security features
- Streamlined user experience

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

## License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project is based on OpenAI's Codex CLI. We thank the original developers for their excellent work.