# Windows Build Guide for Instagram Automation Tool

## Prerequisites for Building on Windows

1. **Install Rust**: Download from https://rustup.rs/
2. **Install Node.js**: Download from https://nodejs.org/ (v18 or higher)
3. **Install Visual Studio Build Tools**: Required for Windows compilation
   - Download Visual Studio 2022 Community
   - Install "Desktop development with C++" workload

## Building for Windows

### Option 1: Build on Your Mac (Cross-compilation)

**Note**: Cross-compiling from macOS to Windows is complex. It's recommended to build on a Windows machine or use GitHub Actions.

### Option 2: Build on Windows (Recommended)

1. **Clone the repository on Windows**
2. **Install dependencies**:
   ```powershell
   cd dashboard
   npm install
   npm run build
   cd ../browser-core
   ```

3. **Build the release version**:
   ```powershell
   cargo tauri build
   ```

This will create:
- `.msi` installer in `browser-core/target/release/bundle/msi/`
- `.exe` installer in `browser-core/target/release/bundle/nsis/`

## Current Status

⚠️ **Not Production Ready** - The following issues need to be addressed:

1. ✅ Database initialization error (fixed)
2. ❌ No proper app icon (using placeholder)
3. ❌ Not tested on Windows
4. ❌ No code signing certificate
5. ❌ Anti-detection features need Windows testing

## Quick Start for Your Friend

### Temporary Solution (Development Build):

1. Send your friend the source code
2. Have them install Rust and Node.js on Windows
3. Run these commands:
   ```powershell
   # Install dependencies
   cd dashboard
   npm install
   npm run build
   
   # Run development version
   cd ../browser-core
   cargo tauri dev
   ```

### Production Solution (Coming Soon):

Once properly built and tested, you'll have:
- `InstagramAutomationTool-1.0.0.msi` - Windows installer
- Portable `.exe` version

## Security Warnings

Your friend might see Windows Defender warnings because:
- The app isn't code-signed
- It performs browser automation
- It's not from a recognized publisher

To fix this, you'd need to purchase a code signing certificate (~$200-500/year).

## Next Steps to Make Production-Ready:

1. **Test on Windows**: Ensure all features work
2. **Create proper icon**: Design a 256x256 PNG icon
3. **Add error handling**: Improve user experience
4. **Code signing**: Get a certificate to avoid security warnings
5. **Auto-updater**: Add Tauri's updater for easy updates 