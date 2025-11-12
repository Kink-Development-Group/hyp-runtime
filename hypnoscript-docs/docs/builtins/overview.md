---
sidebar_position: 1
---

# Builtin-Funktionen Übersicht

HypnoScript bietet eine umfassende Standardbibliothek mit über **200+ eingebauten Funktionen**, die in verschiedene Kategorien unterteilt sind. Diese Funktionen sind direkt in der Sprache verfügbar und erfordern keine zusätzlichen Imports.

## Kategorien

### 🔢 Array-Funktionen

Funktionen für die Arbeit mit Arrays und Listen.

| Funktion                      | Beschreibung          | Beispiel                          |
| ----------------------------- | --------------------- | --------------------------------- |
| `ArrayLength(arr)`            | Länge des Arrays      | `ArrayLength([1,2,3])` → `3`      |
| `ArrayGet(arr, index)`        | Element an Index      | `ArrayGet([1,2,3], 1)` → `2`      |
| `ArraySet(arr, index, value)` | Setzt Wert an Index   | `ArraySet(arr, 0, "neu")`         |
| `ArraySort(arr)`              | Sortiert Array        | `ArraySort([3,1,2])` → `[1,2,3]`  |
| `ShuffleArray(arr)`           | Mischt Array zufällig | `ShuffleArray([1,2,3,4,5])`       |
| `SumArray(arr)`               | Summe aller Werte     | `SumArray([1,2,3,4,5])` → `15`    |
| `AverageArray(arr)`           | Durchschnitt          | `AverageArray([1,2,3,4,5])` → `3` |

[→ Detaillierte Array-Funktionen](./array-functions)

### 📝 String-Funktionen

Funktionen für String-Manipulation und -Analyse.

| Funktion                        | Beschreibung    | Beispiel                             |
| ------------------------------- | --------------- | ------------------------------------ |
| `Length(str)`                   | String-Länge    | `Length("Hallo")` → `5`              |
| `Substring(str, start, length)` | Teilstring      | `Substring("Hallo", 1, 3)` → `"all"` |
| `ToUpper(str)`                  | Großbuchstaben  | `ToUpper("hallo")` → `"HALLO"`       |
| `Reverse(str)`                  | Kehrt String um | `Reverse("Hallo")` → `"ollaH"`       |
| `IsPalindrome(str)`             | Prüft Palindrom | `IsPalindrome("anna")` → `true`      |
| `CountWords(str)`               | Zählt Wörter    | `CountWords("Hallo Welt")` → `2`     |

[→ Detaillierte String-Funktionen](./string-functions)

### 🧮 Mathematische Funktionen

Umfassende mathematische Operationen und Berechnungen.

| Funktion                     | Beschreibung                | Beispiel                |
| ---------------------------- | --------------------------- | ----------------------- |
| `Sin(x)`, `Cos(x)`, `Tan(x)` | Trigonometrische Funktionen | `Sin(90)` → `1.0`       |
| `Sqrt(x)`                    | Quadratwurzel               | `Sqrt(16)` → `4.0`      |
| `Pow(x, y)`                  | Potenz                      | `Pow(2, 3)` → `8.0`     |
| `Factorial(n)`               | Fakultät                    | `Factorial(5)` → `120`  |
| `Random()`                   | Zufallszahl [0,1)           | `Random()` → `0.123...` |
| `IsPrime(n)`                 | Prüft Primzahl              | `IsPrime(17)` → `true`  |

[→ Detaillierte Mathematische Funktionen](./math-functions)

### 🛠️ Utility-Funktionen

Allgemeine Hilfsfunktionen für verschiedene Anwendungsfälle.

| Funktion                | Beschreibung         | Beispiel                                                    |
| ----------------------- | -------------------- | ----------------------------------------------------------- |
| `Clamp(x, min, max)`    | Begrenzt Wert        | `Clamp(15, 0, 10)` → `10`                                   |
| `IsEven(x)`, `IsOdd(x)` | Gerade/Ungerade      | `IsEven(4)` → `true`                                        |
| `IsValidEmail(str)`     | E-Mail-Validierung   | `IsValidEmail("test@example.com")` → `true`                 |
| `GenerateUUID()`        | UUID generieren      | `GenerateUUID()` → `"123e4567-e89b-12d3-a456-426614174000"` |
| `FormatCurrency(x)`     | Währungsformatierung | `FormatCurrency(1234.56)` → `"$1,234.56"`                   |

[→ Detaillierte Utility-Funktionen](./utility-functions)

### 💻 System-Funktionen

Funktionen für System-Interaktion und -Informationen.

