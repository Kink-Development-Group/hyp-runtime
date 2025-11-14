---
sidebar_position: 1
---

# Builtin-Funktionen Übersicht

HypnoScript bietet eine umfassende Standardbibliothek mit über **110 eingebauten Funktionen** in der Rust-Edition. Diese Funktionen sind direkt in der Sprache verfügbar und erfordern keine zusätzlichen Imports.

## Kategorien

### 🧠 Core & Hypnotische Funktionen

Grundlegende I/O, Konvertierung und hypnotische Spezialfunktionen.

| Funktion                  | Beschreibung                        | Beispiel                            |
| ------------------------- | ----------------------------------- | ----------------------------------- |
| `observe(text)`           | Standard-Ausgabe mit Zeilenumbruch  | `observe "Hallo Welt";`             |
| `whisper(text)`           | Ausgabe ohne Zeilenumbruch          | `whisper "Teil1"; whisper "Teil2";` |
| `command(text)`           | Imperative Ausgabe (Großbuchstaben) | `command "Wichtig!";`               |
| `drift(ms)`               | Pause/Sleep in Millisekunden        | `drift(2000);`                      |
| `DeepTrance(duration)`    | Tiefe Trance-Induktion              | `DeepTrance(5000);`                 |
| `HypnoticCountdown(from)` | Hypnotischer Countdown              | `HypnoticCountdown(10);`            |
| `TranceInduction(name)`   | Vollständige Trance-Induktion       | `TranceInduction("Max");`           |
| `ToInt(value)`            | Zu Integer konvertieren             | `ToInt(3.14)` → `3`                 |
| `ToString(value)`         | Zu String konvertieren              | `ToString(42)` → `"42"`             |
| `ToBoolean(value)`        | Zu Boolean konvertieren             | `ToBoolean("true")` → `true`        |

### 🔢 Math-Funktionen

Umfassende mathematische Operationen und Berechnungen.

| Kategorie              | Funktionen                                        |
| ---------------------- | ------------------------------------------------- |
| **Trigonometrie**      | `Sin`, `Cos`, `Tan`                               |
| **Wurzeln & Potenzen** | `Sqrt`, `Pow`                                     |
| **Logarithmen**        | `Log` (ln), `Log10`                               |
| **Rundung**            | `Abs`, `Floor`, `Ceil`, `Round`, `Clamp`          |
| **Min/Max**            | `Min`, `Max`                                      |
| **Zahlentheorie**      | `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci` |

**Beispiel:**

```hyp
induce result: number = Sqrt(16);  // 4.0
induce isPrime: boolean = IsPrime(17);  // true
induce fib: number = Fibonacci(10);  // 55
```

### 📝 String-Funktionen

Funktionen für String-Manipulation und -Analyse.

| Kategorie        | Funktionen                                                      |
| ---------------- | --------------------------------------------------------------- |
| **Basis**        | `Length`, `ToUpper`, `ToLower`, `Trim`, `Reverse`, `Capitalize` |
| **Suchen**       | `IndexOf`, `Contains`, `StartsWith`, `EndsWith`                 |
| **Manipulation** | `Replace`, `Split`, `Substring`, `Repeat`                       |
| **Padding**      | `PadLeft`, `PadRight`                                           |
| **Prüfungen**    | `IsEmpty`, `IsWhitespace`                                       |

**Beispiel:**

```hyp
induce text: string = "  Hallo Welt  ";
induce cleaned: string = Trim(text);  // "Hallo Welt"
induce upper: string = ToUpper(cleaned);  // "HALLO WELT"
induce words: string[] = Split(cleaned, " ");  // ["Hallo", "Welt"]
```

### 📦 Array-Funktionen

Funktionen für die Arbeit mit Arrays und Listen.

| Kategorie          | Funktionen                                        |
| ------------------ | ------------------------------------------------- |
| **Basis**          | `Length`, `IsEmpty`, `Get`, `IndexOf`, `Contains` |
| **Transformation** | `Reverse`, `Sort`, `Distinct`                     |
| **Aggregation**    | `Sum`, `Average`, `Min`, `Max`                    |
| **Slicing**        | `First`, `Last`, `Take`, `Skip`, `Slice`          |
| **Weitere**        | `Join`, `Count`                                   |

**Beispiel:**

