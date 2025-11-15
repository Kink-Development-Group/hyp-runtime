# build_winget.ps1
# Creates a Windows release package for HypnoScript Rust Runtime
# Usage: pwsh scripts/build_winget.ps1

$ErrorActionPreference = 'Stop'

Write-Host "=== HypnoScript Windows Release Builder ===" -ForegroundColor Cyan
Write-Host ""

# Configuration
$projectRoot = Split-Path -Parent $PSScriptRoot
$releaseDir = Join-Path $projectRoot "release"
$winDir = Join-Path $releaseDir "windows-x64"
$zipPath = Join-Path $releaseDir "HypnoScript-windows-x64.zip"

# Clean previous release
if (Test-Path $releaseDir) {
    Write-Host "Cleaning previous release..." -ForegroundColor Yellow
    Remove-Item $releaseDir -Recurse -Force
}

# Create release directory
New-Item -ItemType Directory -Path $winDir -Force | Out-Null

# Build release binary
Write-Host "Building release binary for Windows x64..." -ForegroundColor Green
Push-Location $projectRoot
try {
    cargo build --release --package hypnoscript-cli

    if ($LASTEXITCODE -ne 0) {
        throw "Cargo build failed with exit code $LASTEXITCODE"
    }
} finally {
    Pop-Location
}

# Copy binary to release directory
$binarySource = Join-Path $projectRoot "target\release\hypnoscript-cli.exe"

if (-not (Test-Path $binarySource)) {
    throw "Could not find compiled binary at $binarySource"
}

Write-Host "Copying binary to release directory..." -ForegroundColor Green
Copy-Item $binarySource -Destination (Join-Path $winDir "hypnoscript.exe")

# Copy additional files
Write-Host "Copying additional files..." -ForegroundColor Green

$readmePath = Join-Path $projectRoot "README.md"
if (Test-Path $readmePath) {
    Copy-Item $readmePath -Destination $winDir
}

$licensePath = Join-Path $projectRoot "LICENSE"
if (Test-Path $licensePath) {
    Copy-Item $licensePath -Destination $winDir
}

# Create VERSION file
$version = "1.0.0"
$versionFile = Join-Path $winDir "VERSION.txt"
Set-Content -Path $versionFile -Value "HypnoScript Runtime v$version`n`nBuilt: $(Get-Date -Format 'yyyy-MM-dd HH:mm:ss')"

# Create ZIP archive
Write-Host "Creating ZIP archive..." -ForegroundColor Green
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}

Compress-Archive -Path "$winDir\*" -DestinationPath $zipPath -CompressionLevel Optimal

# Calculate SHA256 hash
Write-Host "Calculating SHA256 hash..." -ForegroundColor Green
$sha256 = (Get-FileHash $zipPath -Algorithm SHA256).Hash
Write-Host "SHA256: $sha256" -ForegroundColor Yellow

# Update manifest if it exists
$manifestPath = Join-Path $PSScriptRoot "winget-manifest.yaml"
if (Test-Path $manifestPath) {
    Write-Host "Updating winget manifest..." -ForegroundColor Green
    $manifestContent = Get-Content $manifestPath -Raw
    $manifestContent = $manifestContent -replace '(InstallerSha256:\s*)([a-fA-F0-9]+)', "`${1}$sha256"
    Set-Content -Path $manifestPath -Value $manifestContent -NoNewline
    Write-Host "Manifest updated with new SHA256 hash" -ForegroundColor Green
}

# Display summary
Write-Host ""
Write-Host "=== Build Summary ===" -ForegroundColor Cyan
Write-Host "Release Directory: $releaseDir" -ForegroundColor White
Write-Host "Binary Location:   $(Join-Path $winDir 'hypnoscript.exe')" -ForegroundColor White
Write-Host "ZIP Archive:       $zipPath" -ForegroundColor White
Write-Host "SHA256 Hash:       $sha256" -ForegroundColor White
Write-Host ""

# Get file sizes
$binarySize = [math]::Round((Get-Item (Join-Path $winDir "hypnoscript.exe")).Length / 1MB, 2)
$zipSize = [math]::Round((Get-Item $zipPath).Length / 1MB, 2)

Write-Host "Binary Size:       $binarySize MB" -ForegroundColor White
Write-Host "Archive Size:      $zipSize MB" -ForegroundColor White
Write-Host ""
Write-Host "✓ Windows release package created successfully!" -ForegroundColor Green
Write-Host ""

# Test the binary
Write-Host "=== Testing Binary ===" -ForegroundColor Cyan
$testBinary = Join-Path $winDir "hypnoscript.exe"
& $testBinary version
Write-Host ""
Write-Host "✓ All done!" -ForegroundColor Green
