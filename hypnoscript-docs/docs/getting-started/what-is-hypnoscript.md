# Was ist HypnoScript?

HypnoScript ist eine statisch typisierte Skriptsprache mit hypnotischer Syntax. Statt `class`, `function` oder `print` findest du Begriffe wie `session`, `suggestion` und `observe`. Die Rust-basierte Implementierung liefert Lexer, Parser, Type Checker, Interpreter und einen WASM-Codegenerator in einem kompakten Toolchain-Bundle.

## Designprinzipien

- **Lesbarkeit vor allem** – Hypnotische Schlüsselwörter sollen Spaß machen, ohne die Verständlichkeit zu verlieren.
- **Statische Sicherheit** – Der Type Checker validiert Variablen, Funktionssignaturen, Rückgabewerte und Session-Mitglieder.
- **Deterministische Ausführung** – Der Interpreter führt Programme reproduzierbar aus und meldet Typfehler, bricht aber nicht zwangsläufig ab.
- **Ein Binary, alle Schritte** – Die CLI deckt Lexing, Parsing, Type Checking, Ausführung und optionales WASM-Target ab.

## Sprache auf einen Blick

| Element                           | Beschreibung                                                                                        |
| --------------------------------- | --------------------------------------------------------------------------------------------------- |
| `Focus { ... } Relax`             | Umschließt jedes Programm. `Relax` markiert das Ende und ist obligatorisch.                         |
| `entrance { ... }`                | Optionaler Startblock für Initialisierung, Begrüßung oder Setup.                                    |
| `finale { ... }`                  | Optionaler Cleanup-Block, der vor `Relax` ausgeführt wird.                                          |
| `induce` / `implant`              | Deklariert veränderbare Variablen mit optionalem Typ.                                               |
| `freeze`                          | Deklariert Konstanten.                                                                              |
| `observe` / `whisper` / `command` | Ausgabe mit Zeilenumbruch, ohne Zeilenumbruch bzw. fett/imperativ.                                  |
| `suggestion`                      | Definiert Funktionen; `awaken` (oder `return`) gibt Werte zurück.                                   |
| `session`                         | Objektorientierte Strukturen mit `expose` (öffentlich), `conceal` (privat) und `dominant` (static). |
| `anchor`                          | Speichert den aktuellen Wert eines Ausdrucks für später.                                            |
| `oscillate`                       | Toggle für boolesche Variablen.                                                                     |
| `deepFocus`                       | Optionaler Zusatz hinter `if (...)` für etwas dramatischere Bedingungsblöcke.                       |

## Beispielprogramm

```hyp
Focus {
    entrance {
        observe "Willkommen bei HypnoScript";
    }

    freeze MAX_DEPTH: number = 3;
    induce depth: number = 0;

    while (depth goingDeeper MAX_DEPTH) {
        observe "Tiefe: " + depth;
        depth = depth + 1;
    }

    suggestion introduce(name: string): string {
        awaken "Hallo, " + name + "!";
    }

    observe introduce("Hypnotisierte Person");

    session Subject {
        expose name: string;
        conceal level: number;

        suggestion constructor(name: string) {
            this.name = name;
            this.level = 0;
        }

        expose suggestion deepen() {
            this.level = this.level + 1;
            observe this.name + " geht tiefer: " + this.level;
        }
    }

    induce alice: Subject = Subject("Alice");
    alice.deepen();
} Relax
```

## Plattform-Komponenten

- **Lexer & Parser** – Liefern Token-Streams und ASTs, inkl. hypnotischer Operator-Synonyme (`youAreFeelingVerySleepy`, `underMyControl`, …).
- **Type Checker** – Registriert alle Builtins, prüft Funktions- und Sessionsignaturen, Sichtbarkeiten und Konversionen.
- **Interpreter** – Führt AST-Knoten aus, verwaltet Sessions, statische Felder, Trigger und Builtins.
- **WASM-Codegenerator** – Erstellt WebAssembly Text (.wat) für ausgewählte Konstrukte.
- **CLI** – `hypnoscript` vereint alle Schritte: `run`, `lex`, `parse`, `check`, `compile-wasm`, `builtins`, `version`.

## Typische Einsatzfelder

- **Skript-Experimente** – Kombination aus ungewöhnlicher Syntax und vertrauten Kontrollstrukturen.
- **Lehre & Workshops** – Zeigt, wie Parser, Type Checker und Interpreter zusammenarbeiten.
- **Tooling-Demos** – Beispiel dafür, wie eine Sprache komplett in Rust abgebildet werden kann.
- **Web-WASM-Experimente** – Programmteile nach `.wat` exportieren und in WebAssembly-Projekten einsetzen.

## Weiterführende Ressourcen

- [Core Concepts](./core-concepts) – Überblick über Sprachelemente, Typsystem und Runtime.
- [Installation](./installation) – Lokale Einrichtung der Toolchain.
- [Quick Start](./quick-start) – Dein erstes Skript in wenigen Minuten.
- [Sprachreferenz](../language-reference/syntax) – Grammatik, Operatoren, Funktionen, Sessions.
- [Builtin-Übersicht](../builtins/overview) – Alle Standardfunktionen nach Kategorien.

HypnoScript macht hypnotische Metaphern programmierbar – mit einer ehrlichen Rust-Basis unter der Haube.
