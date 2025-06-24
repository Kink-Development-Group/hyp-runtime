# HypnoScript - Eine esoterische, TypeScript-inspirierte Sprache mit hypnotischem Flair

**HypnoScript** ist eine vollständig implementierte, esoterische Programmiersprache, die sich an TypeScript/JavaScript anlehnt und dabei alle Klischees rund um Hypnose, Trance und hypnotische Induktion verwendet. Trotz des humorvollen Charakters ist sie Turing-vollständig und unterstützt moderne Sprachfeatures.

## 🎯 Features

### ✅ Vollständig implementiert

- **Grundlegende Syntax**: `Focus { ... } Relax` Programmstruktur
- **Variablen**: `induce x: number = 5;` mit Typisierung
- **Externe Eingabe**: `induce userInput: string from external;`
- **Kontrollstrukturen**: `if`, `while`, `loop` (for-Schleife)
- **Funktionen**: `suggestion`, `imperative suggestion`, `dominant suggestion`
- **Objektorientierung**: `session` (Klassen) mit Konstruktoren und Methoden
- **Strukturen**: `tranceify` (Records/Structs)
- **Arrays**: `[1, 2, 3]` und Array-Zugriffe `array[index]`
- **Hypnotische Operatoren**: Synonyme für Standardoperatoren
- **Builtin-Funktionen**: Umfassende mathematische, String- und hypnotische Funktionen
- **Ein-/Ausgabe**: `observe`, `drift(ms)`
- **Module**: `mindLink` (Import)
- **Globale Variablen**: `sharedTrance`
- **Labels und Goto**: `label:`, `sinkTo label`

- **Machine Learning Funktionen**: `LinearRegression`, `CalculateMean`, `CalculateStandardDeviation`
- **Netzwerk-Funktionen**: `HttpGet`, `HttpPost` für HTTP-Requests
- **Datenbank-ähnliche Funktionen**: `CreateRecord`, `GetRecordValue`, `SetRecordValue`
- **Erweiterte hypnotische Funktionen**: `HypnoticPatternMatching`, `HypnoticTimeDilation`, `HypnoticMemoryEnhancement`, `HypnoticCreativityBoost`
- **Performance-Monitoring**: `GetPerformanceMetrics` für detaillierte Performance-Daten
- **Erweiterte Validierungsfunktionen**: `IsValidEmail`, `IsValidUrl`, `IsValidJson`
- **Erweiterte Formatierungsfunktionen**: `FormatNumber`, `FormatCurrency`, `FormatPercentage`
- **Erweiterte Array-Operationen**: `ArrayMap`, `ArrayReduce`, `ArrayFlatten`
- **Erweiterte String-Operationen**: `StringSplitByLength`, `StringRotate`, `StringShuffle`

#### **Bestehende Runtime-Features:**

- **Erweiterte hypnotische Funktionen**: `HypnoticBreathing`, `HypnoticAnchoring`, `HypnoticRegression`, `HypnoticFutureProgression`
- **Datei- und Verzeichnisoperationen**: `ReadFile`, `WriteFile`, `CreateDirectory`, `GetFiles`, etc.
- **JSON-Verarbeitung**: `ToJson`, `FromJson`
- **Erweiterte Mathematik**: `Factorial`, `GCD`, `LCM`, `Asin`, `Acos`, `Atan`, etc.
- **Erweiterte String-Funktionen**: `Reverse`, `Capitalize`, `TitleCase`, `CountOccurrences`, etc.
- **Erweiterte Array-Funktionen**: `ArrayReverse`, `ArraySort`, `ArrayUnique`, `ArrayFilter`
- **Kryptologische Funktionen**: `HashMD5`, `HashSHA256`, `Base64Encode`, `Base64Decode`
- **Erweiterte Zeit-Funktionen**: `FormatDateTime`, `GetDayOfWeek`, `IsLeapYear`, etc.
- **Erweiterte System-Funktionen**: `GetCurrentDirectory`, `GetMachineName`, `GetProcessorCount`, etc.
- **Erweiterte Debugging-Funktionen**: `DebugPrintMemory`, `DebugPrintStackTrace`, `DebugPrintEnvironment`
- **Performance-Optimierungen**: Caching, verbesserte Typüberprüfung
- **Erweiterte CLI-Befehle**: `web`, `api`, `deploy`, `monitor`, `test`, `docs`, `benchmark`, `profile`, `lint`, `optimize`

### 🧠 Hypnotische Sprachfeatures

