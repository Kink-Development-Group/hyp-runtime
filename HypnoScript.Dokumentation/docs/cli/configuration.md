---
sidebar_position: 3
---

# CLI-Konfiguration

Die HypnoScript CLI kann über Konfigurationsdateien, Umgebungsvariablen und Kommandozeilenoptionen konfiguriert werden.

## Konfigurationsdatei

Die Hauptkonfigurationsdatei ist `hypnoscript.config.json` im Projektverzeichnis.

### Grundlegende Konfiguration

```json
{
  "defaultOutput": "console",
  "enableDebug": false,
  "logLevel": "info",
  "timeout": 30000,
  "maxMemory": 512,
  "testFramework": {
    "autoRun": true,
    "reportFormat": "detailed"
  },
  "server": {
    "port": 8080,
    "host": "localhost"
  },
  "formatting": {
    "indentSize": 2,
    "maxLineLength": 80
  },
  "linting": {
    "rules": ["style", "performance", "security"],
    "severity": "warning"
  }
}
```

### Erweiterte Konfiguration

```json
{
  "defaultOutput": "console",
  "enableDebug": false,
  "logLevel": "info",
  "timeout": 30000,
  "maxMemory": 512,
  "testFramework": {
    "autoRun": true,
    "reportFormat": "detailed",
    "parallelExecution": true,
    "coverage": {
      "enabled": true,
      "threshold": 80
    }
  },
  "server": {
    "port": 8080,
    "host": "localhost",
    "ssl": {
      "enabled": false,
      "certPath": "",
      "keyPath": ""
    },
    "cors": {
      "enabled": true,
      "origins": ["*"]
    }
  },
  "formatting": {
    "indentSize": 2,
    "maxLineLength": 80,
    "useTabs": false,
    "trimTrailingWhitespace": true,
    "insertFinalNewline": true
  },
  "linting": {
    "rules": ["style", "performance", "security"],
    "severity": "warning",
    "ignorePatterns": ["node_modules/**", "dist/**"],
    "customRules": []
  },
  "compilation": {
    "target": "il",
    "optimization": {
      "enabled": true,
      "level": "standard"
    },
    "debug": {
      "enabled": false,
      "symbols": true
    }
  },
  "packaging": {
    "includeDependencies": true,
    "runtime": "win-x64",
    "compression": true
  },
  "monitoring": {
    "metrics": {
      "enabled": true,
      "interval": 5000
    },
    "profiling": {
      "enabled": false,
      "output": "profile.json"
    }
  }
}
```

## Konfigurationsoptionen

### Allgemeine Einstellungen

| Option          | Typ     | Standard  | Beschreibung                         |
| --------------- | ------- | --------- | ------------------------------------ |
| `defaultOutput` | string  | "console" | Standard-Ausgabekanal                |
| `enableDebug`   | boolean | false     | Debug-Modus aktivieren               |
| `logLevel`      | string  | "info"    | Log-Level (debug, info, warn, error) |
| `timeout`       | number  | 30000     | Timeout in Millisekunden             |
| `maxMemory`     | number  | 512       | Maximaler Speicherverbrauch in MB    |

### Test-Framework

| Option                             | Typ     | Standard   | Beschreibung                |
| ---------------------------------- | ------- | ---------- | --------------------------- |
| `testFramework.autoRun`            | boolean | true       | Tests automatisch ausführen |
| `testFramework.reportFormat`       | string  | "detailed" | Test-Report-Format          |
| `testFramework.parallelExecution`  | boolean | true       | Parallele Test-Ausführung   |
| `testFramework.coverage.enabled`   | boolean | false      | Code-Coverage aktivieren    |
| `testFramework.coverage.threshold` | number  | 80         | Mindest-Coverage in Prozent |

### Server-Konfiguration

