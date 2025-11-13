# HypnoScript Rust CI/CD Pipelines

This directory contains GitHub Actions workflows for building, testing, and deploying the Rust-based HypnoScript implementation.

## Workflows

### 1. `rust-build-and-test.yml` - Main CI Pipeline

**Triggers:**

- Push to `main` or `develop` branches
- Pull requests to `main` or `develop`

**Jobs:**

#### `build-and-test`

- **Platforms:** Windows, Linux, macOS
- **Rust Version:** Stable
- **Steps:**
  - Code formatting check (`cargo fmt`)
  - Linting with Clippy (`cargo clippy`)
  - Build all workspace crates
  - Run all unit and integration tests
  - Test CLI functionality (lex, parse, check, run)
  - Upload binaries as artifacts

#### `code-quality`

- **Platform:** Ubuntu
- **Steps:**
  - CodeQL security analysis (Rust)
  - Cargo audit for vulnerability scanning
  - Check for unsafe code blocks
  - Cargo deny for license and security checks

#### `performance`

- **Platform:** Ubuntu
- **Steps:**
  - Run benchmark tests
  - Generate performance reports
  - Time execution of sample programs

#### `coverage`

- **Platform:** Ubuntu
- **Steps:**
  - Generate code coverage with `cargo-llvm-cov`
  - Upload to Codecov

#### `deployment`

- **Platform:** Ubuntu
- **Triggers:** Only on `main` branch
- **Steps:**
  - Build release binaries
  - Create release package (tar.gz)
  - Upload artifacts

---

### 2. `rust-build-and-release.yml` - Release Pipeline

**Triggers:**

- Tags matching `v*.*.*` or `rust-v*.*.*`

**Jobs:**

#### `build-release`

- **Strategy:** Matrix build for multiple platforms
- **Targets:**
  - Linux x64 (glibc)
  - Linux x64 (musl - static)
  - Windows x64
  - macOS x64
  - macOS ARM64
- **Steps:**
  - Cross-compile for target platform
  - Strip binaries (Unix only)
  - Create platform-specific archives
  - Compute SHA256 checksums

#### `build-deb-package`

- **Platform:** Ubuntu
- **Steps:**
  - Build Debian package with `cargo-deb`
  - Package for APT repositories

#### `create-release`

- **Depends:** build-release, build-deb-package
- **Steps:**
  - Download all platform artifacts
  - Create GitHub Release
  - Upload all binaries and checksums
  - Include installation instructions

#### `publish-crates`

- **Triggers:** Only on version tags
- **Steps:**
  - Publish all crates to crates.io
  - Sequential publishing with delays

---

### 3. `deploy-docs.yml` - Documentation Pipeline

**Triggers:**

- Push to `main` affecting documentation or Rust code
- Pull requests affecting documentation

**Jobs:**

#### `build-and-deploy`

- **Platform:** Ubuntu
- **Steps:**
  - Build Rust API documentation (`cargo doc`)
  - Build user documentation (npm)
  - Combine both documentation sources
  - Deploy to GitHub Pages (main branch only)

#### `test-build`

- **Platform:** Ubuntu
- **Triggers:** Pull requests only
- **Steps:**
  - Build Rust documentation
  - Build user documentation
  - Check for broken links

---

## Required Secrets

For full functionality, configure these GitHub repository secrets:

- `CARGO_TOKEN` - Token for publishing to crates.io (optional)
- `GITHUB_TOKEN` - Automatically provided by GitHub Actions

## Caching Strategy

All workflows use caching to speed up builds:

- **Cargo registry** - Downloaded dependencies
- **Cargo git** - Git dependencies
- **Cargo build** - Compiled artifacts
- **NPM packages** - Node.js dependencies

## Testing Strategy

### Unit Tests

```bash
cargo test --workspace
```

### Integration Tests

```bash
cargo test --package hypnoscript-lexer-parser
cargo test --package hypnoscript-compiler
cargo test --package hypnoscript-runtime
```