```hyp
induce numbers: number[] = [5, 2, 8, 1, 9];
induce sorted: number[] = Sort(numbers);  // [1, 2, 5, 8, 9]
induce sum: number = Sum(numbers);  // 25
induce avg: number = Average(numbers);  // 5.0
```

[→ Detaillierte Array-Funktionen](./array-functions)

### 📊 Statistik-Funktionen

Funktionen für statistische Berechnungen und Analysen.

| Kategorie            | Funktionen                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------ |
| **Zentrale Tendenz** | `CalculateMean`, `CalculateMedian`, `CalculateMode`                                        |
| **Streuung**         | `CalculateVariance`, `CalculateStandardDeviation`, `CalculateRange`, `CalculatePercentile` |
| **Korrelation**      | `CalculateCorrelation`, `LinearRegression`                                                 |

**Beispiel:**

```hyp
induce data: number[] = [1, 2, 3, 4, 5];
induce mean: number = CalculateMean(data);  // 3.0
induce stddev: number = CalculateStandardDeviation(data);  // 1.58...
```

[→ Detaillierte Statistik-Funktionen](./statistics-functions)

### 🕒 Zeit & Datum

Funktionen für Zeit- und Datumsverarbeitung.

| Kategorie         | Funktionen                                                           |
| ----------------- | -------------------------------------------------------------------- |
| **Aktuelle Zeit** | `GetCurrentTime`, `GetCurrentDate`, `GetCurrentDateTime`             |
| **Komponenten**   | `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond` |
| **Berechnungen**  | `GetDayOfWeek`, `GetDayOfYear`, `IsLeapYear`, `GetDaysInMonth`       |

**Beispiel:**

```hyp
induce timestamp: number = GetCurrentTime();  // Unix timestamp
induce date: string = GetCurrentDate();  // "2025-01-15"
induce year: number = GetYear();  // 2025
```

[→ Detaillierte Zeit/Datum-Funktionen](./time-date-functions)

### 💻 System-Funktionen

Funktionen für System-Interaktion und -Informationen.

| Kategorie         | Funktionen                                                                           |
| ----------------- | ------------------------------------------------------------------------------------ |
| **System-Info**   | `GetOperatingSystem`, `GetArchitecture`, `GetCpuCount`, `GetHostname`, `GetUsername` |
| **Verzeichnisse** | `GetCurrentDirectory`, `GetHomeDirectory`, `GetTempDirectory`                        |
| **Umgebung**      | `GetEnvVar`, `SetEnvVar`, `GetArgs`                                                  |
| **Prozess**       | `Exit`                                                                               |

**Beispiel:**

```hyp
induce os: string = GetOperatingSystem();  // "Windows", "Linux", "macOS"
induce cores: number = GetCpuCount();  // 8
induce home: string = GetHomeDirectory();  // "/home/user" oder "C:\\Users\\user"
```

[→ Detaillierte System-Funktionen](./system-functions)

### 📁 Datei-Funktionen

Funktionen für Dateisystem-Operationen.

| Kategorie           | Funktionen                                                             |
| ------------------- | ---------------------------------------------------------------------- |
| **Lesen/Schreiben** | `ReadFile`, `WriteFile`, `AppendFile`                                  |
| **Verwaltung**      | `DeleteFile`, `CopyFile`, `RenameFile`                                 |
| **Prüfungen**       | `FileExists`, `IsFile`, `IsDirectory`                                  |
| **Informationen**   | `GetFileSize`, `GetFileExtension`, `GetFileName`, `GetParentDirectory` |
| **Verzeichnisse**   | `CreateDirectory`, `ListDirectory`                                     |

**Beispiel:**

```hyp
if (FileExists("config.txt")) {
    induce content: string = ReadFile("config.txt");
    observe "Config: " + content;
} else {
    WriteFile("config.txt", "default config");
}
```

[→ Detaillierte Datei-Funktionen](./file-functions)

### ✅ Validierung

Funktionen für Datenvalidierung.

| Kategorie   | Funktionen                                                                  |
| ----------- | --------------------------------------------------------------------------- |
| **Format**  | `IsValidEmail`, `IsValidUrl`, `IsValidPhoneNumber`                          |
| **Zeichen** | `IsAlphanumeric`, `IsAlphabetic`, `IsNumeric`, `IsLowercase`, `IsUppercase` |
| **Weitere** | `IsInRange`, `MatchesPattern`                                               |

**Beispiel:**