- **Operator-Synonyme**:

  - `youAreFeelingVerySleepy` = `==`
  - `notSoDeep` = `!=`
  - `lookAtTheWatch` = `>`
  - `fallUnderMySpell` = `<`
  - `deeplyGreater` = `>=`
  - `deeplyLess` = `<=`

- **Hypnotische Builtins**:
  - `DeepTrance(duration)`
  - `HypnoticCountdown(from)`
  - `TranceInduction(subjectName)`
  - `HypnoticVisualization(scene)`
  - `ProgressiveRelaxation(steps)`
  - `HypnoticSuggestion(suggestion)`
  - `TranceDeepening(levels)`
  - `HypnoticBreathing(cycles)`
  - `HypnoticAnchoring(anchor)`
  - `HypnoticRegression(age)`
  - `HypnoticFutureProgression(years)`
  - **v1.0.0**: `HypnoticPatternMatching(pattern)`
  - **v1.0.0**: `HypnoticTimeDilation(factor)`
  - **v1.0.0**: `HypnoticMemoryEnhancement()`
  - **v1.0.0**: `HypnoticCreativityBoost()`

## 🏗️ Architektur

Die Implementierung besteht aus mehreren .NET-Projekten:

- **HypnoScript.Core**: Grundlegende Typen und Symbol-Tabellen (erweitert mit v1.0.0)
- **HypnoScript.LexerParser**: Lexer, Parser und AST
- **HypnoScript.Compiler**: TypeChecker, Interpreter und WASM-Codegenerator
- **HypnoScript.Runtime**: Umfassende Builtin-Funktionen (150+ Funktionen in v1.0.0)
- **HypnoScript.CLI**: Erweiterte Kommandozeilen-Interface (18 Befehle in v1.0.0)

## 🚀 Verwendung

### Kompilierung

```bash
dotnet build
```

### Ausführung

```bash
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp
```

### Debug-Modus

```bash
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp --debug --verbose
```

## 📝 CLI-Befehle

### Grundlegende Befehle

```bash
# Programm ausführen
dotnet run -- run <file.hyp> [--debug] [--verbose]

# Zu WASM kompilieren
dotnet run -- compile <file.hyp> [--debug] [--verbose]

# Statische Analyse
dotnet run -- analyze <file.hyp> [--debug] [--verbose]

# Datei-Informationen anzeigen
dotnet run -- info <file.hyp> [--debug] [--verbose]

# Syntax validieren
dotnet run -- validate <file.hyp> [--debug] [--verbose]

# Code formatieren
dotnet run -- format <file.hyp> [--debug] [--verbose]

# Performance-Benchmark
dotnet run -- benchmark <file.hyp> [--debug] [--verbose]

# Code-Profiling
dotnet run -- profile <file.hyp> [--debug] [--verbose]

# Code-Linting
dotnet run -- lint <file.hyp> [--debug] [--verbose]

# Code-Optimierung
dotnet run -- optimize <file.hyp> [--debug] [--verbose]

# Version anzeigen
dotnet run -- version

# Hilfe anzeigen
dotnet run -- help
```

### **Runtime v1.0.0 Befehle:**

```bash
# Web-Server starten
dotnet run -- web <file.hyp> [--debug] [--verbose]

# API-Server starten
dotnet run -- api <file.hyp> [--debug] [--verbose]

# Anwendung deployen
dotnet run -- deploy <file.hyp> [--debug] [--verbose]

# Anwendung überwachen
dotnet run -- monitor <file.hyp> [--debug] [--verbose]

# Tests ausführen
dotnet run -- test <file.hyp> [--debug] [--verbose]

# Dokumentation generieren
dotnet run -- docs <file.hyp> [--debug] [--verbose]
```

### Optionen

- `--debug`: Aktiviert Debug-Ausgabe
- `--verbose`: Aktiviert detaillierte Ausgabe

## 📝 Beispiele

### Grundlegendes Programm

```hypnoscript
Focus {
    entrance {
        observe "Willkommen in HypnoScript!";
    }

    induce greeting: string = "Hello Trance!";
    observe greeting;

    if (true) deepFocus {
        observe "You are feeling very relaxed...";
    }
} Relax
```