| Option                | Typ     | Standard    | Beschreibung          |
| --------------------- | ------- | ----------- | --------------------- |
| `server.port`         | number  | 8080        | Server-Port           |
| `server.host`         | string  | "localhost" | Server-Host           |
| `server.ssl.enabled`  | boolean | false       | SSL aktivieren        |
| `server.ssl.certPath` | string  | ""          | SSL-Zertifikatspfad   |
| `server.ssl.keyPath`  | string  | ""          | SSL-Schlüsselpfad     |
| `server.cors.enabled` | boolean | true        | CORS aktivieren       |
| `server.cors.origins` | array   | ["*"]       | Erlaubte CORS-Origins |

### Formatierung

| Option                              | Typ     | Standard | Beschreibung                  |
| ----------------------------------- | ------- | -------- | ----------------------------- |
| `formatting.indentSize`             | number  | 2        | Einrückungsgröße              |
| `formatting.maxLineLength`          | number  | 80       | Maximale Zeilenlänge          |
| `formatting.useTabs`                | boolean | false    | Tabs statt Leerzeichen        |
| `formatting.trimTrailingWhitespace` | boolean | true     | Trailing Whitespace entfernen |
| `formatting.insertFinalNewline`     | boolean | true     | Finale Newline einfügen       |

### Linting

| Option                   | Typ    | Standard                             | Beschreibung              |
| ------------------------ | ------ | ------------------------------------ | ------------------------- |
| `linting.rules`          | array  | ["style", "performance", "security"] | Lint-Regeln               |
| `linting.severity`       | string | "warning"                            | Mindest-Schweregrad       |
| `linting.ignorePatterns` | array  | []                                   | Zu ignorierende Dateien   |
| `linting.customRules`    | array  | []                                   | Benutzerdefinierte Regeln |

### Kompilierung

| Option                             | Typ     | Standard   | Beschreibung                 |
| ---------------------------------- | ------- | ---------- | ---------------------------- |
| `compilation.target`               | string  | "il"       | Kompilierungsziel (il, wasm) |
| `compilation.optimization.enabled` | boolean | true       | Optimierungen aktivieren     |
| `compilation.optimization.level`   | string  | "standard" | Optimierungslevel            |
| `compilation.debug.enabled`        | boolean | false      | Debug-Informationen          |
| `compilation.debug.symbols`        | boolean | true       | Debug-Symbole                |

### Packaging

| Option                          | Typ     | Standard  | Beschreibung                |
| ------------------------------- | ------- | --------- | --------------------------- |
| `packaging.includeDependencies` | boolean | true      | Abhängigkeiten einschließen |
| `packaging.runtime`             | string  | "win-x64" | Ziel-Runtime                |
| `packaging.compression`         | boolean | true      | Kompression aktivieren      |

### Monitoring

| Option                         | Typ     | Standard       | Beschreibung           |
| ------------------------------ | ------- | -------------- | ---------------------- |
| `monitoring.metrics.enabled`   | boolean | true           | Metriken aktivieren    |
| `monitoring.metrics.interval`  | number  | 5000           | Metrik-Intervall in ms |
| `monitoring.profiling.enabled` | boolean | false          | Profiling aktivieren   |
| `monitoring.profiling.output`  | string  | "profile.json" | Profiling-Ausgabedatei |

## Umgebungsvariablen

### HypnoScript-spezifische Variablen

| Variable                 | Beschreibung             | Standard                  |
| ------------------------ | ------------------------ | ------------------------- |
| `HYPNOSCRIPT_HOME`       | Installationsverzeichnis | -                         |
| `HYPNOSCRIPT_LOG_LEVEL`  | Log-Level                | "info"                    |
| `HYPNOSCRIPT_CONFIG`     | Konfigurationsdatei      | "hypnoscript.config.json" |
| `HYPNOSCRIPT_TIMEOUT`    | Standard-Timeout         | "30000"                   |
| `HYPNOSCRIPT_MAX_MEMORY` | Maximaler Speicher       | "512"                     |

