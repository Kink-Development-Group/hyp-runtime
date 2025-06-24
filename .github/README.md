# GitHub Actions Workflows

This directory contains all GitHub Actions workflows for the HypnoScript project. The workflows have been optimized for reliability, security, and performance.

## 🔄 Active Workflows

### Primary Workflows

1. **main.yml** - Main CI/CD Pipeline

   - Builds and tests on multiple platforms
   - Code quality analysis with CodeQL
   - Documentation building
   - Performance benchmarks
   - Comprehensive summary reporting

2. **security.yml** - Security Analysis

   - Static Application Security Testing (SAST)
   - Dependency vulnerability scanning
   - Secret detection
   - Automated security reporting

3. **release.yml** - Release Management
   - Automated releases on version tags
   - Multi-platform package building
   - Release notes generation
   - Winget manifest updates

### Legacy Workflows (To Be Deprecated)

- `build-and-test.yml` - Replaced by `main.yml`
- `ci.yml` - Replaced by `main.yml`
- `security-scan.yml` - Replaced by `security.yml`
- `build-and-release.yml` - Replaced by `release.yml`

### Specialized Workflows

- `deploy-docs.yml` - Documentation deployment
- `performance-monitoring.yml` - Performance tracking
- `dependency-update.yml` - Automated dependency updates

## 🛠️ Key Improvements Made

### 1. **Updated Action Versions**

- All actions updated to latest stable versions
- Removed deprecated actions like `actions/create-release@v1`
- Used `softprops/action-gh-release@v2` for releases

### 2. **Enhanced Error Handling**

- Added `continue-on-error` for non-critical steps
- Proper validation before file operations
- Graceful handling of missing files/dependencies

### 3. **Security Improvements**

- Conditional secret usage to prevent errors
- Enhanced secret detection patterns
- TruffleHog integration for comprehensive secret scanning

### 4. **Performance Optimizations**

- Strategic caching for .NET packages and Node modules
- Parallel job execution where possible
- Conditional workflow execution based on file changes

### 5. **Better Observability**

- Comprehensive job summaries with status tables
- Detailed artifact uploads with appropriate retention
- Clear success/failure indicators

## 🔧 Configuration

### Environment Variables

```yaml
DOTNET_VERSION: '8.0.x'
NODE_VERSION: '18'
DOTNET_SKIP_FIRST_TIME_EXPERIENCE: 1
DOTNET_CLI_TELEMETRY_OPTOUT: 1
```

### Required Secrets

- `GITHUB_TOKEN` - Automatically provided by GitHub
- `DISCORD_WEBHOOK` - Optional, for notifications
- `SNYK_TOKEN` - Optional, for Snyk security scanning
- `NETLIFY_AUTH_TOKEN` - Optional, for documentation deployment
- `NETLIFY_SITE_ID` - Optional, for documentation deployment

## 📊 Workflow Triggers

| Workflow                   | Push      | PR  | Schedule    | Manual |
| -------------------------- | --------- | --- | ----------- | ------ |
| main.yml                   | ✅        | ✅  | ❌          | ✅     |
| security.yml               | ✅        | ✅  | ✅ (Daily)  | ✅     |
| release.yml                | ✅ (Tags) | ❌  | ❌          | ✅     |
| deploy-docs.yml            | ✅        | ✅  | ❌          | ✅     |
| performance-monitoring.yml | ✅        | ❌  | ✅ (Daily)  | ✅     |
| dependency-update.yml      | ❌        | ❌  | ✅ (Weekly) | ✅     |

## 🔍 Monitoring & Debugging

### Checking Workflow Status

1. Go to the "Actions" tab in GitHub
2. Select the workflow you want to check
3. Review the job summaries and artifacts

### Common Issues & Solutions

**Build Failures:**

- Check that all .csproj files are valid
- Ensure dependencies are properly restored
- Verify .NET version compatibility

**Test Failures:**

- Review test output in artifacts
- Check for missing test files
- Validate test environment setup

**Security Scan Issues:**

- Review vulnerability reports in artifacts
- Update dependencies with known issues
- Check for hardcoded secrets in code

**Documentation Build Issues:**

- Ensure package.json exists in documentation folder
- Check Node.js version compatibility
- Verify all documentation dependencies

## 📈 Performance Considerations

- **Caching Strategy**: Aggressive caching for NuGet packages and Node modules
- **Matrix Strategy**: Parallel builds across multiple platforms
- **Conditional Execution**: Workflows only run when relevant files change
- **Artifact Management**: Strategic retention periods to balance storage costs

## 🚀 Future Improvements

1. **Container Registry Integration**: Build and push Docker images
2. **Multi-Environment Deployments**: Staging and production environments
3. **Advanced Performance Monitoring**: Trend analysis and regression detection
4. **Enhanced Notifications**: Slack/Teams integration
5. **Automated Security Fixes**: Auto-PRs for security updates

## 📝 Contributing

When modifying workflows:

1. Test changes in a feature branch first
2. Use `workflow_dispatch` for manual testing
3. Keep backward compatibility in mind
4. Update this README with significant changes
5. Consider the impact on build times and costs

## 🔗 Related Documentation

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [.NET CLI Reference](https://docs.microsoft.com/en-us/dotnet/core/tools/)
- [CodeQL Documentation](https://codeql.github.com/docs/)
- [Security Best Practices](https://docs.github.com/en/actions/security-guides)
