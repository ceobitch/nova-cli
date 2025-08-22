#!/bin/bash

# CyberSec AI Terminal - Quick Start Script
# This script builds and runs the cybersecurity terminal application

set -e

echo "🛡️  CyberSec AI Terminal - Build & Run Script"
echo "=============================================="

# Check if .example.env exists and copy to .env if needed
if [ ! -f ".env" ] && [ -f ".example.env" ]; then
    echo "📋 Copying .example.env to .env..."
    cp .example.env .env
    echo "✅ Environment file created. You may want to edit .env with your settings."
fi

# Check if we're in the right directory
if [ ! -d "codex-rs" ]; then
    echo "❌ Error: This script should be run from the project root directory"
    echo "Make sure you're in the directory containing the codex-rs folder"
    exit 1
fi

# Load environment variables
if [ -f ".env" ]; then
    echo "🔧 Loading environment variables..."
    export $(cat .env | grep -v '^#' | xargs)
fi

# Navigate to the cybersec-terminal directory
cd codex-rs/cybersec-terminal

echo "🔨 Building CyberSec AI Terminal..."
echo "This may take a few minutes on first build..."

# Build the application
cargo build --release

if [ $? -eq 0 ]; then
    echo "✅ Build successful!"
    echo ""
    echo "🚀 Starting CyberSec AI Terminal..."
    echo "Press Ctrl+C to exit when you're done"
    echo ""
    
    # Run the application
    if [ "$DEV_MODE" = "true" ]; then
        echo "🔧 Running in development mode (developer diagnostics enabled; user approvals still required)"
        cargo run --release --bin nova -- --debug
    else
        echo "🔒 Running in production mode"
        cargo run --release --bin nova
    fi
else
    echo "❌ Build failed. Please check the error messages above."
    exit 1
fi