### CLI Tests

```bash
hypnoscript-cli version
hypnoscript-cli builtins
hypnoscript-cli lex <file>
hypnoscript-cli parse <file>
hypnoscript-cli check <file>
hypnoscript-cli run <file>
```

### Performance Tests

```bash
cargo test --release -- --ignored --nocapture
```

## Code Quality Checks

### Formatting

```bash
cargo fmt --all -- --check
```

### Linting

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

### Security Audit

```bash
cargo audit
```

### Coverage

```bash
cargo llvm-cov --all-features --workspace
```

## Release Process

1. **Update version** in all `Cargo.toml` files
2. **Create tag:**

   ```bash
   git tag -a v1.0.0 -m "Release v1.0.0"
   git push origin v1.0.0
   ```

3. **GitHub Actions automatically:**
   - Builds binaries for all platforms
   - Creates Debian package
   - Generates checksums
   - Creates GitHub Release
   - Publishes to crates.io (optional)

## Platform-Specific Notes

### Linux (glibc)

- Target: `x86_64-unknown-linux-gnu`
- Requires glibc 2.17+
- Most compatible with modern Linux distributions

### Linux (musl)

- Target: `x86_64-unknown-linux-musl`
- Static binary, no runtime dependencies
- Ideal for containers and embedded systems

### Windows

- Target: `x86_64-pc-windows-msvc`
- Requires Visual C++ runtime (usually pre-installed)

### macOS x64

- Target: `x86_64-apple-darwin`
- Intel-based Macs

### macOS ARM64

- Target: `aarch64-apple-darwin`
- Apple Silicon (M1/M2/M3) Macs

## Continuous Deployment

The `deployment` job on the main branch automatically:

1. Builds release binaries
2. Creates a release package
3. Uploads to GitHub Artifacts

For tagged releases, the full release workflow:

1. Builds for all platforms
2. Creates GitHub Release
3. Publishes to crates.io

## Monitoring

- **Test Results:** Available in Actions artifacts
- **Code Coverage:** Uploaded to Codecov
- **Security:** CodeQL alerts in Security tab
- **Performance:** Benchmark results in artifacts

## Development Workflow

1. **Create feature branch**
2. **Make changes** to Rust code
3. **Run local tests:**

   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

4. **Push to branch** - triggers CI
5. **Create PR** - full test suite runs
6. **Merge to main** - triggers deployment
7. **Tag release** - triggers multi-platform build

## Migration from C# Pipelines

The Rust pipelines replace the C# pipelines with equivalent functionality:

| C# Pipeline             | Rust Pipeline                | Notes                       |
| ----------------------- | ---------------------------- | --------------------------- |
| `build-and-test.yml`    | `rust-build-and-test.yml`    | Same structure, Rust tools  |
| `build-and-release.yml` | `rust-build-and-release.yml` | Multi-platform support      |
| `deploy-docs.yml`       | `deploy-docs.yml`            | Enhanced with Rust API docs |

## Performance Comparison

Rust CI is generally faster than C#:

- **Build time:** ~2-5 minutes (vs 5-10 for C#)
- **Test time:** ~1-2 minutes (vs 3-5 for C#)
- **Binary size:** 5-10MB (vs 60+MB for C#)
- **Cache efficiency:** Better with incremental compilation

## Troubleshooting

### Build Failures

- Check Rust version compatibility
- Verify Cargo.lock is committed
- Review clippy warnings

### Test Failures

- Run tests locally first
- Check for platform-specific issues
- Review test output in artifacts

### Release Issues

- Ensure all Cargo.toml versions match
- Check tag format (v*.*.\*)
- Verify CARGO_TOKEN is set for crates.io

## Further Reading

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Rust CI/CD Best Practices](https://doc.rust-lang.org/cargo/guide/continuous-integration.html)
- [cargo-deb Documentation](https://github.com/kornelski/cargo-deb)
- [Cross-compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
