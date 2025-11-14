#!/usr/bin/env pwsh
# build_macos.ps1
# Erstellt macOS-Binary und DMG/PKG für HypnoScript (Rust-Implementation)
# Kann unter Windows/Linux mit Cross-Compilation oder nativ auf macOS ausgeführt werden

param(
    [switch]$SkipBuild = $false,
    [ValidateSet('x64', 'arm64', 'universal')]
    [string]$Architecture = 'universal',
    [ValidateSet('dmg', 'pkg', 'tar.gz', 'all')]
    [string]$PackageType = 'all'
)

$ErrorActionPreference = "Stop"

# Konfiguration
$NAME = "HypnoScript"
$BUNDLE_ID = "com.kinkdev.hypnoscript"
$VERSION = "1.0.0-rc3"
$BINARY_NAME = "hypnoscript-cli"
$INSTALL_NAME = "hypnoscript"

# Projektverzeichnis ermitteln
$ScriptDir = Split-Path -Parent $PSScriptRoot
$ProjectRoot = $ScriptDir
$ReleaseDir = Join-Path $ProjectRoot "release" "macos-$Architecture"

Write-Host "=== HypnoScript macOS Release Builder ===" -ForegroundColor Cyan
Write-Host "Architecture: $Architecture" -ForegroundColor Yellow
Write-Host "Package Type: $PackageType" -ForegroundColor Yellow
Write-Host ""

# Check für Cargo
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: cargo is not installed. Please install Rust toolchain first." -ForegroundColor Red
    Write-Host "Visit https://rustup.rs/ to install Rust" -ForegroundColor Yellow
    exit 1
}

# Detect current OS
$RunningOnMacOS = $RunningOnMacOS -or ($PSVersionTable.PSVersion.Major -ge 6 -and $PSVersionTable.OS -like "*Darwin*")
$RunningOnLinux = $RunningOnLinux -or ($PSVersionTable.PSVersion.Major -ge 6 -and $PSVersionTable.OS -like "*Linux*")
$RunningOnWindows = $RunningOnWindows -or ($PSVersionTable.PSVersion.Major -le 5) -or ($PSVersionTable.OS -like "*Windows*")

# Target definitions
$TargetX64 = "x86_64-apple-darwin"
$TargetArm64 = "aarch64-apple-darwin"

# 1. Verzeichnisse vorbereiten
Write-Host "📦 Preparing release directory..." -ForegroundColor Green
if (Test-Path $ReleaseDir) {
    Remove-Item -Recurse -Force $ReleaseDir
}
New-Item -ItemType Directory -Force -Path $ReleaseDir | Out-Null

