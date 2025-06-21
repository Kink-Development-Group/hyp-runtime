---
sidebar_position: 1
---

# Debugging-Tools

HypnoScript bietet umfassende Debugging-Funktionalitäten für die Entwicklung und Fehlerbehebung von Skripten.

## Debug-Modi

### Grundlegender Debug-Modus

```bash
# Debug-Modus starten
dotnet run --project HypnoScript.CLI -- debug script.hyp

# Mit detaillierter Ausgabe
dotnet run --project HypnoScript.CLI -- debug script.hyp --verbose

# Mit Timeout
dotnet run --project HypnoScript.CLI -- debug script.hyp --timeout 60
```

### Schritt-für-Schritt-Debugging

```bash
# Schritt-für-Schritt-Ausführung
dotnet run --project HypnoScript.CLI -- debug script.hyp --step

# Mit Variablen-Anzeige
dotnet run --project HypnoScript.CLI -- debug script.hyp --step --variables

# Mit Call-Stack
dotnet run --project HypnoScript.CLI -- debug script.hyp --step --call-stack
```

### Trace-Modus

```bash
# Ausführungs-Trace aktivieren
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace

# Trace in Datei speichern
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --output trace.log

# Detaillierter Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --detailed
```

## Breakpoints

### Breakpoint-Datei erstellen

```txt
# breakpoints.txt
10          # Zeile 10
25          # Zeile 25
math.hyp:15 # Zeile 15 in math.hyp
utils.hyp:* # Alle Zeilen in utils.hyp
```

### Breakpoints verwenden

```bash
# Mit Breakpoint-Datei
dotnet run --project HypnoScript.CLI -- debug script.hyp --breakpoints breakpoints.txt

# Interaktive Breakpoints
dotnet run --project HypnoScript.CLI -- debug script.hyp --interactive

# Bedingte Breakpoints
dotnet run --project HypnoScript.CLI -- debug script.hyp --breakpoints conditional.txt
```

### Bedingte Breakpoints

```txt
# conditional.txt
10:result > 100          # Zeile 10, wenn result > 100
15:IsEmpty(input)        # Zeile 15, wenn input leer ist
20:ArrayLength(arr) == 0 # Zeile 20, wenn Array leer ist
```

## Variablen-Inspektion

### Variablen anzeigen

```bash
# Alle Variablen anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --variables

# Spezifische Variablen überwachen
dotnet run --project HypnoScript.CLI -- debug script.hyp --watch "result,sum,total"

# Variablen-Historie
dotnet run --project HypnoScript.CLI -- debug script.hyp --variable-history
```

### Variablen-Monitoring

```bash
# Variablen in Echtzeit überwachen
dotnet run --project HypnoScript.CLI -- debug script.hyp --monitor-variables

# Variablen-Änderungen loggen
dotnet run --project HypnoScript.CLI -- debug script.hyp --log-variables --output var-changes.log
```

## Call-Stack und Performance

### Call-Stack-Analyse

```bash
# Call-Stack anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --call-stack

# Detaillierter Call-Stack
dotnet run --project HypnoScript.CLI -- debug script.hyp --call-stack --detailed

# Call-Stack in Datei
dotnet run --project HypnoScript.CLI -- debug script.hyp --call-stack --output stack.log
```

### Performance-Profiling

```bash
# Performance-Profiling aktivieren
dotnet run --project HypnoScript.CLI -- debug script.hyp --profile

# Profiling-Report generieren
dotnet run --project HypnoScript.CLI -- debug script.hyp --profile --output profile.json

# Memory-Profiling
dotnet run --project HypnoScript.CLI -- debug script.hyp --profile --memory
```

## Debugging-Befehle

### Interaktive Debugging-Befehle

```bash
# Debug-Session starten
dotnet run --project HypnoScript.CLI -- debug script.hyp --interactive

# Verfügbare Befehle:
# continue (c)     - Weiter ausführen
# step (s)         - Nächste Zeile
# next (n)         - Nächste Anweisung
# break (b)        - Breakpoint setzen
# variables (v)    - Variablen anzeigen
# stack (st)       - Call-Stack anzeigen
# quit (q)         - Beenden
```

### Beispiel für interaktive Session

