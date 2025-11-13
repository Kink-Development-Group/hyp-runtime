---
sidebar_position: 1
---

# Willkommen bei HypnoScript

HypnoScript ist eine moderne, esoterische Programmiersprache, die hypnotische Metaphern mit einer pragmatischen Rust-Toolchain verbindet. Die Syntax erinnert an TypeScript, nutzt aber Schlüsselwörter wie `Focus`, `induce`, `observe` oder `Relax`, um hypnotische Konzepte direkt auszudrücken.

## Was ist HypnoScript?

Die aktuelle Runtime besteht vollständig aus Rust-Crates und liefert:

- 🦀 **Native Toolchain** – Lexer, Parser, statischer Type Checker, Interpreter und WASM-Codegenerator sind vollständig in Rust implementiert.
- 🎯 **Hypnotische Syntax** – Sprachkonstrukte wie `deepFocus`, `snap`, `anchor` oder `oscillate` übersetzen hypnotische Bilder in Code.
- 🔒 **Statisches Typ-System** – Der Type Checker kennt Zahlen, Strings, Booleans, Arrays, Funktionen und Sessions inklusive Sichtbarkeiten.
- 📦 **Standardbibliothek** – Mathe-, String-, Array-, Datei-, Statistik-, System-, Zeit- und Validierungs-Builtins stehen direkt bereit.
- 🛠️ **CLI für den gesamten Workflow** – Ein einzelnes Binary (`hypnoscript`) bietet `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins` und `version`.

Die Sprache ist plattformübergreifend (Windows/macOS/Linux) und erzeugt native Binaries sowie optional WebAssembly-Ausgabe.

## Grundelemente der Syntax

| Konzept                | Beschreibung                                                                                             |
| ---------------------- | -------------------------------------------------------------------------------------------------------- |
| `Focus { ... } Relax`  | Umschließt jedes Programm (Entry- und Exit-Punkt).                                                       |
| `entrance { ... }`     | Optionaler Startblock für Initialisierung oder Begrüßung.                                                |
| `finale { ... }`       | Optionaler Cleanup-Block, der am Ende garantiert ausgeführt wird.                                        |
| `induce` / `freeze`    | Deklariert Variablen (`induce`/`implant`) oder Konstanten (`freeze`).                                    |
| `observe` / `whisper`  | Ausgabe mit bzw. ohne Zeilenumbruch. `command` hebt Text emphatisch hervor.                              |
| `if`, `while`, `loop`  | Kontrollstrukturen mit hypnotischen Operator-Synonymen (`youAreFeelingVerySleepy`, `underMyControl`, …). |
| `suggestion`           | Funktionsdefinition (global oder innerhalb von Sessions).                                                |
| `session`              | Objektorientierte Strukturen mit Feldern (`expose`/`conceal`), Konstruktoren und statischen Mitgliedern. |
| `anchor` / `oscillate` | Speichert Werte zwischen oder toggelt Booleans.                                                          |

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

Die Builtins sind in Kategorien organisiert. Eine detaillierte Referenz findest du unter [Standardbibliothek](./builtins/overview).

- **Mathematik** – `Sin`, `Cos`, `Tan`, `Sqrt`, `Pow`, `Factorial`, `Clamp`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, …
- **Strings** – `Length`, `ToUpper`, `ToLower`, `Trim`, `Reverse`, `Replace`, `Split`, `Substring`, `PadLeft`, `IsWhitespace`, …
- **Arrays** – `ArrayLength`, `ArrayIsEmpty`, `ArraySum`, `ArrayAverage`, `ArraySlice`, `ArrayDistinct`, …
- **Dateien** – `ReadFile`, `WriteFile`, `AppendFile`, `ListDirectory`, `GetFileExtension`, …
- **System** – `GetOperatingSystem`, `GetUsername`, `GetArgs`, `Exit`, `GetCurrentDirectory`, …
- **Zeit & Datum** – `CurrentTimestamp`, `CurrentDateTime`, `IsLeapYear`, `DayOfWeek`, …
- **Statistik** – `Mean`, `Median`, `Mode`, `StandardDeviation`, `Correlation`, `LinearRegression`, …
- **Validierung** – `IsValidEmail`, `MatchesPattern`, `IsInRange`, `IsNumeric`, `IsLowercase`, …
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
