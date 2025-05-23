# Instagram Automation Tool - Setup Instructions

## ⚠️ Current Status: Development Build
This is a development version. Not yet ready for production use.

## What Your Friend Needs to Install First:

1. **Node.js** (v18 or newer)
   - Download: https://nodejs.org/
   - Choose "LTS" version

2. **Rust Programming Language**
   - Download: https://www.rust-lang.org/tools/install
   - Run the installer and follow prompts

3. **Visual Studio Build Tools** (Windows only)
   - Download: https://visualstudio.microsoft.com/downloads/
   - Select "Build Tools for Visual Studio"
   - Install "Desktop development with C++" workload

## How to Run:

1. **Extract the ZIP file** I sent you

2. **Double-click `windows-setup.bat`**
   - This will install dependencies and start the app
   - First run will take 5-10 minutes (it's compiling)

3. **If that doesn't work**, open Command Prompt and run:
   ```
   cd path\to\instagram-tool
   cd dashboard
   npm install
   npm run build
   cd ..\browser-core
   cargo tauri dev
   ```

## What to Expect:

- A window will open with the Instagram Automation dashboard
- You can add Instagram accounts (username, password, email)
- Proxy support is built-in but needs configuration
- **WARNING**: This is for educational purposes only

## Known Issues:

1. **Windows Defender Warning**: The app isn't code-signed, so Windows will warn you
   - Click "More info" → "Run anyway"

2. **First Launch is Slow**: Rust needs to compile everything

3. **Not All Features Work Yet**: This is still in development

## Important Notes:

- This tool includes anti-detection features but use at your own risk
- Instagram may still detect and ban accounts used with automation
- Always use proxies for safety
- Start slow with any automation

## If You Have Problems:

Check that you can run these commands:
- `node --version` (should show v18.x.x or higher)
- `cargo --version` (should show cargo 1.x.x)

## Files Included:

- `/dashboard` - The user interface
- `/browser-core` - The main application
- `/config` - Example configuration files
- `windows-setup.bat` - Quick setup script
- This README file 