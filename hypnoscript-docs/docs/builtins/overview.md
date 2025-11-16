---
sidebar_position: 1
---

# Builtin Functions Overview

HypnoScript bietet eine umfassende Standardbibliothek mit über **110 eingebauten Functionen** in der Rust-Edition. Diese Functionen are available directly in the language and require no additional imports.

## Categoryn

### 🧠 Core & Hypnotische Functionen

Basic I/O, Konvertierung und hypnotische Spezialfunktionen.

| Function                  | Description                        | Example                            |
| ------------------------- | ----------------------------------- | ----------------------------------- |
| `observe(text)`           | Standard-Output mit Zeilenumbruch  | `observe "Hello World";`             |
| `whisper(text)`           | Output ohne Zeilenumbruch          | `whisper "Teil1"; whisper "Teil2";` |
| `command(text)`           | Imperative Output (Großbuchstaben) | `command "Wichtig!";`               |
| `drift(ms)`               | Pause/Sleep in Millisekunden        | `drift(2000);`                      |
| `DeepTrance(duration)`    | Tiefe Trance-Induktion              | `DeepTrance(5000);`                 |
| `HypnoticCountdown(from)` | Hypnotischer Countdown              | `HypnoticCountdown(10);`            |
| `TranceInduction(name)`   | Vollständige Trance-Induktion       | `TranceInduction("Max");`           |
| `ToInt(value)`            | Zu Integer konvertieren             | `ToInt(3.14)` → `3`                 |
| `ToString(value)`         | Zu String konvertieren              | `ToString(42)` → `"42"`             |
| `ToBoolean(value)`        | Zu Boolean konvertieren             | `ToBoolean("true")` → `true`        |

### 🔢 Math-Functionen

Umfassende mathematische Operationen und Berechnungen.

| Category              | Functionen                                        |
| ---------------------- | ------------------------------------------------- |
| **Trigonometrie**      | `Sin`, `Cos`, `Tan`                               |
| **Wurzeln & Potenzen** | `Sqrt`, `Pow`                                     |
| **Logarithmen**        | `Log` (ln), `Log10`                               |
| **Rundung**            | `Abs`, `Floor`, `Ceil`, `Round`, `Clamp`          |
| **Min/Max**            | `Min`, `Max`                                      |
| **Zahlentheorie**      | `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci` |

**Example:**

```hyp
induce result: number = Sqrt(16);  // 4.0
induce isPrime: boolean = IsPrime(17);  // true
induce fib: number = Fibonacci(10);  // 55
```

### 📝 String-Functionen

Functionen für String-Manipulation und -Analyse.

| Category        | Functionen                                                      |
| ---------------- | --------------------------------------------------------------- |
| **Basis**        | `Length`, `ToUpper`, `ToLower`, `Trim`, `Reverse`, `Capitalize` |
| **Suchen**       | `IndexOf`, `Contains`, `StartsWith`, `EndsWith`                 |
| **Manipulation** | `Replace`, `Split`, `Substring`, `Repeat`                       |
| **Padding**      | `PadLeft`, `PadRight`                                           |
| **Prüfungen**    | `IsEmpty`, `IsWhitespace`                                       |

**Example:**

```hyp
induce text: string = "  Hallo Welt  ";
induce cleaned: string = Trim(text);  // "Hallo Welt"
induce upper: string = ToUpper(cleaned);  // "HALLO WELT"
induce words: string[] = Split(cleaned, " ");  // ["Hallo", "Welt"]
```

### 📦 Array-Functionen

Functionen für die Arbeit mit Arrays und Listen.

| Category          | Functionen                                        |
| ------------------ | ------------------------------------------------- |
| **Basis**          | `Length`, `IsEmpty`, `Get`, `IndexOf`, `Contains` |
| **Transformation** | `Reverse`, `Sort`, `Distinct`                     |
| **Aggregation**    | `Sum`, `Average`, `Min`, `Max`                    |
| **Slicing**        | `First`, `Last`, `Take`, `Skip`, `Slice`          |
| **Weitere**        | `Join`, `Count`                                   |

**Example:**

```hyp
induce numbers: number[] = [5, 2, 8, 1, 9];
induce sorted: number[] = Sort(numbers);  // [1, 2, 5, 8, 9]
induce sum: number = Sum(numbers);  // 25
induce avg: number = Average(numbers);  // 5.0
```

[→ Detaillierte Array-Functionen](./array-functions)

### 📊 Statistik-Functionen

Functionen für statistische Berechnungen und Analysen.

| Category            | Functionen                                                                                 |
| -------------------- | ------------------------------------------------------------------------------------------ |
| **Zentrale Tendenz** | `CalculateMean`, `CalculateMedian`, `CalculateMode`                                        |
| **Streuung**         | `CalculateVariance`, `CalculateStandardDeviation`, `CalculateRange`, `CalculatePercentile` |
| **Korrelation**      | `CalculateCorrelation`, `LinearRegression`                                                 |

