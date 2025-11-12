# HypnoScript Build Scripts

This directory contains build and packaging scripts for creating release artifacts of HypnoScript.

## 📦 Available Scripts

### Windows Release

**Script**: `build_winget.ps1`
**Usage**: `npm run release:windows` or `pwsh scripts/build_winget.ps1`

Creates a Windows release package including:

- ✅ Optimized binary (`hypnoscript.exe`)
- ✅ ZIP archive for distribution
- ✅ SHA256 checksum
- ✅ WinGet manifest update

**Output**:

- `release/windows-x64/hypnoscript.exe`
- `release/HypnoScript-windows-x64.zip`
- `release/HypnoScript-windows-x64.zip.sha256`

**Requirements**:

- PowerShell 7+
- Rust toolchain (cargo)

---

### Linux Release

**Script**: `build_linux.ps1`
**Usage**: `npm run release:linux` or `pwsh scripts/build_linux.ps1`

Creates a Linux release package including:

- ✅ Binary for Linux (`hypnoscript`)
- ✅ TAR.GZ archive for distribution
- ✅ Installation script
- ✅ SHA256 checksum

**Output**:

- `release/linux-x64/hypnoscript`
- `release/linux-x64/install.sh`
- `release/hypnoscript-1.0.0-linux-x64.tar.gz`
- `release/hypnoscript-1.0.0-linux-x64.tar.gz.sha256`

**Requirements**:

- PowerShell 7+ (cross-platform)
- Rust toolchain (cargo)
- Optional: Linux cross-compilation target (`rustup target add x86_64-unknown-linux-gnu`)

**Installation on Linux**:

```bash
tar -xzf hypnoscript-1.0.0-linux-x64.tar.gz
cd linux-x64
sudo bash install.sh
```

---

### Debian Package (Legacy)

**Script**: `build_deb.sh` (deprecated in favor of `build_linux.ps1`)
**Usage**: `bash scripts/build_deb.sh`

Creates a `.deb` package for Debian/Ubuntu systems.

**Requirements**:

- Bash
- Ruby gem: `fpm` (install via `gem install fpm`)
- Rust toolchain

**Note**: This script has cross-platform issues when run on Windows. Use `build_linux.ps1` instead.

---

## 🚀 Complete Release Pipeline

To prepare and build releases for all platforms:

```bash
# 1. Prepare: Format, Lint, Test, Build
npm run release:prepare

# 2. Build platform-specific packages
npm run release:windows  # Windows x64
npm run release:linux    # Linux x64
```

## 🛠 Cross-Compilation Setup

### Linux Target (for building Linux binaries on Windows/macOS)

```bash
rustup target add x86_64-unknown-linux-gnu
```

### Windows Target (for building Windows binaries on Linux/macOS)

```bash
rustup target add x86_64-pc-windows-msvc
```

## 📝 Version Management

Version information is defined in:

- `Cargo.toml` (workspace root)
- `scripts/build_winget.ps1` (line 8: `$VERSION = "1.0.0"`)
- `scripts/build_linux.ps1` (line 10: `$VERSION = "1.0.0"`)
- `scripts/build_deb.sh` (line 7: `VERSION=1.0.0`)

**Important**: Keep versions synchronized across all files!

## 🔐 Checksum Verification

All release packages include SHA256 checksums:

**Windows**:

```powershell
Get-FileHash -Algorithm SHA256 HypnoScript-windows-x64.zip
```

**Linux**:

```bash
sha256sum hypnoscript-1.0.0-linux-x64.tar.gz
cat hypnoscript-1.0.0-linux-x64.tar.gz.sha256
```

## 📊 Build Artifacts

After running release scripts, the `release/` directory contains:

```
release/
├── windows-x64/
│   ├── hypnoscript.exe
│   ├── README.md
│   ├── LICENSE
│   └── VERSION.txt
├── linux-x64/
│   ├── hypnoscript
│   ├── install.sh
│   ├── README.md
│   ├── LICENSE
│   └── VERSION.txt
├── HypnoScript-windows-x64.zip
├── HypnoScript-windows-x64.zip.sha256
├── hypnoscript-1.0.0-linux-x64.tar.gz
└── hypnoscript-1.0.0-linux-x64.tar.gz.sha256
```

## 🐛 Troubleshooting

### "cargo: command not found"

Ensure Rust is installed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Cross-compilation linker errors

Install the required linker for your target platform:

**For Linux target on Windows**:

- Install WSL2 with Ubuntu
- Or use cross-compilation tools like `cross`

**For Windows target on Linux**:

```bash
sudo apt install mingw-w64
```

### PowerShell execution policy error

```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## 📚 Publishing

### WinGet (Windows Package Manager)

1. Upload release ZIP to GitHub Releases
2. Update `winget-manifest.yaml` with new SHA256
3. Submit manifest as Pull Request to [winget-pkgs](https://github.com/microsoft/winget-pkgs)

### GitHub Releases

1. Create a new release tag (e.g., `v1.0.0`)
2. Upload artifacts:
   - `HypnoScript-windows-x64.zip`
   - `hypnoscript-1.0.0-linux-x64.tar.gz`
   - Checksum files (`.sha256`)
3. Add release notes

---

**Note**: All binaries are statically compiled with Rust and don't require external dependencies!
