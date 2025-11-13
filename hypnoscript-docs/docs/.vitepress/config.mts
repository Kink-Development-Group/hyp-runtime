import type { LanguageRegistration } from 'shiki';
import { createHighlighter } from 'shiki';
import { defineConfig } from 'vitepress';
import hypnoscriptGrammar from './hypnoscript.tmLanguage.json' with { type: 'json' };

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
  base: '/hyp-runtime/',

  // Ignoriere tote Links während der Migration
  ignoreDeadLinks: true,

  head: [['link', { rel: 'icon', href: '/hyp-runtime/favicon.ico' }]],

  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    logo: '/img/logo.svg',

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Dokumentation', link: '/intro' },
      {
        text: 'Erste Schritte',
        items: [
          { text: 'Installation', link: '/getting-started/installation' },
          { text: 'Quick Start', link: '/getting-started/quick-start' },
          {
            text: 'Tutorial',
            link: '/tutorial-basics/create-your-first-script',
          },
        ],
      },
      {
        text: 'Referenz',
        items: [
          { text: 'Sprachreferenz', link: '/language-reference/syntax' },
          { text: 'Builtin-Funktionen', link: '/builtins/overview' },
          { text: 'CLI', link: '/cli/overview' },
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
          ],
        },
        {
          text: 'Tutorial',
          collapsed: false,
          items: [
            {
              text: 'Dein erstes Skript',
              link: '/tutorial-basics/create-your-first-script',
            },
            {
              text: 'Variablen & Typen',
              link: '/tutorial-basics/variables-and-types',
            },
            { text: 'Funktionen', link: '/tutorial-basics/functions' },
            {
              text: 'Arrays & Collections',
              link: '/tutorial-basics/arrays-and-collections',
            },
            { text: 'Records', link: '/tutorial-basics/records' },
            { text: 'Sessions', link: '/tutorial-basics/sessions' },
          ],
        },
        {
          text: 'Sprachreferenz',
          collapsed: true,
          items: [
            { text: 'Syntax', link: '/language-reference/syntax' },
            { text: 'Datentypen', link: '/language-reference/data-types' },
            { text: 'Operatoren', link: '/language-reference/operators' },
            {
              text: 'Kontrollstrukturen',
              link: '/language-reference/control-flow',
            },
            { text: 'Funktionen', link: '/language-reference/functions' },
            { text: 'Records', link: '/language-reference/records' },
            { text: 'Sessions', link: '/language-reference/sessions' },
            { text: 'Kommentare', link: '/language-reference/comments' },
          ],
        },
        {
          text: 'Builtin-Funktionen',
          collapsed: true,
          items: [
            { text: 'Übersicht', link: '/builtins/overview' },
            { text: 'Core Builtins', link: '/builtins/core' },
            { text: 'Array Builtins', link: '/builtins/arrays' },
            { text: 'String Builtins', link: '/builtins/strings' },
            { text: 'Math Builtins', link: '/builtins/math' },
            { text: 'File Builtins', link: '/builtins/files' },
            { text: 'Time Builtins', link: '/builtins/time' },
            { text: 'System Builtins', link: '/builtins/system' },
            { text: 'Hashing Builtins', link: '/builtins/hashing' },
            { text: 'Statistics Builtins', link: '/builtins/statistics' },
            { text: 'Validation Builtins', link: '/builtins/validation' },
          ],
        },
        {
          text: 'CLI Tools',
          collapsed: true,
          items: [
            { text: 'Übersicht', link: '/cli/overview' },
            { text: 'hyp run', link: '/cli/run' },
            { text: 'hyp test', link: '/cli/test' },
            { text: 'hyp debug', link: '/cli/debug' },
          ],
        },
        {
          text: 'Testing',
          collapsed: true,
          items: [
            { text: 'Test Framework', link: '/testing/framework' },
            { text: 'Assertions', link: '/testing/assertions' },
            { text: 'Best Practices', link: '/testing/best-practices' },
          ],
        },
        {
          text: 'Debugging',
          collapsed: true,
          items: [
            { text: 'Debug-Modus', link: '/debugging/debug-mode' },
            { text: 'Breakpoints', link: '/debugging/breakpoints' },
            { text: 'Troubleshooting', link: '/debugging/troubleshooting' },
          ],
        },
        {
          text: 'Error Handling',
          collapsed: true,
          items: [
            { text: 'Fehlerbehandlung', link: '/error-handling/basics' },
            { text: 'Häufige Fehler', link: '/error-handling/common-errors' },
          ],
        },
        {
          text: 'Erweiterte Features',
          collapsed: true,
          items: [
            { text: 'Enterprise Features', link: '/enterprise/overview' },
            { text: 'Performance', link: '/tutorial-extras/performance' },
            { text: 'Best Practices', link: '/tutorial-extras/best-practices' },
          ],
        },
        {
          text: 'Beispiele',
          collapsed: true,
          items: [
            { text: 'Code-Beispiele', link: '/examples/overview' },
            { text: 'Praxisbeispiele', link: '/examples/practical-examples' },
          ],
        },
        {
          text: 'Entwicklung',
          collapsed: true,
          items: [
            { text: 'Mitwirken', link: '/development/contributing' },
            { text: 'Architektur', link: '/development/architecture' },
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

    editLink: {
      pattern:
        'https://github.com/Kink-Development-Group/hyp-runtime/edit/main/HypnoScript.Dokumentation/docs/:path',
      text: 'Diese Seite auf GitHub bearbeiten',
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