**Example:**

```hyp
induce data: number[] = [1, 2, 3, 4, 5];
induce mean: number = CalculateMean(data);  // 3.0
induce stddev: number = CalculateStandardDeviation(data);  // 1.58...
```

[→ Detaillierte Statistik-Functionen](./statistics-functions)

### 🕒 Zeit & Datum

Functionen für Zeit- und Datumsverarbeitung.

| Category         | Functionen                                                           |
| ----------------- | -------------------------------------------------------------------- |
| **Aktuelle Zeit** | `GetCurrentTime`, `GetCurrentDate`, `GetCurrentDateTime`             |
| **Komponenten**   | `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond` |
| **Berechnungen**  | `GetDayOfWeek`, `GetDayOfYear`, `IsLeapYear`, `GetDaysInMonth`       |

**Example:**

```hyp
induce timestamp: number = GetCurrentTime();  // Unix timestamp
induce date: string = GetCurrentDate();  // "2025-01-15"
induce year: number = GetYear();  // 2025
```

[→ Detaillierte Zeit/Datum-Functionen](./time-date-functions)

### 💻 System-Functionen

Functionen für System-Interaktion und -Informationen.

| Category         | Functionen                                                                           |
| ----------------- | ------------------------------------------------------------------------------------ |
| **System-Info**   | `GetOperatingSystem`, `GetArchitecture`, `GetCpuCount`, `GetHostname`, `GetUsername` |
| **Directories** | `GetCurrentDirectory`, `GetHomeDirectory`, `GetTempDirectory`                        |
| **Umgebung**      | `GetEnvVar`, `SetEnvVar`, `GetArgs`                                                  |
| **Prozess**       | `Exit`                                                                               |

**Example:**

```hyp
induce os: string = GetOperatingSystem();  // "Windows", "Linux", "macOS"
induce cores: number = GetCpuCount();  // 8
induce home: string = GetHomeDirectory();  // "/home/user" oder "C:\\Users\\user"
```

[→ Detaillierte System-Functionen](./system-functions)

### 📁 File-Functionen

Functionen für Filesystem-Operationen.

| Category           | Functionen                                                             |
| ------------------- | ---------------------------------------------------------------------- |
| **Lesen/Schreiben** | `ReadFile`, `WriteFile`, `AppendFile`                                  |
| **Verwaltung**      | `DeleteFile`, `CopyFile`, `RenameFile`                                 |
| **Prüfungen**       | `FileExists`, `IsFile`, `IsDirectory`                                  |
| **Informationen**   | `GetFileSize`, `GetFileExtension`, `GetFileName`, `GetParentDirectory` |
| **Directories**   | `CreateDirectory`, `ListDirectory`                                     |

**Example:**

```hyp
if (FileExists("config.txt")) {
    induce content: string = ReadFile("config.txt");
    observe "Config: " + content;
} else {
    WriteFile("config.txt", "default config");
}
```

[→ Detaillierte File-Functionen](./file-functions)

### 🧩 CLI & Automation

Neue Builtins helfen beim Bau interaktiver Tools und Skripte.

| Function         | Description                                          |
| ---------------- | ----------------------------------------------------- |
| `CliPrompt`      | Lokalisierte Texteingabe mit Defaultwerten            |
| `CliConfirm`     | Ja/Nein-Bestätigung mit `Y/n` bzw. `J/n`-Hinweis      |
| `ParseArguments` | Zerlegt CLI-Arguments in Flags und Positionsparameter |
| `HasFlag`        | Checks, ob ein Flag gesetzt wurde                      |
| `FlagValue`      | Reads den Wert eines Flags (`--port 8080` → `8080`)   |

**Example:**

```hyp
induce args: string[] = GetArgs();
if (HasFlag(args, "help")) {
    observe "Nutze --port <PORT>";
    Exit(0);
}

induce port = FlagValue(args, "port") ?? "8080";
induce answer = CliPrompt("Service-Name", "demo", false, "de-DE");
induce confirm = CliConfirm("Deployment starten?", true, "de-DE");
```

### 🌐 API- & Service-Functionen

Kombiniert HTTP-Clients mit Service-Health-Werkzeugen.

| Function            | Description                                              |
| ------------------- | --------------------------------------------------------- |
| `HttpSend`          | Allgemeiner HTTP-Client (Methoden, Header, Auth, Timeout) |
| `HttpGetJson`       | `GET` mit automatischem JSON-Parsing                      |
| `HttpPostJson`      | `POST` JSON → JSON (inkl. Content-Type)                   |
| `ServiceHealth`     | Creates Health-Report (Uptime, Latenz, P95, SLO)         |
| `RetrySchedule`     | Liefert exponentiellen Backoff-Plan mit optionalem Jitter |
| `CircuitShouldOpen` | Bewertet Fehlerfenster für Circuit-Breaker                |

