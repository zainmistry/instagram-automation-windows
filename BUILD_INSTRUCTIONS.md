# Building Windows Installer for Instagram Automation Tool

## Method 1: GitHub Actions (Recommended) ✅

This method uses GitHub's servers to build a proper Windows installer without needing Windows on your machine.

### Steps:

1. **Create a GitHub Repository** (if you haven't already)
   ```bash
   git init
   git add .
   git commit -m "Initial commit"
   git remote add origin https://github.com/YOUR_USERNAME/instagram-automation.git
   git push -u origin main
   ```

2. **Trigger the Build**
   
   Option A - Via GitHub Website:
   - Go to your repository on GitHub
   - Click on "Actions" tab
   - Select "Build Release" workflow
   - Click "Run workflow"
   - Enter version (e.g., "1.0.0")
   - Click "Run workflow"

   Option B - Via Git Tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

3. **Download the Installer**
   - Go to "Actions" tab on GitHub
   - Click on the completed workflow run
   - Scroll down to "Artifacts"
   - Download "windows-installers"
   - Extract the ZIP file

   You'll get:
   - `Instagram Automation Tool_1.0.0_x64.msi` - MSI installer
   - `Instagram Automation Tool_1.0.0_x64-setup.exe` - NSIS installer

## Method 2: Local Build (Quick Test)

For testing the build locally on macOS:

```bash
# Install dependencies
cd dashboard
npm install
npm run build

# Test the app locally
cd ../browser-core
cargo tauri dev
```

## What Your Friend Gets

The Windows installer includes:
- ✅ Fully compiled Windows executable
- ✅ Embedded WebView (no browser needed)
- ✅ Dashboard interface
- ✅ All dependencies bundled
- ✅ Desktop shortcuts
- ✅ Start menu entry
- ✅ Uninstaller

## Installation Process for Your Friend

1. Download the `.msi` or `.exe` installer
2. Double-click to install
3. Follow the installation wizard
4. Launch from Start Menu or Desktop

## Features Included

- **Multi-Account Management**: Add up to 20 Instagram accounts
- **Proxy Support**: Load proxies from configuration
- **Anti-Detection**: Browser fingerprint spoofing, human-like behavior
- **Campaign Management**: Create and manage DM campaigns
- **Activity Logging**: Track all actions
- **2FA Support**: Manual code entry

## Security Notes

⚠️ **Windows Defender Warning**: Since the app isn't code-signed, Windows will show a security warning:
1. Click "More info"
2. Click "Run anyway"

To avoid this in the future, you'd need a code signing certificate ($200-500/year).

## Troubleshooting

If the build fails:
1. Check that all files are committed to Git
2. Ensure the dashboard builds locally first
3. Check the Actions tab for error logs

## Next Steps

After your friend installs:
1. Configure proxies in Settings
2. Add Instagram accounts
3. Start with slow, careful automation
4. Monitor account health regularly 