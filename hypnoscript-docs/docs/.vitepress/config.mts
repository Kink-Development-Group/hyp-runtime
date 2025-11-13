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
          { text: 'CLI Kommandos', link: '/cli/commands' },
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
            { text: 'Hello World', link: '/getting-started/hello-world' },
            { text: 'Grundkonzepte', link: '/getting-started/core-concepts' },
            { text: 'CLI Basics', link: '/getting-started/cli-basics' },
          ],
        },
        {
          text: 'Sprachreferenz',
          collapsed: false,
          items: [
            { text: 'Syntax & Struktur', link: '/language-reference/syntax' },
            { text: 'Variablen & Typen', link: '/language-reference/variables' },
            { text: 'Operatoren', link: '/language-reference/operators' },
            { text: 'Kontrollfluss', link: '/language-reference/control-flow' },
            { text: 'Funktionen & Trigger', link: '/language-reference/functions' },
            { text: 'Sessions', link: '/language-reference/sessions' },
            { text: 'Schlüsselwörter', link: '/language-reference/_keywords-reference' },
          ],
        },
        {
          text: 'Standardbibliothek',
          collapsed: false,
          items: [
            { text: 'Builtin-Übersicht', link: '/builtins/overview' },
          ],
        },
        {
          text: 'CLI',
          collapsed: false,
          items: [
            { text: 'Überblick', link: '/cli/overview' },
            { text: 'Befehle', link: '/cli/commands' },
          ],
        },
        {
          text: 'Referenz',
          collapsed: false,
          items: [
            { text: 'Runtime-Architektur', link: '/reference/runtime' },
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