```hypnoscript
Focus {
    entrance {
        observe "🚀 HypnoScript Runtime Edition v1.0.0 Demo";
    }

    // Machine Learning
    induce xValues = [1, 2, 3, 4, 5];
    induce yValues = [2, 4, 6, 8, 10];
    induce slope = LinearRegression(xValues, yValues);
    observe "Linear Regression slope: " + slope;

    // Database-like operations
    induce record = CreateRecord(["name", "age"], ["Alice", 30]);
    induce name = GetRecordValue(record, "name");
    observe "Record: " + name;

    // Advanced hypnotic functions
    HypnoticPatternMatching("success");
    HypnoticTimeDilation(1.5);
    HypnoticMemoryEnhancement();

    // Performance monitoring
    induce metrics = GetPerformanceMetrics();
    observe "Performance: " + metrics;

    // Advanced array operations
    induce numbers = [1, 2, 3, 4, 5];
    induce doubled = ArrayMap(numbers, x => x * 2);
    induce sum = ArrayReduce(numbers, (acc, val) => acc + val, 0);
    observe "Doubled: " + doubled;
    observe "Sum: " + sum;

    // Advanced string operations
    induce rotated = StringRotate("HypnoScript", 3);
    induce shuffled = StringShuffle("HypnoScript");
    observe "Rotated: " + rotated;
    observe "Shuffled: " + shuffled;

} Relax
```

### Erweiterte mathematische Funktionen

```hypnoscript
Focus {
    induce x: number = 10.5;
    induce y: number = 3.2;

    observe "Max(10.5, 3.2) = " + Max(x, y);
    observe "Min(10.5, 3.2) = " + Min(x, y);
    observe "Log(100) = " + Log(100);
    observe "Random number = " + Random();
    observe "Random int (1-10) = " + RandomInt(1, 10);
    observe "Factorial(6) = " + Factorial(6);
    observe "GCD(48, 18) = " + GCD(48, 18);
    observe "Asin(0.5) = " + Asin(0.5);
} Relax
```

### Erweiterte String-Manipulation

```hypnoscript
Focus {
    induce text: string = "  HypnoScript is amazing!  ";

    observe "Original: '" + text + "'";
    observe "Trimmed: '" + Trim(text) + "'";
    observe "Uppercase: '" + ToUpper(text) + "'";
    observe "Contains 'Script': " + Contains(text, "Script");
    observe "IndexOf 'Script': " + IndexOf(text, "Script");
    observe "PadLeft(30, '*'): '" + PadLeft(text, 30, '*') + "'";
    observe "Reversed: '" + Reverse(text) + "'";
    observe "Capitalized: '" + Capitalize(text) + "'";
    observe "Title Case: '" + TitleCase(text) + "'";
    observe "Count of 'e': " + CountOccurrences(text, "e");
} Relax
```

### Array-Operationen

```hypnoscript
Focus {
    induce numbers = [1, 2, 3, 4, 5];
    induce moreNumbers = [6, 7, 8];

    observe "Array length: " + ArrayLength(numbers);
    observe "First element: " + ArrayGet(numbers, 0);
    observe "Array slice (1-3): " + ArraySlice(numbers, 1, 3);
    observe "Array contains 3: " + ArrayContains(numbers, 3);

    induce combined = ArrayConcat(numbers, moreNumbers);
    observe "Combined arrays: " + combined;

    induce sorted = ArraySort(combined);
    observe "Sorted: " + sorted;

    induce unique = ArrayUnique([1, 2, 2, 3, 3, 3, 4]);
    observe "Unique: " + unique;
} Relax
```

### Objektorientierung

```hypnoscript
Focus {
    session Person {
        expose name: string;
        expose age: number;

        suggestion constructor(personName: string, personAge: number) {
            this.name = personName;
            this.age = personAge;
        }

        suggestion greet() {
            observe "Hello, I am " + this.name + " and I am " + this.age + " years old";
        }

        suggestion celebrateBirthday() {
            this.age = this.age + 1;
            observe "Happy Birthday, " + this.name + "! You are now " + this.age + " years old!";
        }
    }

    induce person = Person("Alice", 30);
    person.greet();
    person.celebrateBirthday();
} Relax
```

### Hypnotische Spezialfunktionen

```hypnoscript
Focus {
    HypnoticVisualization("a peaceful mountain landscape");
    ProgressiveRelaxation(3);
    HypnoticSuggestion("You are becoming more confident");
    TranceDeepening(2);
    HypnoticBreathing(5);
    HypnoticAnchoring("tranquil");
    HypnoticRegression(10);
    HypnoticFutureProgression(5);

    // v1.0.0 Features
    HypnoticPatternMatching("success");
    HypnoticTimeDilation(2.0);
    HypnoticMemoryEnhancement();
    HypnoticCreativityBoost();
} Relax
```

