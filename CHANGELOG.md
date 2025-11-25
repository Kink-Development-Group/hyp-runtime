# Changelog

All notable changes to this project will be documented in this file. The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) and this project adheres to [Semantic Versioning](https://semver.org/).

## [1.0.0] - 2025-11-15

### Added

- Initial release of the complete **HypnoScript** stack with compiler (`hypnoscript-compiler`), runtime (`hypnoscript-runtime`), and core library (`hypnoscript-core`).
- Integration of the Cranelift backend for native code generation including linker workflow and platform support for Linux, Windows, and macOS.
- Comprehensive CLI (`hypnoscript-cli`) with commands for running scripts, test runs, builtin listing, and version output.
- Asynchronous runtime ecosystem with Promise support, channel system, and extended builtins (strings, arrays, files, hashing, localization, and much more).
- Complete language documentation with VitePress, including getting-started guides, language reference, builtin catalog, and enterprise chapter.
- Automated build and release scripts for Linux, Windows (Winget), and macOS (Universal/x64/arm64, pkg & dmg).

### Changed

- Consolidated type checking, parser improvements, and iterator-based implementations to comply with strict `cargo clippy` warning guidelines.
- Unified handling of linker arguments, record types, and function signatures to ensure stable release builds across the entire workspace.

### Fixed

- Resolved borrow checker issues in the native code generator and stabilized channel synchronization in the async runtime module.
- Reduced error and warning messages in interpreter, optimizer, and parser through targeted refactorings.
- Added missing type system documentation and corrected unreachable documentation links (e.g., `language-reference/types.html`).

### Security & Compliance

- Updated `deny.toml`, including MPL-2.0 license exception for `webpki-roots` and ignoring the documented advisory `RUSTSEC-2020-0168`.
- Successfully completed `cargo deny check` with cleaned-up license and advisory checks.

[1.0.0]: https://github.com/Kink-Development-Group/hyp-runtime/releases/tag/1.0.0