# 2. Build
if (-not $SkipBuild) {
    Write-Host "🔨 Building HypnoScript CLI for macOS ($Architecture)..." -ForegroundColor Green

    # Check if we're on a non-macOS system - cross compilation needs special setup
    if (-not $RunningOnMacOS) {
        Write-Host ""
        Write-Host "⚠ Warning: Cross-compiling for macOS from Windows/Linux" -ForegroundColor Yellow
        Write-Host "   This requires:" -ForegroundColor Yellow
        Write-Host "   - macOS SDK/toolchain" -ForegroundColor Yellow
        Write-Host "   - C linker for macOS (cc)" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "   Recommended: Run this script on macOS for best results" -ForegroundColor Yellow
        Write-Host "   Or use: npm run release:windows / npm run release:linux" -ForegroundColor Yellow
        Write-Host ""
        Write-Host "   Skipping build - documentation-only release will be created" -ForegroundColor Cyan
        Write-Host ""
        $SkipBuild = $true
    } else {
        Push-Location $ProjectRoot

        if ($Architecture -eq 'universal') {
            # Universal Binary (beide Architekturen)
            Write-Host "  Building for x86_64 (Intel)..." -ForegroundColor Cyan

            # Check if targets are installed
            $InstalledTargets = rustup target list --installed 2>$null

            if (-not ($InstalledTargets -match $TargetX64)) {
                Write-Host "  Installing target: $TargetX64" -ForegroundColor Yellow
                rustup target add $TargetX64
            }

            if (-not ($InstalledTargets -match $TargetArm64)) {
                Write-Host "  Installing target: $TargetArm64" -ForegroundColor Yellow
                rustup target add $TargetArm64
            }

            cargo build --release --package hypnoscript-cli --target $TargetX64
            Write-Host "  Building for aarch64 (Apple Silicon)..." -ForegroundColor Cyan
            cargo build --release --package hypnoscript-cli --target $TargetArm64

            # Create universal binary with lipo
            Write-Host "  Creating universal binary with lipo..." -ForegroundColor Cyan
            $BinaryX64 = Join-Path "target" $TargetX64 "release" $BINARY_NAME
            $BinaryArm64 = Join-Path "target" $TargetArm64 "release" $BINARY_NAME
            $BinaryUniversal = Join-Path $ReleaseDir $INSTALL_NAME

            & lipo -create $BinaryX64 $BinaryArm64 -output $BinaryUniversal
            chmod +x $BinaryUniversal
        } elseif ($Architecture -eq 'x64') {
            # Nur Intel
            cargo build --release --package hypnoscript-cli --target $TargetX64
            $BinaryPath = Join-Path "target" $TargetX64 "release" $BINARY_NAME
            Copy-Item $BinaryPath (Join-Path $ReleaseDir $INSTALL_NAME)
        } elseif ($Architecture -eq 'arm64') {
            # Nur Apple Silicon
            cargo build --release --package hypnoscript-cli --target $TargetArm64
            $BinaryPath = Join-Path "target" $TargetArm64 "release" $BINARY_NAME
            Copy-Item $BinaryPath (Join-Path $ReleaseDir $INSTALL_NAME)
        }

        Pop-Location
    }
}

if ($SkipBuild) {
    Write-Host "⏩ Skipping build..." -ForegroundColor Yellow
    if ($RunningOnMacOS) {
        Write-Host "   Using existing binaries from previous build" -ForegroundColor Yellow
    } else {
        # Erstelle Platzhalter-Readme für Doc-Only Release
        $ReadmeContent = @"
# HypnoScript for macOS

This is a documentation-only release package.

To build HypnoScript for macOS, please:
1. Clone the repository on a macOS system
2. Run: ``npm run release:macos``

Or download pre-built binaries from GitHub Releases.

## Manual Build on macOS

``````bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone repository
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Build
npm run release:macos
``````
"@
        Set-Content -Path (Join-Path $ReleaseDir "BUILD_INSTRUCTIONS.md") -Value $ReadmeContent
    }
} else {
    # Build completed successfully on macOS
    Write-Host "✓ Build completed successfully" -ForegroundColor Green
}

# 3. Zusätzliche Dateien kopieren
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

# 4. Installation-Script hinzufügen
$InstallerSource = Join-Path $ProjectRoot "install.sh"
if (Test-Path $InstallerSource) {
    Copy-Item $InstallerSource (Join-Path $ReleaseDir "install.sh") -Force
} else {
    Write-Host "⚠ install.sh not found at project root" -ForegroundColor Yellow
}

# 5. TAR.GZ erstellen (immer)
if ($PackageType -eq 'tar.gz' -or $PackageType -eq 'all') {
    Write-Host "📦 Creating TAR.GZ archive..." -ForegroundColor Green

    $TarOut = Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.tar.gz"

    Push-Location (Join-Path $ProjectRoot "release")

    if ($RunningOnMacOS -or $RunningOnLinux) {
        # Native tar auf macOS/Linux
        tar -czf (Split-Path -Leaf $TarOut) -C "macos-$Architecture" .
    } elseif ($RunningOnWindows) {
        # Windows tar (verfügbar ab Windows 10 1803)
        if (Get-Command tar -ErrorAction SilentlyContinue) {
            & tar -czf (Split-Path -Leaf $TarOut) -C "macos-$Architecture" .
        } else {
            Write-Host "  ⚠ tar not found on Windows - installing 7zip or update Windows" -ForegroundColor Yellow
        }
    }

    Pop-Location

    # Checksum
    Write-Host "🔐 Generating SHA256 checksum for tar.gz..." -ForegroundColor Green
    $Hash = Get-FileHash -Path $TarOut -Algorithm SHA256
    $HashString = "$($Hash.Hash.ToLower())  $(Split-Path -Leaf $TarOut)"
    Set-Content -Path "$TarOut.sha256" -Value $HashString

    Write-Host "✓ TAR.GZ: $TarOut" -ForegroundColor Green
    $TarSize = (Get-Item $TarOut).Length / 1MB
    Write-Host "  Size: $([math]::Round($TarSize, 2)) MB" -ForegroundColor Cyan
}

