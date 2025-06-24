# HypnoScript Dokumentation

Dies ist die vollständige Dokumentation für HypnoScript - Die hypnotische Programmiersprache. Die Dokumentation wird mit [Docusaurus 3.8](https://docusaurus.io/) erstellt und automatisch zu GitHub Pages deployed.

## 🚀 Schnellstart

### Voraussetzungen

- Node.js 18.0 oder höher
- npm oder yarn

### Installation

```bash
# Dependencies installieren
npm install

# Entwicklungsserver starten
npm start

# Dokumentation bauen
npm run build

# Lokalen Server für gebaute Dokumentation starten
npm run serve
```

## 📁 Projektstruktur

```
HypnoScript.Dokumentation/
├── docs/                    # Dokumentationsseiten
│   ├── intro.md            # Einführung
│   ├── getting-started/    # Erste Schritte
│   ├── language-reference/ # Sprachreferenz
│   ├── builtins/           # Builtin-Funktionen
│   ├── cli/                # CLI & Tools
│   ├── examples/           # Beispiele
│   ├── development/        # Entwicklung
│   └── reference/          # Referenz
├── blog/                   # Blog-Posts
├── src/                    # Quellcode
│   ├── css/               # Custom CSS
│   └── pages/             # Zusätzliche Seiten
├── static/                # Statische Dateien
│   └── img/               # Bilder
├── docusaurus.config.js   # Docusaurus-Konfiguration
├── sidebars.js            # Sidebar-Struktur
└── package.json           # Dependencies
```

## 🛠️ Entwicklung

### Neue Seite hinzufügen

1. Erstelle eine neue `.md` Datei im entsprechenden Verzeichnis unter `docs/`
2. Füge Frontmatter hinzu:
   ```markdown
   ---
   sidebar_position: 1
   ---
   ```
3. Aktualisiere `sidebars.js` um die Seite in die Navigation einzufügen

### Styling anpassen

- Custom CSS: `src/css/custom.css`
- Theme-Komponenten: `src/theme/`

### Lokale Entwicklung

```bash
npm start
```

Öffne [http://localhost:3000](http://localhost:3000) im Browser.

## 🚀 Deployment

Die Dokumentation wird automatisch zu GitHub Pages deployed über GitHub Actions:

- **Trigger**: Push zu `main` Branch mit Änderungen in `HypnoScript.Dokumentation/`
- **Workflow**: `.github/workflows/deploy-docs.yml`
- **URL**: https://Kink-Development-Group.github.io/hyp-runtime/

### Manuelles Deployment

```bash
npm run build
npm run deploy
```

## 📚 Dokumentationsstruktur

### Erste Schritte

- Installation und Setup
- Schnellstart-Guide
- Hello World
- CLI-Grundlagen

### Sprachreferenz

- Syntax
- Variablen und Datentypen
- Operatoren
- Kontrollstrukturen
- Funktionen
- Sessions und Tranceify
- Arrays und Records
- Imports und Assertions

### Builtin-Funktionen

- Übersicht aller 200+ Funktionen
- Array-Funktionen
- String-Funktionen
- Mathematische Funktionen
- Utility-Funktionen
- System-Funktionen
- Zeit- und Datumsfunktionen
- Statistik-Funktionen
- Hashing/Encoding
- Hypnotische Spezialfunktionen
- Dictionary-Funktionen
- Datei-Funktionen
- Netzwerk-Funktionen
- Validierung-Funktionen
- Performance-Funktionen

### CLI & Tools

- CLI-Übersicht
- Kommandos
- Konfiguration
- Testing
- Debugging
- Runtime-Features

### Beispiele

- Grundlegende Beispiele
- Array-Beispiele
- String-Beispiele
- Mathematische Beispiele
- Datei-Beispiele
- Hypnotische Beispiele
- Erweiterte Beispiele

### Entwicklung

- Architektur
- Contributing
- Building
- Testing
- Debugging
- Extending

### Referenz

- Grammatik
- AST
- Interpreter
- Compiler
- Runtime
- API
- Changelog

## 🌐 Internationalisierung

Die Dokumentation unterstützt mehrere Sprachen:

- **Deutsch** (Standard)
- **Englisch**

### Neue Sprache hinzufügen

1. Aktualisiere `docusaurus.config.js`:

   ```javascript
   i18n: {
     defaultLocale: 'de',
     locales: ['de', 'en', 'neue-sprache'],
   },
   ```

2. Erstelle Übersetzungen:
   ```bash
   npm run write-translations
   ```

## 🔍 Suchfunktion

Die Dokumentation verwendet Algolia für die Suchfunktion. Konfiguration in `docusaurus.config.js`:

```javascript
algolia: {
  appId: 'YOUR_APP_ID',
  apiKey: 'YOUR_SEARCH_API_KEY',
  indexName: 'hypnoscript',
}
```

## 📝 Blog

Blog-Posts können unter `blog/` hinzugefügt werden. Jede `.md` Datei wird automatisch als Blog-Post erkannt.

## 🤝 Contributing

1. Fork das Repository
2. Erstelle einen Feature-Branch
3. Mache deine Änderungen
4. Teste lokal mit `npm start`
5. Erstelle einen Pull Request

## 📄 Lizenz

MIT License - siehe [LICENSE](../../LICENSE) für Details.

## 🔗 Links

- **Live-Dokumentation**: https://Kink-Development-Group.github.io/hyp-runtime/
- **GitHub Repository**: https://github.com/Kink-Development-Group/hyp-runtime
- **Docusaurus**: https://docusaurus.io/
- **Issues**: https://github.com/Kink-Development-Group/hyp-runtime/issues

---

**Bereit, die hypnotische Welt der Programmierung zu dokumentieren?** 🧠✨
