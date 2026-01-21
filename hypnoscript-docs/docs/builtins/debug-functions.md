---
sidebar_position: 18
---

# Debug Functions

Diese Funktionen helfen bei der Entwicklung und Fehlerbehebung von HypnoScript-Programmen.

## Inspektion

### inspect(value)

Gibt eine detaillierte Repräsentation eines Wertes zurück, inklusive Typ-Information.

```hyp
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

Gibt den Typ eines Wertes als String zurück.

```hyp
Focus
    observe(typeOf(42));       // "Int"
    observe(typeOf("Hello"));  // "String"
    observe(typeOf([1,2,3]));  // "Array"
    observe(typeOf(true));     // "Bool"
    observe(typeOf(null));     // "Null"
Relax
```

### stackTrace()

Gibt den aktuellen Call-Stack als String zurück.

```hyp
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

Gibt den Wert formatiert aus und gibt ihn zurück. Nützlich für Debugging in Ausdrücken.

```hyp
Focus
    // Inline-Debugging
    induce result = dump(calculate(5, 3)) * 2;
    // Gibt calculate(5, 3) aus und verwendet das Ergebnis weiter
Relax
```

## Assertions

### assertEqual(actual, expected, message?)

Prüft, ob zwei Werte gleich sind. Wirft einen Fehler wenn nicht.

```hyp
Focus
    induce result = add(2, 3);
    assertEqual(result, 5);                    // Ohne Nachricht
    assertEqual(result, 5, "add sollte 5 ergeben");  // Mit Nachricht
Relax
```

### assertTruthy(value, message?)

Prüft, ob ein Wert als wahr ausgewertet wird.

```hyp
Focus
    induce items = getItems();
    assertTruthy(ArrayLength(items) > 0, "Sollte Items enthalten");

    induce user = getCurrentUser();
    assertTruthy(user, "User sollte existieren");
Relax
```

## Timing

### time(label)

Startet einen Timer mit dem angegebenen Label.

```hyp
Focus
    time("database-query");

    // Zeitaufwändige Operation...
    induce result = queryDatabase();
Relax
```

### timeEnd(label)

Beendet den Timer und gibt die vergangene Zeit aus.

```hyp
Focus
    time("operation");

    induce result = complexCalculation();

    timeEnd("operation");
    // Ausgabe: operation: 123.45ms
Relax
```

### measureTime(label, callback)

Misst die Ausführungszeit einer Funktion und gibt das Ergebnis zurück.

```hyp
Focus
    induce sorted = measureTime("sort", suggestion() {
        awaken sortArray(largeArray);
    });
    // Ausgabe: sort: 45.67ms
    // sorted enthält das sortierte Array
Relax
```

## Logging

### log(message)

Gibt eine Info-Nachricht aus.

```hyp
Focus
    log("Verarbeitung gestartet");
    log("Schritt 1 abgeschlossen");
Relax
```

### warn(message)

Gibt eine Warnung aus.

```hyp
Focus
    warn("Konfiguration nicht gefunden, verwende Standard");
    warn("Deprecated API-Aufruf");
Relax
```

### error(message)

Gibt eine Fehlermeldung aus.

```hyp
Focus
    error("Kritischer Fehler: Datei nicht gefunden");
Relax
```

### trace(message)

Gibt eine Nachricht mit Stack-Trace aus.

```hyp
Focus
    suggestion processItem(item) {
        trace("Verarbeite Item");
        // Ausgabe enthält Nachricht + aktuellen Call-Stack
    }
Relax
```

## Breakpoints

### breakpoint()

Pausiert die Ausführung wenn im Debug-Modus (`--debug`).

```hyp
Focus
    induce x = 10;

    breakpoint();  // Pausiert hier im Debug-Modus

    induce y = x * 2;
    observe(y);
Relax
```

Bei normaler Ausführung (ohne `--debug`) hat diese Funktion keine Auswirkung.

## Verwendung

### Debugging aktivieren

```bash
# Debug-Modus starten
hypnoscript exec script.hyp --debug

# Mit initialen Breakpoints
hypnoscript exec script.hyp --debug --breakpoints 10,25

# Mit Watch-Expressions
hypnoscript exec script.hyp --debug --watch counter,result
```

### Debugging-Code entfernen

Vor dem Produktiveinsatz sollten Debug-Aufrufe entfernt werden:

```hyp
// Entwicklung
breakpoint();
dump(value);
time("operation");
timeEnd("operation");

// Diese Zeilen vor Production entfernen
```

## Siehe auch

- [Debug-Modus](../debugging/debug-mode) - Interaktiver Debugger
- [Breakpoints](../debugging/breakpoints) - Breakpoint-Verwaltung
- [Debugging-Tools](../debugging/tools) - Vollständige Referenz