### Zeit- und Systemfunktionen

```hypnoscript
Focus {
    observe "Current time: " + GetCurrentTime();
    observe "Current date: " + GetCurrentDate();
    observe "Current time string: " + GetCurrentTimeString();
    observe "Formatted datetime: " + FormatDateTime("yyyy-MM-dd HH:mm:ss");
    observe "Day of week: " + GetDayOfWeek();
    observe "Is 2024 leap year: " + IsLeapYear(2024);
    observe "PATH length: " + Length(GetEnvironmentVariable("PATH"));
    observe "Current directory: " + GetCurrentDirectory();
    observe "Machine name: " + GetMachineName();
    observe "User name: " + GetUserName();
    observe "Processor count: " + GetProcessorCount();

    DebugPrint("This is a debug message");
    DebugPrintType(42);
    DebugPrintMemory();
    DebugPrintEnvironment();
} Relax
```

## 🔧 Builtin-Funktionen

### Mathematische Funktionen

- **Grundfunktionen**: `Sin(x)`, `Cos(x)`, `Tan(x)`, `Sqrt(x)`, `Pow(x, y)`
- **Erweiterte Funktionen**: `Log(x)`, `Log10(x)`, `Exp(x)`, `Abs(x)`
- **Rundungsfunktionen**: `Floor(x)`, `Ceiling(x)`, `Round(x)`
- **Vergleichsfunktionen**: `Max(x, y)`, `Min(x, y)`
- **Zufallsfunktionen**: `Random()`, `RandomInt(min, max)`
- **Runtime-Funktionen**: `Factorial(n)`, `GCD(a, b)`, `LCM(a, b)`
- **Trigonometrische Funktionen**: `Asin(x)`, `Acos(x)`, `Atan(x)`, `Atan2(y, x)`
- **Winkelkonvertierung**: `DegreesToRadians(degrees)`, `RadiansToDegrees(radians)`

### **v1.0.0 Machine Learning Funktionen:**

- **Statistische Funktionen**: `LinearRegression(x, y)`, `CalculateMean(values)`, `CalculateStandardDeviation(values)`

### String-Funktionen

- **Grundfunktionen**: `Length(str)`, `ToUpper(str)`, `ToLower(str)`
- **Erweiterte Funktionen**: `Trim(str)`, `TrimStart(str)`, `TrimEnd(str)`
- **Suchfunktionen**: `IndexOf(str, substring)`, `LastIndexOf(str, substring)`
- **Prüffunktionen**: `Contains(str, substring)`, `StartsWith(str, prefix)`, `EndsWith(str, suffix)`
- **Manipulationsfunktionen**: `Substring(str, start, length)`, `Replace(str, old, new)`
- **Formatierungsfunktionen**: `PadLeft(str, width, char)`, `PadRight(str, width, char)`
- **Array-Funktionen**: `Split(str, separator)`, `Join(array, separator)`
- **Runtime-Funktionen**: `Reverse(str)`, `Capitalize(str)`, `TitleCase(str)`, `CountOccurrences(str, substring)`, `RemoveWhitespace(str)`

### **v1.0.0 Erweiterte String-Funktionen:**

- **String-Manipulation**: `StringSplitByLength(str, maxLength)`, `StringRotate(str, positions)`, `StringShuffle(str)`

### Array-Funktionen

- **Grundfunktionen**: `ArrayLength(arr)`, `ArrayGet(arr, index)`, `ArraySet(arr, index, value)`
- **Erweiterte Funktionen**: `ArraySlice(arr, start, length)`, `ArrayConcat(arr1, arr2)`
- **Suchfunktionen**: `ArrayIndexOf(arr, value)`, `ArrayContains(arr, value)`
- **Runtime-Funktionen**: `ArrayReverse(arr)`, `ArraySort(arr)`, `ArrayUnique(arr)`, `ArrayFilter(arr, predicate)`

### **v1.0.0 Erweiterte Array-Funktionen:**

- **Funktionale Programmierung**: `ArrayMap(arr, mapper)`, `ArrayReduce(arr, reducer, initial)`, `ArrayFlatten(arr)`

### Konvertierungsfunktionen

- **Typkonvertierung**: `ToInt(value)`, `ToDouble(value)`, `ToString(value)`, `ToBoolean(value)`, `ToChar(value)`

### Hypnotische Spezialfunktionen

