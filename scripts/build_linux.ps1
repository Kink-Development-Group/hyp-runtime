#!/usr/bin/env pwsh
# build_linux.ps1
# Erstellt Linux-Binary und TAR.GZ-Archiv für HypnoScript (Rust-Implementation)
# Kann unter Windows mit WSL oder direkt unter Linux ausgeführt werden

param(
    [switch]$SkipBuild = $false
)

$ErrorActionPreference = "Stop"

# Konfiguration
$NAME = "hypnoscript"
$VERSION = "1.0.0"
$ARCH = "amd64"

# Projektverzeichnis ermitteln
$ScriptDir = Split-Path -Parent $PSScriptRoot
$ProjectRoot = $ScriptDir
$ReleaseDir = Join-Path $ProjectRoot "release" "linux-x64"
$TarOut = Join-Path $ProjectRoot "release" "$NAME-$VERSION-linux-x64.tar.gz"
$BinaryName = "hypnoscript-cli"
$InstallName = "hypnoscript"

Write-Host "=== HypnoScript Linux Release Builder ===" -ForegroundColor Cyan
Write-Host ""

# Check für Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo is not installed. Please install Rust toolchain first." -ForegroundColor Red
    Write-Host "Visit https://rustup.rs/ to install Rust" -ForegroundColor Yellow
    exit 1
}

# 1. Verzeichnisse vorbereiten
Write-Host "📦 Preparing release directory..." -ForegroundColor Green
if (Test-Path $ReleaseDir) {
    Remove-Item -Recurse -Force $ReleaseDir
}
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# 2. Build für Linux (falls WSL verfügbar, sonst für aktuelles System)
if (-not $SkipBuild) {
    Write-Host "🔨 Building HypnoScript CLI (Release for Linux)..." -ForegroundColor Green
    Push-Location $ProjectRoot

    # Versuche Cross-Compilation für Linux
    $LinuxTarget = "x86_64-unknown-linux-gnu"

    # Check ob Linux-Target installiert ist
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

# 3. Binary kopieren
Write-Host "📋 Copying binary..." -ForegroundColor Green
$DestBinary = Join-Path $ReleaseDir $InstallName
Copy-Item $BinaryPath $DestBinary -Force

# 4. Zusätzliche Dateien
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

# Installation-Script erstellen
$InstallScript = @"
#!/bin/bash
# HypnoScript Installation Script

set -e

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="hypnoscript"

echo "Installing HypnoScript to `$INSTALL_DIR..."

# Check for sudo
if [ "`$EUID" -ne 0 ]; then
    echo "Please run with sudo:"
    echo "  sudo bash install.sh"
    exit 1
fi

# Copy binary
cp `$BINARY_NAME `$INSTALL_DIR/`$BINARY_NAME
chmod +x `$INSTALL_DIR/`$BINARY_NAME

echo "✓ HypnoScript installed successfully!"
echo ""
echo "Run 'hypnoscript --version' to verify the installation."
"@

Set-Content -Path (Join-Path $ReleaseDir "install.sh") -Value $InstallScript

# 5. TAR.GZ-Archiv erstellen
Write-Host "📦 Creating TAR.GZ archive..." -ForegroundColor Green

# Unter Windows: tar.exe verwenden (verfügbar ab Windows 10 1803)
if ($IsWindows -or ($PSVersionTable.PSVersion.Major -le 5)) {
    Push-Location (Join-Path $ProjectRoot "release")
    & tar -czf (Split-Path -Leaf $TarOut) -C "linux-x64" .
    Pop-Location
} else {
    # Unter Linux: natives tar
    Push-Location (Join-Path $ProjectRoot "release")
    tar -czf (Split-Path -Leaf $TarOut) -C "linux-x64" .
    Pop-Location
}

# 6. Checksum erstellen
Write-Host "🔐 Generating SHA256 checksum..." -ForegroundColor Green
$Hash = Get-FileHash -Path $TarOut -Algorithm SHA256
$HashString = "$($Hash.Hash.ToLower())  $(Split-Path -Leaf $TarOut)"
Set-Content -Path "$TarOut.sha256" -Value $HashString

# 7. Informationen ausgeben
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