```hyp
induce email: string = "user@example.com";
if (IsValidEmail(email)) {
    observe "Gültige E-Mail!";
}
```

[→ Detaillierte Validierung-Funktionen](./validation-functions)

### 🔐 Hashing & String-Analyse

Funktionen für Hashing und erweiterte String-Operationen.

| Kategorie          | Funktionen                                                          |
| ------------------ | ------------------------------------------------------------------- |
| **Hashing**        | `HashString`, `HashNumber`, `SimpleRandom`                          |
| **Analyse**        | `AreAnagrams`, `IsPalindrome`, `CountOccurrences`                   |
| **Transformation** | `RemoveDuplicates`, `UniqueCharacters`, `ReverseWords`, `TitleCase` |

**Beispiel:**

```hyp
induce hash: number = HashString("password");
induce isPalin: boolean = IsPalindrome("anna");  // true
induce titleText: string = TitleCase("hello world");  // "Hello World"
```

[→ Detaillierte Hashing-Funktionen](./hashing-encoding)

### 🧠 DeepMind (Higher-Order Functions)

Erweiterte funktionale Programmierung und Kontrollfluss.

| Kategorie            | Funktionen                                                       |
| -------------------- | ---------------------------------------------------------------- |
| **Schleifen**        | `RepeatAction`, `RepeatUntil`, `RepeatWhile`                     |
| **Verzögerung**      | `DelayedSuggestion`                                              |
| **Komposition**      | `Compose`, `Pipe`                                                |
| **Fehlerbehandlung** | `TryOrAwaken`, `EnsureAwakening`                                 |
| **Weitere**          | `IfTranced`, `SequentialTrance`, `MeasureTranceDepth`, `Memoize` |

**Beispiel:**

```hyp
// Aktion 5 mal wiederholen
RepeatAction(5, suggestion() {
    observe "Wiederholt!";
});

// Funktionskomposition
suggestion double(x: number): number {
    awaken x * 2;
}

suggestion addTen(x: number): number {
    awaken x + 10;
}

induce composed = Compose(double, addTen);
induce result: number = composed(5);  // double(addTen(5)) = 30
```

[→ Detaillierte DeepMind-Funktionen](./deepmind-functions)

## Verwendung

Alle Builtin-Funktionen können direkt in HypnoScript-Code verwendet werden, ohne Import:

```hyp
Focus {
    entrance {
        observe "=== Builtin-Funktionen Demo ===";
    }

    // Array-Funktionen
    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce sum: number = Sum(numbers);
    observe "Summe: " + sum;

    // String-Funktionen
    induce text: string = "Hallo Welt";
    induce reversed: string = Reverse(text);
    observe "Umgekehrt: " + reversed;

    // Mathematische Funktionen
    induce sqrt: number = Sqrt(16);
    observe "Quadratwurzel von 16: " + sqrt;

    // System-Funktionen
    induce os: string = GetOperatingSystem();
    observe "Betriebssystem: " + os;

    // Validierung
    induce isValid: boolean = IsValidEmail("test@example.com");
    observe "E-Mail gültig: " + isValid;

    // Statistik
    induce mean: number = CalculateMean([1, 2, 3, 4, 5]);
    observe "Mittelwert: " + mean;

    finale {
        observe "=== Demo beendet ===";
    }
} Relax
```

## CLI-Befehl

Liste alle Builtin-Funktionen im Terminal:

```bash
hypnoscript builtins
```

## Vollständige Referenz

Für eine vollständige alphabetische Liste aller 110+ Funktionen siehe:

[→ Vollständige Builtin-Referenz](./_complete-reference)

## Kategorien-Index

- [Math-Funktionen](./math-functions)
- [String-Funktionen](./string-functions)
- [Array-Funktionen](./array-functions)
- [Statistik-Funktionen](./statistics-functions)
- [Zeit/Datum-Funktionen](./time-date-functions)
- [System-Funktionen](./system-functions)
- [Datei-Funktionen](./file-functions)
- [Validierung-Funktionen](./validation-functions)
- [Hashing-Funktionen](./hashing-encoding)
- [DeepMind-Funktionen](./deepmind-functions)
- [Hypnotische Funktionen](./hypnotic-functions)

## Nächste Schritte

- [Beispiele](../examples/basic-examples) - Praktische Beispiele
- [Language Reference](../language-reference/syntax) - Sprachsyntax
- [CLI Commands](../cli/commands) - Kommandozeilenbefehle
