#!/bin/bash
set -euo pipefail

# Nova Shield - One-line installer
# Usage: curl -fsSL https://raw.githubusercontent.com/ceobitch/nova-cli/main/install.sh | bash

REPO_URL="https://github.com/ceobitch/nova-cli.git"
INSTALL_DIR="$HOME/.nova-install"
NODE_MIN_VERSION="20"

echo "🛡️ Installing Nova Shield - AI Cybersecurity Assistant"
echo "=================================================="

# Check if Node.js is installed and meets minimum version
check_node() {
    if ! command -v node &> /dev/null; then
        echo "❌ Node.js is not installed. Please install Node.js ${NODE_MIN_VERSION}+ first:"
        echo "   Visit: https://nodejs.org/"
        exit 1
    fi
    
    NODE_VERSION=$(node --version | sed 's/v//' | cut -d. -f1)
    if [ "$NODE_VERSION" -lt "$NODE_MIN_VERSION" ]; then
        echo "❌ Node.js version ${NODE_VERSION} is too old. Please upgrade to ${NODE_MIN_VERSION}+:"
        echo "   Visit: https://nodejs.org/"
        exit 1
    fi
    
    echo "✅ Node.js $(node --version) found"
}

# Check if git is available
check_git() {
    if ! command -v git &> /dev/null; then
        echo "❌ Git is not installed. Please install git first."
        exit 1
    fi
    echo "✅ Git $(git --version | cut -d' ' -f3) found"
}

# Clean up any existing installation
cleanup_existing() {
    echo "🧹 Cleaning up any existing installations..."
    
    # Stop running processes
    pkill -f 'nova|codex' 2>/dev/null || true
    
    # Remove existing npm packages
    npm uninstall -g nova-cli 2>/dev/null || true
    
    # Remove old install directory
    rm -rf "$INSTALL_DIR" 2>/dev/null || true
    
    echo "✅ Cleanup completed"
}

# Clone and install
install_nova() {
    echo "📦 Installing Nova Shield..."
    
    # Clone the repository
    echo "Cloning repository..."
    git clone "$REPO_URL" "$INSTALL_DIR"
    
    # Navigate to the CLI directory
    cd "$INSTALL_DIR/codex-cli"
    
    # Install dependencies
    echo "Installing dependencies..."
    npm install
    
    # Install globally
    echo "Installing Nova CLI globally..."
    npm install -g .
    
    echo "✅ Nova Shield installed successfully!"
}

# Verify installation
verify_installation() {
    echo "🔍 Verifying installation..."
    
    if command -v nova &> /dev/null; then
        echo "✅ Nova CLI is available in PATH"
        echo "📍 Location: $(which nova)"
    else
        echo "❌ Nova CLI not found in PATH"
        echo "You may need to restart your terminal or add npm global bin to your PATH"
        exit 1
    fi
}

# Show completion message
show_completion() {
    echo ""
    echo "🎉 Installation Complete!"
    echo "======================="
    echo ""
    echo "Nova Shield is now installed and ready to use!"
    echo ""
    echo "Quick start:"
    echo "  nova                    # Start interactive mode"
    echo "  nova \"scan my system\"   # Direct command"
    echo "  nova --help             # Show help"
    echo "  nova --uninstall        # Uninstall Nova"
    echo ""
    echo "🛡️ Nova is your AI cybersecurity expert ready to:"
    echo "   • Scan systems for malware and threats"
    echo "   • Detect and eliminate security vulnerabilities" 
    echo "   • Harden your system against attacks"
    echo "   • Provide coding assistance with security focus"
    echo ""
    echo "Get started: nova"
}

# Main installation flow
main() {
    check_node
    check_git
    cleanup_existing
    install_nova
    verify_installation
    show_completion
}

# Run the installer
main "$@"