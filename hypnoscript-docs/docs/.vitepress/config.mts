import type { LanguageRegistration } from 'shiki';
import { createHighlighter } from 'shiki';
import { defineConfig } from 'vitepress';
import hypnoscriptGrammar from './hypnoscript.tmLanguage.json' with { type: 'json' };

const BASE_PATH = '/hyp-runtime/';

const hypnoscriptLanguage = {
  ...hypnoscriptGrammar,
  name: 'HypnoScript',
  aliases: ['hypnoscript', 'hyp', 'hypno'],
  embeddedLangs: ['json', 'javascript'],
} satisfies LanguageRegistration;

const LANGUAGE_ALIASES: Record<string, string> = {
  bash: 'bash',
  console: 'bash',
  sh: 'bash',
  shell: 'bash',
  shellscript: 'bash',
  js: 'javascript',
  javascript: 'javascript',
  ts: 'typescript',
  typescript: 'typescript',
  json: 'json',
  jsonc: 'json',
  yml: 'yaml',
  yaml: 'yaml',
  md: 'markdown',
  markdown: 'markdown',
  hyp: 'hypnoscript',
  hypno: 'hypnoscript',
  hypnoscript: 'hypnoscript',
};

const highlighter = await createHighlighter({
  themes: ['github-light', 'github-dark'],
  langs: [
    'bash',
    'css',
    'html',
    'javascript',
    'json',
    'markdown',
    'powershell',
    'rust',
    'toml',
    'typescript',
    'yaml',
    'text',
    hypnoscriptLanguage,
  ],
});

const loadedLanguages = new Set(
  highlighter
    .getLoadedLanguages()
    .map((lang) => (typeof lang === 'string' ? lang.toLowerCase() : ''))
    .filter(Boolean) as string[],
);

const resolveLanguage = (rawLang: string | undefined): string => {
  const normalized = (rawLang ?? '').trim().toLowerCase();
  if (!normalized) {
    return 'text';
  }

  const aliased = LANGUAGE_ALIASES[normalized];
  if (aliased && loadedLanguages.has(aliased)) {
    return aliased;
  }

  if (loadedLanguages.has(normalized)) {
    return normalized;
  }

  for (const candidate of loadedLanguages) {
    const language = highlighter.getLanguage(candidate) as unknown as
      | { aliases?: string[] }
      | undefined;
    if (language?.aliases?.some((alias) => alias.toLowerCase() === normalized)) {
      return candidate;
    }
  }

  return 'text';
};

