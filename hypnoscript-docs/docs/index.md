---
layout: home

hero:
  name: 'HypnoScript'
  text: 'Die hypnotische Programmiersprache'
  tagline: Moderne Skripte mit hypnotischer Syntax und einer soliden Rust-Basis
  image:
    src: /img/logo.svg
    alt: HypnoScript Logo
  actions:
    - theme: brand
      text: Schnellstart
      link: /getting-started/quick-start
    - theme: alt
      text: Dokumentation
      link: /intro
    - theme: alt
      text: GitHub
      link: https://github.com/Kink-Development-Group/hyp-runtime

features:
  - icon: 🎯
    title: Hypnotische Syntax
    details: Schlüsselwörter wie Focus, Relax, induce, observe oder deepFocus bringen hypnotische Metaphern direkt in deinen Code.

  - icon: 🦀
    title: Vollständig in Rust umgesetzt
    details: Lexer, Parser, statischer Type Checker, Interpreter und WASM-Codegen laufen nativ auf Windows, macOS und Linux.

  - icon: 🧠
    title: Statisches Typ-System
    details: Der Type Checker versteht Zahlen, Strings, Booleans, Arrays, Funktionen und Sessions inklusive Sichtbarkeiten.

  - icon: 📦
    title: Standardbibliothek inklusive
    details: Mathe, Strings, Arrays, Dateien, Statistik, Systeminformationen, Zeit & Datum sowie Validierungsfunktionen sind sofort verfügbar.

  - icon: 🛠️
    title: Schlanke CLI
    details: Ein einziges Binary liefert run, lex, parse, check, compile-wasm, builtins und version – mehr brauchst du nicht.

  - icon: 🧩
    title: Sessions mit Sichtbarkeit
    details: Definiere Sessions mit `expose`/`conceal`, Konstruktoren und statischen (`dominant`) Mitgliedern.

  - icon: 🌐
    title: WebAssembly Export
    details: Erzeuge optional WebAssembly Textdateien (.wat) und nutze HypnoScript im Browser.
---

## Schneller Einstieg

### Installation

```bash
# Repository klonen
git clone https://github.com/Kink-Development-Group/hyp-runtime.git
cd hyp-runtime

# HypnoScript CLI in Release-Qualität bauen
cargo build -p hypnoscript-cli --release

# Optional global installieren (binary heißt hypnoscript)
cargo install --path hypnoscript-cli
```

Fertige Artefakte (Windows, macOS, Linux) findest du außerdem im Ordner `release/` sowie unter [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases).

### Dein erstes HypnoScript-Programm

```hyp
Focus {
    entrance {
        observe "Willkommen bei HypnoScript!";
    }

    induce name: string = "Entwickler";
    observe "Hallo, " + name + "!";

    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce sum: number = ArraySum(numbers);
    observe "Summe: " + ToString(sum);

    if (sum lookAtTheWatch 10) deepFocus {
      observe "Die Erinnerung wird jetzt intensiver.";
    }
}
```

### Ausführen

```bash
hypnoscript run mein_script.hyp
```

## Warum HypnoScript?

HypnoScript kombiniert die Eleganz moderner Programmiersprachen mit einer einzigartigen, hypnotisch inspirierten Syntax. Die Rust-Implementierung bringt dir:

- **🎯 Einzigartige Syntax** – Focus/Relax-Blöcke, hypnotische Operatoren wie `youAreFeelingVerySleepy` (`==`) oder `underMyControl` (`&&`).
- **🦾 Rust-Performance** – Keine externen Laufzeitabhängigkeiten, schnelle Binaries und optionaler WASM-Export.
- **🔒 Statische Sicherheit** – Der Type Checker prüft Variablen, Funktionen, Sessions sowie Zugriffe auf statische und private Mitglieder.
- **🧰 Standardbibliothek** – Mathe, Strings, Arrays, Dateien, Statistik, Validierung, System- und Zeitfunktionen sind direkt integriert.
- **🧪 Entwicklungs-Workflow** – Die CLI unterstützt Lexing, Parsing, Type Checking und Programmausführung im gleichen Tool.
- **📄 Beispiele & Tests** – `.hyp`-Beispiele und Regressionstests im Repository zeigen reale Sprachfeatures.

## Community & Support

- **GitHub**: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- **Dokumentation**: Diese Seite
- **Issues**: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- **Community Updates**: Verfolge den Fortschritt im [GitHub Repository](https://github.com/Kink-Development-Group/hyp-runtime)

## Lizenz

HypnoScript ist Open Source und unter der MIT-Lizenz verfügbar.
