---
title: CLI Basics
---

# CLI Basics

The HypnoScript Command Line Interface (CLI) is your primary tool for working with HypnoScript. This guide covers all the essential commands and options you need to know.

## Overview

The HypnoScript CLI provides a comprehensive set of commands for:

- Running scripts
- Analyzing code quality
- Measuring performance
- Generating documentation
- Managing configuration
- Testing and validation

## Getting Help

### General Help

```bash
# Show main help
hyp --help

# Show version information
hyp --version
```

### Command-Specific Help

```bash
# Help for specific commands
hyp run --help
hyp lint --help
hyp benchmark --help
hyp profile --help
hyp optimize --help
hyp docs --help
hyp config --help
```

## Core Commands

### Running Scripts

The `run` command executes HypnoScript files:

```bash
# Basic script execution
hyp run script.hyp

# Run with specific arguments
hyp run script.hyp --arg1 value1 --arg2 value2

# Run with verbose output
hyp run script.hyp --verbose

# Run with debug information
hyp run script.hyp --debug

# Run and save output to file
hyp run script.hyp --output result.txt
```

**Options:**

- `--verbose, -v`: Enable verbose logging
- `--debug, -d`: Enable debug mode
- `--output, -o <file>`: Save output to specified file
- `--timeout <seconds>`: Set execution timeout
- `--memory-limit <mb>`: Set memory usage limit

### Code Analysis (Linting)

The `lint` command analyzes your code for potential issues:

```bash
# Basic linting
hyp lint script.hyp

# Lint with detailed output
hyp lint script.hyp --verbose

# Lint multiple files
hyp lint *.hyp

# Lint with specific rules
hyp lint script.hyp --strict

# Generate lint report
hyp lint script.hyp --output lint-report.json
```

**Options:**

- `--verbose, -v`: Show detailed analysis
- `--strict`: Enable strict mode (more warnings)
- `--output, -o <file>`: Save report to file
- `--format <format>`: Output format (text, json, xml)

**What it checks:**

- Syntax errors
- Type mismatches
- Undefined variables
- Unused variables
- Potential runtime issues
- Code style violations

### Performance Benchmarking

The `benchmark` command measures script performance:

```bash
# Basic benchmarking
hyp benchmark script.hyp

# Benchmark with multiple iterations
hyp benchmark script.hyp --iterations 100

# Benchmark with warm-up runs
hyp benchmark script.hyp --warmup 10 --iterations 50

# Detailed performance analysis
hyp benchmark script.hyp --detailed

# Save benchmark results
hyp benchmark script.hyp --output benchmark.json
```

**Options:**

- `--iterations, -i <count>`: Number of test iterations
- `--warmup <count>`: Number of warm-up runs
- `--detailed, -d`: Show detailed statistics
- `--output, -o <file>`: Save results to file
- `--timeout <seconds>`: Timeout per iteration

### Performance Profiling

The `profile` command provides detailed performance analysis:

```bash
# Basic profiling
hyp profile script.hyp

# Profile with memory tracking
hyp profile script.hyp --memory

# Profile with call stack analysis
hyp profile script.hyp --call-stack

# Generate profiling report
hyp profile script.hyp --output profile.html
```

**Options:**

- `--memory, -m`: Track memory usage
- `--call-stack, -c`: Analyze function calls
- `--detailed, -d`: Detailed profiling data
- `--output, -o <file>`: Save profile report
- `--format <format>`: Report format (text, html, json)

### Code Optimization

The `optimize` command provides optimization suggestions:

```bash
# Basic optimization analysis
hyp optimize script.hyp

# Detailed optimization report
hyp optimize script.hyp --detailed

# Generate optimization suggestions
hyp optimize script.hyp --suggestions

# Save optimization report
hyp optimize script.hyp --output optimize.json
```

**Options:**

- `--detailed, -d`: Detailed analysis
- `--suggestions, -s`: Show optimization suggestions
- `--output, -o <file>`: Save report to file
- `--format <format>`: Output format

### Documentation Generation

The `docs` command generates documentation from your scripts:

```bash
# Generate basic documentation
hyp docs script.hyp

# Generate HTML documentation
hyp docs script.hyp --format html

# Generate documentation with examples
hyp docs script.hyp --include-examples

# Generate documentation for multiple files
hyp docs *.hyp --output docs/

# Generate API documentation
hyp docs script.hyp --api
```

**Options:**

- `--format <format>`: Output format (markdown, html, pdf)
- `--include-examples, -e`: Include code examples
- `--api, -a`: Generate API documentation
- `--output, -o <dir>`: Output directory
- `--template <file>`: Custom template file

### Configuration Management

The `config` command manages HypnoScript configuration:

```bash
# Show current configuration
hyp config show

# Get specific setting
hyp config get logging.level

# Set configuration value
hyp config set logging.level DEBUG

# Reset configuration to defaults
hyp config reset

# Export configuration
hyp config export --output config.json

# Import configuration
hyp config import config.json
```

**Subcommands:**

- `show`: Display current configuration
- `get <key>`: Get specific configuration value
- `set <key> <value>`: Set configuration value
- `reset`: Reset to default configuration
- `export`: Export configuration to file
- `import`: Import configuration from file

