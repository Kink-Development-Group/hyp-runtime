# Breakpoints

Breakpoints ermöglichen das Pausieren einer HypnoScript-Ausführung an präzisen Stellen, um Variablen und den Programmzustand zu inspizieren.

## Breakpoints über CLI setzen

### Mit dem --breakpoints Flag

Setzen Sie initiale Breakpoints beim Start des Debug-Modus:

```bash
# Einzelner Breakpoint auf Zeile 10
hypnoscript exec --debug --breakpoints 10 script.hyp

# Mehrere Breakpoints
hypnoscript exec --debug --breakpoints 10,25,42 script.hyp
```

### Interaktiv im Debug-Modus

Im interaktiven Debug-Modus können Breakpoints dynamisch gesetzt werden:

```bash
(hypno-debug) break 15
Breakpoint gesetzt auf Zeile 15

(hypno-debug) b 30
Breakpoint gesetzt auf Zeile 30
```

## Breakpoint-Befehle

| Befehl          | Alias      | Beschreibung                                      |
| --------------- | ---------- | ------------------------------------------------- |
| `break <line>`  | `b <line>` | Setzt einen Breakpoint auf der angegebenen Zeile  |
| `delete <line>` | `d <line>` | Entfernt den Breakpoint auf der angegebenen Zeile |
| `breakpoints`   | `bl`       | Zeigt alle aktiven Breakpoints an                 |
| `clear`         |            | Entfernt alle Breakpoints                         |

## Breakpoints auflisten

```bash
(hypno-debug) breakpoints
Aktive Breakpoints:
  Zeile 10
  Zeile 25
  Zeile 42

(hypno-debug) bl
Aktive Breakpoints:
  Zeile 10
  Zeile 25
  Zeile 42
```

## Breakpoints löschen

```bash
# Einzelnen Breakpoint löschen
(hypno-debug) delete 25
Breakpoint auf Zeile 25 entfernt

# Alias verwenden
(hypno-debug) d 42
Breakpoint auf Zeile 42 entfernt

# Alle Breakpoints löschen
(hypno-debug) clear
Alle Breakpoints entfernt
```

## Zustand bei Breakpoint inspizieren

Wenn die Ausführung an einem Breakpoint pausiert:

```bash
-> Breakpoint erreicht auf Zeile 10
   10 |   induce result = calculate(a, b);

(hypno-debug) locals
Lokale Variablen:
  a: Int = 42
  b: Int = 17

(hypno-debug) print result
result: Null (nicht initialisiert)

(hypno-debug) where
Call Stack:
  #0: processData() at script.hyp:10
  #1: main() at script.hyp:5
  #2: <entry> at script.hyp:1
```

## Stepping-Kontrolle

Nach einem Breakpoint können Sie die Ausführung steuern:

| Befehl     | Alias | Beschreibung                                              |
| ---------- | ----- | --------------------------------------------------------- |
| `step`     | `s`   | Führt eine Zeile aus und stoppt (step into)               |
| `next`     | `n`   | Führt eine Zeile aus, springt über Funktionen (step over) |
| `finish`   | `f`   | Läuft bis zum Ende der aktuellen Funktion (step out)      |
| `continue` | `c`   | Setzt die Ausführung bis zum nächsten Breakpoint fort     |

### Beispiel

```bash
-> Breakpoint erreicht auf Zeile 10
   10 |   induce result = calculate(a, b);

(hypno-debug) step
   -> Zeile 20 (in calculate())
   20 |   awaken a + b;

(hypno-debug) finish
   -> Zurück in Zeile 11
   11 |   observe(result);

(hypno-debug) continue
Ergebnis: 59
Programm beendet.
```

## Breakpoints im Code setzen

Sie können auch programmatisch Breakpoints mit der `breakpoint()` Builtin-Funktion setzen:

```hypnoscript
Focus
    induce x = 10;

    breakpoint();  // Pausiert hier wenn im Debug-Modus

    induce y = x * 2;
    observe(y);
Relax
```

Die `breakpoint()` Funktion wird nur im Debug-Modus aktiv und hat keine Auswirkung bei normaler Ausführung.

## Best Practices

1. **Strategische Platzierung**: Setzen Sie Breakpoints vor komplexen Berechnungen oder nach Funktionsaufrufen
2. **Sparsamer Einsatz**: Zu viele Breakpoints können das Debugging erschweren
3. **Watch-Expressions nutzen**: Kombinieren Sie Breakpoints mit `--watch` für automatische Variablenüberwachung
4. **Cleanup nicht vergessen**: Entfernen Sie `breakpoint()` Aufrufe vor dem Produktiveinsatz
