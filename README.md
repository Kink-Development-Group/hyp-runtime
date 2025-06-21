# HypnoScript - Eine esoterische, TypeScript-inspirierte Sprache mit hypnotischem Flair

**HypnoScript** ist eine vollständig implementierte, esoterische Programmiersprache, die sich an TypeScript/JavaScript anlehnt und dabei alle Klischees rund um Hypnose, Trance und hypnotische Induktion verwendet. Trotz des humorvollen Charakters ist sie Turing-vollständig und unterstützt moderne Sprachfeatures.

## 🎯 Features

### ✅ Vollständig implementiert:

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

### 🚀 **Enterprise Edition Features (v3.0.0)**:

#### **Neue v3.0.0 Features:**

- **Machine Learning Funktionen**: `LinearRegression`, `CalculateMean`, `CalculateStandardDeviation`
- **Netzwerk-Funktionen**: `HttpGet`, `HttpPost` für HTTP-Requests
- **Datenbank-ähnliche Funktionen**: `CreateRecord`, `GetRecordValue`, `SetRecordValue`
- **Erweiterte hypnotische Funktionen**: `HypnoticPatternMatching`, `HypnoticTimeDilation`, `HypnoticMemoryEnhancement`, `HypnoticCreativityBoost`
- **Performance-Monitoring**: `GetPerformanceMetrics` für detaillierte Performance-Daten
- **Erweiterte Validierungsfunktionen**: `IsValidEmail`, `IsValidUrl`, `IsValidJson`
- **Erweiterte Formatierungsfunktionen**: `FormatNumber`, `FormatCurrency`, `FormatPercentage`
- **Erweiterte Array-Operationen**: `ArrayMap`, `ArrayReduce`, `ArrayFlatten`
- **Erweiterte String-Operationen**: `StringSplitByLength`, `StringRotate`, `StringShuffle`

#### **Bestehende Enterprise-Features:**

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

### 🧠 Hypnotische Sprachfeatures:

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
  - **v3.0.0**: `HypnoticPatternMatching(pattern)`
  - **v3.0.0**: `HypnoticTimeDilation(factor)`
  - **v3.0.0**: `HypnoticMemoryEnhancement()`
  - **v3.0.0**: `HypnoticCreativityBoost()`

## 🏗️ Architektur

Die Implementierung besteht aus mehreren .NET-Projekten:

- **HypnoScript.Core**: Grundlegende Typen und Symbol-Tabellen (erweitert mit v3.0.0)
- **HypnoScript.LexerParser**: Lexer, Parser und AST
- **HypnoScript.Compiler**: TypeChecker, Interpreter und WASM-Codegenerator
- **HypnoScript.Runtime**: Umfassende Builtin-Funktionen (150+ Funktionen in v3.0.0)
- **HypnoScript.CLI**: Erweiterte Kommandozeilen-Interface (18 Befehle in v3.0.0)

## 🚀 Verwendung

### Kompilierung:

```bash
dotnet build
```

### Ausführung:

```bash
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp
```

### Debug-Modus:

```bash
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp --debug --verbose
```

## 📝 CLI-Befehle

### Grundlegende Befehle:

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

### **Enterprise v3.0.0 Befehle:**

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

### Optionen:

- `--debug`: Aktiviert Debug-Ausgabe
- `--verbose`: Aktiviert detaillierte Ausgabe

## 📝 Beispiele

### Grundlegendes Programm:

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

### Enterprise v3.0.0 Features Demonstration:

```hypnoscript
Focus {
    entrance {
        observe "🚀 HypnoScript Enterprise Edition v3.0.0 Demo";
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

### Erweiterte mathematische Funktionen:

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

### Erweiterte String-Manipulation:

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

### Array-Operationen:

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

### Objektorientierung:

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

### Hypnotische Spezialfunktionen:

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

    // v3.0.0 Features
    HypnoticPatternMatching("success");
    HypnoticTimeDilation(2.0);
    HypnoticMemoryEnhancement();
    HypnoticCreativityBoost();
} Relax
```

### Zeit- und Systemfunktionen:

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

### Mathematische Funktionen:

