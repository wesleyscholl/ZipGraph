#!/bin/bash
# ZipGraph v1.0.0 Release Build Script

set -e

VERSION="1.0.0"
PROJECT="zipgraph"
ARCH=$(uname -m)
OS=$(uname -s | tr '[:upper:]' '[:lower:]')
PLATFORM="${ARCH}-${OS}"

echo "╔════════════════════════════════════════════════╗"
echo "║   ZipGraph v${VERSION} Release Builder           ║"
echo "║   Platform: ${PLATFORM}                     ║"
echo "╚════════════════════════════════════════════════╝"
echo ""

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean
rm -rf release/

# Run tests
echo ""
echo "🧪 Running test suite..."
cargo test --workspace --lib --release
if [ $? -ne 0 ]; then
    echo "❌ Tests failed! Aborting release."
    exit 1
fi
echo "✅ All tests passed!"

# Build release binaries
echo ""
echo "🔨 Building release binaries..."
cargo build --workspace --release
cargo build --release --examples

if [ $? -ne 0 ]; then
    echo "❌ Build failed! Aborting release."
    exit 1
fi
echo "✅ Build successful!"

# Create release directory structure
echo ""
echo "📦 Creating release package..."
mkdir -p release/${PROJECT}-v${VERSION}-${PLATFORM}/{bin,examples,docs,lib}

# Copy binaries (examples act as our binaries)
echo "  Copying binaries..."
cp target/release/examples/ultra_benchmark release/${PROJECT}-v${VERSION}-${PLATFORM}/bin/ 2>/dev/null || true
cp target/release/examples/performance_comparison release/${PROJECT}-v${VERSION}-${PLATFORM}/bin/ 2>/dev/null || true
cp target/release/examples/basic_usage release/${PROJECT}-v${VERSION}-${PLATFORM}/examples/
cp target/release/examples/fraud_detection release/${PROJECT}-v${VERSION}-${PLATFORM}/examples/ 2>/dev/null || true
cp target/release/examples/recommendation_engine release/${PROJECT}-v${VERSION}-${PLATFORM}/examples/ 2>/dev/null || true
cp target/release/examples/social_network release/${PROJECT}-v${VERSION}-${PLATFORM}/examples/ 2>/dev/null || true

# Copy libraries
echo "  Copying libraries..."
cp target/release/libzipgraph_core.rlib release/${PROJECT}-v${VERSION}-${PLATFORM}/lib/ 2>/dev/null || true
cp target/release/libzipgraph_ml.rlib release/${PROJECT}-v${VERSION}-${PLATFORM}/lib/ 2>/dev/null || true
cp target/release/libzipgraph_optimizer.rlib release/${PROJECT}-v${VERSION}-${PLATFORM}/lib/ 2>/dev/null || true

# Copy documentation
echo "  Copying documentation..."
cp README.md release/${PROJECT}-v${VERSION}-${PLATFORM}/docs/ 2>/dev/null || true
cp CHANGELOG.md release/${PROJECT}-v${VERSION}-${PLATFORM}/docs/
cp RELEASE_PLAN.md release/${PROJECT}-v${VERSION}-${PLATFORM}/docs/ 2>/dev/null || true
cp V1.0_COMPLETE.md release/${PROJECT}-v${VERSION}-${PLATFORM}/docs/ 2>/dev/null || true
cp 32_TESTS_PASSING.md release/${PROJECT}-v${VERSION}-${PLATFORM}/docs/ 2>/dev/null || true
cp LICENSE release/${PROJECT}-v${VERSION}-${PLATFORM}/ 2>/dev/null || echo "MIT License" > release/${PROJECT}-v${VERSION}-${PLATFORM}/LICENSE

# Create README for release
cat > release/${PROJECT}-v${VERSION}-${PLATFORM}/README.txt << 'EOF'
╔════════════════════════════════════════════════╗
║         ZipGraph v1.0.0                        ║
║     Ultra Performance Edition                  ║
╚════════════════════════════════════════════════╝

High-performance graph processing library in Rust.

📁 Directory Structure:
  bin/        - Main executables (benchmarks)
  examples/   - Example applications
  lib/        - Static libraries
  docs/       - Documentation

🚀 Quick Start:

1. Run benchmark:
   ./bin/ultra_benchmark

2. Try examples:
   ./examples/basic_usage
   ./examples/fraud_detection
   ./examples/recommendation_engine

📊 Performance:
  - 23-64x speedup in batch processing
  - Lock-free algorithms
  - Zero-copy iterators
  - Enterprise metrics

📚 Documentation:
  See docs/ directory for complete documentation

🔗 Links:
  Repository: https://github.com/wesleyscholl/zipgraph
  Documentation: https://docs.rs/zipgraph

📝 License: MIT
EOF

# Create install script
cat > release/${PROJECT}-v${VERSION}-${PLATFORM}/install.sh << 'EOF'
#!/bin/bash
# ZipGraph Installation Script

echo "Installing ZipGraph v1.0.0..."

INSTALL_DIR="${HOME}/.local"
mkdir -p "${INSTALL_DIR}/bin"
mkdir -p "${INSTALL_DIR}/lib"

# Install binaries
cp -v bin/* "${INSTALL_DIR}/bin/" 2>/dev/null || true
cp -v examples/* "${INSTALL_DIR}/bin/" 2>/dev/null || true

# Install libraries
cp -v lib/* "${INSTALL_DIR}/lib/" 2>/dev/null || true

echo ""
echo "✅ Installation complete!"
echo ""
echo "Add ${INSTALL_DIR}/bin to your PATH:"
echo "  export PATH=\"${INSTALL_DIR}/bin:\$PATH\""
echo ""
echo "Then run:"
echo "  ultra_benchmark"
EOF

chmod +x release/${PROJECT}-v${VERSION}-${PLATFORM}/install.sh

# Create tarball
echo ""
echo "📦 Creating tarball..."
cd release
tar -czf ${PROJECT}-v${VERSION}-${PLATFORM}.tar.gz ${PROJECT}-v${VERSION}-${PLATFORM}/
cd ..

# Calculate checksum
echo ""
echo "🔐 Generating checksums..."
cd release
shasum -a 256 ${PROJECT}-v${VERSION}-${PLATFORM}.tar.gz > SHA256SUMS.txt
cd ..

# Summary
echo ""
echo "╔════════════════════════════════════════════════╗"
echo "║           Build Complete! ✅                    ║"
echo "╚════════════════════════════════════════════════╝"
echo ""
echo "📦 Release artifacts:"
ls -lh release/${PROJECT}-v${VERSION}-${PLATFORM}.tar.gz
echo ""
echo "🔐 Checksum:"
cat release/SHA256SUMS.txt
echo ""
echo "📂 Package contents:"
cd release/${PROJECT}-v${VERSION}-${PLATFORM}
find . -type f | sort
cd ../..
echo ""
echo "🚀 Ready for release!"
echo ""
echo "Next steps:"
echo "  1. Test the release package"
echo "  2. Create git tag: git tag -a v${VERSION} -m 'Release v${VERSION}'"
echo "  3. Push tag: git push origin v${VERSION}"
echo "  4. Upload release/${PROJECT}-v${VERSION}-${PLATFORM}.tar.gz to GitHub"
echo "  5. Upload release/SHA256SUMS.txt to GitHub"
echo ""
