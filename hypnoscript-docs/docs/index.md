---
layout: home

hero:
  name: 'HypnoScript'
  text: 'Die hypnotische Programmiersprache'
  tagline: Code with style – moderne Programmierung mit hypnotischer Eleganz
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
    details: Schlüsselwörter wie Focus, Relax, induce, observe oder deepFocus übersetzen hypnotische Metaphern direkt in Code.

  - icon: 🦀
    title: Vollständig in Rust umgesetzt
    details: Lexer, Parser, Type Checker, Interpreter und WASM-Codegen laufen nativ auf Windows, macOS und Linux.

  - icon: 🧠
    title: Statisches Typ-System
    details: Der Type Checker entdeckt Fehler frühzeitig und versteht Sessions, Records und Funktionen.

  - icon: 📦
    title: Umfangreiche Standardbibliothek
    details: Über 110 eingebaute Funktionen für Arrays, Strings, Mathematik, Dateien, Statistik, System- und Zeitoperationen.

  - icon: 🛠️
    title: Produktive CLI
    details: Ein einzelnes Binary bietet run, lex, parse, check, compile-wasm, builtins und version.

  - icon: 🧩
    title: Sessions & Tranceify
    details: Objektorientierte Sessions mit Sichtbarkeiten sowie Record-Typen für strukturierte Daten.

  - icon: 🌐
    title: Webready mit WASM
    details: Programme lassen sich optional nach WebAssembly (.wat) generieren und weiterverarbeiten.
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
    induce sum = ArraySum(numbers);
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

HypnoScript kombiniert die Eleganz moderner Programmiersprachen mit einer einzigartigen, hypnotisch inspirierten Syntax. Die aktuelle Rust-Implementierung liefert:

- **🎯 Einzigartige Syntax** – Focus/Relax-Blöcke, hypnotische Operatoren wie `youAreFeelingVerySleepy` (`==`) und `underMyControl` (`&&`).
- **🦾 Rust-Performance** – Keine .NET-Abhängigkeiten, schnelle Binaries, optionale WASM-Ausgabe.
- **🔒 Statische Sicherheit** – Der Type Checker versteht Variablen, Funktionen, Sessions und Record-Typen (`tranceify`).
- **🧰 Standardbibliothek** – Mathe, Strings, Arrays, Dateien, Statistik, Validierung, System- und Zeitfunktionen.
- **🧪 Entwicklungs-Workflow** – CLI unterstützt Lexing, Parsing, Type Checking und die Programmausführung.
- **📄 Beispiele & Tests** – Umfangreiche `.hyp`-Beispiele sowie Regressionstests im Repository.

## Community & Support

- **GitHub**: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- **Dokumentation**: Diese Seite
- **Issues**: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- **Community Updates**: Verfolge den Fortschritt im [GitHub Repository](https://github.com/Kink-Development-Group/hyp-runtime)

## Lizenz

HypnoScript ist Open Source und unter der MIT-Lizenz verfügbar.