## Advanced Usage

### Batch Processing

Process multiple files at once:

```bash
# Run multiple scripts
hyp run *.hyp

# Lint all scripts in directory
hyp lint src/**/*.hyp

# Benchmark all test scripts
hyp benchmark tests/*.hyp --iterations 10

# Generate docs for all scripts
hyp docs src/**/*.hyp --output docs/
```

### Script Arguments

Pass arguments to your scripts:

```bash
# Pass named arguments
hyp run script.hyp --name "John" --age 30

# Pass positional arguments
hyp run script.hyp arg1 arg2 arg3

# Pass complex data
hyp run script.hyp --config config.json --data data.csv
```

### Output Redirection

```bash
# Save output to file
hyp run script.hyp > output.txt

# Save errors to file
hyp run script.hyp 2> errors.log

# Save both output and errors
hyp run script.hyp > output.txt 2>&1

# Pipe output to another command
hyp run script.hyp | grep "ERROR"
```

### Environment Variables

Set environment variables for script execution:

```bash
# Set single variable
DEBUG=true hyp run script.hyp

# Set multiple variables
DEBUG=true LOG_LEVEL=INFO hyp run script.hyp

# Use environment file
hyp run script.hyp --env-file .env
```

## Configuration

### Global Configuration

HypnoScript uses a global configuration file:

**Location:**

- Windows: `%APPDATA%\HypnoScript\config.json`
- Linux/macOS: `~/.config/hypnoscript/config.json`

**Example configuration:**

```json
{
  "logging": {
    "level": "INFO",
    "format": "text"
  },
  "runtime": {
    "timeout": 300,
    "memoryLimit": 512
  },
  "cli": {
    "defaultFormat": "text",
    "colorOutput": true
  }
}
```

### Project Configuration

Create a `hypnoscript.json` file in your project root:

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "scripts": {
    "test": "hyp run tests/*.hyp",
    "lint": "hyp lint src/**/*.hyp",
    "docs": "hyp docs src/**/*.hyp --output docs/"
  },
  "config": {
    "logging": {
      "level": "DEBUG"
    }
  }
}
```

## Troubleshooting

### Common Issues

1. **"Command not found"**:

   ```bash
   # Check installation
   hyp --version

   # Reinstall if needed
   winget install HypnoScript.HypnoScript
   ```

2. **Permission errors**:

   ```bash
   # On Linux/macOS
   chmod +x script.hyp

   # Check file permissions
   ls -la script.hyp
   ```

3. **Script execution fails**:

   ```bash
   # Check for syntax errors
   hyp lint script.hyp

   # Run with debug mode
   hyp run script.hyp --debug
   ```

4. **Performance issues**:

   ```bash
   # Profile the script
   hyp profile script.hyp --memory

   # Check for memory leaks
   hyp benchmark script.hyp --iterations 100
   ```

### Debug Mode

Enable debug mode for detailed information:

```bash
# Enable debug logging
hyp run script.hyp --debug

# Set debug environment variable
DEBUG=true hyp run script.hyp

# Use verbose output
hyp run script.hyp --verbose
```

### Log Files

HypnoScript creates log files for debugging:

**Location:**

- Windows: `%TEMP%\hypnoscript\logs\`
- Linux/macOS: `/tmp/hypnoscript/logs/`

**Log levels:**

- `ERROR`: Error messages only
- `WARNING`: Warnings and errors
- `INFO`: General information (default)
- `DEBUG`: Detailed debugging information
- `TRACE`: Very detailed tracing

## Best Practices

### 1. Use Consistent Naming

```bash
# Good
hyp run user-authentication.hyp
hyp lint data-processing.hyp

# Avoid
hyp run script1.hyp
hyp lint temp.hyp
```

### 2. Organize Your Projects

```
project/
├── src/
│   ├── main.hyp
│   └── utils.hyp
├── tests/
│   ├── test-main.hyp
│   └── test-utils.hyp
├── docs/
├── hypnoscript.json
└── README.md
```

### 3. Use Configuration Files

```bash
# Create project configuration
hyp config export --output hypnoscript.json

# Use project-specific settings
hyp run script.hyp --config hypnoscript.json
```

### 4. Automate Common Tasks

Create shell scripts or batch files:

```bash
#!/bin/bash
# build.sh
hyp lint src/**/*.hyp
hyp run tests/*.hyp
hyp docs src/**/*.hyp --output docs/
```

### 5. Version Control Integration

```bash
# Pre-commit hooks
hyp lint staged-files.hyp
hyp run tests/*.hyp

# CI/CD integration
hyp benchmark critical-script.hyp --iterations 100
hyp profile performance-test.hyp
```

## Conclusion

The HypnoScript CLI provides powerful tools for development, testing, and deployment. By mastering these commands, you can:

- Write better code with linting and optimization
- Measure and improve performance
- Generate comprehensive documentation
- Manage configuration effectively
- Automate your development workflow

Start with the basic commands and gradually explore the advanced features as you become more comfortable with HypnoScript development.
