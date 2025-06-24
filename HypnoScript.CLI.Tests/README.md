# HypnoScript.CLI.Tests – Teststrategie

Dieses Testprojekt stellt die Integrationstests für die wichtigsten CLI-Kommandos von HypnoScript bereit.

## Ziele

- Sicherstellen, dass die CLI-Kommandos (lint, benchmark, profile, optimize) mit echten Skripten korrekt funktionieren.
- Fehlerfälle und Grenzfälle automatisiert abdecken.
- Testdaten und -skripte sind im Verzeichnis `TestData` getrennt abgelegt.

## Teststruktur

- **TestData/**: Enthält Beispielskripte für valide und fehlerhafte HypnoScript-Programme.
- **UnitTest1.cs**: Enthält Integrationstests für die CLI-Kommandos. Jeder Test prüft den Rückgabewert und damit die Fehlererkennung.

## Erweiterung

- Weitere Tests für Grenzfälle (leere Datei, große Skripte, ungültige Syntax) können einfach ergänzt werden.
- Neue Kommandos sollten durch eigene Integrationstests abgedeckt werden.

## Ausführung

Die Tests können mit folgendem Befehl ausgeführt werden:

    dotnet test HypnoScript.CLI.Tests/HypnoScript.CLI.Tests.csproj