- **Trance-Management**: `DeepTrance(duration)`, `TranceDeepening(levels)`
- **Induktion**: `TranceInduction(subjectName)`, `HypnoticSuggestion(suggestion)`
- **Visualisierung**: `HypnoticVisualization(scene)`
- **Entspannung**: `ProgressiveRelaxation(steps)`, `HypnoticCountdown(from)`
- **Runtime-Funktionen**: `HypnoticBreathing(cycles)`, `HypnoticAnchoring(anchor)`, `HypnoticRegression(age)`, `HypnoticFutureProgression(years)`

### **v1.0.0 Erweiterte hypnotische Funktionen:**

- **Erweiterte Induktion**: `HypnoticPatternMatching(pattern)`, `HypnoticTimeDilation(factor)`
- **Kognitive Enhancement**: `HypnoticMemoryEnhancement()`, `HypnoticCreativityBoost()`

### Zeit- und Datumsfunktionen

- **Zeitstempel**: `GetCurrentTime()`, `GetCurrentDate()`, `GetCurrentTimeString()`, `GetCurrentDateTime()`
- **Runtime-Funktionen**: `FormatDateTime(format)`, `GetDayOfWeek()`, `GetDayOfYear()`, `IsLeapYear(year)`, `GetDaysInMonth(year, month)`

### System-Funktionen

- **System-Info**: `GetEnvironmentVariable(name)`, `ClearScreen()`, `Beep(frequency, duration)`
- **Debugging**: `DebugPrint(value)`, `DebugPrintType(value)`
- **Programm-Kontrolle**: `Exit(code)`
- **Runtime-Funktionen**: `GetCurrentDirectory()`, `GetMachineName()`, `GetUserName()`, `GetOSVersion()`, `GetProcessorCount()`, `GetWorkingSet()`, `PlaySound(frequency, duration)`, `Vibrate(duration)`, `DebugPrintMemory()`, `DebugPrintStackTrace()`, `DebugPrintEnvironment()`

### **v1.0.0 Performance-Monitoring:**

- **Performance-Metriken**: `GetPerformanceMetrics()` - Liefert detaillierte Performance-Daten

### Datei- und Verzeichnisoperationen

- **Datei-Operationen**: `FileExists(path)`, `ReadFile(path)`, `WriteFile(path, content)`, `AppendFile(path, content)`, `ReadLines(path)`, `WriteLines(path, lines)`, `GetFileSize(path)`, `GetFileExtension(path)`, `GetFileName(path)`, `GetDirectoryName(path)`
- **Verzeichnis-Operationen**: `DirectoryExists(path)`, `CreateDirectory(path)`, `GetFiles(path, pattern)`, `GetDirectories(path)`

### JSON- und Kryptologische Funktionen

- **JSON-Verarbeitung**: `ToJson(obj)`, `FromJson(json)`
- **Kryptologische Funktionen**: `HashMD5(input)`, `HashSHA256(input)`, `Base64Encode(input)`, `Base64Decode(input)`

### **v1.0.0 Netzwerk-Funktionen:**

- **HTTP-Requests**: `HttpGet(url)`, `HttpPost(url, data)`

### **v1.0.0 Datenbank-ähnliche Funktionen:**

- **Record-Management**: `CreateRecord(keys, values)`, `GetRecordValue(record, key)`, `SetRecordValue(record, key, value)`

### **v1.0.0 Validierungsfunktionen:**

- **Input-Validierung**: `IsValidEmail(email)`, `IsValidUrl(url)`, `IsValidJson(json)`

### **v1.0.0 Formatierungsfunktionen:**

- **Formatierung**: `FormatNumber(number, decimals)`, `FormatCurrency(amount, currency)`, `FormatPercentage(value)`

## 🎨 Sprachdesign

HypnoScript kombiniert:

- **TypeScript-ähnliche Syntax** für Vertrautheit
- **Hypnotische Terminologie** für den esoterischen Charme
- **Moderne Sprachfeatures** für praktische Nutzbarkeit
- **Turing-Vollständigkeit** für universelle Berechnungsfähigkeit
- **Umfassende Standardbibliothek** für produktive Entwicklung
- **Runtime-Features** für professionelle Anwendungen
- **Machine Learning Integration** für KI-gestützte Entwicklung
- **Netzwerk- und Datenbank-Funktionen** für moderne Anwendungen

## 📊 Status

