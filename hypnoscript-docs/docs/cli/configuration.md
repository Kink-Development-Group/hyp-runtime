---
sidebar_position: 3
---

# CLI-Konfiguration

Die Rust-basierte HypnoScript CLI verzichtet bewusst auf globale Konfigurationsdateien. Stattdessen steuerst du das Verhalten ausschließlich über Subcommands und deren Flags. Dieser Leitfaden zeigt, welche Schalter es gibt und wie du sie mit Shell-Skripten oder Tooling automatisieren kannst.

## Laufzeit-Flags der CLI

| Subcommand                          | Optionen               | Wirkung                                                                   |
| ----------------------------------- | ---------------------- | ------------------------------------------------------------------------- |
| `run <file>`                        | `--debug`, `--verbose` | Debug zeigt Tokens, AST und Type Checks, verbose gibt Statusmeldungen aus |
| `compile-wasm`                      | `--output <file>`      | Wählt den Namen der `.wat`-Datei (Standard: `<input>.wat`)                |
| `version`                           | _(keine)_              | Gibt Toolchain-Informationen aus                                          |
| `lex`, `parse`, `check`, `builtins` | _(keine)_              | Nutzen keine Zusatzoptionen                                               |

Mehr Flags existieren aktuell nicht. Das macht die CLI zwar simpel, aber auch sehr vorhersehbar – gerade für Skripte und CI.

## Eigene Wrapper erstellen

Wenn du häufig dieselben Optionen verwenden möchtest, lohnt sich ein kleines Wrapper-Skript.

### PowerShell (Windows)

```powershell
function Invoke-HypnoScriptRun {
    param(
        [Parameter(Mandatory=$true)]
        [string]$File,
        [switch]$Debug,
        [switch]$Verbose
    )

    $args = @('run', $File)
    if ($Debug) { $args += '--debug' }
    if ($Verbose) { $args += '--verbose' }
    hypnoscript @args
}

# Nutzung
Invoke-HypnoScriptRun -File 'scripts/demo.hyp' -Verbose
```

### Bash / Zsh (macOS, Linux)

```bash
hyp() {
  local mode="$1"; shift
  case "$mode" in
    run)
      hypnoscript run "$@" --verbose ;;
    check)
      hypnoscript check "$@" ;;
    *)
      hypnoscript "$mode" "$@" ;;
  esac
}

# Beispiel
hyp run scripts/demo.hyp
```

Solche Wrapper kannst du versionskontrolliert im Projekt ablegen (`scripts/`).

## Projektbezogene Workflows

Auch ohne Konfigurationsdatei kannst du Abläufe bündeln:

- **`package.json` / npm scripts:** `"check": "hypnoscript check src/**/*.hyp"`
- **Makefile:** `check: ; hypnoscript check $(FILE)`
- **CI-Pipeline:** Verwende die `run`, `check` und `compile-wasm` Befehle direkt in deinen Jobs.

Damit dokumentierst du, wie das Projekt gebaut oder geprüft werden soll – ohne eigene CLI-Config.

## Umgebungsvariablen

Die CLI liest derzeit keine speziellen `HYPNOSCRIPT_*` Variablen ein. Du kannst trotzdem Umgebungsvariablen nutzen, um Dateipfade oder Flags zu steuern:

```bash
export HYPNO_DEFAULT=examples/intro.hyp
hypnoscript run "$HYPNO_DEFAULT"
```

Oder in PowerShell:

```powershell
$env:DEFAULT_HYP = 'examples/intro.hyp'
hypnoscript run $env:DEFAULT_HYP --debug
```

Solche Variablen sind rein konventionell – die CLI greift nicht automatisch darauf zu.

## Empfehlungen

- **Dokumentiere Wrapper:** Lege ein README im `scripts/`-Ordner an, damit andere den Workflow nachvollziehen können.
- **Nutze `--debug` sparsam:** In CI-Pipelines reicht oft `--verbose`. Debug-Ausgaben können riesig werden.
- **Version pinnen:** Referenziere in Skripten eine konkrete Version (`hypnoscript version`) oder lege den Binary als Artefakt ab, um reproduzierbare Builds zu erhalten.

## Troubleshooting

1. **`hypnoscript` wird nicht gefunden**

```bash
# Prüfe, ob der Binary im PATH liegt
which hypnoscript    # macOS/Linux
Get-Command hypnoscript | Select-Object Source  # PowerShell

# Falls nicht vorhanden: Pfad ergänzen
export PATH="$PATH:$HOME/.cargo/bin"          # Beispiel Linux
```

1. **Keine Ausführungsrechte**

```bash
chmod +x hypnoscript            # macOS/Linux
Set-ExecutionPolicy RemoteSigned # Windows PowerShell (falls nötig)
```

1. **Unerwartete Ausgaben / Syntaxfehler**

```bash
# Mit Debug-Infos erneut ausführen
hypnoscript run script.hyp --debug

# Tokens prüfen
hypnoscript lex script.hyp
```

## Nächste Schritte

- [CLI Übersicht](./overview) – Installationswege & Workflow
- [CLI-Befehle](./commands) – Vollständige Referenz der Subcommands
- [CLI Basics](../getting-started/cli-basics) – Alltagstaugliche Beispiele

---

**Tipp:** Baue eigene Wrapper in `scripts/`, um wiederkehrende Aufrufe zu vereinfachen.