### Plattform-spezifische Variablen

| Variable                  | Beschreibung        |
| ------------------------- | ------------------- |
| `HYPNOSCRIPT_SERVER_PORT` | Server-Port         |
| `HYPNOSCRIPT_SERVER_HOST` | Server-Host         |
| `HYPNOSCRIPT_SSL_CERT`    | SSL-Zertifikatspfad |
| `HYPNOSCRIPT_SSL_KEY`     | SSL-Schlüsselpfad   |

### Beispiel für Umgebungsvariablen

```bash
# Linux/macOS
export HYPNOSCRIPT_HOME="/opt/hypnoscript"
export HYPNOSCRIPT_LOG_LEVEL="debug"
export HYPNOSCRIPT_CONFIG="./config.json"
export HYPNOSCRIPT_TIMEOUT="60000"
export HYPNOSCRIPT_MAX_MEMORY="1024"

# Windows (PowerShell)
$env:HYPNOSCRIPT_HOME = "C:\Program Files\HypnoScript"
$env:HYPNOSCRIPT_LOG_LEVEL = "debug"
$env:HYPNOSCRIPT_CONFIG = ".\config.json"
$env:HYPNOSCRIPT_TIMEOUT = "60000"
$env:HYPNOSCRIPT_MAX_MEMORY = "1024"

# Windows (CMD)
set HYPNOSCRIPT_HOME=C:\Program Files\HypnoScript
set HYPNOSCRIPT_LOG_LEVEL=debug
set HYPNOSCRIPT_CONFIG=.\config.json
set HYPNOSCRIPT_TIMEOUT=60000
set HYPNOSCRIPT_MAX_MEMORY=1024
```

## Konfigurationshierarchie

Die CLI verwendet eine Hierarchie für Konfigurationswerte:

1. **Kommandozeilenoptionen** (höchste Priorität)
2. **Umgebungsvariablen**
3. **Projekt-Konfigurationsdatei** (`hypnoscript.config.json`)
4. **Benutzer-Konfigurationsdatei** (`~/.hypnoscript/config.json`)
5. **System-Konfigurationsdatei** (`/etc/hypnoscript/config.json`)
6. **Standardwerte** (niedrigste Priorität)

### Beispiel für Konfigurationshierarchie

```bash
# 1. Kommandozeilenoption überschreibt alles
dotnet run --project HypnoScript.CLI -- run script.hyp --timeout 120

# 2. Umgebungsvariable überschreibt Konfigurationsdatei
export HYPNOSCRIPT_TIMEOUT=60
dotnet run --project HypnoScript.CLI -- run script.hyp

# 3. Projekt-Konfigurationsdatei
# hypnoscript.config.json: { "timeout": 30000 }

# 4. Benutzer-Konfigurationsdatei
# ~/.hypnoscript/config.json: { "timeout": 60000 }

# 5. System-Konfigurationsdatei
# /etc/hypnoscript/config.json: { "timeout": 300000 }
```

## Profilbasierte Konfiguration

Sie können verschiedene Konfigurationsprofile für unterschiedliche Umgebungen erstellen:

### Profil-Konfiguration

```json
{
  "profiles": {
    "development": {
      "logLevel": "debug",
      "enableDebug": true,
      "timeout": 60000,
      "testFramework": {
        "autoRun": true,
        "reportFormat": "detailed"
      }
    },
    "production": {
      "logLevel": "warn",
      "enableDebug": false,
      "timeout": 30000,
      "testFramework": {
        "autoRun": false,
        "reportFormat": "summary"
      },
      "compilation": {
        "optimization": {
          "enabled": true,
          "level": "aggressive"
        }
      }
    },
    "testing": {
      "logLevel": "info",
      "testFramework": {
        "autoRun": true,
        "coverage": {
          "enabled": true,
          "threshold": 90
        }
      }
    }
  }
}
```