| Funktion              | Beschreibung    | Beispiel                                |
| --------------------- | --------------- | --------------------------------------- |
| `GetCurrentTime()`    | Unix-Timestamp  | `GetCurrentTime()` → `1640995200`       |
| `GetCurrentDate()`    | Aktuelles Datum | `GetCurrentDate()` → `"2024-01-01"`     |
| `GetMachineName()`    | Rechnername     | `GetMachineName()` → `"DESKTOP-ABC123"` |
| `GetUserName()`       | Benutzername    | `GetUserName()` → `"john.doe"`          |
| `GetProcessorCount()` | CPU-Kerne       | `GetProcessorCount()` → `8`             |
| `ClearScreen()`       | Konsole löschen | `ClearScreen()`                         |

[→ Detaillierte System-Funktionen](./system-functions)

### 🕒 Zeit- und Datumsfunktionen

Erweiterte Funktionen für Zeit- und Datumsverarbeitung.

| Funktion            | Beschreibung    | Beispiel                                    |
| ------------------- | --------------- | ------------------------------------------- |
| `GetDayOfWeek()`    | Wochentag       | `GetDayOfWeek()` → `1` (Montag)             |
| `GetDayOfYear()`    | Tag im Jahr     | `GetDayOfYear()` → `1`                      |
| `IsLeapYear(y)`     | Schaltjahr      | `IsLeapYear(2024)` → `true`                 |
| `AddDays(date, n)`  | Tage addieren   | `AddDays("2024-01-01", 7)` → `"2024-01-08"` |
| `GetAge(birthDate)` | Alter berechnen | `GetAge("1990-01-01")` → `34`               |

[→ Detaillierte Zeit- und Datumsfunktionen](./time-date-functions)

### 📊 Statistik-Funktionen

Funktionen für statistische Berechnungen und Analysen.

| Funktion                          | Beschreibung       | Beispiel                                           |
| --------------------------------- | ------------------ | -------------------------------------------------- |
| `CalculateMean(arr)`              | Mittelwert         | `CalculateMean([1,2,3,4,5])` → `3`                 |
| `CalculateStandardDeviation(arr)` | Standardabweichung | `CalculateStandardDeviation([1,2,3,4,5])` → `1.58` |
| `LinearRegression(x, y)`          | Lineare Regression | `LinearRegression([1,2,3], [2,4,6])` → `2.0`       |

[→ Detaillierte Statistik-Funktionen](./statistics-functions)

### 🔐 Hashing/Encoding

Funktionen für Kryptographie und Datenkodierung.

| Funktion            | Beschreibung       | Beispiel                                                                                    |
| ------------------- | ------------------ | ------------------------------------------------------------------------------------------- |
| `HashMD5(str)`      | MD5-Hash           | `HashMD5("test")` → `"098f6bcd4621d373cade4e832627b4f6"`                                    |
| `HashSHA256(str)`   | SHA256-Hash        | `HashSHA256("test")` → `"9f86d081884c7d659a2feaa0c55ad015a3bf4f1b2b0b822cd15d6c15b0f00a08"` |
| `Base64Encode(str)` | Base64-Kodierung   | `Base64Encode("test")` → `"dGVzdA=="`                                                       |
| `Base64Decode(str)` | Base64-Dekodierung | `Base64Decode("dGVzdA==")` → `"test"`                                                       |

[→ Detaillierte Hashing/Encoding-Funktionen](./hashing-encoding)

### 🧠 Hypnotische Spezialfunktionen

Einzigartige Funktionen für hypnotische Anwendungen.

| Funktion                       | Beschreibung            | Beispiel                                  |
| ------------------------------ | ----------------------- | ----------------------------------------- |
| `DeepTrance(duration)`         | Tiefe Trance            | `DeepTrance(5000)`                        |
| `HypnoticCountdown(from)`      | Countdown               | `HypnoticCountdown(10)`                   |
| `TranceInduction(name)`        | Trance-Induktion        | `TranceInduction("Max")`                  |
| `HypnoticSuggestion(msg)`      | Suggestion              | `HypnoticSuggestion("Du bist entspannt")` |
| `ProgressiveRelaxation(steps)` | Progressive Entspannung | `ProgressiveRelaxation(5)`                |

[→ Detaillierte Hypnotische Funktionen](./hypnotic-functions)

### 📚 Dictionary-Funktionen

Funktionen für die Arbeit mit Key-Value-Paaren.

| Funktion                          | Beschreibung      | Beispiel                                    |
| --------------------------------- | ----------------- | ------------------------------------------- |
| `CreateDictionary()`              | Leeres Dictionary | `CreateDictionary()` → `{}`                 |
| `DictionaryKeys(dict)`            | Alle Keys         | `DictionaryKeys(dict)` → `["key1", "key2"]` |
| `DictionaryGet(dict, key)`        | Wert abrufen      | `DictionaryGet(dict, "key1")` → `"value1"`  |
| `DictionarySet(dict, key, value)` | Wert setzen       | `DictionarySet(dict, "key1", "value1")`     |

