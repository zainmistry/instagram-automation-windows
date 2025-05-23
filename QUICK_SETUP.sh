#!/bin/bash

echo "🚀 Instagram Automation Tool - Quick GitHub Setup"
echo "================================================"
echo ""

# Check if git is initialized
if [ ! -d ".git" ]; then
    echo "📁 Initializing Git repository..."
    git init
fi

# Add all files
echo "📦 Adding all files to Git..."
git add .

# Create initial commit
echo "💾 Creating initial commit..."
git commit -m "Instagram Automation Tool v1.0.0 - Full Release" || echo "Already committed"

# Ask for GitHub username
echo ""
echo "📝 Please enter your GitHub username:"
read github_username

# Check if remote already exists
if git remote | grep -q origin; then
    echo "🔄 Updating existing remote..."
    git remote set-url origin https://github.com/$github_username/instagram-automation.git
else
    echo "🔗 Adding GitHub remote..."
    git remote add origin https://github.com/$github_username/instagram-automation.git
fi

echo ""
echo "✅ Setup complete!"
echo ""
echo "📋 Now follow these steps:"
echo ""
echo "1. Go to https://github.com/new"
echo "2. Create a new repository named: instagram-automation"
echo "3. Make it PUBLIC (so GitHub Actions can run)"
echo "4. DON'T initialize with README or any files"
echo "5. Click 'Create repository'"
echo ""
echo "6. Then come back here and run:"
echo "   ./QUICK_SETUP.sh push"
echo ""

if [ "$1" == "push" ]; then
    echo "🚀 Pushing to GitHub..."
    git push -u origin main --force
    
    echo ""
    echo "🏷️ Creating release tag..."
    git tag -f v1.0.0
    git push origin v1.0.0 --force
    
    echo ""
    echo "✅ ALL DONE!"
    echo ""
    echo "🎉 Your Windows installer will be ready in 10-15 minutes!"
    echo ""
    echo "📍 To download it:"
    echo "1. Go to: https://github.com/$github_username/instagram-automation/actions"
    echo "2. Click on the latest workflow run"
    echo "3. Scroll down to 'Artifacts'"
    echo "4. Download 'windows-installers'"
    echo ""
fi 