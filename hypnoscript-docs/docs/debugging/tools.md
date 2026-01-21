---
sidebar_position: 1
---

# Debugging-Tools

HypnoScript bietet umfassende Debugging-Funktionalitäten für die Entwicklung und Fehlerbehebung von Skripten.

## CLI Debug-Befehle

### Debug-Modus starten

```bash
# Debug-Modus starten
hypnoscript exec --debug script.hyp

# Mit detaillierter Ausgabe
hypnoscript exec --debug --verbose script.hyp

# Mit initialen Breakpoints
hypnoscript exec --debug --breakpoints 10,25,42 script.hyp

# Mit Watch-Expressions
hypnoscript exec --debug --watch counter,result script.hyp

# Mit Trace-Datei
hypnoscript exec --debug --trace-file debug.log script.hyp
```

### Kombinierte Optionen

```bash
# Vollständige Debug-Session mit allen Optionen
hypnoscript exec --debug \
    --breakpoints 10,25 \
    --watch x,y,result \
    --trace-file session.log \
    --verbose \
    script.hyp
```

## Debug Builtin-Funktionen

HypnoScript bietet mehrere eingebaute Funktionen für das Debugging:

### inspect(value)

Gibt eine detaillierte Repräsentation eines Wertes zurück, inklusive Typ-Information:

```hypnoscript
Focus
    induce arr = [1, 2, 3];
    observe(inspect(arr));
    // Ausgabe: Array[Int](3) = [1, 2, 3]

    induce obj = { name: "Test", value: 42 };
    observe(inspect(obj));
    // Ausgabe: Object { name: String = "Test", value: Int = 42 }
Relax
```

### typeOf(value)

Gibt den Typ eines Wertes als String zurück:

```hypnoscript
Focus
    observe(typeOf(42));       // "Int"
    observe(typeOf("Hello"));  // "String"
    observe(typeOf([1,2,3]));  // "Array"
    observe(typeOf(true));     // "Bool"
Relax
```

### stackTrace()

Gibt den aktuellen Call-Stack als String zurück:

```hypnoscript
Focus
    suggestion innerFunction() {
        observe(stackTrace());
    }

    suggestion outerFunction() {
        innerFunction();
    }

    outerFunction();
    // Ausgabe:
    // Call Stack:
    //   #0: innerFunction() at script.hyp:3
    //   #1: outerFunction() at script.hyp:7
    //   #2: <main> at script.hyp:10
Relax
```

### dump(value)

Gibt den Wert formatiert auf der Konsole aus und gibt ihn zurück (nützlich für Debugging in Ausdrücken):

```hypnoscript
Focus
    induce result = dump(calculateValue()) * 2;
    // Gibt calculateValue() aus und verwendet es weiter
Relax
```

## Assertion-Funktionen

### assertEqual(actual, expected, message?)

Prüft, ob zwei Werte gleich sind:

```hypnoscript
Focus
    induce result = calculate(5, 3);
    assertEqual(result, 8, "Addition sollte 8 ergeben");
Relax
```

### assertTruthy(value, message?)

Prüft, ob ein Wert als wahr ausgewertet wird:

```hypnoscript
Focus
    induce items = getItems();
    assertTruthy(ArrayLength(items) > 0, "Items sollten vorhanden sein");
Relax
```

## Timing-Funktionen

### time(label) / timeEnd(label)

Misst die Ausführungszeit zwischen zwei Punkten:

```hypnoscript
Focus
    time("operation");

    // Zeitaufwändige Operation
    induce result = complexCalculation();

    timeEnd("operation");
    // Ausgabe: operation: 123.45ms
Relax
```

### measureTime(label, callback)

Misst die Ausführungszeit einer Funktion:

```hypnoscript
Focus
    induce result = measureTime("sort", suggestion() {
        awaken sortArray(largeArray);
    });
    // Ausgabe: sort: 45.67ms
Relax
```

## Logging-Funktionen

### log(message) / warn(message) / error(message)

Verschiedene Log-Level für strukturierte Ausgaben:

```hypnoscript
Focus
    log("Info: Verarbeitung gestartet");
    warn("Warnung: Datei nicht gefunden, verwende Standard");
    error("Fehler: Ungültiger Eingabewert");
Relax
```

### trace(message)

Gibt eine Nachricht mit Stack-Trace aus:

```hypnoscript
Focus
    suggestion processItem(item) {
        trace("Verarbeite Item");
        // Ausgabe enthält aktuelle Position und Call-Stack
    }
Relax
```