### Profil verwenden

```bash
# Profil über Umgebungsvariable
export HYPNOSCRIPT_PROFILE=production
dotnet run --project HypnoScript.CLI -- run script.hyp

# Profil über Kommandozeile
dotnet run --project HypnoScript.CLI -- run script.hyp --profile production
```

## Erweiterte Konfigurationsszenarien

### Multi-Environment Setup

```json
{
  "environments": {
    "local": {
      "server": {
        "port": 3000,
        "host": "localhost"
      },
      "database": {
        "connectionString": "localhost:5432"
      }
    },
    "staging": {
      "server": {
        "port": 8080,
        "host": "staging.example.com"
      },
      "database": {
        "connectionString": "staging-db:5432"
      }
    },
    "production": {
      "server": {
        "port": 443,
        "host": "app.example.com",
        "ssl": {
          "enabled": true
        }
      },
      "database": {
        "connectionString": "prod-db:5432"
      }
    }
  }
}
```

### Team-Konfiguration

```json
{
  "team": {
    "codeStyle": {
      "formatting": {
        "indentSize": 2,
        "maxLineLength": 100
      },
      "linting": {
        "rules": ["style", "performance", "security"],
        "severity": "error"
      }
    },
    "testing": {
      "coverage": {
        "enabled": true,
        "threshold": 85
      },
      "parallelExecution": true
    },
    "ci": {
      "autoFormat": true,
      "autoLint": true,
      "requireTests": true
    }
  }
}
```

## Best Practices

### Konfigurationsdatei organisieren

```bash
project/
├── config/
│   ├── hypnoscript.config.json      # Hauptkonfiguration
│   ├── development.config.json      # Entwicklung
│   ├── staging.config.json          # Staging
│   └── production.config.json       # Produktion
├── scripts/
│   ├── setup-dev.sh                 # Entwicklung einrichten
│   └── setup-prod.sh                # Produktion einrichten
└── .env.example                     # Umgebungsvariablen-Beispiel
```

### Sichere Konfiguration

```json
{
  "security": {
    "secrets": {
      "useEnvVars": true,
      "envPrefix": "HYPNOSCRIPT_"
    },
    "ssl": {
      "enabled": true,
      "certPath": "${SSL_CERT_PATH}",
      "keyPath": "${SSL_KEY_PATH}"
    }
  }
}
```

### Performance-Optimierung

```json
{
  "performance": {
    "compilation": {
      "optimization": {
        "enabled": true,
        "level": "aggressive"
      },
      "parallel": true
    },
    "runtime": {
      "gc": {
        "enabled": true,
        "interval": 1000
      }
    }
  }
}
```

## Troubleshooting

### Häufige Konfigurationsprobleme

1. **Konfigurationsdatei wird nicht gefunden**

   ```bash
   # Prüfen Sie den Pfad
   ls -la hypnoscript.config.json

   # Verwenden Sie absolute Pfade
   export HYPNOSCRIPT_CONFIG="/absolute/path/config.json"
   ```

2. **Umgebungsvariablen werden nicht erkannt**

   ```bash
   # Prüfen Sie die Variablen
   echo $HYPNOSCRIPT_LOG_LEVEL

   # Starten Sie die Shell neu
   source ~/.bashrc
   ```

3. **Konflikte zwischen Profilen**

   ```bash
   # Profil explizit setzen
   export HYPNOSCRIPT_PROFILE=development

   # Profil über Kommandozeile
   dotnet run --project HypnoScript.CLI -- run script.hyp --profile development
   ```

## Nächste Schritte

- [Testing](../testing/overview) - Test-Framework-Konfiguration
- [Debugging](../debugging/tools) - Debugging-Tools
- [Runtime-Features](../enterprise/features) - Runtime-Konfiguration

---

**Konfiguration gemeistert? Dann lerne das [Test-Framework](../testing/overview) kennen!** 🧪
