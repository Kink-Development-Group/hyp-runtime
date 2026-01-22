#!/usr/bin/env pwsh
# build_linux.ps1
# Creates Linux binary and TAR.GZ archive for HypnoScript (Rust implementation)
# Can be run on Windows with WSL or directly on Linux

param(
    [switch]$SkipBuild = $false
)

$ErrorActionPreference = "Stop"

# Configuration
$NAME = "hypnoscript"
$VERSION = "1.2.0"
$ARCH = "amd64"

# Determine project directory
$ScriptDir = Split-Path -Parent $PSScriptRoot
$ProjectRoot = $ScriptDir
$ReleaseDir = Join-Path $ProjectRoot "release" "linux-x64"
$TarOut = Join-Path $ProjectRoot "release" "$NAME-$VERSION-linux-x64.tar.gz"
$BinaryName = "hypnoscript-cli"
$InstallName = "hypnoscript"

Write-Host "=== HypnoScript Linux Release Builder ===" -ForegroundColor Cyan
Write-Host ""

# Check for Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo is not installed. Please install Rust toolchain first." -ForegroundColor Red
    Write-Host "Visit https://rustup.rs/ to install Rust" -ForegroundColor Yellow
    exit 1
}

# 1. Prepare directories
Write-Host "📦 Preparing release directory..." -ForegroundColor Green
if (Test-Path $ReleaseDir) {
    Remove-Item -Recurse -Force $ReleaseDir
}
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# 2. Build for Linux (if WSL available, otherwise for current system)
if (-not $SkipBuild) {
    Write-Host "🔨 Building HypnoScript CLI (Release for Linux)..." -ForegroundColor Green
    Push-Location $ProjectRoot

    # Try cross-compilation for Linux
    $LinuxTarget = "x86_64-unknown-linux-gnu"

    # Check if Linux target is installed
    $InstalledTargets = rustup target list --installed 2>$null
    if ($InstalledTargets -match $LinuxTarget) {
        Write-Host "  Using cross-compilation target: $LinuxTarget" -ForegroundColor Cyan
        cargo build --release --package hypnoscript-cli --target $LinuxTarget
        $BinaryPath = Join-Path "target" $LinuxTarget "release" $BinaryName
    } else {
        Write-Host "  ⚠ Linux target not installed, building for current platform" -ForegroundColor Yellow
        Write-Host "    (To enable Linux builds: rustup target add $LinuxTarget)" -ForegroundColor Yellow
        cargo build --release --package hypnoscript-cli
        $BinaryPath = Join-Path "target" "release" "$BinaryName.exe"
    }

    Pop-Location
} else {
    Write-Host "⏩ Skipping build (using existing binary)..." -ForegroundColor Yellow
    $BinaryPath = Join-Path $ProjectRoot "target" "release" $BinaryName
}

# 3. Copy binary
Write-Host "📋 Copying binary..." -ForegroundColor Green
$DestBinary = Join-Path $ReleaseDir $InstallName
Copy-Item $BinaryPath $DestBinary -Force

# 4. Additional files
Write-Host "📄 Adding additional files..." -ForegroundColor Green

$ReadmePath = Join-Path $ProjectRoot "README.md"
if (Test-Path $ReadmePath) {
    Copy-Item $ReadmePath $ReleaseDir
}

$LicensePath = Join-Path $ProjectRoot "LICENSE"
if (Test-Path $LicensePath) {
    Copy-Item $LicensePath $ReleaseDir
}

Set-Content -Path (Join-Path $ReleaseDir "VERSION.txt") -Value $VERSION

# Add installation script
$InstallerSource = Join-Path $ProjectRoot "install.sh"
if (Test-Path $InstallerSource) {
    Copy-Item $InstallerSource (Join-Path $ReleaseDir "install.sh") -Force
} else {
    Write-Host "⚠ Warning: install.sh not found at project root" -ForegroundColor Yellow
}

# 5. Create TAR.GZ archive
Write-Host "📦 Creating TAR.GZ archive..." -ForegroundColor Green

# On Windows: use tar.exe (available since Windows 10 1803)
if ($IsWindows -or ($PSVersionTable.PSVersion.Major -le 5)) {
    Push-Location (Join-Path $ProjectRoot "release")
    & tar -czf (Split-Path -Leaf $TarOut) -C "linux-x64" .
    Pop-Location
} else {
    # On Linux: native tar
    Push-Location (Join-Path $ProjectRoot "release")
    tar -czf (Split-Path -Leaf $TarOut) -C "linux-x64" .
    Pop-Location
}

# 6. Generate checksum
Write-Host "🔐 Generating SHA256 checksum..." -ForegroundColor Green
$Hash = Get-FileHash -Path $TarOut -Algorithm SHA256
$HashString = "$($Hash.Hash.ToLower())  $(Split-Path -Leaf $TarOut)"
Set-Content -Path "$TarOut.sha256" -Value $HashString

# 7. Output information
Write-Host ""
Write-Host "✅ Build complete!" -ForegroundColor Green
Write-Host "📦 TAR.GZ Archive: $TarOut" -ForegroundColor Cyan
Write-Host "🔐 Checksum: $TarOut.sha256" -ForegroundColor Cyan
Write-Host ""

$TarSize = (Get-Item $TarOut).Length / 1MB
Write-Host "Archive size: $([math]::Round($TarSize, 2)) MB"
Write-Host ""

Write-Host "To install on Linux:" -ForegroundColor Yellow
Write-Host "  tar -xzf $(Split-Path -Leaf $TarOut)" -ForegroundColor White
Write-Host "  cd linux-x64" -ForegroundColor White
Write-Host "  sudo bash install.sh" -ForegroundColor White
Write-Host ""
Write-Host "Or manually:" -ForegroundColor Yellow
Write-Host "  sudo mv hypnoscript /usr/local/bin/" -ForegroundColor White
Write-Host ""
Write-Host "To verify:" -ForegroundColor Yellow
Write-Host "  hypnoscript --version" -ForegroundColor White
Write-Host ""
Write-Host "✓ All done!" -ForegroundColor Green