```bash
$ dotnet run --project HypnoScript.CLI -- debug script.hyp --interactive

HypnoScript Debugger v1.0
> break 15
Breakpoint set at line 15
> continue
Stopped at line 15: induce result = a + b;
> variables
a = 5
b = 3
> step
Stopped at line 16: observe "Ergebnis: " + result;
> variables
a = 5
b = 3
result = 8
> continue
Ergebnis: 8
Debug session ended.
```

## Debugging in der Praxis

### Einfaches Debugging-Beispiel

```hyp
Focus {
    entrance {
        induce a = 5;
        induce b = 3;

        // Debug-Punkt 1: Werte prüfen
        observe "Debug: a = " + a + ", b = " + b;

        induce result = a + b;

        // Debug-Punkt 2: Ergebnis prüfen
        observe "Debug: result = " + result;

        if (result > 10) {
            observe "Ergebnis ist größer als 10";
        } else {
            observe "Ergebnis ist kleiner oder gleich 10";
        }
    }
} Relax;
```

### Debugging mit Breakpoints

```hyp
Focus {
    Trance calculateSum(a, b) {
        // Breakpoint hier setzen
        induce sum = a + b;
        return sum;
    }

    entrance {
        induce x = 10;
        induce y = 20;

        // Breakpoint hier setzen
        induce total = calculateSum(x, y);

        observe "Summe: " + total;
    }
} Relax;
```

### Debugging mit Trace

```hyp
Focus {
    entrance {
        observe "=== Debug-Trace Start ===";

        induce numbers = [1, 2, 3, 4, 5];
        observe "Debug: Array erstellt: " + numbers;

        induce sum = 0;
        observe "Debug: Summe initialisiert: " + sum;

        for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
            induce num = ArrayGet(numbers, i);
            induce oldSum = sum;
            induce sum = sum + num;
            observe "Debug: i=" + i + ", num=" + num + ", " + oldSum + " + " + num + " = " + sum;
        }

        observe "Debug: Finale Summe: " + sum;
        observe "=== Debug-Trace Ende ===";
    }
} Relax;
```

## Erweiterte Debugging-Features

### Memory-Debugging

```bash
# Memory-Usage überwachen
dotnet run --project HypnoScript.CLI -- debug script.hyp --memory-tracking

# Memory-Leaks erkennen
dotnet run --project HypnoScript.CLI -- debug script.hyp --memory-leak-detection

# Memory-Report generieren
dotnet run --project HypnoScript.CLI -- debug script.hyp --memory-report --output memory.json
```

### Exception-Debugging

```bash
# Exception-Details anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --exception-details

# Exception-Handling debuggen
dotnet run --project HypnoScript.CLI -- debug script.hyp --exception-tracking

# Exception-Stack-Trace
dotnet run --project HypnoScript.CLI -- debug script.hyp --stack-trace
```

### Thread-Debugging

```bash
# Thread-Informationen anzeigen
dotnet run --project HypnoScript.CLI -- debug script.hyp --thread-info

# Thread-Switches verfolgen
dotnet run --project HypnoScript.CLI -- debug script.hyp --thread-tracking

# Deadlock-Erkennung
dotnet run --project HypnoScript.CLI -- debug script.hyp --deadlock-detection
```

## Debugging-Konfiguration

### Debug-Konfiguration in hypnoscript.config.json

```json
{
  "debugging": {
    "enabled": true,
    "breakOnError": true,
    "showVariables": true,
    "showCallStack": true,
    "traceExecution": false,
    "memoryTracking": false,
    "profiling": {
      "enabled": false,
      "output": "profile.json"
    },
    "breakpoints": {
      "file": "breakpoints.txt",
      "conditional": true
    },
    "logging": {
      "level": "debug",
      "output": "debug.log"
    }
  }
}
```

### Debug-Umgebungsvariablen

```bash
# Debug-spezifische Umgebungsvariablen
export HYPNOSCRIPT_DEBUG=true
export HYPNOSCRIPT_DEBUG_LEVEL=verbose
export HYPNOSCRIPT_BREAK_ON_ERROR=true
export HYPNOSCRIPT_SHOW_VARIABLES=true
export HYPNOSCRIPT_TRACE_EXECUTION=true
```

