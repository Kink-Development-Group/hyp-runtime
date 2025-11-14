#!/usr/bin/env pwsh
# sync-version.ps1
# Synchronizes version from package.json to all project files

param(
    [switch]$DryRun = $false
)

$ErrorActionPreference = "Stop"

# Get project root
$ScriptDir = Split-Path -Parent $PSScriptRoot
$ProjectRoot = $ScriptDir

Write-Host "=== HypnoScript Version Synchronizer ===" -ForegroundColor Cyan
Write-Host ""

# Read version from package.json
$PackageJsonPath = Join-Path $ProjectRoot "package.json"
if (-not (Test-Path $PackageJsonPath)) {
    Write-Host "Error: package.json not found at $PackageJsonPath" -ForegroundColor Red
    exit 1
}

$PackageJson = Get-Content $PackageJsonPath -Raw | ConvertFrom-Json
$Version = $PackageJson.version

Write-Host "Source version from package.json: $Version" -ForegroundColor Green
Write-Host ""

if ($DryRun) {
    Write-Host "DRY RUN MODE - No files will be modified" -ForegroundColor Yellow
    Write-Host ""
}

# Files to update with their patterns
$Updates = @(
    @{
        File = "Cargo.toml"
        Pattern = '(?m)^\[workspace\.package\]\s*\nversion\s*=\s*"[^"]*"'
        Replacement = "[workspace.package]`nversion = `"$Version`""
        Description = "Workspace Cargo.toml"
    },
    @{
        File = "scripts/build_linux.ps1"
        Pattern = '\$VERSION\s*=\s*"[^"]*"'
        Replacement = "`$VERSION = `"$Version`""
        Description = "Linux build script"
    },
    @{
        File = "scripts/build_macos.ps1"
        Pattern = '\$VERSION\s*=\s*"[^"]*"'
        Replacement = "`$VERSION = `"$Version`""
        Description = "macOS build script"
    },
    @{
        File = "scripts/build_winget.ps1"
        Pattern = '\$version\s*=\s*"[^"]*"'
        Replacement = "`$version = `"$Version`""
        Description = "Windows build script"
    },
    @{
        File = "scripts/build_deb.sh"
        Pattern = 'VERSION=[^\n]*'
        Replacement = "VERSION=$Version"
        Description = "Debian build script"
    },
    @{
        File = "scripts/winget-manifest.yaml"
        Pattern = 'PackageVersion:\s*[^\n]*'
        Replacement = "PackageVersion: $Version"
        Description = "WinGet manifest"
    }
)

$UpdatedCount = 0
$SkippedCount = 0

foreach ($Update in $Updates) {
    $FilePath = Join-Path $ProjectRoot $Update.File

    if (-not (Test-Path $FilePath)) {
        Write-Host "⚠ Skipping $($Update.Description): File not found" -ForegroundColor Yellow
        $SkippedCount++
        continue
    }

    $Content = Get-Content $FilePath -Raw

    if ($Content -match $Update.Pattern) {
        $OldMatch = $Matches[0]

        if ($DryRun) {
            Write-Host "Would update $($Update.Description):" -ForegroundColor Cyan
            Write-Host "  From: $OldMatch" -ForegroundColor Gray
            Write-Host "  To:   $($Update.Replacement)" -ForegroundColor Gray
        } else {
            $NewContent = $Content -replace $Update.Pattern, $Update.Replacement
            Set-Content -Path $FilePath -Value $NewContent -NoNewline
            Write-Host "✓ Updated $($Update.Description)" -ForegroundColor Green
            Write-Host "  $($Update.File)" -ForegroundColor Gray
        }

        $UpdatedCount++
    } else {
        Write-Host "⚠ Pattern not found in $($Update.Description)" -ForegroundColor Yellow
        Write-Host "  Expected pattern: $($Update.Pattern)" -ForegroundColor Gray
        $SkippedCount++
    }
}

Write-Host ""
Write-Host "=== Summary ===" -ForegroundColor Cyan
Write-Host "Version: $Version" -ForegroundColor White
Write-Host "Updated: $UpdatedCount files" -ForegroundColor Green
if ($SkippedCount -gt 0) {
    Write-Host "Skipped: $SkippedCount files" -ForegroundColor Yellow
}

if ($DryRun) {
    Write-Host ""
    Write-Host "This was a dry run. Run without -DryRun to apply changes." -ForegroundColor Yellow
} else {
    Write-Host ""
    Write-Host "✓ Version synchronization complete!" -ForegroundColor Green
}
