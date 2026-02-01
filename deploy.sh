#!/bin/bash

# Deployment script for fnjoe to GitHub Pages
# Usage: ./deploy.sh

set -e

echo "🚀 Deploying fnjoe to GitHub Pages"
echo "===================================="
echo ""

# Step 1: Clean previous build
if [ -d "docs" ]; then
    echo "🧹 Cleaning previous build..."
    rm -rf docs/
fi

# Step 2: Build
echo "📦 Building with dx bundle..."
dx bundle --out-dir docs --release

if [ ! -d "docs" ]; then
    echo "❌ Error: Build failed - docs directory not created"
    exit 1
fi

# Step 3: Move static files
echo "📂 Moving static files from docs/public/ to docs/..."
if [ -d "docs/public" ]; then
    # Move all files
    mv docs/public/* docs/ 2>/dev/null || true
    
    # Remove empty public directory
    rm -rf docs/public
    
    echo "✅ Files moved successfully"
else
    echo "⚠️  No docs/public directory found (might be okay)"
fi

# Step 4: Create 404.html
echo "📄 Creating 404.html for client-side routing..."
if [ -f "docs/index.html" ]; then
    cp docs/index.html docs/404.html
    echo "✅ 404.html created"
else
    echo "❌ Error: docs/index.html not found!"
    exit 1
fi

# Step 5: Verify build
echo ""
echo "🔍 Verifying build..."
echo "Files in docs/:"
ls -lh docs/ | head -10

if [ -f "docs/index.html" ] && [ -f "docs/404.html" ]; then
    echo "✅ Build verification passed"
else
    echo "❌ Build verification failed"
    exit 1
fi

# Step 6: Git operations
echo ""
echo "📤 Committing changes..."
git add docs/

if git diff --staged --quiet; then
    echo "⚠️  No changes to commit"
else
    git commit -m "Deploy to GitHub Pages: $(date '+%Y-%m-%d %H:%M:%S')"
    echo "✅ Changes committed"
fi

echo ""
echo "🚀 Pushing to GitHub..."
git push -f origin gh-pages

echo ""
echo "🎉 Deployment complete!"
echo ""
echo "Your app will be live at:"
echo "👉 https://j03-dev.github.io/fnjoe/"
echo ""
echo "⏱️  Note: It may take 1-2 minutes for GitHub Pages to update"
echo ""
echo "Next steps:"
echo "1. Go to: https://github.com/j03-dev/fnjoe/settings/pages"
echo "2. Make sure Source is set to: main branch, /docs folder"
echo "3. Wait for deployment to complete"
echo ""
