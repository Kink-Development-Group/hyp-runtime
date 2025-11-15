# Changelog

Alle wesentlichen Änderungen an diesem Projekt werden in dieser Datei festgehalten. Das Format orientiert sich an [Keep a Changelog](https://keepachangelog.com/de/1.1.0/) und alle Versionen folgen [Semantic Versioning](https://semver.org/lang/de/).

## [1.0.0] - 2025-11-15

### Added

- Erstveröffentlichung des vollständigen **HypnoScript**-Stacks mit Compiler (`hypnoscript-compiler`), Laufzeit (`hypnoscript-runtime`) und Kernbibliothek (`hypnoscript-core`).
- Integration des Cranelift-Backends zur nativen Codegenerierung inkl. Linker-Workflow und Plattformunterstützung für Linux, Windows und macOS.
- Umfangreiche CLI (`hypnoscript-cli`) mit Befehlen zum Ausführen von Skripten, Testläufen, Builtin-Auflistung und Versionsausgabe.
- Asynchrones Runtime-Ökosystem mit Promise-Unterstützung, Kanal-System und erweiterten Builtins (Strings, Arrays, Dateien, Hashing, Lokalisierung u. v. m.).
- Vollständige Sprachdokumentation mit VitePress, inklusive Getting-Started-Leitfäden, Sprachreferenz, Builtin-Katalog und Enterprise-Kapitel.
- Automatisierte Build- und Release-Skripte für Linux, Windows (Winget) und macOS (Universal/x64/arm64, pkg & dmg).

### Changed

- Konsolidierte Typprüfung, Parser-Verbesserungen und Iterator-basierte Implementierungen zur Einhaltung der strengen `cargo clippy`-Warnungsrichtlinien.
- Vereinheitlichter Umgang mit Linker-Argumenten, Record-Typen und Funktionssignaturen, um stabile Release-Builds über das gesamte Workspace zu gewährleisten.

### Fixed

- Behebung von Borrow-Checker-Problemen im nativen Codegenerator und Stabilisierung der Channel-Synchronisation im Async-Runtime-Modul.
- Reduzierte Fehler- und Warnmeldungen in Interpreter, Optimizer und Parser durch gezielte Refactorings.
- Ergänzung der fehlenden Type-System-Dokumentation sowie Korrektur nicht erreichbarer Dokumentationslinks (z. B. `language-reference/types.html`).

### Security & Compliance

- Aktualisierte `deny.toml`, einschließlich MPL-2.0-Lizenzausnahme für `webpki-roots` und Ignorierung des dokumentierten Advisories `RUSTSEC-2020-0168`.
- Erfolgreicher Abschluss von `cargo deny check` mit bereinigten Lizenz- und Advisory-Prüfungen.

[1.0.0]: https://github.com/Kink-Development-Group/hyp-runtime/releases/tag/1.0.0
