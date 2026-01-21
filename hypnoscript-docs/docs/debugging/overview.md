# Debugging Overview

HypnoScript bietet umfassende Debugging-Funktionen, um Fehler in Ihren Skripten zu identifizieren und zu beheben.

## Debugging-Features

### 1. Interaktiver Debug-Modus

Der Debug-Modus startet eine interaktive REPL-Session mit voller Kontrolle über die Ausführung:

```bash
# Debug-Modus starten
hypnoscript exec script.hyp --debug

# Mit initialen Breakpoints
hypnoscript exec script.hyp --debug --breakpoints 10,25,42

# Mit Watch-Expressions
hypnoscript exec script.hyp --debug --watch counter,result
```

Im interaktiven Modus können Sie:

- Breakpoints setzen und entfernen
- Schritt-für-Schritt durch den Code navigieren
- Variablen inspizieren
- Den Call-Stack anzeigen

### 2. Builtin Debug-Funktionen

HypnoScript enthält mehrere eingebaute Funktionen für das Debugging:

```hyp
Focus
    // Detaillierte Wert-Inspektion
    induce data = { name: "Test", value: 42 };
    observe(inspect(data));

    // Typ-Überprüfung
    observe(typeOf(data));  // "Object"

    // Stack-Trace ausgeben
    observe(stackTrace());

    // Debug-Ausgaben mit verschiedenen Leveln
    log("Info-Nachricht");
    warn("Warnung");
    error("Fehler");
    trace("Mit Stack-Trace");

    // Assertions für Tests
    assertEqual(1 + 1, 2, "Mathe sollte funktionieren");
    assertTruthy(data.value > 0, "Wert sollte positiv sein");

    // Performance-Messung
    time("operation");
    // ... Code ...
    timeEnd("operation");  // Gibt Zeit aus

    // Programmatischer Breakpoint
    breakpoint();  // Pausiert im Debug-Modus
Relax
```

### 3. CLI Debug-Optionen

| Option                  | Beschreibung                                           |
| ----------------------- | ------------------------------------------------------ |
| `--debug`               | Aktiviert den interaktiven Debug-Modus                 |
| `--verbose`             | Zeigt zusätzliche Informationen während der Ausführung |
| `--breakpoints <LINES>` | Setzt initiale Breakpoints (komma-separiert)           |
| `--watch <VARS>`        | Überwacht Variablen (komma-separiert)                  |
| `--trace-file <FILE>`   | Speichert Debug-Trace in eine Datei                    |

### 4. Debug-Befehle

Im interaktiven Debug-Modus stehen diese Befehle zur Verfügung:

#### Ausführungssteuerung

- `continue` / `c` - Bis zum nächsten Breakpoint fortfahren
- `step` / `s` - Eine Zeile ausführen (step into)
- `next` / `n` - Eine Zeile ausführen, Funktionen überspringen
- `finish` / `f` - Bis zum Ende der Funktion laufen
- `run` / `r` - Ausführung neu starten

#### Breakpoint-Verwaltung

- `break <line>` / `b <line>` - Breakpoint setzen
- `delete <line>` / `d <line>` - Breakpoint löschen
- `breakpoints` / `bl` - Alle Breakpoints anzeigen
- `clear` - Alle Breakpoints löschen

#### Variablen-Inspektion

- `locals` / `l` - Lokale Variablen anzeigen
- `globals` / `g` - Globale Variablen anzeigen
- `print <var>` / `p <var>` - Variable anzeigen
- `watch <expr>` / `w <expr>` - Watch-Expression hinzufügen

#### Navigation

- `list` - Quellcode um aktuelle Position
- `where` / `bt` - Call-Stack anzeigen
- `help` - Hilfe anzeigen
- `quit` - Debug-Session beenden

### 5. Trace-Dateien

Mit `--trace-file` können Sie eine detaillierte Protokolldatei erstellen:

```bash
hypnoscript exec script.hyp --debug --trace-file debug.log
```

Die Trace-Datei enthält:

- Ausgeführte Zeilen mit Zeitstempeln
- Variablenänderungen
- Breakpoint-Treffer
- Call-Stack-Änderungen

### 6. Fehlerberichterstattung

HypnoScript bietet detaillierte Fehlerberichte mit:

- **Zeilennummern und Datei-Positionen**
- **Stack-Traces** für Funktionsaufrufe
- **Typ-Informationen** für Variablen
- **Kontext-Informationen** für besseres Verständnis

## Beispiel Debug-Session

```hyp
$ hypnoscript exec calculator.hyp --debug --breakpoints 10

HypnoScript Debugger v1.2.0
Type 'help' for available commands.

(hypno-debug) run
Starting execution...

-> Breakpoint hit at line 10
   10 |   induce result = add(a, b);

(hypno-debug) locals
Local variables:
  a: Int = 5
  b: Int = 3

(hypno-debug) step
   -> Line 15 (in add())
   15 |   awaken x + y;

(hypno-debug) print x
x = 5

(hypno-debug) finish
   -> Back in line 11
   11 |   observe(result);

(hypno-debug) print result
result = 8

(hypno-debug) continue
8
Program finished.
```

## Weitere Dokumentation

- [Debug-Modus](./debug-mode) - Vollständige Befehlsreferenz
- [Breakpoints](./breakpoints) - Detaillierte Breakpoint-Dokumentation
- [Debugging-Tools](./tools) - Alle Builtin-Funktionen
- [Best Practices](./best-practices) - Tipps für effektives Debugging
- [Troubleshooting](./troubleshooting) - Häufige Probleme lösen
