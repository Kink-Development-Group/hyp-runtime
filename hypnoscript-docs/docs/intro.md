---
sidebar_position: 1
---

# Willkommen bei HypnoScript

HypnoScript ist eine moderne, esoterische Programmiersprache, die hypnotische Metaphern mit einer pragmatischen, Rust-basierten Toolchain verbindet. Die Sprache orientiert sich syntaktisch an TypeScript/JavaScript, ersetzt klassische Schlüsselwörter aber durch hypnotische Begriffe wie `Focus`, `induce`, `observe` oder `Relax`.

## Was ist HypnoScript?

Die aktuelle Runtime besteht vollständig aus Rust-Crates und liefert:

- 🦀 **Native Toolchain** – Lexer, Parser, statischer Type Checker, Interpreter und WASM-Codegenerator sind vollständig in Rust umgesetzt.
- 🎯 **Hypnotische Syntax** – Sprachkonstrukte wie `deepFocus`, `snap`, `anchor` oder `oscillate` transportieren hypnotische Bilder.
- 🔒 **Statisches Typ-System** – Der Type Checker kennt Zahlen, Strings, Booleans, Arrays, Sessions, Funktionen sowie `tranceify`-Records.
- 📦 **Standardbibliothek** – Über 110 Builtins für Mathematik, Strings, Arrays, Dateien, Statistik, Systeminformationen, Zeit & Datum sowie Validierung.
- 🛠️ **CLI für den gesamten Workflow** – Ein einzelnes Binary (`hypnoscript`) bietet `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins` und `version`.

Die Sprache ist cross-platform (Windows/macOS/Linux) und erzeugt native Binaries oder optional WebAssembly-Ausgabe.

## Grundelemente der Syntax

| Konzept                | Beschreibung                                                                                             |
| ---------------------- | -------------------------------------------------------------------------------------------------------- |
| `Focus { ... } Relax`  | Umschließt jedes Programm (Entry- und Exit-Punkt).                                                       |
| `entrance { ... }`     | Optionaler Startblock für Initialisierung oder Begrüßung.                                                |
| `finale { ... }`       | Optionaler Cleanup-Block, der am Ende garantiert ausgeführt wird.                                        |
| `induce` / `freeze`    | Deklariert Variablen (`induce`) oder Konstanten (`freeze`).                                              |
| `observe` / `whisper`  | Ausgabe mit bzw. ohne Zeilenumbruch. `command` hebt Text emphatisch hervor.                              |
| `if`, `while`, `loop`  | Kontrollstrukturen mit hypnotischen Operator-Synonymen (`youAreFeelingVerySleepy`, `underMyControl`, …). |
| `suggestion`           | Funktionsdefinition (global oder innerhalb von Sessions).                                                |
| `session`              | Objektorientierte Strukturen mit Feldern (`expose`/`conceal`) und Methoden.                              |
| `tranceify`            | Deklariert Record-Typen mit festen Feldern.                                                              |
| `anchor` / `oscillate` | Speichert Zustände oder toggelt Booleans.                                                                |

```hyp
Focus {
    entrance {
        observe "Willkommen in der Trance";
    }

    induce counter: number = 0;
    while (counter goingDeeper 3) {
        observe "Tiefe: " + counter;
        counter = counter + 1;
    }

    suggestion hypnoticSum(values: number[]): number {
        awaken ArraySum(values);
    }

    observe "Summe: " + ToString(hypnoticSum([2, 4, 6]));
} Relax
```

## Standardbibliothek im Überblick

Die Builtins sind in Modulen organisiert. Eine detaillierte Referenz findest du unter [Standardbibliothek](./builtins/overview).

- **Mathematik** – `Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`, `Factorial`, `Clamp`, …
- **Strings** – `Length`, `ToUpper`, `Trim`, `Replace`, `Split`, `PadLeft`, `IsWhitespace`, …
- **Arrays** – `ArrayLength`, `ArrayIsEmpty`, `ArraySum`, `ArraySort`, `ArrayDistinct`, …
- **Dateien** – `ReadFile`, `WriteFile`, `ListDirectory`, `GetFileExtension`, …
- **System** – `GetOperatingSystem`, `GetUsername`, `GetArgs`, `Exit`, …
- **Zeit & Datum** – `CurrentTimestamp`, `FormatDateTime`, `IsLeapYear`, …
- **Statistik** – `Mean`, `Median`, `StandardDeviation`, `Correlation`, …
- **Validierung** – `IsValidEmail`, `MatchesPattern`, `IsInRange`, …
- **Hypnotische Kernfunktionen** – `Observe`, `Whisper`, `Command`, `Drift`, `DeepTrance`, `HypnoticCountdown`, `TranceInduction`, `HypnoticVisualization`.

## Entwicklungs-Workflow

```bash
# Quelle lesen, lexen, parsen, checken und ausführen
hypnoscript lex   examples/test.hyp
hypnoscript parse examples/test.hyp
hypnoscript check examples/test.hyp
hypnoscript run   examples/test.hyp

# Zu WebAssembly (wat) generieren
hypnoscript compile-wasm examples/test.hyp --output output.wat

# Listing aller Builtins
hypnoscript builtins
```

Der Interpreter führt Programme deterministisch aus. Typprüfungsfehler werden gemeldet, blockieren die Ausführung aber nicht – ideal für exploratives Arbeiten.

## Nächste Schritte

- [Installation](./getting-started/installation)
- [Quick Start](./getting-started/quick-start)
- [Grundkonzepte](./getting-started/core-concepts)
- [Sprachreferenz](./language-reference/syntax)
- [Standardbibliothek](./builtins/overview)

## Community & Lizenz

- GitHub: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- Issues & Roadmap: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- Lizenz: [MIT](https://opensource.org/license/mit/)

Tauche ein, hypnotisiere deinen Code und genieße eine Sprache, die humorvollen Flair mit ernstzunehmender Infrastruktur verbindet. 🧠✨