[→ Detaillierte Dictionary-Funktionen](./dictionary-functions)

### 📁 Datei-Funktionen

Funktionen für Dateisystem-Operationen.

| Funktion                   | Beschreibung    | Beispiel                             |
| -------------------------- | --------------- | ------------------------------------ |
| `FileExists(path)`         | Datei existiert | `FileExists("test.txt")` → `true`    |
| `ReadFile(path)`           | Datei lesen     | `ReadFile("test.txt")` → `"Inhalt"`  |
| `WriteFile(path, content)` | Datei schreiben | `WriteFile("test.txt", "Hallo")`     |
| `GetFileSize(path)`        | Dateigröße      | `GetFileSize("test.txt")` → `1024`   |
| `FileCopy(source, dest)`   | Datei kopieren  | `FileCopy("source.txt", "dest.txt")` |

[→ Detaillierte Datei-Funktionen](./file-functions)

### 🌐 Netzwerk-Funktionen

Funktionen für Web- und Netzwerk-Operationen.

| Funktion              | Beschreibung       | Beispiel                                                      |
| --------------------- | ------------------ | ------------------------------------------------------------- |
| `HttpGet(url)`        | HTTP GET-Request   | `HttpGet("https://api.example.com/data")`                     |
| `HttpPost(url, data)` | HTTP POST-Request  | `HttpPost("https://api.example.com", "data")`                 |
| `IsValidUrl(str)`     | URL-Validierung    | `IsValidUrl("https://example.com")` → `true`                  |
| `ExtractDomain(url)`  | Domain extrahieren | `ExtractDomain("https://example.com/path")` → `"example.com"` |

[→ Detaillierte Netzwerk-Funktionen](./network-functions)

### ✅ Validierung-Funktionen

Funktionen für Datenvalidierung und -formatierung.

| Funktion                  | Beschreibung              | Beispiel                                               |
| ------------------------- | ------------------------- | ------------------------------------------------------ |
| `IsValidEmail(str)`       | E-Mail-Validierung        | `IsValidEmail("test@example.com")` → `true`            |
| `IsValidPhoneNumber(str)` | Telefonnummer             | `IsValidPhoneNumber("+49123456789")` → `true`          |
| `IsValidCreditCard(str)`  | Kreditkarte               | `IsValidCreditCard("4111111111111111")` → `true`       |
| `FormatPhoneNumber(str)`  | Telefonnummer formatieren | `FormatPhoneNumber("1234567890")` → `"(123) 456-7890"` |

[→ Detaillierte Validierung-Funktionen](./validation-functions)

### ⚡ Performance-Funktionen

Funktionen für Performance-Monitoring und Debugging.

| Funktion              | Beschreibung          | Beispiel                                               |
| --------------------- | --------------------- | ------------------------------------------------------ |
| `GetMemoryUsage()`    | Speicherverbrauch     | `GetMemoryUsage()` → `1048576`                         |
| `GetCPUUsage()`       | CPU-Auslastung        | `GetCPUUsage()` → `25.5`                               |
| `GetProcessInfo()`    | Prozess-Informationen | `GetProcessInfo()` → `{id: 1234, name: "hypnoscript"}` |
| `Log(message, level)` | Logging               | `Log("Debug info", "DEBUG")`                           |
| `Trace(message)`      | Tracing               | `Trace("Function called")`                             |

[→ Detaillierte Performance-Funktionen](./performance-functions)

## Verwendung

Alle Builtin-Funktionen können direkt in HypnoScript-Code verwendet werden:

```hyp
Focus {
    entrance {
        observe "Builtin-Funktionen Demo";
    }

    // Array-Funktionen
    induce numbers = [1, 2, 3, 4, 5];
    induce sum = SumArray(numbers);
    observe "Summe: " + sum;

    // String-Funktionen
    induce text = "Hallo Welt";
    induce reversed = Reverse(text);
    observe "Umgekehrt: " + reversed;

    // Mathematische Funktionen
    induce sqrt = Sqrt(16);
    observe "Quadratwurzel von 16: " + sqrt;

    // System-Funktionen
    induce currentTime = GetCurrentTime();
    observe "Aktuelle Zeit: " + currentTime;

    // Validierung
    induce isValid = IsValidEmail("test@example.com");
    observe "E-Mail gültig: " + isValid;
} Relax;
```

## Nächste Schritte

- [Array-Funktionen](./array-functions) - Detaillierte Dokumentation aller Array-Funktionen
- [String-Funktionen](./string-functions) - Umfassende String-Manipulation
- [Mathematische Funktionen](./math-functions) - Mathematische Operationen und Berechnungen
- [Beispiele](../examples/basic-examples) - Praktische Beispiele für Builtin-Funktionen
