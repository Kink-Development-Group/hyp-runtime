---
sidebar_position: 1
---

# Installation

Dieser Leitfaden führt dich durch die Installation der Rust-basierten HypnoScript-Toolchain.

## Voraussetzungen

| Komponente      | Empfehlung                                                                 |
| --------------- | -------------------------------------------------------------------------- |
| Betriebssystem  | Windows 10+, macOS 12+, Linux (Ubuntu 20.04+, Fedora 38+, Arch)            |
| Rust Toolchain  | `rustup` mit Rust 1.76 oder neuer (`rustup --version` zur Kontrolle)       |
| Build-Werkzeuge | Git, C/C++ Build-Tools (werden von `rustup` / Paketmanager bereitgestellt) |

Optional für die Dokumentation: Node.js 18+.

### Rust installieren

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Nach der Installation ein neues Terminal öffnen und prüfen
rustc --version
cargo --version
```

Unter Windows empfiehlt sich alternativ der [rustup-init.exe Download](https://win.rustup.rs/).

## HypnoScript aus dem Repository bauen (empfohlen)

```bash
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# Release-Build der CLI erzeugen
cargo build -p hypnoscript-cli --release

# Optional global installieren (legt hypnoscript ins Cargo-Bin-Verzeichnis)
cargo install --path hypnoscript-cli
```

Die fertig gebaute CLI liegt anschließend unter `./target/release/hypnoscript` bzw. nach der Installation im Cargo-Bin-Verzeichnis (`~/.cargo/bin` bzw. `%USERPROFILE%\.cargo\bin`).

## Vorbereitete Release-Pakete verwenden

Wenn du nicht selbst bauen möchtest, findest du unter [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases) signierte Artefakte für Windows, macOS und Linux. Nach dem Entpacken kannst du die enthaltene Binärdatei direkt ausführen.

## Installation prüfen

```bash
# Version und verfügbare Befehle anzeigen
hypnoscript version
hypnoscript builtins

# Minimales Testprogramm
echo 'Focus { entrance { observe "Installation erfolgreich!"; } } Relax' > test.hyp
hypnoscript run test.hyp
```

Erwartete Ausgabe (gekürzt):

```text
HypnoScript v1.0.0 (Rust Edition)
Installation erfolgreich!
```

## Häufige Probleme

| Problem                   | Lösung                                                                                              |
| ------------------------- | --------------------------------------------------------------------------------------------------- |
| `cargo` nicht gefunden    | Prüfe, ob `~/.cargo/bin` (Linux/macOS) bzw. `%USERPROFILE%\.cargo\bin` (Windows) im `PATH` liegt.   |
| Linker-Fehler unter Linux | Installiere Build-Abhängigkeiten (`sudo apt install build-essential` oder Distribution-Äquivalent). |
| Keine Ausführungsrechte   | Setze `chmod +x hypnoscript` nach dem Entpacken eines Release-Artefakts.                            |

## Optional: Entwicklungskomfort

- **VS Code**: Installiere die Extensions _Rust Analyzer_ und _Even Better TOML_. Das Repo enthält eine `hyp-runtime.code-workspace`-Datei.
- **Shell Alias**: `alias hyp="hypnoscript"` für kürzere Befehle.
- **Dokumentation bauen**: `npm install` & `npm run dev` im Ordner `hypnoscript-docs`.

## Nächste Schritte

- [Quick Start](./quick-start)
- [CLI Basics](./cli-basics)
- [Sprachreferenz](../language-reference/syntax)
- [Standardbibliothek](../builtins/overview)

Viel Spaß beim hypnotischen Coden! 🌀