# 6. DMG erstellen (nur auf macOS)
if (($PackageType -eq 'dmg' -or $PackageType -eq 'all') -and $RunningOnMacOS) {
    Write-Host "📦 Creating DMG image..." -ForegroundColor Green

    $DmgDir = Join-Path $ProjectRoot "release" "dmg-staging"
    $DmgOut = Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.dmg"

    # DMG staging vorbereiten
    if (Test-Path $DmgDir) {
        Remove-Item -Recurse -Force $DmgDir
    }
    New-Item -ItemType Directory -Force -Path $DmgDir | Out-Null

    # Binary in staging kopieren
    Copy-Item (Join-Path $ReleaseDir $INSTALL_NAME) $DmgDir
    Copy-Item (Join-Path $ReleaseDir "README.md") $DmgDir -ErrorAction SilentlyContinue
    Copy-Item (Join-Path $ReleaseDir "LICENSE") $DmgDir -ErrorAction SilentlyContinue

    # Symlink zu /usr/local/bin erstellen
    Push-Location $DmgDir
    New-Item -ItemType SymbolicLink -Name "Install to /usr/local/bin" -Target "/usr/local/bin" -ErrorAction SilentlyContinue
    Pop-Location

    # DMG erstellen
    & hdiutil create -volname "$NAME $VERSION" `
        -srcfolder $DmgDir `
        -ov -format UDZO `
        $DmgOut

    # Cleanup
    Remove-Item -Recurse -Force $DmgDir

    # Checksum
    Write-Host "🔐 Generating SHA256 checksum for dmg..." -ForegroundColor Green
    $Hash = Get-FileHash -Path $DmgOut -Algorithm SHA256
    $HashString = "$($Hash.Hash.ToLower())  $(Split-Path -Leaf $DmgOut)"
    Set-Content -Path "$DmgOut.sha256" -Value $HashString

    Write-Host "✓ DMG: $DmgOut" -ForegroundColor Green
    $DmgSize = (Get-Item $DmgOut).Length / 1MB
    Write-Host "  Size: $([math]::Round($DmgSize, 2)) MB" -ForegroundColor Cyan

} elseif (($PackageType -eq 'dmg' -or $PackageType -eq 'all') -and -not $RunningOnMacOS) {
    Write-Host "⚠ DMG creation requires macOS - skipped" -ForegroundColor Yellow
}

# 7. PKG erstellen (nur auf macOS)
if (($PackageType -eq 'pkg' -or $PackageType -eq 'all') -and $RunningOnMacOS) {
    Write-Host "📦 Creating PKG installer..." -ForegroundColor Green

    $PkgDir = Join-Path $ProjectRoot "release" "pkg-staging"
    $PkgRoot = Join-Path $PkgDir "root"
    $PkgScripts = Join-Path $PkgDir "scripts"
    $PkgOut = Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.pkg"

    # PKG staging vorbereiten
    if (Test-Path $PkgDir) {
        Remove-Item -Recurse -Force $PkgDir
    }
    New-Item -ItemType Directory -Force -Path $PkgRoot | Out-Null
    New-Item -ItemType Directory -Force -Path (Join-Path $PkgRoot "usr" "local" "bin") | Out-Null
    New-Item -ItemType Directory -Force -Path $PkgScripts | Out-Null

    # Binary in staging kopieren
    Copy-Item (Join-Path $ReleaseDir $INSTALL_NAME) (Join-Path $PkgRoot "usr" "local" "bin" $INSTALL_NAME)

    # Postinstall script
    $PostInstall = @"
#!/bin/bash
chmod +x /usr/local/bin/$INSTALL_NAME
echo "HypnoScript installed to /usr/local/bin/$INSTALL_NAME"
exit 0
"@
    Set-Content -Path (Join-Path $PkgScripts "postinstall") -Value $PostInstall
    chmod +x (Join-Path $PkgScripts "postinstall")

    # PKG erstellen
    & pkgbuild --root $PkgRoot `
        --scripts $PkgScripts `
        --identifier $BUNDLE_ID `
        --version $VERSION `
        --install-location "/" `
        $PkgOut

    # Cleanup
    Remove-Item -Recurse -Force $PkgDir

    # Checksum
    Write-Host "🔐 Generating SHA256 checksum for pkg..." -ForegroundColor Green
    $Hash = Get-FileHash -Path $PkgOut -Algorithm SHA256
    $HashString = "$($Hash.Hash.ToLower())  $(Split-Path -Leaf $PkgOut)"
    Set-Content -Path "$PkgOut.sha256" -Value $HashString

    Write-Host "✓ PKG: $PkgOut" -ForegroundColor Green
    $PkgSize = (Get-Item $PkgOut).Length / 1MB
    Write-Host "  Size: $([math]::Round($PkgSize, 2)) MB" -ForegroundColor Cyan

} elseif (($PackageType -eq 'pkg' -or $PackageType -eq 'all') -and -not $RunningOnMacOS) {
    Write-Host "⚠ PKG creation requires macOS - skipped" -ForegroundColor Yellow
}

