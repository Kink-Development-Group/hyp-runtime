# GitHub Actions - HypnoScript CI/CD Pipeline

Diese Dokumentation beschreibt die umfassende CI/CD-Pipeline für das HypnoScript-Projekt.

## 📋 Übersicht

Die GitHub Actions sind in mehrere spezialisierte Workflows aufgeteilt, die verschiedene Aspekte der Softwareentwicklung abdecken:

### 🔄 Haupt-Workflows

1. **`ci.yml`** - Continuous Integration

   - Build, Test, Code Quality, Security, Performance, Documentation
   - Läuft bei jedem Push und Pull Request

2. **`release.yml`** - Release Management

   - Automatische Paketierung und Distribution
   - Erstellt GitHub Releases und aktualisiert winget

3. **`deploy-docs.yml`** - Dokumentations-Deployment
   - Baut und deployed die Docusaurus-Dokumentation
   - GitHub Pages und Staging-Deployment

### 🔧 Spezialisierte Workflows

4. **`dependency-update.yml`** - Dependency Management

   - Automatische Dependency-Updates
   - Security-Patch-Management

5. **`performance-monitoring.yml`** - Performance-Monitoring

   - Kontinuierliche Performance-Tests
   - Regression-Detection

6. **`security-scan.yml`** - Security-Scanning
   - Umfassende Sicherheitsprüfungen
   - SAST, Dependency-Scanning, Secret-Scanning

## 🚀 Workflow-Details

### CI Pipeline (`ci.yml`)

**Trigger:** Push zu `main`, `develop`, `feature/*` und Pull Requests

**Jobs:**

- **Build**: Multi-Platform Build (Windows, Linux, macOS)
- **Test**: Unit-Tests, Integration-Tests, Coverage
- **Code Quality**: CodeQL, SonarCloud, Format-Check
- **Security**: OWASP Dependency Check, Snyk
- **Performance**: Benchmarks, Profiling
- **Documentation**: Build und Link-Check
- **Summary**: Zusammenfassung aller Jobs

**Features:**

- Dependency Caching für schnellere Builds
- Matrix-Tests für verschiedene Plattformen
- Code Coverage mit Codecov-Integration
- Automatische Test-Artifacts

### Release Pipeline (`release.yml`)

**Trigger:** Tags (`v*.*.*`) und manuelle Ausführung

**Jobs:**

- **Build and Package**: Erstellt Windows ZIP und Linux DEB
- **Create Release**: GitHub Release mit automatischen Release Notes
- **Update winget**: Automatische winget-Manifest-Aktualisierung
- **Notify**: Discord-Benachrichtigung

**Features:**

- Automatische Version-Extraktion aus Tags
- Multi-Platform-Paketierung
- SHA256-Checksum-Generierung
- Automatische Release Notes aus Git-History

### Documentation Deployment (`deploy-docs.yml`)

**Trigger:** Änderungen in `HypnoScript.Dokumentation/` und manuelle Ausführung

**Jobs:**

- **Build Documentation**: Docusaurus-Build mit Validierung
- **Deploy**: GitHub Pages Deployment
- **Deploy Staging**: Netlify-Staging für PRs
- **Notify**: Deployment-Status-Benachrichtigung

**Features:**

- Automatische Builtin-Dokumentation-Generierung
- HTML-Validierung und Accessibility-Checks
- Broken Link Detection
- Staging-Umgebung für PRs

### Dependency Updates (`dependency-update.yml`)

**Trigger:** Wöchentlich (Montag 9 AM UTC) und manuelle Ausführung

**Jobs:**

- **Check Updates**: Prüft .NET und npm Dependencies
- **Create Update PRs**: Automatische PR-Erstellung
- **Security Alerts**: Erstellt Issues bei Security-Vulnerabilities
- **Notify**: Benachrichtigung über verfügbare Updates

**Features:**

- Intelligente Update-Strategien (Security, Minor, Major)
- Automatische PR-Erstellung mit Tests
- Security-Vulnerability-Detection
- Discord-Benachrichtigungen

### Performance Monitoring (`performance-monitoring.yml`)

**Trigger:** Täglich (2 AM UTC) und bei Runtime/Compiler-Änderungen

**Jobs:**