### breakpoint()

Pausiert die Ausführung im Debug-Modus:

```hypnoscript
Focus
    induce x = 10;

    breakpoint();  // Pausiert hier wenn --debug aktiv ist

    induce y = x * 2;
Relax
```

## Interaktive Debug-Befehle

Im interaktiven Debug-Modus stehen diese Befehle zur Verfügung:

### Ausführungssteuerung

| Befehl     | Alias | Beschreibung                                  |
| ---------- | ----- | --------------------------------------------- |
| `continue` | `c`   | Bis zum nächsten Breakpoint fortfahren        |
| `step`     | `s`   | Eine Zeile ausführen (step into)              |
| `next`     | `n`   | Eine Zeile ausführen, Funktionen überspringen |
| `finish`   | `f`   | Bis zum Ende der Funktion laufen              |
| `run`      | `r`   | Ausführung neu starten                        |

### Variablen-Inspektion

| Befehl         | Alias      | Beschreibung                |
| -------------- | ---------- | --------------------------- |
| `locals`       | `l`        | Lokale Variablen anzeigen   |
| `globals`      | `g`        | Globale Variablen anzeigen  |
| `print <var>`  | `p <var>`  | Variable anzeigen           |
| `watch <expr>` | `w <expr>` | Watch-Expression hinzufügen |
| `watches`      |            | Alle Watches anzeigen       |

### Breakpoint-Verwaltung

| Befehl          | Alias      | Beschreibung              |
| --------------- | ---------- | ------------------------- |
| `break <line>`  | `b <line>` | Breakpoint setzen         |
| `delete <line>` | `d <line>` | Breakpoint löschen        |
| `breakpoints`   | `bl`       | Alle Breakpoints anzeigen |
| `clear`         |            | Alle Breakpoints löschen  |

### Navigation

| Befehl               | Alias | Beschreibung                   |
| -------------------- | ----- | ------------------------------ |
| `list`               |       | Quellcode um aktuelle Position |
| `list <start> <end>` |       | Quellcode-Bereich anzeigen     |
| `where`              | `bt`  | Call-Stack anzeigen            |
| `help`               |       | Hilfe anzeigen                 |
| `quit`               |       | Debug-Session beenden          |

## Debug-Trace-Datei

Mit `--trace-file` wird eine detaillierte Protokolldatei erstellt:

```bash
hypnoscript exec --debug --trace-file debug.log script.hyp
```

Die Trace-Datei enthält:

- **Ausgeführte Zeilen** mit Zeitstempeln
- **Variablenänderungen** bei jedem Schritt
- **Breakpoint-Treffer** mit Kontext
- **Call-Stack-Änderungen** bei Funktionsaufrufen
- **Timing-Informationen** für Performance-Analyse

### Beispiel Trace-Ausgabe

```bash
[00:00.001] EXEC   script.hyp:5   induce x = 10;
[00:00.001] VAR    x = Int(10)
[00:00.002] EXEC   script.hyp:6   induce y = 20;
[00:00.002] VAR    y = Int(20)
[00:00.003] BREAK  script.hyp:7   Breakpoint erreicht
[00:00.015] EXEC   script.hyp:7   induce result = x + y;
[00:00.015] VAR    result = Int(30)
[00:00.016] CALL   script.hyp:8   -> processResult()
[00:00.020] RET    script.hyp:8   <- processResult() = Null
```

## Best Practices

### 1. Debugging-Code entfernen

Entfernen Sie Debug-Funktionen vor dem Produktiveinsatz:

```hypnoscript
// Entwicklung
breakpoint();
dump(value);

// Produktion - diese Zeilen entfernen
```

### 2. Sinnvolle Timer-Labels

Verwenden Sie beschreibende Labels für Timing:

```hypnoscript
time("database-query");
time("json-parsing");
time("api-call-users");
```

### 3. Assertions für Tests

Nutzen Sie Assertions für automatisierte Tests:

```hypnoscript
Focus
    induce result = add(2, 3);
    assertEqual(result, 5, "add() sollte korrekt addieren");

    induce isEmpty = isListEmpty([]);
    assertTruthy(isEmpty, "Leere Liste sollte leer sein");
Relax
```

### 4. Trace-Dateien für Analyse

Nutzen Sie Trace-Dateien für Post-Mortem-Analyse:

```bash
# Trace erstellen
hypnoscript exec --debug --trace-file crash.log problematic.hyp

# Später analysieren
cat crash.log | grep "ERROR\|BREAK"
```