- **Grundfunktionen**: `Sin(x)`, `Cos(x)`, `Tan(x)`, `Sqrt(x)`, `Pow(x, y)`
- **Erweiterte Funktionen**: `Log(x)`, `Log10(x)`, `Exp(x)`, `Abs(x)`
- **Rundungsfunktionen**: `Floor(x)`, `Ceiling(x)`, `Round(x)`
- **Vergleichsfunktionen**: `Max(x, y)`, `Min(x, y)`
- **Zufallsfunktionen**: `Random()`, `RandomInt(min, max)`
- **Enterprise-Funktionen**: `Factorial(n)`, `GCD(a, b)`, `LCM(a, b)`
- **Trigonometrische Funktionen**: `Asin(x)`, `Acos(x)`, `Atan(x)`, `Atan2(y, x)`
- **Winkelkonvertierung**: `DegreesToRadians(degrees)`, `RadiansToDegrees(radians)`

### **v3.0.0 Machine Learning Funktionen:**

- **Statistische Funktionen**: `LinearRegression(x, y)`, `CalculateMean(values)`, `CalculateStandardDeviation(values)`

### String-Funktionen:

- **Grundfunktionen**: `Length(str)`, `ToUpper(str)`, `ToLower(str)`
- **Erweiterte Funktionen**: `Trim(str)`, `TrimStart(str)`, `TrimEnd(str)`
- **Suchfunktionen**: `IndexOf(str, substring)`, `LastIndexOf(str, substring)`
- **Prüffunktionen**: `Contains(str, substring)`, `StartsWith(str, prefix)`, `EndsWith(str, suffix)`
- **Manipulationsfunktionen**: `Substring(str, start, length)`, `Replace(str, old, new)`
- **Formatierungsfunktionen**: `PadLeft(str, width, char)`, `PadRight(str, width, char)`
- **Array-Funktionen**: `Split(str, separator)`, `Join(array, separator)`
- **Enterprise-Funktionen**: `Reverse(str)`, `Capitalize(str)`, `TitleCase(str)`, `CountOccurrences(str, substring)`, `RemoveWhitespace(str)`

### **v3.0.0 Erweiterte String-Funktionen:**

- **String-Manipulation**: `StringSplitByLength(str, maxLength)`, `StringRotate(str, positions)`, `StringShuffle(str)`

### Array-Funktionen:

- **Grundfunktionen**: `ArrayLength(arr)`, `ArrayGet(arr, index)`, `ArraySet(arr, index, value)`
- **Erweiterte Funktionen**: `ArraySlice(arr, start, length)`, `ArrayConcat(arr1, arr2)`
- **Suchfunktionen**: `ArrayIndexOf(arr, value)`, `ArrayContains(arr, value)`
- **Enterprise-Funktionen**: `ArrayReverse(arr)`, `ArraySort(arr)`, `ArrayUnique(arr)`, `ArrayFilter(arr, predicate)`

### **v3.0.0 Erweiterte Array-Funktionen:**

- **Funktionale Programmierung**: `ArrayMap(arr, mapper)`, `ArrayReduce(arr, reducer, initial)`, `ArrayFlatten(arr)`

### Konvertierungsfunktionen:

- **Typkonvertierung**: `ToInt(value)`, `ToDouble(value)`, `ToString(value)`, `ToBoolean(value)`, `ToChar(value)`

### Hypnotische Spezialfunktionen:

- **Trance-Management**: `DeepTrance(duration)`, `TranceDeepening(levels)`
- **Induktion**: `TranceInduction(subjectName)`, `HypnoticSuggestion(suggestion)`
- **Visualisierung**: `HypnoticVisualization(scene)`
- **Entspannung**: `ProgressiveRelaxation(steps)`, `HypnoticCountdown(from)`
- **Enterprise-Funktionen**: `HypnoticBreathing(cycles)`, `HypnoticAnchoring(anchor)`, `HypnoticRegression(age)`, `HypnoticFutureProgression(years)`

### **v3.0.0 Erweiterte hypnotische Funktionen:**

- **Erweiterte Induktion**: `HypnoticPatternMatching(pattern)`, `HypnoticTimeDilation(factor)`
- **Kognitive Enhancement**: `HypnoticMemoryEnhancement()`, `HypnoticCreativityBoost()`

### Zeit- und Datumsfunktionen:

- **Zeitstempel**: `GetCurrentTime()`, `GetCurrentDate()`, `GetCurrentTimeString()`, `GetCurrentDateTime()`
- **Enterprise-Funktionen**: `FormatDateTime(format)`, `GetDayOfWeek()`, `GetDayOfYear()`, `IsLeapYear(year)`, `GetDaysInMonth(year, month)`

### System-Funktionen:

