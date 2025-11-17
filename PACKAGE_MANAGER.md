# HypnoScript Package Manager

The HypnoScript Package Manager is an integrated dependency management tool for HypnoScript projects, similar to npm for JavaScript or cargo for Rust.

**Note:** All commands can be run with either `hypnoscript` or the shorter `hyp` alias.

## Overview

The package manager uses two main files:

- **`trance.json`**: The manifest file that defines your project and its dependencies
- **`trance-lock.json`**: The lock file that captures the exact versions of installed dependencies

## Quick Start

### Initialize a New Project

```bash
# Initialize with default settings (library)
hypnoscript init
# or use the short form:
hyp init

# Initialize with a custom name
hypnoscript init --name my-awesome-project

# Initialize using a template
hypnoscript init --template cli
hypnoscript init --template library
```

### Managing Dependencies

```bash
# Add a production dependency
hypnoscript add hypnoscript-runtime --version "^1.0.0"

# Add a development dependency
hypnoscript add @hypno/testing-lab --version "^0.3.0" --dev

# Remove a dependency
hypnoscript remove hypnoscript-runtime

# List all dependencies
hypnoscript list

# Install all dependencies
hypnoscript install
```

### Working with Scripts

```bash
# Run a script defined in suggestions
hypnoscript run focus
hypnoscript run test
hypnoscript run build
```

### Validation

```bash
# Validate your trance.json
hypnoscript validate
```

## The trance.json Manifest

The `trance.json` file follows the HypnoScript theming with hypnotic terminology:

```json
{
  "ritualName": "hypno-cli-starter",
  "mantra": "0.1.0",
  "intent": "cli",
  "induction": {
    "description": "Starter template for a HypnoScript CLI app",
    "entryScript": "src/main.hyp",
    "keywords": ["hypnoscript", "cli", "template", "starter"],
    "license": "MIT"
  },
  "hypnotists": [
    {
      "name": "Your Name",
      "role": "Lead Hypnotist",
      "contact": "mailto:you@example.com"
    }
  ],
  "auras": {
    "repository": "https://github.com/your-org/hypno-cli",
    "homepage": "https://your-org.dev/hypno-cli",
    "documentation": "docs/index.md",
    "supportChannel": "https://chat.your-org.dev/hypno"
  },
  "suggestions": {
    "focus": "hypnoscript exec src/main.hyp -- help",
    "status": "hypnoscript exec src/main.hyp -- status",
    "test": "hypnoscript exec tests/smoke.hyp"
  },
  "anchors": {
    "hypnoscript-runtime": "^1.0.0"
  },
  "deepAnchors": {
    "@hypno/testing-lab": "^0.3.0"
  },
  "channels": {
    "binary": "hypno-cli",
    "entry": "focus",
    "targets": ["windows-x64", "linux-x64", "macos-universal"],
    "telemetry": {
      "enabled": false,
      "endpoint": ""
    }
  },
  "triggers": {
    "preFocus": "scripts/pre-focus.hyp",
    "postRelax": "scripts/post-relax.hyp"
  }
}
```

### Field Descriptions

| Field | Description | npm Equivalent |
|-------|-------------|----------------|
| `ritualName` | Package name | `name` |
| `mantra` | Package version (semver) | `version` |
| `intent` | Project type (cli, library) | N/A |
| `induction` | Package metadata | Combined from multiple fields |
| `hypnotists` | Contributors/authors | `contributors` |
| `auras` | Links and resources | `repository`, `homepage`, etc. |
| `suggestions` | Runnable scripts | `scripts` |
| `anchors` | Production dependencies | `dependencies` |
| `deepAnchors` | Development dependencies | `devDependencies` |
| `channels` | Binary/CLI configuration | `bin` |
| `triggers` | Lifecycle hooks | `scripts` (lifecycle) |

## The trance-lock.json Lock File

The lock file ensures reproducible builds by capturing exact dependency versions:

```json
{
  "lockVersion": "1.0.0",
  "lockedAnchors": {
    "hypnoscript-runtime": {
      "version": "^1.0.0",
      "source": "registry",
      "integrity": null,
      "dependencies": {}
    }
  }
}
```

## Templates

### CLI Template

For command-line applications:

```bash
hypnoscript init --template cli
```

Creates a project with:
- Binary configuration in `channels`
- Default scripts for running and testing
- CLI-specific metadata

### Library Template

For reusable libraries:

```bash
hypnoscript init --template library
```

Creates a project with:
- Library-focused structure
- Build and test scripts
- Entry point at `src/lib.hyp`

## Integration with Tools

The package manager is designed to work seamlessly with:

- **Formatter**: Respects project dependencies when formatting
- **Linter**: Uses dependency information for linting
- **Compiler**: Understands the module structure

## Version Specifications

The package manager supports standard semver version specifications:

- `^1.0.0`: Compatible with version 1.x.x (>= 1.0.0, < 2.0.0)
- `~1.2.3`: Approximately equivalent to version 1.2.x (>= 1.2.3, < 1.3.0)
- `1.2.3`: Exact version
- `>=1.0.0`: Greater than or equal to 1.0.0

## Future Enhancements

The current implementation provides the foundation for:

1. **Package Registry**: A centralized server for hosting HypnoScript packages
2. **Dependency Resolution**: Automatic resolution of transitive dependencies
3. **Package Publishing**: Commands to publish packages to the registry
4. **Workspaces**: Support for monorepo-style projects
5. **Script Execution**: Direct execution of suggestion scripts
6. **Dependency Auditing**: Security vulnerability scanning

## Examples

See the `examples/trance.json` file for a complete example of all available fields.

## Contributing

The package manager is part of the HypnoScript CLI. To contribute:

1. Add new features to `hypnoscript-cli/src/package.rs`
2. Add tests to verify functionality
3. Update this documentation
4. Submit a pull request

## License

MIT License - Same as HypnoScript Runtime
