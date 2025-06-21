import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Heading from '@theme/Heading';
import Layout from '@theme/Layout';
import clsx from 'clsx';
import type { ReactNode } from 'react';

import styles from './index.module.css';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <Heading as="h1" className="hero__title">
          {siteConfig.title}
        </Heading>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/intro"
          >
            Erste Schritte - 5min ⏱️
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home(): ReactNode {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} - ${siteConfig.tagline}`}
      description="HypnoScript - Die hypnotische Programmiersprache mit über 200+ Builtin-Funktionen, moderner Syntax und umfassenden Entwicklungstools."
    >
      <HomepageHeader />
      <main>
        <div className="container margin-vert--xl">
          <div className="text--center">
            <Heading as="h2">🚀 Download</Heading>
            <p className="text--lg">
              Lade die neueste Version der HypnoScript-Runtime herunter und
              starte deine hypnotische Programmierreise.
            </p>
            <div className={styles.buttons}>
              <Link
                className="button button--primary button--lg margin-horiz--sm"
                to="/downloads/HypnoScript.Runtime.exe"
                download
              >
                Windows (x64)
              </Link>
              <Link
                className="button button--secondary button--lg margin-horiz--sm"
                to="#"
              >
                Linux (x64)
              </Link>
              <Link
                className="button button--secondary button--lg margin-horiz--sm"
                to="#"
              >
                macOS (x64)
              </Link>
            </div>
          </div>
        </div>
        <HomepageFeatures />
        <div className="container margin-vert--xl">
          <div className="row">
            <div className="col col--8 col--offset--2">
              <div className="text--center">
                <h2>
                  🧠 Willkommen in der hypnotischen Welt der Programmierung
                </h2>
                <p className="text--lg">
                  HypnoScript verbindet hypnotische Konzepte mit moderner
                  Softwareentwicklung. Erlebe eine einzigartige Syntax, die
                  sowohl intuitiv als auch mächtig ist.
                </p>
              </div>
            </div>
          </div>

          <div className="row margin-vert--lg">
            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <h3>🚀 Schnellstart</h3>
                </div>
                <div className="card__body">
                  <p>
                    Beginne in wenigen Minuten mit HypnoScript. Lerne die
                    Grundlagen und erstelle dein erstes Programm.
                  </p>
                </div>
                <div className="card__footer">
                  <Link
                    className="button button--primary button--block"
                    to="/docs/getting-started/installation"
                  >
                    Installation
                  </Link>
                </div>
              </div>
            </div>

            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <h3>📚 Sprachreferenz</h3>
                </div>
                <div className="card__body">
                  <p>
                    Lerne die hypnotische Syntax kennen. Von Variablen über
                    Funktionen bis hin zu Sessions und Tranceify.
                  </p>
                </div>
                <div className="card__footer">
                  <Link
                    className="button button--primary button--block"
                    to="/docs/language-reference/syntax"
                  >
                    Syntax lernen
                  </Link>
                </div>
              </div>
            </div>

            <div className="col col--4">
              <div className="card">
                <div className="card__header">
                  <h3>🔧 Builtin-Funktionen</h3>
                </div>
                <div className="card__body">
                  <p>
                    Entdecke über 200+ eingebaute Funktionen für Arrays,
                    Strings, Mathematik, System und mehr.
                  </p>
                </div>
                <div className="card__footer">
                  <Link
                    className="button button--primary button--block"
                    to="/docs/builtins/overview"
                  >
                    Funktionen entdecken
                  </Link>
                </div>
              </div>
            </div>
          </div>

          <div className="row margin-vert--lg">
            <div className="col col--6">
              <div className="card">
                <div className="card__header">
                  <h3>💡 Beispiel-Code</h3>
                </div>
                <div className="card__body">
                  <pre className="language-hyp">
                    {`Focus {
    entrance {
        observe "Willkommen bei HypnoScript!";
    }

    induce name = "Welt";
    observe "Hallo, " + name + "!";

    induce numbers = [1, 2, 3, 4, 5];
    induce sum = SumArray(numbers);
    observe "Summe: " + sum;
} Relax;`}
                  </pre>
                </div>
              </div>
            </div>

            <div className="col col--6">
              <div className="card">
                <div className="card__header">
                  <h3>🎯 Hauptmerkmale</h3>
                </div>
                <div className="card__body">
                  <ul>
                    <li>
                      <strong>Hypnotische Syntax:</strong> Focus, Trance,
                      Induce, Observe, Relax
                    </li>
                    <li>
                      <strong>200+ Builtin-Funktionen:</strong> Arrays, Strings,
                      Math, System, etc.
                    </li>
                    <li>
                      <strong>Moderne Features:</strong> Arrays, Records,
                      Funktionen, Sessions
                    </li>
                    <li>
                      <strong>Enterprise-Ready:</strong> CLI, Tests, Debugging,
                      Deployment
                    </li>
                    <li>
                      <strong>Plattformübergreifend:</strong> Windows, macOS,
                      Linux
                    </li>
                    <li>
                      <strong>Open Source:</strong> MIT-Lizenz, aktive Community
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>

          <div className="row margin-vert--lg">
            <div className="col col--12">
              <div className="text--center">
                <h2>🤝 Community & Support</h2>
                <p className="text--lg">
                  Werde Teil der HypnoScript-Community und erhalte Hilfe bei der
                  Entwicklung.
                </p>
                <div className="margin-vert--md">
                  <Link
                    className="button button--secondary margin-horiz--sm"
                    to="https://github.com/hypnoscript/hyp-runtime"
                  >
                    GitHub Repository
                  </Link>
                  <Link
                    className="button button--secondary margin-horiz--sm"
                    to="https://github.com/hypnoscript/hyp-runtime/issues"
                  >
                    Issues melden
                  </Link>
                  <Link
                    className="button button--secondary margin-horiz--sm"
                    to="https://github.com/hypnoscript/hyp-runtime/discussions"
                  >
                    Diskussionen
                  </Link>
                </div>
              </div>
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}
