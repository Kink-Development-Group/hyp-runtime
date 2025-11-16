---
sidebar_position: 1
---

# Installation

This guide will walk you through installing the Rust-based HypnoScript toolchain.

## Prerequisites

| Component     | Recommendation                                                                   |
| ------------- | -------------------------------------------------------------------------------- |
| Operating System | Windows 10+, macOS 12+, Linux (Ubuntu 20.04+, Fedora 38+, Arch)              |
| Rust Toolchain  | `rustup` with Rust 1.76 or newer (check with `rustup --version`)              |
| Build Tools     | Git, C/C++ Build Tools (provided by `rustup` / package manager)               |

Optional for documentation: Node.js 18+.

### Installing Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# After installation, open a new terminal and verify
rustc --version
cargo --version
```

On Windows, alternatively download [rustup-init.exe](https://win.rustup.rs/).

## Building HypnoScript from Repository (recommended)

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Create release build of the CLI
cargo build -p hypnoscript-cli --release

# Optionally install globally (places hypnoscript in Cargo bin directory)
cargo install --path hypnoscript-cli
```

The built CLI will be located at `./target/release/hypnoscript` or after installation in the Cargo bin directory (`~/.cargo/bin` or `%USERPROFILE%\.cargo\bin`).

## Automatic Installer (recommended for releases)

For production systems or quick tests, you can use the official installer. The script detects your operating system (Linux / macOS), automatically downloads the appropriate runtime from the current release, and updates existing installations. Since the current release series, the `install.sh` script is automatically copied into every release archive as well as the documentation assets – so you always get the same signed source, whether you manually extract the archive or use the online invocation.

```bash
curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash
```

The installer now offers a unified workflow experience:

- ✅ **Auto-Detection** for architecture, platform, and existing installations
- ♻️ **Update & Re-Install** without re-downloading complete archives
- 🧹 **Cleanup/Uninstall** including metadata (`installation.json`)
- 📦 **Offline-Support** via release archive (included `share/hypnoscript/install.sh`)

Important options at a glance:

| Option                 | Description                                                    |
| ---------------------- | -------------------------------------------------------------- |
| `--prefix <path>`      | Target directory (default: `/usr/local/bin`)                   |
| `--check`              | Only check for updates (exit code `0` = current, `2` = update) |
| `--version <v>`        | Install specific version                                       |
| `--include-prerelease` | Also consider pre-releases                                     |
| `--force`              | Force installation even if version already exists              |
| `--quiet`              | Minimize installer output (errors only)                        |
| `--no-sudo`            | Never automatically request `sudo`                             |
| `--uninstall`          | Remove installed runtime (binary & metadata)                   |

The script can be run again at any time. If it detects a new version, an update will be automatically applied.

### Updates & Uninstallation

The CLI includes an integrated `self-update` command that implements the most important installer options:

- **Check for updates:** `hypnoscript self-update --check`
- **Update:** `hypnoscript self-update`
- **Allow pre-releases:** `hypnoscript self-update --include-prerelease`
- **Force reinstall:** `hypnoscript self-update --force`
- **Quiet/No-Sudo mode:** `hypnoscript self-update --quiet --no-sudo`

> **Note:** On Windows, currently only the check function is available. The actual installation must still be done manually from the release.

For complete uninstallation, continue to use the installer script with `--uninstall`:

```bash
curl -fsSL https://kink-development-group.github.io/hyp-runtime/install.sh | bash -s -- --uninstall
```

## Using Pre-built Release Packages

If you don't want to build yourself, you can find signed artifacts for Windows, macOS, and Linux at [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases). After extracting, you can run the included binary directly.

## Verifying Installation

```bash
# Display version and available commands
hypnoscript version
hypnoscript builtins

# Minimal test program
echo 'Focus { entrance { observe "Installation successful!"; } } Relax' > test.hyp
hypnoscript run test.hyp
```

Expected output (abbreviated):

```text
HypnoScript v1.0.0
Installation successful!
```

## Common Issues

| Problem                  | Solution                                                                                            |
| ------------------------ | --------------------------------------------------------------------------------------------------- |
| `cargo` not found        | Check if `~/.cargo/bin` (Linux/macOS) or `%USERPROFILE%\.cargo\bin` (Windows) is in your `PATH`.   |
| Linker errors on Linux   | Install build dependencies (`sudo apt install build-essential` or distribution equivalent).         |
| No execution permissions | Set `chmod +x hypnoscript` after extracting a release artifact.                                     |

## Optional: Development Comfort

- **VS Code**: Install the _Rust Analyzer_ and _Even Better TOML_ extensions. The repo includes a `hyp-runtime.code-workspace` file.
- **Shell Alias**: `alias hyp="hypnoscript"` for shorter commands.
- **Building documentation**: `npm install` & `npm run dev` in the `hypnoscript-docs` folder.

## Next Steps

- [Quick Start](./quick-start)
- [CLI Basics](./cli-basics)
- [Language Reference](../language-reference/syntax)
- [Standard Library](../builtins/overview)

Happy hypnotic coding! 🌀
