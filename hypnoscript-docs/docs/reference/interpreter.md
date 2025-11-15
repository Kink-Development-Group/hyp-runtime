---
sidebar_position: 3
---

# Interpreter

Der HypnoScript-Interpreter ist das Herzstück der Runtime und verarbeitet HypnoScript-Code zur Laufzeit.

## Architektur

### Komponenten

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Lexer         │    │   Parser        │    │   Interpreter   │
│                 │    │                 │    │                 │
│ - Tokenisierung │───▶│ - AST-Erstellung│───▶│ - Code-Ausführung│
│ - Syntax-Check  │    │ - Semantik-Check│    │ - Session-Mgmt  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Verarbeitungspipeline

1. **Lexer**: Zerlegt Quellcode in Tokens
2. **Parser**: Erstellt Abstract Syntax Tree (AST)
3. **Interpreter**: Führt AST aus

## Interpreter-Features

### Dynamische Typisierung

```hyp
// Variablen können ihren Typ zur Laufzeit ändern
induce x = 42;        // Integer
induce x = "Hallo";   // String
induce x = [1,2,3];   // Array
```

### Session-Management

```hyp
// Sessions werden automatisch verwaltet
induce session = Session("MeineSession");
SessionSet(session, "key", "value");
induce value = SessionGet(session, "key");
```

### Fehlerbehandlung

```hyp
// Robuste Fehlerbehandlung
if (ArrayLength(arr) > 0) {
    induce element = ArrayGet(arr, 0);
} else {
    observe "Array ist leer";
}
```

## Interpreter-Konfiguration

### Memory Management

```json
{
  "maxMemory": 512,
  "gcThreshold": 0.8,
  "stackSize": 1024
}
```

### Performance-Optimierungen

- **JIT-Compilation**: Häufig ausgeführte Code-Blöcke werden kompiliert
- **Caching**: Funktionsergebnisse werden gecacht
- **Lazy Evaluation**: Ausdrücke werden erst bei Bedarf ausgewertet

## Debugging-Features

### Trace-Modus

```bash
dotnet run --project HypnoScript.CLI -- debug script.hyp --trace
```

### Breakpoints

```hyp
// Breakpoint setzen
breakpoint;

// Bedingte Breakpoints
if (zaehler == 42) {
    breakpoint;
}
```

### Variable Inspection

```hyp
// Variablen zur Laufzeit inspizieren
observe "Variable x: " + x;
observe "Array-Länge: " + ArrayLength(arr);
```

## Session-Management

### Session-Lifecycle

1. **Erstellung**: `Session("name")`
2. **Verwendung**: `SessionSet()`, `SessionGet()`
3. **Bereinigung**: Automatisch nach Programmende

### Session-Typen

```hyp
// Standard-Session
induce session = Session("Standard");

// Persistente Session
induce persistentSession = Session("Persistent", true);

// Geteilte Session
induce sharedSession = Session("Shared", false, true);
```

## Builtin-Funktionen Integration

### Funktionsaufruf-Mechanismus

```hyp
// Direkter Aufruf
induce result = ArraySum([1,2,3]);

// Mit Fehlerbehandlung
if (IsValidEmail(email)) {
    observe "E-Mail ist gültig";
} else {
    observe "E-Mail ist ungültig";
}
```

### Funktionskategorien

- **Array-Funktionen**: `ArrayGet`, `ArraySet`, `ArraySort`
- **String-Funktionen**: `Length`, `Substring`, `ToUpper`
- **Math-Funktionen**: `Sin`, `Cos`, `Sqrt`, `Pow`
- **System-Funktionen**: `GetCurrentTime`, `GetMachineName`
- **Utility-Funktionen**: `Clamp`, `IsEven`, `GenerateUUID`

## Performance-Monitoring

### Memory Usage

```hyp
induce memoryUsage = GetMemoryUsage();
observe "Speicherverbrauch: " + memoryUsage + " bytes";
```

### CPU Usage

```hyp
induce cpuUsage = GetCPUUsage();
observe "CPU-Auslastung: " + cpuUsage + "%";
```

### Execution Time

```hyp
induce startTime = GetCurrentTime();
// Code ausführen
induce endTime = GetCurrentTime();
induce executionTime = endTime - startTime;
observe "Ausführungszeit: " + executionTime + " ms";
```

## Erweiterbarkeit

### Custom Functions

```hyp
// Eigene Funktionen definieren
suggestion customFunction(param) {
    awaken param * 2;
}

// Verwenden
induce result = customFunction(21);
```

### Plugin-System

```hyp
// Plugins laden (konzeptionell)
LoadPlugin("math-extensions");
LoadPlugin("network-utils");
```

## Best Practices

### Memory Management

```hyp
// Große Arrays vermeiden
induce largeArray = [];
for (induce i = 0; i < 1000000; induce i = i + 1) {
    // Verarbeitung in Chunks
    if (i % 1000 == 0) {
        // Chunk verarbeiten
    }
}
```

### Error Handling

```hyp
// Robuste Fehlerbehandlung
suggestion safeArrayAccess(arr, index) {
    if (index < 0 || index >= Length(arr)) {
        awaken null;
    }
    return ArrayGet(arr, index);
}
```

### Performance Optimization

```hyp
// Effiziente Schleifen
induce length = Length(arr);
for (induce i = 0; i < length; induce i = i + 1) {
    // Code
}
```

## Troubleshooting

### Häufige Probleme

#### Memory Leaks

```hyp
// Sessions explizit löschen
SessionDelete(session);
```

#### Endlosschleifen

```hyp
// Timeout setzen
induce startTime = GetCurrentTime();
while (condition) {
    if (GetCurrentTime() - startTime > 5000) {
        break; // 5 Sekunden Timeout
    }
    // Code
}
```

#### Stack Overflow

```hyp
// Rekursion begrenzen
suggestion factorial(n, depth = 0) {
    if (depth > 1000) {
        awaken null; // Stack Overflow vermeiden
    }
    if (n <= 1) return 1;
    return n * factorial(n - 1, depth + 1);
}
```

## Nächste Schritte

- [Runtime](./runtime) - Runtime-Architektur
- [Compiler](./compiler) - Code-Generierung
- [API](./api) - Programmierschnittstelle
- [Debugging](../cli/debugging) - Debugging-Tools

---

**Verstehst du den Interpreter? Dann lerne die [Runtime-Architektur](./runtime) kennen!** ⚙️
