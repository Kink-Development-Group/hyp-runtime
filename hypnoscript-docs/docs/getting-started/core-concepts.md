# Core Concepts

Dieser Überblick fasst die wichtigsten Bausteine der aktuellen HypnoScript-Implementierung zusammen. Wenn du den Code oder die Tests im Repository liest, findest du genau diese Konzepte wieder.

## Programmstruktur

- **Focus/Relax**: Jedes Skript startet mit `Focus {` und endet mit `} Relax`.
- **`entrance`**: Optionaler Block direkt nach `Focus`, ideal für Setup und Begrüßung.
- **`finale`**: Optionaler Block vor `Relax`, wird immer ausgeführt (Cleanup).

```hyp
Focus {
    entrance { observe "Hallo"; }
    // ... regulärer Code ...
    finale { observe "Auf Wiedersehen"; }
} Relax
```

## Deklarationen & Typen

- `induce name: string = "Text";` – veränderbare Variable.
- `implant` – Alias für `induce`.
- `freeze PI: number = 3.14159;` – Konstante.
- Arrays werden mit `[]` notiert: `induce values: number[] = [1, 2, 3];`.
- Unterstützte Typen: `number`, `string`, `boolean`, Arrays, Funktionen, Sessions. Ein `trance`-Typ existiert im Typsystem, wird aber derzeit nicht aktiv verwendet.

## Kontrolle & Operatoren

- `if`, `else if`, `else`
- `while` für bedingte Schleifen
- `loop { ... }` als endlose Schleife (Beenden via `snap`/`break`)
- `snap` (Alias `break`), `sink` (Alias `continue`)
- Hypnotische Operatoren wie `youAreFeelingVerySleepy` (`==`) oder `underMyControl` (`&&`)
- Booleans können mit `oscillate flag;` umgeschaltet werden

## Funktionen

- Definiert mit `suggestion name(params): returnType { ... }`
- `awaken` (oder `return`) beendet eine Funktion.
- Trigger verwenden `trigger name = suggestion(...) { ... }` und verhalten sich wie Callbacks.

```hyp
suggestion greet(name: string) {
    observe "Hallo, " + name + "!";
}

trigger onWelcome = suggestion(person: string) {
    greet(person);
}
```

## Sessions (Objektorientierung)

- `session Name { ... }` erzeugt eine Klasse.
- Felder: `expose` (öffentlich) oder `conceal` (privat). `dominant` macht Felder oder Methoden statisch.
- Methoden nutzen `suggestion`, `imperativeSuggestion` oder `dominantSuggestion` (Letzteres erzwingt statisch).
- Konstruktoren: `suggestion constructor(...) { ... }`.
- Der Interpreter injiziert `this` für Instanzmethoden und verhindert, dass statische Mitglieder über Instanzen angesprochen werden (und umgekehrt).

```hyp
session Counter {
    expose name: string;
    conceal value: number = 0;

    suggestion constructor(name: string) {
        this.name = name;
    }

    expose suggestion increment() {
        this.value = this.value + 1;
        observe this.name + ": " + this.value;
    }
}

induce c: Counter = Counter("HypnoBot");
c.increment();
```

## Builtins

Der Type Checker registriert sämtliche Standardfunktionen. Wichtige Kategorien:

- **Mathe**: `Sin`, `Cos`, `Sqrt`, `Pow`, `Clamp`, `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci`, …
- **Strings**: `Length`, `ToUpper`, `Trim`, `Replace`, `Split`, `Substring`, `PadLeft`, `IsWhitespace`, …
- **Arrays**: `ArrayLength`, `ArrayIsEmpty`, `ArraySum`, `ArrayAverage`, `ArraySlice`, `ArrayDistinct`, …
- **System & Dateien**: `GetOperatingSystem`, `GetUsername`, `GetArgs`, `ReadFile`, `WriteFile`, `ListDirectory`, …
- **Zeit & Statistik**: `CurrentTimestamp`, `CurrentDate`, `Mean`, `Median`, `StandardDeviation`, `Correlation`, …
- **Validierung & Utility**: `IsValidEmail`, `MatchesPattern`, `HashString`, `SimpleRandom`, …

Alle Builtins gibt es kompakt per `hypnoscript builtins`.

## CLI-Workflow

```bash
hypnoscript lex file.hyp       # Tokens anzeigen
hypnoscript parse file.hyp     # AST inspizieren
hypnoscript check file.hyp     # Typprüfung
hypnoscript run file.hyp       # Ausführen
hypnoscript compile-wasm file.hyp -o file.wat
hypnoscript version            # Toolchain-Infos
```

- `--debug` beim `run`-Befehl zeigt Zwischenschritte (Source, Tokens, Type Check).
- `--verbose` fügt zusätzliche Statusmeldungen hinzu.

## Wo du weiterliest

- [Quick Start](./quick-start) – Dein erstes Skript Schritt für Schritt
- [CLI Basics](./cli-basics) – Alle Subcommands im Detail
- [Syntax-Referenz](../language-reference/syntax) – Vollständige Grammatik
- [Builtin-Übersicht](../builtins/overview) – Alle Funktionen nach Kategorien

Mit diesen Konzepten liest du den Repository-Code problemlos und kannst eigene Skripte schreiben.