**Example:**

```hyp
induce response = HttpGetJson("https://api.example.com/status");
if (response.ok != true) {
    observe "API meldet Fehler";
}

induce schedule: number[] = RetrySchedule(5, 250, 2.0, 50, 4000);
observe "Versuche alle " + schedule[0] + "ms";
```

### 🧾 Datenformate (JSON & CSV)

| Function           | Description                                  |
| ------------------ | --------------------------------------------- |
| `JsonPretty`       | Formatiert JSON für Logs                      |
| `JsonQuery`        | Pfadabfrage (`data.items[0].name`)            |
| `JsonMerge`        | Rekursive Zusammenführung zweier Dokumente    |
| `ParseCsv`         | Reads CSV (Delimiter + Header konfigurierbar) |
| `CsvSelectColumns` | Projiziert Spalten nach Namen                 |
| `CsvToString`      | Baut wieder CSV-Text aus Tabellenstruktur     |

**Example:**

```hyp
induce payload = JsonPretty(ReadFile("response.json"));
induce table = ParseCsv(ReadFile("data.csv"));
induce namesOnly = CsvSelectColumns(table, ["name"]);
WriteFile("names.csv", CsvToString(namesOnly));
```

### ✅ Validierung

Functionen für Datenvalidierung.

| Category   | Functionen                                                                  |
| ----------- | --------------------------------------------------------------------------- |
| **Format**  | `IsValidEmail`, `IsValidUrl`, `IsValidPhoneNumber`                          |
| **Zeichen** | `IsAlphanumeric`, `IsAlphabetic`, `IsNumeric`, `IsLowercase`, `IsUppercase` |
| **Weitere** | `IsInRange`, `MatchesPattern`                                               |

**Example:**

```hyp
induce email: string = "user@example.com";
if (IsValidEmail(email)) {
    observe "Gültige E-Mail!";
}
```

[→ Detaillierte Validierung-Functionen](./validation-functions)

### 🔐 Hashing & String-Analyse

Functionen für Hashing und erweiterte String-Operationen.

| Category          | Functionen                                                          |
| ------------------ | ------------------------------------------------------------------- |
| **Hashing**        | `HashString`, `HashNumber`, `SimpleRandom`                          |
| **Analyse**        | `AreAnagrams`, `IsPalindrome`, `CountOccurrences`                   |
| **Transformation** | `RemoveDuplicates`, `UniqueCharacters`, `ReverseWords`, `TitleCase` |

**Example:**

```hyp
induce hash: number = HashString("password");
induce isPalin: boolean = IsPalindrome("anna");  // true
induce titleText: string = TitleCase("hello world");  // "Hello World"
```

[→ Detaillierte Hashing-Functionen](./hashing-encoding)

### 🧠 DeepMind (Higher-Order Functions)

Advanced funktionale Programmierung und Kontrollfluss.

| Category            | Functionen                                                       |
| -------------------- | ---------------------------------------------------------------- |
| **Schleifen**        | `RepeatAction`, `RepeatUntil`, `RepeatWhile`                     |
| **Verzögerung**      | `DelayedSuggestion`                                              |
| **Komposition**      | `Compose`, `Pipe`                                                |
| **Fehlerbehandlung** | `TryOrAwaken`, `EnsureAwakening`                                 |
| **Weitere**          | `IfTranced`, `SequentialTrance`, `MeasureTranceDepth`, `Memoize` |

**Example:**

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

[→ Detaillierte DeepMind-Functionen](./deepmind-functions)

## Usage

Alle Builtin Functions können direkt in HypnoScript-Code without imports:

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

## CLI-Command

Liste alle Builtin Functions im Terminal:

```bash
hypnoscript builtins
```

## Complete Reference

Für eine vollständige alphabetische Liste aller 110+ Functionen siehe:

[→ Vollständige Builtin-Referenz](./_complete-reference)

## Categoryn-Index

- [Math-Functionen](./math-functions)
- [String-Functionen](./string-functions)
- [Array-Functionen](./array-functions)
- [Statistik-Functionen](./statistics-functions)
- [Zeit/Datum-Functionen](./time-date-functions)
- [System-Functionen](./system-functions)
- [File-Functionen](./file-functions)
- [Validierung-Functionen](./validation-functions)
- [Hashing-Functionen](./hashing-encoding)
- [DeepMind-Functionen](./deepmind-functions)
- [Hypnotische Functionen](./hypnotic-functions)

## Next Steps

- [Examplee](../examples/basic-examples) - Praktische Examplee
- [Language Reference](../language-reference/syntax) - Sprachsyntax
- [CLI Commands](../cli/commands) - Kommandozeilenbefehle
