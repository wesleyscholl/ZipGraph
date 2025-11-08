#!/bin/bash
# Fix GitHub Language Detection for ZipGraph

echo "🔧 Fixing GitHub language detection..."
echo ""

# Stage the changes
echo "📝 Staging changes..."
git add .gitignore
git add .gitattributes

# Remove old coverage files if they exist in git
echo "🧹 Removing old coverage files from git..."
git rm --cached coverage/lcov.info 2>/dev/null || true
git rm --cached coverage/tarpaulin-report.html 2>/dev/null || true

# Show what will be committed
echo ""
echo "📋 Changes to be committed:"
git status --short

echo ""
echo "✅ Ready to commit!"
echo ""
echo "Run:"
echo "  git commit -m 'Fix GitHub language detection - mark as Rust project'"
echo "  git push"