- **Benchmark**: Performance-Tests für verschiedene Test-Dateien
- **Memory Analysis**: Memory-Usage-Analyse
- **Load Testing**: Concurrent-Execution-Tests
- **Regression Detection**: Performance-Trend-Analyse
- **Performance Alerts**: Automatische Issues bei Regressions

**Features:**

- Matrix-Tests für verschiedene Test-Szenarien
- Memory-Leak-Detection
- Load-Testing mit 10 concurrent executions
- Performance-Trend-Analyse
- Automatische Regression-Detection

### Security Scanning (`security-scan.yml`)

**Trigger:** Täglich (6 AM UTC), Push/PR und manuelle Ausführung

**Jobs:**

- **SAST**: CodeQL und Semgrep-Analyse
- **Dependency Scan**: OWASP Dependency Check, Snyk, npm audit
- **Secret Scan**: TruffleHog und hardcoded secrets detection
- **Container Security**: Trivy (falls Dockerfile vorhanden)
- **Infrastructure Security**: Checkov
- **Security Report**: Zusammenfassender Security-Report
- **Security Alerts**: Automatische Issues bei Security-Problemen

**Features:**

- Umfassende SAST-Analyse
- Dependency-Vulnerability-Scanning
- Secret-Leak-Detection
- Container-Security-Scanning
- Infrastructure-as-Code-Security
- Automatische Security-Reports

## 🔧 Konfiguration

### Required Secrets

```yaml
# Discord Notifications
DISCORD_WEBHOOK: 'https://discord.com/api/webhooks/...'

# Security Scanning
SNYK_TOKEN: 'your-snyk-token'
SONAR_TOKEN: 'your-sonarcloud-token'

# Staging Deployment (optional)
NETLIFY_AUTH_TOKEN: 'your-netlify-token'
NETLIFY_SITE_ID: 'your-netlify-site-id'
```

### Environment Variables

Alle Workflows verwenden:

- `DOTNET_VERSION: '8.0.x'`
- `DOTNET_SKIP_FIRST_TIME_EXPERIENCE: 1`
- `DOTNET_CLI_TELEMETRY_OPTOUT: 1`

## 📊 Monitoring und Reporting

### Artifacts

Jeder Workflow erstellt spezifische Artifacts:

- **Build Artifacts**: Kompilierte Binaries
- **Test Results**: Test-Reports und Coverage
- **Security Reports**: Scan-Ergebnisse
- **Performance Data**: Benchmark-Ergebnisse
- **Documentation**: Gebaute Doku

### Notifications

- **Discord**: Erfolgs-/Fehler-Benachrichtigungen
- **GitHub Issues**: Automatische Issues bei Problemen
- **GitHub Releases**: Automatische Release-Erstellung

### Dashboards

- **Code Coverage**: Codecov-Integration
- **Security**: GitHub Security Tab
- **Performance**: Performance-Reports als Artifacts

## 🛠️ Wartung und Troubleshooting

### Häufige Probleme

1. **Build-Fehler**: Prüfe .NET-Version und Dependencies
2. **Test-Fehler**: Überprüfe Test-Dateien und Assertions
3. **Security-Alerts**: Review und fixe Security-Vulnerabilities
4. **Performance-Regressions**: Analysiere Benchmark-Ergebnisse

### Debugging

- Alle Workflows haben detaillierte Logs
- Artifacts enthalten Debug-Informationen
- Summary-Jobs zeigen Job-Status

### Updates

- Workflows verwenden neueste Action-Versionen
- Dependency-Updates werden automatisch erstellt
- Security-Scans laufen täglich

## 📈 Metriken und KPIs

### Build-Metriken

- Build-Zeit: < 10 Minuten
- Test-Coverage: > 80%
- Success Rate: > 95%

### Security-Metriken

- Critical Vulnerabilities: 0
- Security Score: > 90%
- Dependency Updates: Wöchentlich

### Performance-Metriken

- Benchmark-Zeit: < 5 Sekunden
- Memory Usage: < 100MB
- Load Test Success: 100%

## 🔮 Zukünftige Erweiterungen

- **Kubernetes-Deployment**: Container-Deployment-Pipeline
- **Mobile Testing**: iOS/Android-Test-Integration
- **AI/ML Integration**: Automatische Code-Review
- **Advanced Monitoring**: Prometheus/Grafana-Integration
- **Compliance**: SOC2, GDPR-Compliance-Checks

---

_Diese Dokumentation wird automatisch mit den Workflows synchronisiert._
