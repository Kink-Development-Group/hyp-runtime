# HypnoScript Documentation

This is the complete documentation for HypnoScript — the hypnotic programming language. The documentation is built with [VitePress](https://vitepress.dev/) and is automatically deployed to GitHub Pages.

## 🚀 Quick Start

### Prerequisites

- Node.js 18.0 or higher
- npm, yarn, or pnpm

### Installation

```bash
# Install dependencies
npm install

# Start the dev server
npm run dev

# Build documentation
npm run build

# Preview the built documentation
npm run preview
```

## 📁 Project Structure

```bash
HypnoScript.Dokumentation/
├── docs/                    # Documentation pages
│   ├── .vitepress/         # VitePress configuration
│   │   ├── config.mts      # Main configuration
│   │   └── theme/          # Custom theme
│   │       ├── index.ts    # Theme entry point
│   │       └── style.css   # Custom CSS
│   ├── index.md            # Homepage
│   ├── intro.md            # Introduction
│   ├── getting-started/    # Getting started
│   ├── language-reference/ # Language reference
│   ├── builtins/           # Builtin functions
│   ├── cli/                # CLI & Tools
│   ├── examples/           # Examples
│   ├── development/        # Development
│   └── reference/          # Reference
├── static/                 # Static files
│   └── img/                # Images
└── package.json            # Dependencies
```

## 🛠️ Development

### Add a New Page

1. Create a new `.md` file in the appropriate directory under `docs/`
2. Add frontmatter (optional):

   ```markdown
   ---
   title: Page Title
   description: Description
   ---
   ```

3. Update `docs/.vitepress/config.mts` to include the page in the sidebar

### Customize Styling

- Custom CSS: `docs/.vitepress/theme/style.css`
- Theme components: `docs/.vitepress/theme/index.ts`

### Local Development

```bash
npm run dev
```

Open [http://localhost:5173](http://localhost:5173) in your browser.

## 🚀 Deployment

The documentation is automatically deployed to GitHub Pages via GitHub Actions:

- **Trigger**: Push to `main` with changes in `HypnoScript.Dokumentation/`
- **Workflow**: `.github/workflows/deploy-docs.yml`
- **URL**: <https://Kink-Development-Group.github.io/hyp-runtime/>

### Manual Deployment

```bash
npm run build
# The built documentation is located in docs/.vitepress/dist/
```

## 📚 Documentation Structure

### Getting Started

- Installation and setup
- Quick start guide
- Hello World
- CLI basics

### Language Reference

- Syntax
- Variables and data types
- Operators
- Control structures
- Functions
- Sessions and Tranceify
- Arrays and records
- Imports and assertions

### Builtin Functions

- Overview of all 200+ functions
- Array functions
- String functions
- Math functions
- Utility functions
- System functions
- Time and date functions
- Statistics functions
- Hashing/encoding
- Hypnotic specialty functions
- Dictionary functions
- File functions
- Network functions
- Validation functions
- Performance functions

### CLI & Tools

- CLI overview
- Commands
- Configuration
- Testing
- Debugging
- Runtime features

### Examples

- Basic examples
- Array examples
- String examples
- Math examples
- File examples
- Hypnotic examples
- Advanced examples

### Development

- Architecture
- Contributing
- Building
- Testing
- Debugging
- Extending

## 🔁 Installer Synchronization

The unified installer (`install.sh`) lives in the repository root and is automatically mirrored into the documentation. The `scripts/sync-installer.mjs` script copies it to `static/install.sh` before every `dev`, `build`, or `preview` run (see the `package.json` `pre*` hooks). This ensures the published handbook provides the exact same installer that ships inside release archives.

Manual run — for example, after changes to the installer without a docs build:

```bash
npm run sync-installer
```

Alternatively, run the script directly:

```bash
node ./scripts/sync-installer.mjs
```

The GitHub Actions that build releases run the same step and also place the script into the release archives (`share/hypnoscript/install.sh`).

### Reference

- Grammar
- AST
- Interpreter
- Compiler
- Runtime
- API
- Changelog

## 🌐 Internationalization

The documentation supports multiple languages:

- **German** (default)
- **English**

### Add a New Language

1. Update `docusaurus.config.js`:

   ```javascript
   i18n: {
     defaultLocale: 'de',
     locales: ['de', 'en', 'new-language'],
   },
   ```

2. Generate translations:

   ```bash
   npm run write-translations
   ```

## 🔍 Search

The documentation uses Algolia for search. Configure it in `docusaurus.config.js`:

```javascript
algolia: {
  appId: 'YOUR_APP_ID',
  apiKey: 'YOUR_SEARCH_API_KEY',
  indexName: 'hypnoscript',
}
```

## 📝 Blog

Blog posts can be added under `blog/`. Each `.md` file is automatically treated as a blog post.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test locally with `npm start`
5. Open a pull request

## 📄 License

MIT License — see [LICENSE](../../LICENSE) for details.

## 🔗 Links

- **Live documentation**: <https://Kink-Development-Group.github.io/hyp-runtime/>
- **GitHub repository**: <https://github.com/Kink-Development-Group/hyp-runtime>
- **Docusaurus**: <https://docusaurus.io/>
- **Issues**: <https://github.com/Kink-Development-Group/hyp-runtime/issues>

---

**Ready to document the hypnotic world of programming?** 🧠✨
