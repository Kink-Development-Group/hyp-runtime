# What is HypnoScript?

HypnoScript ist eine domänenspezielle, statisch typisierte Skriptsprache, die hypnotische Sessions, mentale Trainingssequenzen und interaktive Suggestionen reproduzierbar macht. Im Gegensatz zu generischen Automations-Frameworks modelliert HypnoScript alle Schritte einer Session – von der Einleitung bis zum sicheren Ausstieg – als erstklassige Sprachelemente. Dadurch entsteht eine gemeinsam nutzbare Grundlage für Therapeut:innen, Creator und Tool-Entwickler:innen.

## Leitlinien der Sprache

- **Sicherheit zuerst** – Jede Session läuft in einer sand-boxed Runtime und erzwingt Backout-Sequenzen, Timeout-Überwachung sowie Sicherheitsnetze gegen widersprüchliche Suggestionen.
- **Determinismus** – Runs sind reproduzierbar. Zufallsquellen, Zeitfunktionen und externe Integrationen können über Seeds oder Mocking kontrolliert werden.
- **Erklärbarkeit** – Jede Hypnose-Aktion hinterlässt strukturierte Telemetrie. Logs, Visualisierungen und Timeline-Replays helfen bei Training, Compliance und QA.
- **Modularität** – Trance-Bausteine, Suggestionen und Ausleitungsprotokolle lassen sich als wiederverwendbare Bibliotheken versionieren.

## Sprache auf einen Blick

| Element            | Beschreibung                                                                                                      |
| ------------------ | ----------------------------------------------------------------------------------------------------------------- |
| `Focus { ... }`    | Oberster Block einer Session, definiert Ablauf, Variablen und Sicherheitsnetze.                                   |
| `entrance { ... }` | Einleitungsphase. Hier werden Rapport, Atmung, Trigger und vorbereitende Hinweise orchestriert.                   |
| `induce`           | Deklariert Variablen inkl. Typ und initialem Suggestion-Wert.                                                     |
| `observe`          | Sendet Suggestionen oder Debug-Informationen an Klient:innen, Tests oder Logs.                                    |
| `deepFocus {}`     | Leitet eine Vertiefungsphase ein. Variiert je nach Protokoll (z. B. Countdown, Stufen, Fractionation).            |
| `Relax`            | Terminatorblock, sorgt immer für sichere Ausleitung, egal ob die Session regulär endet oder über Fehler abbricht. |

HypnoScript nutzt eine vertraute, blockorientierte Syntax mit geschweiften Klammern. Typannotationen, Kontrollstrukturen und Funktionsaufrufe orientieren sich an moderner Skript-Sprache, bleiben aber bewusst lesbar.

## Beispiel: Geführte Session mit Sicherheitsnetz

```hypnoscript
Focus {
    entrance {
        observe "Willkommen, heute arbeiten wir an tiefer Entspannung.";
    }

    induce depth: number = 0;
    induce affirmations: array = [
        "Dein Atem bleibt ruhig und gleichmäßig.",
        "Jede Ausatmung vertieft deine Entspannung."
    ];

    deepFocus {
        loop each suggestion in affirmations {
            observe suggestion;
            depth = depth + 1;
        }
    }

    on warn (event) {
        log "Warnung: " + event.message;
        suggest safety.reset();
    }

    Relax {
        observe "Du kehrst vollkommen klar und erfrischt zurück.";
        guard ensureAwake();
    }
} Relax
```

Das Beispiel kombiniert Kontrollstrukturen (`loop`), Typannotationen und eingebettete Sicherheitslogik (`on warn`). Die Session endet garantiert mit dem `Relax`-Block und ruft eine Schutz-Routine, sobald eine Warnung auftritt.

## Komponenten des HypnoScript-Ökosystems

- **Compiler & Type Checker** – Validiert Sessions, sorgt für statische Sicherheit und erzeugt optimierte Bytecode-Pipelines.
- **Runtime** – Führt Skripte deterministisch aus, verwaltet Suggestion-Queues, externe Hooks (Audio, Biofeedback) und Telemetrie.
- **CLI** – Startet Skripte (`hyp run`), führt Tests (`hyp test`), leitet Debug-Sitzungen (`hyp debug`) und exportiert Telemetrie.
- **Editor-Integrationen** – VS Code Extension für Syntax-Highlighting, Autovervollständigung, Linting und Timeline-Replay.
- **Testing Framework** – Ermöglicht Smoke-, Regression- und Compliance-Tests mit vordefinierten HypnoScript-Szenarien.

## Typische Anwendungsfälle

- **Therapeutische Skripte** – Standardisierte Induktionsabläufe und Protokolle samt Sicherheitsleitplanken.
- **Unterhaltungs- & Lerninhalte** – Interaktive Hypnose-Erlebnisse oder Gamification-Events mit verzweigten Szenen.
- **Automatisiertes Feedback** – Biofeedback-Geräte oder Sensoren lassen sich einbinden und lösen Suggestionen dynamisch aus.
- **Hypnose-Training** – Simulierte Sessions für Coaching, inklusive Debug-Logs, Breakpoints und Replay.

## Weiterführende Ressourcen

- [Core Concepts](./core-concepts) – Fundamentale Sprachelemente und Ausführungsmodell
- [Installation](./installation) – Starte lokal mit CLI und Runtime
- [Quick Start](./quick-start) – Erste Session in weniger als zehn Minuten
- [Language Reference](../language-reference/syntax) – Vollständige Syntax und Standardbibliotheken

HypnoScript hilft dabei, hypnotische Abläufe transparent, sicher und wiederholbar zu gestalten – ohne die Kreativität oder Individualität einer Session einzuschränken.
