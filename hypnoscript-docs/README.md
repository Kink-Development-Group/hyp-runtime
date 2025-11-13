# HypnoScript Dokumentation

Dies ist die vollständige Dokumentation für HypnoScript - Die hypnotische Programmiersprache. Die Dokumentation wird mit [VitePress](https://vitepress.dev/) erstellt und automatisch zu GitHub Pages deployed.

## 🚀 Schnellstart

### Voraussetzungen

- Node.js 18.0 oder höher
- npm, yarn oder pnpm

### Installation

```bash
# Dependencies installieren
npm install

# Entwicklungsserver starten
npm run dev

# Dokumentation bauen
npm run build

# Vorschau der gebauten Dokumentation
npm run preview
```

## 📁 Projektstruktur

```
HypnoScript.Dokumentation/
├── docs/                    # Dokumentationsseiten
│   ├── .vitepress/         # VitePress-Konfiguration
│   │   ├── config.mts      # Hauptkonfiguration
│   │   └── theme/          # Custom Theme
│   │       ├── index.ts    # Theme-Einstiegspunkt
│   │       └── style.css   # Custom CSS
│   ├── index.md            # Homepage
│   ├── intro.md            # Einführung
│   ├── getting-started/    # Erste Schritte
│   ├── language-reference/ # Sprachreferenz
│   ├── builtins/           # Builtin-Funktionen
│   ├── cli/                # CLI & Tools
│   ├── examples/           # Beispiele
│   ├── development/        # Entwicklung
│   └── reference/          # Referenz
├── static/                 # Statische Dateien
│   └── img/                # Bilder
└── package.json            # Dependencies
```

## 🛠️ Entwicklung

### Neue Seite hinzufügen

1. Erstelle eine neue `.md` Datei im entsprechenden Verzeichnis unter `docs/`
2. Füge Frontmatter hinzu (optional):
   ```markdown
   ---
   title: Seitentitel
   description: Beschreibung
   ---
   ```
3. Aktualisiere `docs/.vitepress/config.mts` um die Seite in die Sidebar einzufügen

### Styling anpassen

- Custom CSS: `docs/.vitepress/theme/style.css`
- Theme-Komponenten: `docs/.vitepress/theme/index.ts`

### Lokale Entwicklung

```bash
npm run dev
```

Öffne [http://localhost:5173](http://localhost:5173) im Browser.

## 🚀 Deployment

Die Dokumentation wird automatisch zu GitHub Pages deployed über GitHub Actions:

- **Trigger**: Push zu `main` Branch mit Änderungen in `HypnoScript.Dokumentation/`
- **Workflow**: `.github/workflows/deploy-docs.yml`
- **URL**: https://Kink-Development-Group.github.io/hyp-runtime/

### Manuelles Deployment

```bash
npm run build
# Die gebaute Dokumentation befindet sich in docs/.vitepress/dist/
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

## 🔁 Installer-Synchronisation

Der neue einheitliche Installer (`install.sh`) lebt im Repository-Wurzelverzeichnis und wird automatisch in die Dokumentation gespiegelt. Das Script `scripts/sync-installer.mjs` kopiert ihn vor jedem `dev`, `build` oder `preview`-Lauf nach `static/install.sh` (siehe `package.json`-`pre*`-Hooks). Dadurch steht im veröffentlichen Handbuch exakt derselbe Installer zum Download bereit, der auch in den Release-Archiven enthalten ist.

Manueller Lauf – z.B. nach Änderungen am Installer ohne Dokumentations-Build:

```bash
npm run sync-installer
```

Alternativ kannst du das Script direkt ausführen:

```bash
node ./scripts/sync-installer.mjs
```

Die GitHub-Actions, die Releases bauen, führen denselben Schritt aus und legen das Skript zusätzlich in den Release-Archiven (`share/hypnoscript/install.sh`) ab.

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

- **Live-Dokumentation**: <https://Kink-Development-Group.github.io/hyp-runtime/>
- **GitHub Repository**: <https://github.com/Kink-Development-Group/hyp-runtime>
- **Docusaurus**: <https://docusaurus.io/>
- **Issues**: <https://github.com/Kink-Development-Group/hyp-runtime/issues>

---

**Bereit, die hypnotische Welt der Programmierung zu dokumentieren?** 🧠✨