const escapeVueInterpolation = (html: string): string =>
  html.replaceAll('{{', '&#123;&#123;').replaceAll('}}', '&#125;&#125;');

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: 'HypnoScript',
  description: 'Code with style - Die hypnotische Programmiersprache',
  base: BASE_PATH,

  // Ignoriere tote Links während der Migration
  ignoreDeadLinks: true,

  head: [['link', { rel: 'icon', href: `${BASE_PATH}img/favicon.ico` }]],

  vite: {
    publicDir: '../static',
  },

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/img/logo.svg',
    editLink: false as unknown as undefined,

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Dokumentation', link: '/intro' },
      {
        text: 'Erste Schritte',
        items: [
          { text: 'Installation', link: '/getting-started/installation' },
          { text: 'Quick Start', link: '/getting-started/quick-start' },
          { text: 'CLI Basics', link: '/getting-started/cli-basics' },
        ],
      },
      {
        text: 'Referenz',
        items: [
          { text: 'Sprachreferenz', link: '/language-reference/syntax' },
          { text: 'Builtin-Funktionen', link: '/builtins/overview' },
          { text: 'CLI', link: '/cli/overview' },
          { text: 'Runtime', link: '/reference/runtime' },
        ],
      },
    ],

    sidebar: {
      '/': [
        {
          text: 'Einführung',
          items: [
            { text: 'Willkommen', link: '/intro' },
            {
              text: 'Was ist HypnoScript?',
              link: '/getting-started/what-is-hypnoscript',
            },
          ],
        },
        {
          text: 'Erste Schritte',
          collapsed: false,
          items: [
            { text: 'Installation', link: '/getting-started/installation' },
            { text: 'Quick Start', link: '/getting-started/quick-start' },
            { text: 'Grundkonzepte', link: '/getting-started/core-concepts' },
            { text: 'Hello World', link: '/getting-started/hello-world' },
            { text: 'CLI Basics', link: '/getting-started/cli-basics' },
          ],
        },
        {
          text: 'Sprachreferenz',
          collapsed: true,
          items: [
            { text: 'Syntax', link: '/language-reference/syntax' },
            { text: 'Variablen', link: '/language-reference/variables' },
            { text: 'Operatoren', link: '/language-reference/operators' },
            {
              text: 'Kontrollstrukturen',
              link: '/language-reference/control-flow',
            },
            { text: 'Funktionen', link: '/language-reference/functions' },
            { text: 'Arrays', link: '/language-reference/arrays' },
            { text: 'Records', link: '/language-reference/records' },
            { text: 'Sessions', link: '/language-reference/sessions' },
            { text: 'Tranceify', link: '/language-reference/tranceify' },
            { text: 'Assertions', link: '/language-reference/assertions' },
          ],
        },
        {
          text: 'Builtin-Funktionen',
          collapsed: true,
          items: [
            { text: 'Übersicht', link: '/builtins/overview' },
            { text: 'Array-Funktionen', link: '/builtins/array-functions' },
            { text: 'String-Funktionen', link: '/builtins/string-functions' },
            { text: 'Math-Funktionen', link: '/builtins/math-functions' },
            { text: 'System-Funktionen', link: '/builtins/system-functions' },
            { text: 'Zeit & Datum', link: '/builtins/time-date-functions' },
            { text: 'Datei-Funktionen', link: '/builtins/file-functions' },
            { text: 'Utility-Funktionen', link: '/builtins/utility-functions' },
            { text: 'Hashing & Encoding', link: '/builtins/hashing-encoding' },
            { text: 'Statistik-Funktionen', link: '/builtins/statistics-functions' },
            { text: 'Validierungs-Funktionen', link: '/builtins/validation-functions' },
            { text: 'Hypnotic Functions', link: '/builtins/hypnotic-functions' },
            { text: 'Performance-Funktionen', link: '/builtins/performance-functions' },
            { text: 'Dictionary-Funktionen', link: '/builtins/dictionary-functions' },
            { text: 'Netzwerk-Funktionen', link: '/builtins/network-functions' },
          ],
        },
        {
          text: 'CLI Tools',
          collapsed: true,
          items: [
            { text: 'Übersicht', link: '/cli/overview' },
            { text: 'Befehle', link: '/cli/commands' },
            { text: 'Konfiguration', link: '/cli/configuration' },
            { text: 'Testing', link: '/cli/testing' },
            { text: 'Debugging', link: '/cli/debugging' },
            { text: 'Erweiterte Befehle', link: '/cli/advanced-commands' },
            { text: 'Enterprise Features', link: '/cli/enterprise-features' },
          ],
        },
        {
          text: 'Testing',
          collapsed: true,
          items: [
            { text: 'Überblick', link: '/testing/overview' },
            { text: 'Assertions', link: '/testing/assertions' },
            { text: 'Best Practices', link: '/testing/best-practices' },
            { text: 'Fixtures', link: '/testing/fixtures' },
            { text: 'Performance', link: '/testing/performance' },
            { text: 'Reporting', link: '/testing/reporting' },
          ],
        },
        {
          text: 'Debugging',
          collapsed: true,
          items: [
            { text: 'Überblick', link: '/debugging/overview' },
            { text: 'Debug-Modus', link: '/debugging/debug-mode' },
            { text: 'Breakpoints', link: '/debugging/breakpoints' },
            { text: 'Tools', link: '/debugging/tools' },
            { text: 'Troubleshooting', link: '/debugging/troubleshooting' },
            { text: 'Best Practices', link: '/debugging/best-practices' },
            { text: 'Performance', link: '/debugging/performance' },
          ],
        },
        {
          text: 'Error Handling',
          collapsed: true,
          items: [
            { text: 'Überblick', link: '/error-handling/overview' },
            { text: 'Fehlerbehandlung', link: '/error-handling/basics' },
            { text: 'Häufige Fehler', link: '/error-handling/common-errors' },
          ],
        },
        {
          text: 'Enterprise',
          collapsed: true,
          items: [
            { text: 'Überblick', link: '/enterprise/overview' },
            { text: 'Features', link: '/enterprise/features' },
            { text: 'Security', link: '/enterprise/security' },
            { text: 'Architecture', link: '/enterprise/architecture' },
            { text: 'Integration', link: '/enterprise/integration' },
            { text: 'Monitoring', link: '/enterprise/monitoring' },
            { text: 'Debugging', link: '/enterprise/debugging' },
            { text: 'API Management', link: '/enterprise/api-management' },
            { text: 'Messaging', link: '/enterprise/messaging' },
            { text: 'Datenbank', link: '/enterprise/database' },
            { text: 'Backup & Recovery', link: '/enterprise/backup-recovery' },
          ],
        },
        {
          text: 'Referenzen',
          collapsed: true,
          items: [
            { text: 'Runtime', link: '/reference/runtime' },
            { text: 'Compiler', link: '/reference/compiler' },
            { text: 'Interpreter', link: '/reference/interpreter' },
            { text: 'API', link: '/reference/api' },
          ],
        },
        {
          text: 'Tutorial Extras',
          collapsed: true,
          items: [
            { text: 'Performance', link: '/tutorial-extras/performance' },
            {
              text: 'Dokumentations-Versionen',
              link: '/tutorial-extras/manage-docs-versions',
            },
            { text: 'Lokalisierung', link: '/tutorial-extras/translate-your-site' },
          ],
        },
        {
          text: 'Beispiele',
          collapsed: true,
          items: [
            { text: 'Einstieg', link: '/examples/basic-examples' },
            { text: 'Array-Beispiele', link: '/examples/array-examples' },
            { text: 'String-Beispiele', link: '/examples/string-examples' },
            { text: 'System-Beispiele', link: '/examples/system-examples' },
            { text: 'Math-Beispiele', link: '/examples/math-examples' },
            { text: 'Utility-Beispiele', link: '/examples/utility-examples' },
            {
              text: 'Therapeutische Beispiele',
              link: '/examples/therapeutic-examples',
            },
            { text: 'CLI Workflows', link: '/examples/cli-workflows' },
          ],
        },
        {
          text: 'Entwicklung',
          collapsed: true,
          items: [
            { text: 'Debugging-Prozesse', link: '/development/debugging' },
          ],
        },
      ],
    },

    socialLinks: [
      {
        icon: 'github',
        link: 'https://github.com/Kink-Development-Group/hyp-runtime',
      },
    ],

    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2024-present HypnoScript Team',
    },

    search: {
      provider: 'local',
    },

    lastUpdated: {
      text: 'Zuletzt aktualisiert',
      formatOptions: {
        dateStyle: 'medium',
        timeStyle: 'short',
      },
    },
  },

  markdown: {
    theme: {
      light: 'github-light',
      dark: 'github-dark',
    },
    lineNumbers: true,
    highlight(code, lang) {
      const resolved = resolveLanguage(lang);
      const highlighted = highlighter.codeToHtml(code, {
        lang: resolved,
        themes: {
          light: 'github-light',
          dark: 'github-dark',
        },
      });

      return escapeVueInterpolation(highlighted);
    },
  },
});
