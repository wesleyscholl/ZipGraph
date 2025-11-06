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
