---
layout: home

hero:
  name: 'HypnoScript'
  text: 'Die hypnotische Programmiersprache'
  tagline: Code with style - Moderne Programmierung mit hypnotischer Eleganz
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
    details: Einzigartige Schlüsselwörter wie Focus, Trance, Induce und Observe machen deinen Code ausdrucksstark und lesbar.

  - icon: 🚀
    title: Modern & Leistungsstark
    details: In Rust entwickelt für maximale Performance, Sicherheit und Zuverlässigkeit. Kompiliert zu nativem Code oder WASM.

  - icon: 📦
    title: Umfangreiche Standardbibliothek
    details: Über 200+ eingebaute Funktionen für Arrays, Strings, Mathematik, Dateien, Hashing, Statistik und mehr.

  - icon: 🎨
    title: Typsicher
    details: Statischer Type Checker für frühe Fehlererkennung und bessere Code-Qualität.

  - icon: 🧪
    title: Integriertes Testing
    details: Eingebautes Test-Framework mit Assertions für TDD und qualitätsgesicherte Entwicklung.

  - icon: 🐛
    title: Debugging-Support
    details: Umfassende Debug-Tools mit Breakpoints, Step-Execution und detaillierten Fehlermeldungen.

  - icon: 📊
    title: Records & Sessions
    details: Strukturierte Datentypen und Sessions für State-Management in komplexen Anwendungen.

  - icon: 🔧
    title: CLI Tools
    details: Leistungsstarke Kommandozeilen-Tools für Build, Run, Test und Debug-Operationen.

  - icon: 🌍
    title: Plattformübergreifend
    details: Läuft auf Windows, macOS und Linux. Kompiliert zu WASM für Web-Integration.
---

## Schneller Einstieg

### Installation

```bash
# Download und Installation (Windows, macOS, Linux)
curl -sSL https://hypnoscript.dev/install.sh | sh

# Oder via Package Manager
cargo install hypnoscript-cli
```

### Dein erstes HypnoScript-Programm

```hyp
Focus {
    entrance {
        observe "Willkommen bei HypnoScript!";
    }

    induce name = "Entwickler";
    observe "Hallo, " + name + "!";

    induce numbers = [1, 2, 3, 4, 5];
    induce sum = ArraySum(numbers);
    observe "Summe: " + ToString(sum);
}
```

### Ausführen

```bash
hyp run mein_script.hyp
```

## Warum HypnoScript?

HypnoScript kombiniert die Eleganz moderner Programmiersprachen mit einer einzigartigen, hypnotisch inspirierten Syntax. Die Sprache ist in Rust entwickelt und bietet:

- **🎯 Einzigartige Syntax** - Ausdrucksstark und intuitiv
- **⚡ Hohe Performance** - Dank Rust-basierter Runtime
- **🔒 Typ-Sicherheit** - Statischer Type Checker verhindert Laufzeitfehler
- **🧩 Reiches Ökosystem** - Umfangreiche Builtin-Bibliothek
- **🧪 Testing First** - Eingebautes Test-Framework
- **📚 Vollständige Dokumentation** - Ausführliche Guides und Tutorials

## Community & Support

- **GitHub**: [Kink-Development-Group/hyp-runtime](https://github.com/Kink-Development-Group/hyp-runtime)
- **Dokumentation**: Diese Seite
- **Issues**: [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- **Diskussionen**: [GitHub Discussions](https://github.com/Kink-Development-Group/hyp-runtime/discussions)

## Lizenz

HypnoScript ist Open Source und unter der MIT-Lizenz verfügbar.
