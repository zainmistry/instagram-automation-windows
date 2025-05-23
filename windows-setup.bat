@echo off
echo ========================================
echo Instagram Automation Tool - Windows Setup
echo ========================================
echo.

REM Check if Node.js is installed
where node >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Node.js is not installed!
    echo Please download and install from: https://nodejs.org/
    pause
    exit /b 1
)

REM Check if Rust is installed
where cargo >nul 2>nul
if %errorlevel% neq 0 (
    echo ERROR: Rust is not installed!
    echo Please download and install from: https://rustup.rs/
    pause
    exit /b 1
)

echo [1/4] Installing dashboard dependencies...
cd dashboard
call npm install
if %errorlevel% neq 0 (
    echo ERROR: Failed to install dashboard dependencies
    pause
    exit /b 1
)

echo.
echo [2/4] Building dashboard...
call npm run build
if %errorlevel% neq 0 (
    echo ERROR: Failed to build dashboard
    pause
    exit /b 1
)

echo.
echo [3/4] Moving to browser-core...
cd ../browser-core

echo.
echo [4/4] Starting application...
echo.
echo The application will now start. This may take a few minutes on first run.
echo.
cargo tauri dev

pause 