## Debugging-Workflows

### Entwicklungsworkflow mit Debugging

```bash
#!/bin/bash
# debug-workflow.sh

echo "=== HypnoScript Debug Workflow ==="

# 1. Syntax prüfen
echo "1. Validating syntax..."
dotnet run --project HypnoScript.CLI -- validate script.hyp

# 2. Debug-Modus mit Trace
echo "2. Running in debug mode..."
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --output debug.log

# 3. Performance-Profiling
echo "3. Performance profiling..."
dotnet run --project HypnoScript.CLI -- debug script.hyp --profile --output profile.json

# 4. Memory-Analyse
echo "4. Memory analysis..."
dotnet run --project HypnoScript.CLI -- debug script.hyp --memory-tracking --output memory.json

echo "Debug workflow completed!"
```

### Automatisierte Debugging-Tests

```bash
#!/bin/bash
# auto-debug.sh

echo "=== Automated Debugging ==="

# Debug-Modus mit allen Features
dotnet run --project HypnoScript.CLI -- debug script.hyp \
    --trace \
    --profile \
    --memory-tracking \
    --variables \
    --call-stack \
    --output debug-complete.log

# Ergebnisse analysieren
echo "Debug results saved to debug-complete.log"
```

## Best Practices

### Effektives Debugging

```hyp
// 1. Strategische Breakpoints setzen
Focus {
    entrance {
        induce input = "test";

        // Breakpoint 1: Eingabe validieren
        if (IsEmpty(input)) {
            observe "Fehler: Leere Eingabe";
            return;
        }

        // Breakpoint 2: Verarbeitung
        induce processed = ToUpper(input);

        // Breakpoint 3: Ergebnis prüfen
        observe "Verarbeitet: " + processed;
    }
} Relax;
```

### Debugging-Logging

```hyp
// 2. Strukturiertes Debug-Logging
Focus {
    Trance debugLog(message, data) {
        induce timestamp = Now();
        observe "[" + timestamp + "] DEBUG: " + message + " = " + data;
    }

    entrance {
        debugLog("Start", "Skript beginnt");

        induce result = 42;
        debugLog("Berechnung", result);

        debugLog("Ende", "Skript beendet");
    }
} Relax;
```

### Performance-Debugging

```hyp
// 3. Performance-kritische Bereiche debuggen
Focus {
    entrance {
        induce startTime = Timestamp();

        // Performance-kritischer Code
        for (induce i = 0; i < 1000; induce i = i + 1) {
            induce result = Pow(i, 2);
        }

        induce endTime = Timestamp();
        induce duration = endTime - startTime;

        if (duration > 1.0) {
            observe "WARNUNG: Langsame Ausführung (" + duration + "s)";
        }
    }
} Relax;
```

## Troubleshooting

### Häufige Debugging-Probleme

1. **Breakpoints werden ignoriert**

   ```bash
   # Prüfen Sie die Zeilennummern
   cat -n script.hyp

   # Verwenden Sie absolute Pfade
   dotnet run --project HypnoScript.CLI -- debug /absolute/path/script.hyp
   ```

2. **Variablen werden nicht angezeigt**

   ```bash
   # Debug-Modus mit expliziter Variablen-Anzeige
   dotnet run --project HypnoScript.CLI -- debug script.hyp --variables --verbose

   # Variablen-Scope prüfen
   dotnet run --project HypnoScript.CLI -- debug script.hyp --variable-scope
   ```

3. **Trace-Datei ist zu groß**

   ```bash
   # Selektives Tracing
   dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --filter "function1,function2"

   # Trace komprimieren
   dotnet run --project HypnoScript.CLI -- debug script.hyp --trace --compressed
   ```

## Nächste Schritte

- [Debugging-Best-Practices](./best-practices) - Erweiterte Debugging-Techniken
- [Performance-Debugging](./performance) - Performance-Optimierung
- [Error-Handling](../error-handling/overview) - Fehlerbehandlung
- [Enterprise-Debugging](../enterprise/debugging) - Enterprise-Debugging-Tools

---

**Debugging-Tools gemeistert? Dann lerne [Debugging-Best-Practices](./best-practices) kennen!** 🔍
