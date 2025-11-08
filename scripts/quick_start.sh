#!/bin/bash
# Quick start script for ZipGraph

set -e

echo "🚀 ZipGraph - Quick Start"
echo "========================="
echo ""

# Check Rust installation
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install from https://rustup.rs/"
    exit 1
fi

echo "✓ Rust installed: $(rustc --version)"
echo ""

# Build the project
echo "📦 Building ZipGraph..."
cargo build --release

if [ $? -eq 0 ]; then
    echo "✓ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "🧪 Running tests..."
cargo test --all

if [ $? -eq 0 ]; then
    echo "✓ All tests passed!"
else
    echo "❌ Some tests failed"
    exit 1
fi

echo ""
echo "📊 Running benchmarks (this may take a few minutes)..."
echo "You can skip this by pressing Ctrl+C"
sleep 2

cargo bench --bench graph_operations

echo ""
echo "✅ Setup complete!"
echo ""
echo "Next steps:"
echo "  1. Run examples:"
echo "     cargo run --example basic_usage"
echo "     cargo run --example recommendation_engine"
echo "     cargo run --example fraud_detection"
echo "     cargo run --example social_network"
echo ""
echo "  2. View documentation:"
echo "     cargo doc --open"
echo ""
echo "  3. Run all benchmarks:"
echo "     cargo bench"
echo ""
echo "Happy graphing! ⚡🤐"
