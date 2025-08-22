# CyberSec AI Terminal

🛡️ **AI-Powered Cybersecurity Terminal Companion**

A specialized cybersecurity-focused terminal application built on Codex, designed to detect threats, analyze malware, monitor clipboard hijacking, and provide real-time security protection with AI assistance.

## 🚀 Features

### 🔓 Free Features
- ✅ Basic threat detection
- ✅ Manual security scanning
- ✅ System health monitoring
- ✅ Clipboard change detection
- ✅ Simple security reports

### 💎 CyberSec Pro Features ($9.99/month)
- 🛡️ **Real-time threat protection**
- 🔍 **Advanced malware detection**
- 🤖 **AI-powered threat analysis**
- 🛠️ **Automated threat remediation**
- 📊 **Detailed security reports**
- 📤 **Export capabilities** (PDF, JSON, CSV)
- 📧 **Email alerts and notifications**
- 🚨 **Priority support**

## 🏃‍♂️ Quick Start

### Prerequisites
- Rust 1.70+ 
- Git

### Installation

1. **Clone the repository:**
   ```bash
   git clone https://github.com/your-org/cybersec-terminal.git
   cd cybersec-terminal
   ```

2. **Set up environment variables:**
   ```bash
   cp .example.env .env
   # Edit .env with your configuration
   ```

3. **Build and run:**
   ```bash
   cd codex-rs/cybersec-terminal
   cargo run --release
   ```

### Environment Configuration

Create a `.env` file in the project root:

```env
# Development mode (enables all features for testing)
DEV_MODE=true

# License configuration  
LICENSE_PUBLIC_KEY=your_public_key_here
# LICENSE_TOKEN={"device_id":"...","exp":1735689600,"product":"CyberSec","sig":"..."}

# Stripe subscription (for $9.99/mo)
STRIPE_SECRET_KEY=sk_test_your_stripe_secret_key
STRIPE_PRICE_ID=price_your_stripe_price_id
```

## 🎮 Usage

### Command Line Options

```bash
# Start with email for subscription verification
./cybersec-terminal --email user@example.com

# Enable debug logging
./cybersec-terminal --debug

# Run offline (skip subscription check)
./cybersec-terminal --offline
```

### Terminal Interface

The application provides a tabbed interface:

1. **🚨 Threats** - View and manage active security threats
2. **🔍 Scanner** - Run security scans and view results
3. **📊 Reports** - Access security reports and analytics (Pro)
4. **⚙️ Settings** - Configure application and view subscription status

### Keyboard Shortcuts

- `←/→` or `Tab/Shift+Tab` - Navigate between tabs
- `S` - Start security scan
- `U` - Show upgrade information
- `H` - Show help
- `Q` - Quit application

## 🛡️ Security Features

### Threat Detection
- **Malware Scanning**: Pattern-based detection of known malware signatures
- **Clipboard Monitoring**: Detect suspicious clipboard manipulation
- **Process Analysis**: Monitor running processes for anomalies
- **Network Monitoring**: Analyze network connections (Pro)

### Subscription-Gated Features

The application implements a freemium model:

- **Free users** can detect threats but cannot fix them
- **Pro subscribers** get full remediation capabilities
- **Dev mode** unlocks all features for development/testing

## 🏗️ Architecture

### Core Components

```
cybersec-terminal/
├── src/
│   └── main.rs              # Main application entry point
├── app-config/
│   ├── tauri.conf.json      # Desktop app configuration
│   └── build.rs             # Build script for packaging
└── Cargo.toml

codex-rs/core/src/
├── cybersec_config.rs       # Configuration management
├── subscription.rs          # Stripe subscription handling
└── cybersec/
    ├── clipboard_monitor.rs # Clipboard hijack detection
    ├── malware_scanner.rs   # Malware detection engine
    ├── threat_detector.rs   # Threat management
    └── security_report.rs   # Security reporting
```

### Technology Stack

- **Backend**: Rust with Tokio async runtime
- **TUI**: Ratatui for terminal interface
- **Subscription**: Stripe API integration
- **Packaging**: Cross-platform desktop app support
- **AI**: Built on Codex architecture

## 🔧 Development

### Setting Up Development Environment

1. **Enable development mode:**
   ```env
   DEV_MODE=true
   ```

2. **Run with debug logging:**
   ```bash
   RUST_LOG=debug cargo run -- --debug
   ```

3. **Test subscription features:**
   ```bash
   cargo run -- --email test@example.com --offline
   ```

### Building for Production

```bash
# Build optimized release
cargo build --release

# Build for specific target
cargo build --release --target x86_64-unknown-linux-musl

# Package as desktop app (requires additional setup)
cargo tauri build
```

### Adding New Threat Signatures

```rust
// Add to malware_scanner.rs
MalwareSignature {
    id: "new_threat".to_string(),
    name: "New Threat Type".to_string(),
    pattern: r"suspicious_pattern_regex".to_string(),
    severity: ThreatLevel::High,
    description: "Description of the threat".to_string(),
}
```

## 💰 Subscription Management

### Stripe Integration

The application integrates with Stripe for subscription management:

1. **Customer Creation**: Automatic customer creation on first subscription
2. **Subscription Validation**: Real-time subscription status checking
3. **Feature Gating**: Subscription-based feature access control
4. **Checkout**: Programmatic checkout session creation

### License Token Format

For offline validation, the application supports JWT-like license tokens:

```json
{
  "device_id": "unique_device_identifier",
  "exp": 1735689600,
  "product": "CyberSec",
  "sig": "signature_for_validation"
}
```

## 📦 Deployment

### Desktop Application

Build as a standalone desktop application:

```bash
# Install Tauri CLI
cargo install tauri-cli

# Build desktop app
cargo tauri build
```

### Distribution

The application can be distributed as:
- **Standalone executable** - Single binary with all dependencies
- **Desktop app bundle** - Platform-specific app packages
- **Web app** - Browser-based version (future)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Submit a pull request

### Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Ensure all tests pass (`cargo test`)
- Add documentation for public APIs
- Use conventional commit messages

## 📄 License

This project is licensed under the Apache 2.0 License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [docs.cybersec-ai.com](https://docs.cybersec-ai.com)
- **Email Support**: support@cybersec-ai.com
- **Issues**: GitHub Issues for bug reports
- **Discussions**: GitHub Discussions for feature requests

## 🔄 Updates

The application includes automatic update checking and can be configured for:
- **Automatic updates** (Pro feature)
- **Manual update notifications** (Free)
- **Security signature updates** (Real-time)

---

**⚡ Ready to secure your system? Download CyberSec AI Terminal today!**

