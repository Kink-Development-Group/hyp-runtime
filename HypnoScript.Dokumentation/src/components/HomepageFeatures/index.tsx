import Heading from '@theme/Heading';
import clsx from 'clsx';
import React from 'react';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  description: React.JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: '🧠 Hypnotische Syntax',
    description: (
      <>
        Verwendet hypnotische Konzepte wie <code>Focus</code>,{' '}
        <code>Trance</code>, <code>Induce</code>,<code>Observe</code> und{' '}
        <code>Relax</code> für eine intuitive und einzigartige Programmierung.
      </>
    ),
  },
  {
    title: '📚 Umfangreiche Bibliothek',
    description: (
      <>
        Über 200+ eingebaute Funktionen für Arrays, Strings, Mathematik,
        System-Operationen, Datei-Handling, Netzwerk und hypnotische
        Spezialfunktionen.
      </>
    ),
  },
  {
    title: '🛠️ Runtime-Ready',
    description: (
      <>
        Vollständige CLI-Tools, Test-Framework mit Assertions,
        Debugging-Unterstützung, Webserver und API-Features für professionelle
        Entwicklung.
      </>
    ),
  },
  {
    title: '🌐 Plattformübergreifend',
    description: (
      <>
        Läuft auf Windows, macOS und Linux. Geschrieben in C# mit .NET für
        maximale Kompatibilität und Performance.
      </>
    ),
  },
  {
    title: '⚡ Moderne Features',
    description: (
      <>
        Unterstützt Arrays, Records, Funktionen, Sessions, Imports, Assertions,
        und vieles mehr für moderne Softwareentwicklung.
      </>
    ),
  },
  {
    title: '🤝 Open Source',
    description: (
      <>
        Unter MIT-Lizenz veröffentlicht. Aktive Community, regelmäßige Updates,
        und Beiträge sind willkommen.
      </>
    ),
  },
];

function Feature({ title, description }: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): React.JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
