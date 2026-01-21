# Debug Mode

Der Debug-Modus bietet eine interaktive REPL-Umgebung für das Debugging von HypnoScript-Programmen mit Breakpoints, Schritt-für-Schritt-Ausführung und Variablen-Inspektion.

## Debug-Modus aktivieren

Starten Sie ein Skript mit dem `--debug` Flag:

```bash
hypnoscript exec --debug session.hyp
```

## CLI Debug-Optionen

| Option                  | Beschreibung                                                           |
| ----------------------- | ---------------------------------------------------------------------- |
| `--debug`               | Aktiviert den interaktiven Debug-Modus                                 |
| `--breakpoints <LINES>` | Setzt initiale Breakpoints auf den angegebenen Zeilen (z.B. `5,10,15`) |
| `--watch <VARS>`        | Überwacht Variablen während der Ausführung (z.B. `counter,result`)     |
| `--trace-file <FILE>`   | Speichert Debug-Trace in eine Datei                                    |
| `--verbose`             | Aktiviert ausführliche Ausgabe                                         |

### Beispiel mit Optionen

```bash
# Debug mit initialen Breakpoints und Watch-Expressions
hypnoscript exec --debug --breakpoints 5,12,25 --watch x,y,result myscript.hyp
```

## Debug-Befehle

Im interaktiven Debug-Modus stehen folgende Befehle zur Verfügung:

### Ausführungssteuerung

| Befehl     | Alias | Beschreibung                                              |
| ---------- | ----- | --------------------------------------------------------- |
| `continue` | `c`   | Setzt die Ausführung bis zum nächsten Breakpoint fort     |
| `step`     | `s`   | Führt eine Zeile aus und stoppt (step into)               |
| `next`     | `n`   | Führt eine Zeile aus, springt über Funktionen (step over) |
| `finish`   | `f`   | Läuft bis zum Ende der aktuellen Funktion (step out)      |
| `run`      | `r`   | Startet die Ausführung von Anfang an                      |

### Breakpoint-Verwaltung

| Befehl          | Alias      | Beschreibung                                      |
| --------------- | ---------- | ------------------------------------------------- |
| `break <line>`  | `b <line>` | Setzt einen Breakpoint auf der angegebenen Zeile  |
| `delete <line>` | `d <line>` | Entfernt den Breakpoint auf der angegebenen Zeile |
| `breakpoints`   | `bl`       | Zeigt alle aktiven Breakpoints an                 |
| `clear`         |            | Entfernt alle Breakpoints                         |

### Variablen-Inspektion

| Befehl         | Alias      | Beschreibung                     |
| -------------- | ---------- | -------------------------------- |
| `locals`       | `l`        | Zeigt alle lokalen Variablen an  |
| `globals`      | `g`        | Zeigt alle globalen Variablen an |
| `print <var>`  | `p <var>`  | Zeigt den Wert einer Variable an |
| `watch <expr>` | `w <expr>` | Fügt eine Watch-Expression hinzu |
| `unwatch <id>` |            | Entfernt eine Watch-Expression   |
| `watches`      |            | Zeigt alle Watch-Expressions an  |

### Quellcode-Anzeige

| Befehl               | Alias | Beschreibung                             |
| -------------------- | ----- | ---------------------------------------- |
| `list`               |       | Zeigt Quellcode um die aktuelle Position |
| `list <start> <end>` |       | Zeigt Quellcode von Zeile start bis end  |
| `where`              | `bt`  | Zeigt den aktuellen Call-Stack           |

### Sonstige Befehle

| Befehl | Beschreibung              |
| ------ | ------------------------- |
| `help` | Zeigt die Hilfe an        |
| `quit` | Beendet die Debug-Sitzung |

## Beispiel Debug-Sitzung

```bash
$ hypnoscript exec --debug examples/calculator.hyp

HypnoScript Debugger v1.2.0
Typ 'help' für verfügbare Befehle.

(hypno-debug) b 10
Breakpoint gesetzt auf Zeile 10

(hypno-debug) b 25
Breakpoint gesetzt auf Zeile 25

(hypno-debug) run
Starte Ausführung...

-> Breakpoint erreicht auf Zeile 10
   10 |   induce result = calculate(a, b);

(hypno-debug) locals
Lokale Variablen:
  a: Int = 42
  b: Int = 17

(hypno-debug) step
   11 |   observe("Ergebnis: " + result);

(hypno-debug) print result
result = 59

(hypno-debug) continue
Ergebnis: 59

-> Breakpoint erreicht auf Zeile 25

(hypno-debug) where
Call Stack:
  #0: main() at calculator.hyp:25
  #1: <entry> at calculator.hyp:1

(hypno-debug) continue
Programm beendet.

(hypno-debug) quit
```

## Quellcode-Kontext

Wenn die Ausführung pausiert, zeigt der Debugger den Quellcode-Kontext mit der aktuellen Position an:

```hyp
    8 |   induce a = 42;
    9 |   induce b = 17;
 -> 10 |   induce result = calculate(a, b);
   11 |   observe("Ergebnis: " + result);
   12 | }
```

Der Pfeil `->` markiert die aktuelle Ausführungsposition.

## Watch-Expressions

Watch-Expressions werden automatisch bei jedem Stopp ausgewertet:

```bash
(hypno-debug) watch counter
Watch #1 hinzugefügt: counter

(hypno-debug) watch result * 2
Watch #2 hinzugefügt: result * 2

(hypno-debug) step
   15 |   counter = counter + 1;

Watch Values:
  #1 counter = 5
  #2 result * 2 = 118
```

## Trace-Datei

Mit `--trace-file` können Sie die Debug-Ausführung in eine Datei protokollieren:

```bash
hypnoscript exec --debug --trace-file debug.log myscript.hyp
```

Die Trace-Datei enthält:

- Ausgeführte Zeilen mit Zeitstempeln
- Variablenänderungen
- Breakpoint-Treffer
- Call-Stack-Änderungen

## Performance-Hinweise

Der Debug-Modus verlangsamt die Ausführung erheblich, da jede Instruktion überwacht wird. Verwenden Sie ihn für:

- Entwicklung und Fehlersuche
- Verstehen des Programmablaufs
- Testen neuer Features

Für produktive Ausführung deaktivieren Sie den Debug-Modus.