# 8. Zusammenfassung
Write-Host ""
Write-Host "=== Build Summary ===" -ForegroundColor Cyan
Write-Host "Architecture: $Architecture" -ForegroundColor Yellow
Write-Host "Binary Location: $(Join-Path $ReleaseDir $INSTALL_NAME)" -ForegroundColor Cyan

if (Test-Path (Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.tar.gz")) {
    Write-Host "TAR.GZ: $(Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.tar.gz")" -ForegroundColor Cyan
}

if (Test-Path (Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.dmg")) {
    Write-Host "DMG: $(Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.dmg")" -ForegroundColor Cyan
}

if (Test-Path (Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.pkg")) {
    Write-Host "PKG: $(Join-Path $ProjectRoot "release" "$NAME-$VERSION-macos-$Architecture.pkg")" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "Installation instructions:" -ForegroundColor Yellow

if ($PackageType -eq 'tar.gz' -or $PackageType -eq 'all') {
    Write-Host ""
    Write-Host "From TAR.GZ:" -ForegroundColor Green
    Write-Host "  tar -xzf $NAME-$VERSION-macos-$Architecture.tar.gz" -ForegroundColor White
    Write-Host "  cd macos-$Architecture" -ForegroundColor White
    Write-Host "  sudo bash install.sh" -ForegroundColor White
}

if ($RunningOnMacOS) {
    if ($PackageType -eq 'dmg' -or $PackageType -eq 'all') {
        Write-Host ""
        Write-Host "From DMG:" -ForegroundColor Green
        Write-Host "  1. Open $NAME-$VERSION-macos-$Architecture.dmg" -ForegroundColor White
        Write-Host "  2. Drag $INSTALL_NAME to 'Install to /usr/local/bin'" -ForegroundColor White
    }

    if ($PackageType -eq 'pkg' -or $PackageType -eq 'all') {
        Write-Host ""
        Write-Host "From PKG:" -ForegroundColor Green
        Write-Host "  sudo installer -pkg $NAME-$VERSION-macos-$Architecture.pkg -target /" -ForegroundColor White
    }
}

Write-Host ""
Write-Host "Verify installation:" -ForegroundColor Yellow
Write-Host "  hypnoscript --version" -ForegroundColor White
Write-Host ""
Write-Host "✓ All done!" -ForegroundColor Green