- ✅ **Lexer**: Vollständig implementiert
- ✅ **Parser**: Vollständig implementiert
- ✅ **AST**: Vollständig implementiert
- ✅ **TypeChecker**: Erweitert implementiert mit Caching
- ✅ **Interpreter**: Vollständig implementiert mit 150+ Builtins
- ✅ **Builtins**: Umfassend implementiert (150+ Funktionen)
- ✅ **CLI**: Erweitert mit 18 Befehlen
- ✅ **WASM-Codegenerator**: Erweitert implementiert
- ✅ **Runtime-Features**: Vollständig implementiert
- ✅ **v1.0.0 Features**: Vollständig implementiert
- 🔄 **Performance-Optimierungen**: Implementiert

## 🎯 Nächste Schritte

1. **Web-Interface** für interaktive Entwicklung
2. **Package Manager** für Bibliotheken
3. **IDE-Integration** (VS Code Extension)
4. **Cloud-Deployment** Tools
5. **Machine Learning** Integration
6. **WebAssembly** Runtime-Optimierungen
7. **Real-time Collaboration** Features
8. **Advanced AI Integration** für Code-Generierung

## 📚 Testprogramme

- `test_basic.hyp` - Grundlegende Funktionalität
- `test_extended_features.hyp` - Erweiterte Builtin-Funktionen
- `test_enterprise_features.hyp` - Vollständige Runtime-Demonstration
- `test_enterprise_v3.hyp` - **NEU**: Runtime v1.0.0 Features
- `test_all_enterprise_features.ps1` - Umfassendes Test-Skript
- `test_enterprise_v3.ps1` - **NEU**: Runtime v1.0.0 Test-Suite

## 🚀 Quick Start

```bash
# Repository klonen
git clone <repository-url>
cd hyp-runtime

# Projekt kompilieren
dotnet build

# v1.0.0 Demo ausführen
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp

# Umfassende Tests ausführen
./test_enterprise_v3.ps1
```

## HypnoScript Builtin-Standardbibliothek

Die HypnoScript-Standardbibliothek bietet zahlreiche eingebaute Funktionen (Builtins) für Arrays, Strings, Mathematik, Utilities, System, Zeit/Datum, Statistik, Hashing/Encoding und hypnotische Spezialfunktionen.

## Inhaltsverzeichnis