- **System-Info**: `GetEnvironmentVariable(name)`, `ClearScreen()`, `Beep(frequency, duration)`
- **Debugging**: `DebugPrint(value)`, `DebugPrintType(value)`
- **Programm-Kontrolle**: `Exit(code)`
- **Enterprise-Funktionen**: `GetCurrentDirectory()`, `GetMachineName()`, `GetUserName()`, `GetOSVersion()`, `GetProcessorCount()`, `GetWorkingSet()`, `PlaySound(frequency, duration)`, `Vibrate(duration)`, `DebugPrintMemory()`, `DebugPrintStackTrace()`, `DebugPrintEnvironment()`

### **v3.0.0 Performance-Monitoring:**

- **Performance-Metriken**: `GetPerformanceMetrics()` - Liefert detaillierte Performance-Daten

### Datei- und Verzeichnisoperationen:

- **Datei-Operationen**: `FileExists(path)`, `ReadFile(path)`, `WriteFile(path, content)`, `AppendFile(path, content)`, `ReadLines(path)`, `WriteLines(path, lines)`, `GetFileSize(path)`, `GetFileExtension(path)`, `GetFileName(path)`, `GetDirectoryName(path)`
- **Verzeichnis-Operationen**: `DirectoryExists(path)`, `CreateDirectory(path)`, `GetFiles(path, pattern)`, `GetDirectories(path)`

### JSON- und Kryptologische Funktionen:

- **JSON-Verarbeitung**: `ToJson(obj)`, `FromJson(json)`
- **Kryptologische Funktionen**: `HashMD5(input)`, `HashSHA256(input)`, `Base64Encode(input)`, `Base64Decode(input)`

### **v3.0.0 Netzwerk-Funktionen:**

- **HTTP-Requests**: `HttpGet(url)`, `HttpPost(url, data)`

### **v3.0.0 Datenbank-ähnliche Funktionen:**

- **Record-Management**: `CreateRecord(keys, values)`, `GetRecordValue(record, key)`, `SetRecordValue(record, key, value)`

### **v3.0.0 Validierungsfunktionen:**

- **Input-Validierung**: `IsValidEmail(email)`, `IsValidUrl(url)`, `IsValidJson(json)`

### **v3.0.0 Formatierungsfunktionen:**

- **Formatierung**: `FormatNumber(number, decimals)`, `FormatCurrency(amount, currency)`, `FormatPercentage(value)`

## 🎨 Sprachdesign

HypnoScript kombiniert:

- **TypeScript-ähnliche Syntax** für Vertrautheit
- **Hypnotische Terminologie** für den esoterischen Charme
- **Moderne Sprachfeatures** für praktische Nutzbarkeit
- **Turing-Vollständigkeit** für universelle Berechnungsfähigkeit
- **Umfassende Standardbibliothek** für produktive Entwicklung
- **Enterprise-Features** für professionelle Anwendungen
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
- ✅ **Enterprise-Features**: Vollständig implementiert
- ✅ **v3.0.0 Features**: Vollständig implementiert
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
- `test_enterprise_features.hyp` - Vollständige Enterprise-Demonstration
- `test_enterprise_v3.hyp` - **NEU**: Enterprise v3.0.0 Features
- `test_all_enterprise_features.ps1` - Umfassendes Test-Skript
- `test_enterprise_v3.ps1` - **NEU**: Enterprise v3.0.0 Test-Suite

## 🚀 Quick Start

```bash
# Repository klonen
git clone <repository-url>
cd hyp-runtime

# Projekt kompilieren
dotnet build

# Enterprise v3.0.0 Demo ausführen
dotnet run --project HypnoScript.CLI -- run test_enterprise_v3.hyp

# Umfassende Tests ausführen
./test_enterprise_v3.ps1
```

---

**HypnoScript Enterprise Edition v3.0.0** - Where programming meets hypnosis! 🧠✨

_Version 3.0.0 - Enterprise Edition with Advanced Features including Machine Learning, Networking, and Enhanced Hypnotic Functions_

**Features**: 150+ Builtin Functions | 18 CLI Commands | File Operations | JSON Processing | Cryptographic Functions | Advanced Mathematics | Performance Optimizations | WebAssembly Support | Machine Learning Integration | Networking Functions | Database-like Operations | Enhanced Hypnotic Functions | Performance Monitoring | Input Validation | Formatting Functions | Advanced Array Operations | Advanced String Operations