- [Array-Funktionen](#array-funktionen)
- [String-Funktionen](#string-funktionen)
- [Mathematische Funktionen](#mathematische-funktionen)
- [Utility-Funktionen](#utility-funktionen)
- [System-Funktionen](#system-funktionen)
- [Zeit- und Datumsfunktionen](#zeit-und-datumsfunktionen)
- [Statistik-Funktionen](#statistik-funktionen)
- [Hashing/Encoding](#hashingencoding)
- [Hypnotische Spezialfunktionen](#hypnotische-spezialfunktionen)

---

## Array-Funktionen

| Funktion                         | Beschreibung                    |
| -------------------------------- | ------------------------------- |
| `ArrayLength(arr)`               | Länge des Arrays                |
| `ArrayGet(arr, index)`           | Element an Index                |
| `ArraySet(arr, index, value)`    | Setzt Wert an Index             |
| `ArraySlice(arr, start, length)` | Teil-Array                      |
| `ArrayConcat(arr1, arr2)`        | Arrays verketten                |
| `ArrayIndexOf(arr, value)`       | Index eines Werts               |
| `ArrayContains(arr, value)`      | Enthält Wert?                   |
| `ArrayReverse(arr)`              | Umgekehrtes Array               |
| `ArraySort(arr)`                 | Sortiertes Array                |
| `ArrayUnique(arr)`               | Duplikate entfernen             |
| `ArrayFilter(arr)`               | Filtert nicht-null Werte        |
| `ArrayMap(arr)`                  | Mappt Werte (Identität)         |
| `ArrayReduce(arr, initial)`      | Reduziert Werte (Identität)     |
| `ArrayFlatten(arr)`              | Flacht verschachtelte Arrays ab |
| `ShuffleArray(arr)`              | Mischt das Array zufällig       |
| `SumArray(arr)`                  | Summe aller Werte               |
| `AverageArray(arr)`              | Durchschnitt aller Werte        |
| `Range(start, count)`            | Array von Zahlen                |
| `Repeat(value, count)`           | Wiederholt Wert                 |
| `Swap(arr, i, j)`                | Vertauscht zwei Elemente        |
| `ChunkArray(arr, chunkSize)`     | Teilt Array in Blöcke           |

## String-Funktionen

| Funktion                        | Beschreibung              |
| ------------------------------- | ------------------------- |
| `Length(str)`                   | Länge des Strings         |
| `Substring(str, start, length)` | Teilstring                |
| `ToUpper(str)`                  | Großbuchstaben            |
| `ToLower(str)`                  | Kleinbuchstaben           |
| `Contains(str, sub)`            | Enthält Teilstring?       |
| `Replace(str, old, new)`        | Ersetzt Teilstring        |
| `Trim(str)`                     | Entfernt Leerzeichen      |
| `IndexOf(str, sub)`             | Index von Teilstring      |
| `Split(str, sep)`               | Teilt String              |
| `Join(arr, sep)`                | Verbindet Strings         |
| `Reverse(str)`                  | Kehrt String um           |
| `Capitalize(str)`               | Erstes Zeichen groß       |
| `TitleCase(str)`                | Jedes Wort groß           |
| `CountOccurrences(str, sub)`    | Zählt Vorkommen           |
| `RemoveWhitespace(str)`         | Entfernt alle Leerzeichen |
| `StringRotate(str, n)`          | Rotiert String            |
| `StringShuffle(str)`            | Mischt Zeichen zufällig   |
| `StringSplitByLength(str, n)`   | Teilt String in Blöcke    |

## Mathematische Funktionen

| Funktion              | Beschreibung                     |
| --------------------- | -------------------------------- |
| `Abs(x)`              | Absolutwert                      |
| `Sin(x)`              | Sinus (Grad)                     |
| `Cos(x)`              | Kosinus (Grad)                   |
| `Tan(x)`              | Tangens (Grad)                   |
| `Sqrt(x)`             | Quadratwurzel                    |
| `Pow(x, y)`           | Potenz                           |
| `Floor(x)`            | Abrunden                         |
| `Ceiling(x)`          | Aufrunden                        |
| `Round(x)`            | Runden                           |
| `Log(x)`              | Natürlicher Logarithmus          |
| `Log10(x)`            | Zehnerlogarithmus                |
| `Exp(x)`              | Exponentialfunktion              |
| `Max(x, y)`           | Maximum                          |
| `Min(x, y)`           | Minimum                          |
| `Random()`            | Zufallszahl [0,1)                |
| `RandomInt(min, max)` | Zufallszahl int                  |
| `Clamp(x, min, max)`  | Begrenzen                        |
| `Sign(x)`             | Vorzeichen                       |
| `IsEven(x)`           | Gerade Zahl?                     |
| `IsOdd(x)`            | Ungerade Zahl?                   |
| `Factorial(n)`        | Fakultät                         |
| `GCD(a, b)`           | Größter gemeinsamer Teiler       |
| `LCM(a, b)`           | Kleinstes gemeinsames Vielfaches |
| `DegreesToRadians(x)` | Grad → Radiant                   |
| `RadiansToDegrees(x)` | Radiant → Grad                   |
| `Asin(x)`             | Arkussinus (Grad)                |
| `Acos(x)`             | Arkuskosinus (Grad)              |
| `Atan(x)`             | Arkustangens (Grad)              |
| `Atan2(y, x)`         | Arkustangens2 (Grad)             |

## Utility-Funktionen

| Funktion                 | Beschreibung       |
| ------------------------ | ------------------ |
| `IsValidEmail(str)`      | Prüft E-Mail       |
| `IsValidUrl(str)`        | Prüft URL          |
| `IsValidJson(str)`       | Prüft JSON         |
| `FormatNumber(x, d)`     | Formatiert Zahl    |
| `FormatCurrency(x, cur)` | Formatiert Währung |
| `FormatPercentage(x)`    | Prozentformat      |

## System-Funktionen

| Funktion                       | Beschreibung          |
| ------------------------------ | --------------------- |
| `ClearScreen()`                | Konsole löschen       |
| `Beep(freq, dur)`              | Piepton               |
| `GetEnvironmentVariable(name)` | Umgebungsvariable     |
| `GetCurrentDirectory()`        | Aktuelles Verzeichnis |
| `GetMachineName()`             | Rechnername           |
| `GetUserName()`                | Benutzername          |
| `GetOSVersion()`               | Betriebssystem        |
| `GetProcessorCount()`          | CPU-Kerne             |
| `GetWorkingSet()`              | RAM-Nutzung           |

## Zeit- und Datumsfunktionen

| Funktion                 | Beschreibung       |
| ------------------------ | ------------------ |
| `GetCurrentTime()`       | Unix-Timestamp     |
| `GetCurrentDate()`       | Datum (yyyy-MM-dd) |
| `GetCurrentTimeString()` | Uhrzeit (HH:mm:ss) |
| `GetCurrentDateTime()`   | Datum & Uhrzeit    |
| `FormatDateTime(fmt)`    | Formatiert Datum   |
| `GetDayOfWeek()`         | Wochentag          |
| `GetDayOfYear()`         | Tag im Jahr        |
| `IsLeapYear(y)`          | Schaltjahr?        |
| `GetDaysInMonth(y, m)`   | Tage im Monat      |

## Statistik-Funktionen

| Funktion                          | Beschreibung       |
| --------------------------------- | ------------------ |
| `CalculateMean(arr)`              | Mittelwert         |
| `CalculateStandardDeviation(arr)` | Standardabweichung |
| `LinearRegression(x, y)`          | Lineare Regression |

## Hashing/Encoding

| Funktion            | Beschreibung       |
| ------------------- | ------------------ |
| `HashMD5(str)`      | MD5-Hash           |
| `HashSHA256(str)`   | SHA256-Hash        |
| `Base64Encode(str)` | Base64-Kodierung   |
| `Base64Decode(str)` | Base64-Dekodierung |

## Hypnotische Spezialfunktionen

| Funktion                           | Beschreibung            |
| ---------------------------------- | ----------------------- |
| `DeepTrance(dur)`                  | Tiefe Trance            |
| `HypnoticCountdown(from)`          | Countdown               |
| `TranceInduction(name)`            | Trance-Induktion        |
| `HypnoticVisualization(scene)`     | Visualisierung          |
| `ProgressiveRelaxation(steps)`     | Progressive Entspannung |
| `HypnoticSuggestion(msg)`          | Suggestion              |
| `TranceDeepening(levels)`          | Trance vertiefen        |
| `HypnoticPatternMatching(pattern)` | Mustererkennung         |
| `HypnoticTimeDilation(factor)`     | Zeitdehnung             |
| `HypnoticMemoryEnhancement()`      | Gedächtnis              |
| `HypnoticCreativityBoost()`        | Kreativität             |

---

**Hinweis:**

- Alle Funktionen sind direkt in HypnoScript aufrufbar.
- Typen: `arr` = Array, `str` = String, `x/y` = Zahl, `obj` = beliebig, `n` = int.
- Viele Funktionen sind überladen und akzeptieren verschiedene Typen.

Für Details siehe die Implementierung in `HypnoScript.Runtime/HypnoBuiltins.cs`.

## 🌐 Plattformübergreifende Installation

### Windows (winget)

Installiere HypnoScript mit dem Windows Package Manager:

```powershell
winget install HypnoScript.HypnoScript
```

### Linux (APT)

Installiere HypnoScript auf Debian/Ubuntu:

```bash
sudo apt update
sudo apt install hypnoscript
```

Weitere Details und manuelle Installationsoptionen findest du im Verzeichnis `scripts/` und in der Dokumentation.

## 🔧 Build- und Installationsskripte

Im Verzeichnis `scripts/` befinden sich:

- Build-Skripte für Windows (winget) und Linux (APT)
- Beispiel-Installer und Paketvorlagen
- Hilfsskripte für Entwickler und Distributoren

## 🚀 Build & Release für Paketmanager

### Windows (self-contained Binary für winget)

```powershell
dotnet publish HypnoScript.CLI -c Release -r win-x64 --self-contained true -p:PublishSingleFile=true -o ./publish/win
```

### Linux (self-contained Binary für APT)

```bash
dotnet publish HypnoScript.CLI -c Release -r linux-x64 --self-contained true -p:PublishSingleFile=true -o ./publish/linux
```

Die resultierenden Binaries werden für die jeweiligen Pakete verwendet.

## 📦 Paketbau & Distribution

- Für Windows: Siehe `scripts/build_winget.ps1` und `scripts/winget-manifest.yaml`
- Für Linux: Siehe `scripts/build_deb.sh` und `scripts/debian/`

Die Skripte automatisieren den Paketbau und die Veröffentlichung.

## scripts/-Verzeichnis

- `build_winget.ps1`: Erstellt das Windows-Binary und bereitet das winget-Paket vor
- `winget-manifest.yaml`: Beispiel für das winget-Manifest
- `build_deb.sh`: Erstellt das Linux-Binary und das .deb-Paket
- `debian/`: Beispielstruktur für das Debian-Paket
