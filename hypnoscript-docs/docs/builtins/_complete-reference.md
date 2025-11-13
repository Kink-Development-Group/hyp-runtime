# Builtin-Funktionen Vollständige Referenz

Vollständige Referenz aller 110+ Builtin-Funktionen in HypnoScript (Rust-Edition).

## Core Builtins (I/O & Konvertierung)

### Ausgabe-Funktionen

| Funktion  | Signatur                  | Beschreibung                       |
| --------- | ------------------------- | ---------------------------------- |
| `observe` | `(value: string) -> void` | Standard-Ausgabe mit Zeilenumbruch |
| `whisper` | `(value: string) -> void` | Ausgabe ohne Zeilenumbruch         |
| `command` | `(value: string) -> void` | Ausgabe in Großbuchstaben          |
| `drift`   | `(ms: number) -> void`    | Pause/Sleep (in Millisekunden)     |

### Hypnotische Funktionen

| Funktion                | Signatur                        | Beschreibung                           |
| ----------------------- | ------------------------------- | -------------------------------------- |
| `DeepTrance`            | `(duration: number) -> void`    | Tiefe Trance-Induktion mit Verzögerung |
| `HypnoticCountdown`     | `(from: number) -> void`        | Hypnotischer Countdown                 |
| `TranceInduction`       | `(subjectName: string) -> void` | Vollständige Trance-Induktion          |
| `HypnoticVisualization` | `(scene: string) -> void`       | Hypnotische Visualisierung             |

### Konvertierungs-Funktionen

| Funktion    | Signatur                     | Beschreibung                      |
| ----------- | ---------------------------- | --------------------------------- |
| `ToInt`     | `(value: number) -> number`  | Konvertiert zu Integer (truncate) |
| `ToDouble`  | `(value: string) -> number`  | Parse String zu number            |
| `ToString`  | `(value: any) -> string`     | Konvertiert zu String             |
| `ToBoolean` | `(value: string) -> boolean` | Parse String zu boolean           |

## Math Builtins

### Trigonometrische Funktionen

| Funktion | Signatur                | Beschreibung |
| -------- | ----------------------- | ------------ |
| `Sin`    | `(x: number) -> number` | Sinus        |
| `Cos`    | `(x: number) -> number` | Cosinus      |
| `Tan`    | `(x: number) -> number` | Tangens      |

### Wurzel & Potenz

| Funktion | Signatur                                     | Beschreibung  |
| -------- | -------------------------------------------- | ------------- |
| `Sqrt`   | `(x: number) -> number`                      | Quadratwurzel |
| `Pow`    | `(base: number, exponent: number) -> number` | Potenz        |

### Logarithmen

| Funktion | Signatur                | Beschreibung                 |
| -------- | ----------------------- | ---------------------------- |
| `Log`    | `(x: number) -> number` | Natürlicher Logarithmus (ln) |
| `Log10`  | `(x: number) -> number` | Logarithmus zur Basis 10     |

### Rundung

| Funktion | Signatur                | Beschreibung          |
| -------- | ----------------------- | --------------------- |
| `Abs`    | `(x: number) -> number` | Absoluter Wert        |
| `Floor`  | `(x: number) -> number` | Abrunden              |
| `Ceil`   | `(x: number) -> number` | Aufrunden             |
| `Round`  | `(x: number) -> number` | Kaufmännisches Runden |

### Min/Max

| Funktion | Signatur                                              | Beschreibung   |
| -------- | ----------------------------------------------------- | -------------- |
| `Min`    | `(a: number, b: number) -> number`                    | Minimum        |
| `Max`    | `(a: number, b: number) -> number`                    | Maximum        |
| `Clamp`  | `(value: number, min: number, max: number) -> number` | Wert begrenzen |

### Zahlentheorie

| Funktion    | Signatur                           | Beschreibung                     |
| ----------- | ---------------------------------- | -------------------------------- |
| `Factorial` | `(n: number) -> number`            | Fakultät                         |
| `Gcd`       | `(a: number, b: number) -> number` | Größter gemeinsamer Teiler       |
| `Lcm`       | `(a: number, b: number) -> number` | Kleinstes gemeinsames Vielfaches |
| `IsPrime`   | `(n: number) -> boolean`           | Prüft ob Primzahl                |
| `Fibonacci` | `(n: number) -> number`            | n-te Fibonacci-Zahl              |

## String Builtins

### Basis-Operationen

| Funktion     | Signatur                | Beschreibung           |
| ------------ | ----------------------- | ---------------------- |
| `Length`     | `(s: string) -> number` | String-Länge           |
| `ToUpper`    | `(s: string) -> string` | In Großbuchstaben      |
| `ToLower`    | `(s: string) -> string` | In Kleinbuchstaben     |
| `Trim`       | `(s: string) -> string` | Whitespace entfernen   |
| `Reverse`    | `(s: string) -> string` | String umkehren        |
| `Capitalize` | `(s: string) -> string` | Ersten Buchstaben groß |

### Suchen & Ersetzen

| Funktion     | Signatur                                          | Beschreibung                                  |
| ------------ | ------------------------------------------------- | --------------------------------------------- |
| `IndexOf`    | `(s: string, pattern: string) -> number`          | Index des Substrings (-1 wenn nicht gefunden) |
| `Replace`    | `(s: string, from: string, to: string) -> string` | Alle Vorkommen ersetzen                       |
| `Contains`   | `(s: string, pattern: string) -> boolean`         | Prüft ob enthalten                            |
| `StartsWith` | `(s: string, prefix: string) -> boolean`          | Prüft Präfix                                  |
| `EndsWith`   | `(s: string, suffix: string) -> boolean`          | Prüft Suffix                                  |

### Manipulation

| Funktion    | Signatur                                             | Beschreibung           |
| ----------- | ---------------------------------------------------- | ---------------------- |
| `Split`     | `(s: string, delimiter: string) -> string[]`         | String aufteilen       |
| `Substring` | `(s: string, start: number, end: number) -> string`  | Teilstring extrahieren |
| `Repeat`    | `(s: string, times: number) -> string`               | String wiederholen     |
| `PadLeft`   | `(s: string, width: number, char: string) -> string` | Links auffüllen        |
| `PadRight`  | `(s: string, width: number, char: string) -> string` | Rechts auffüllen       |

### Prüfungen

| Funktion       | Signatur                 | Beschreibung            |
| -------------- | ------------------------ | ----------------------- |
| `IsEmpty`      | `(s: string) -> boolean` | Prüft ob leer           |
| `IsWhitespace` | `(s: string) -> boolean` | Prüft ob nur Whitespace |

## Array Builtins

:::note Array-Präfix
Alle Array-Funktionen verwenden das Präfix `Array` zur Unterscheidung von String-Funktionen (z.B. `ArrayLength` vs. String `Length`).
:::

### Basis-Operationen

| Funktion        | Signatur                            | Beschreibung                                |
| --------------- | ----------------------------------- | ------------------------------------------- |
| `ArrayLength`   | `(arr: T[]) -> number`              | Array-Länge                                 |
| `ArrayIsEmpty`  | `(arr: T[]) -> boolean`             | Prüft ob leer                               |
| `ArrayGet`      | `(arr: T[], index: number) -> T`    | Element an Index                            |
| `ArrayIndexOf`  | `(arr: T[], element: T) -> number`  | Index des Elements (-1 wenn nicht gefunden) |
| `ArrayContains` | `(arr: T[], element: T) -> boolean` | Prüft ob enthalten                          |

### Transformation

| Funktion        | Signatur                      | Beschreibung        |
| --------------- | ----------------------------- | ------------------- |
| `ArrayReverse`  | `(arr: T[]) -> T[]`           | Array umkehren      |
| `ArraySort`     | `(arr: number[]) -> number[]` | Numerisch sortieren |
| `ArrayDistinct` | `(arr: T[]) -> T[]`           | Duplikate entfernen |

### Aggregation

| Funktion       | Signatur                    | Beschreibung |
| -------------- | --------------------------- | ------------ |
| `ArraySum`     | `(arr: number[]) -> number` | Summe        |
| `ArrayAverage` | `(arr: number[]) -> number` | Durchschnitt |
| `ArrayMin`     | `(arr: number[]) -> number` | Minimum      |
| `ArrayMax`     | `(arr: number[]) -> number` | Maximum      |

### Slicing

| Funktion     | Signatur                                        | Beschreibung                 |
| ------------ | ----------------------------------------------- | ---------------------------- |
| `ArrayFirst` | `(arr: T[]) -> T`                               | Erstes Element               |
| `ArrayLast`  | `(arr: T[]) -> T`                               | Letztes Element              |
| `ArrayTake`  | `(arr: T[], n: number) -> T[]`                  | Erste n Elemente             |
| `ArraySkip`  | `(arr: T[], n: number) -> T[]`                  | Überspringt erste n Elemente |
| `ArraySlice` | `(arr: T[], start: number, end: number) -> T[]` | Teilarray                    |

### Weitere

| Funktion     | Signatur                                  | Beschreibung              |
| ------------ | ----------------------------------------- | ------------------------- |
| `ArrayJoin`  | `(arr: T[], separator: string) -> string` | Array zu String           |
| `ArrayCount` | `(arr: T[], element: T) -> number`        | Häufigkeit eines Elements |

## Statistics Builtins

### Zentrale Tendenz

| Funktion          | Signatur                    | Beschreibung               |
| ----------------- | --------------------------- | -------------------------- |
| `CalculateMean`   | `(arr: number[]) -> number` | Arithmetisches Mittel      |
| `CalculateMedian` | `(arr: number[]) -> number` | Median                     |
| `CalculateMode`   | `(arr: number[]) -> number` | Modus (häufigstes Element) |

### Streuung

| Funktion                     | Signatur                                        | Beschreibung           |
| ---------------------------- | ----------------------------------------------- | ---------------------- |
| `CalculateVariance`          | `(arr: number[]) -> number`                     | Varianz                |
| `CalculateStandardDeviation` | `(arr: number[]) -> number`                     | Standardabweichung     |
| `CalculateRange`             | `(arr: number[]) -> number`                     | Spannweite (Max - Min) |
| `CalculatePercentile`        | `(arr: number[], percentile: number) -> number` | Perzentil berechnen    |

### Korrelation & Regression

| Funktion               | Signatur                                         | Beschreibung                          |
| ---------------------- | ------------------------------------------------ | ------------------------------------- |
| `CalculateCorrelation` | `(x: number[], y: number[]) -> number`           | Korrelationskoeffizient               |
| `LinearRegression`     | `(x: number[], y: number[]) -> (number, number)` | Lineare Regression (slope, intercept) |

## Time Builtins

### Aktuelle Zeit

| Funktion               | Signatur                     | Beschreibung                 |
| ---------------------- | ---------------------------- | ---------------------------- |
| `GetCurrentTime`       | `() -> number`               | Unix Timestamp (Sekunden)    |
| `GetCurrentDate`       | `() -> string`               | Aktuelles Datum (YYYY-MM-DD) |
| `GetCurrentTimeString` | `() -> string`               | Aktuelle Zeit (HH:MM:SS)     |
| `GetCurrentDateTime`   | `() -> string`               | Datum und Zeit               |
| `FormatDateTime`       | `(format: string) -> string` | Formatierte Zeit             |

### Datum-Komponenten

| Funktion       | Signatur       | Beschreibung            |
| -------------- | -------------- | ----------------------- |
| `GetYear`      | `() -> number` | Aktuelles Jahr          |
| `GetMonth`     | `() -> number` | Aktueller Monat (1-12)  |
| `GetDay`       | `() -> number` | Aktueller Tag (1-31)    |
| `GetHour`      | `() -> number` | Aktuelle Stunde (0-23)  |
| `GetMinute`    | `() -> number` | Aktuelle Minute (0-59)  |
| `GetSecond`    | `() -> number` | Aktuelle Sekunde (0-59) |
| `GetDayOfWeek` | `() -> number` | Wochentag (0=Sonntag)   |
| `GetDayOfYear` | `() -> number` | Tag im Jahr (1-366)     |

### Datum-Berechnungen

| Funktion         | Signatur                                  | Beschreibung     |
| ---------------- | ----------------------------------------- | ---------------- |
| `IsLeapYear`     | `(year: number) -> boolean`               | Prüft Schaltjahr |
| `GetDaysInMonth` | `(year: number, month: number) -> number` | Tage im Monat    |

## System Builtins

### System-Informationen

| Funktion              | Signatur       | Beschreibung          |
| --------------------- | -------------- | --------------------- |
| `GetCurrentDirectory` | `() -> string` | Aktuelles Verzeichnis |
| `GetOperatingSystem`  | `() -> string` | Betriebssystem        |
| `GetArchitecture`     | `() -> string` | CPU-Architektur       |
| `GetCpuCount`         | `() -> number` | Anzahl CPU-Kerne      |
| `GetHostname`         | `() -> string` | Hostname              |
| `GetUsername`         | `() -> string` | Benutzername          |
| `GetHomeDirectory`    | `() -> string` | Home-Verzeichnis      |
| `GetTempDirectory`    | `() -> string` | Temp-Verzeichnis      |

### Umgebungsvariablen

| Funktion    | Signatur                                | Beschreibung             |
| ----------- | --------------------------------------- | ------------------------ |
| `GetEnvVar` | `(name: string) -> string`              | Umgebungsvariable lesen  |
| `SetEnvVar` | `(name: string, value: string) -> void` | Umgebungsvariable setzen |

### Prozess

| Funktion  | Signatur                 | Beschreibung            |
| --------- | ------------------------ | ----------------------- |
| `GetArgs` | `() -> string[]`         | Kommandozeilenargumente |
| `Exit`    | `(code: number) -> void` | Programm beenden        |

## File Builtins

### Datei-Operationen

| Funktion     | Signatur                                  | Beschreibung      |
| ------------ | ----------------------------------------- | ----------------- |
| `ReadFile`   | `(path: string) -> string`                | Datei lesen       |
| `WriteFile`  | `(path: string, content: string) -> void` | Datei schreiben   |
| `AppendFile` | `(path: string, content: string) -> void` | An Datei anhängen |
| `DeleteFile` | `(path: string) -> void`                  | Datei löschen     |
| `CopyFile`   | `(from: string, to: string) -> void`      | Datei kopieren    |
| `RenameFile` | `(from: string, to: string) -> void`      | Datei umbenennen  |

### Datei-Informationen

| Funktion             | Signatur                    | Beschreibung                      |
| -------------------- | --------------------------- | --------------------------------- |
| `FileExists`         | `(path: string) -> boolean` | Prüft ob Datei existiert          |
| `IsFile`             | `(path: string) -> boolean` | Prüft ob Pfad eine Datei ist      |
| `IsDirectory`        | `(path: string) -> boolean` | Prüft ob Pfad ein Verzeichnis ist |
| `GetFileSize`        | `(path: string) -> number`  | Dateigröße in Bytes               |
| `GetFileExtension`   | `(path: string) -> string`  | Dateiendung                       |
| `GetFileName`        | `(path: string) -> string`  | Dateiname                         |
| `GetParentDirectory` | `(path: string) -> string`  | Übergeordnetes Verzeichnis        |

### Verzeichnis-Operationen

| Funktion          | Signatur                     | Beschreibung                |
| ----------------- | ---------------------------- | --------------------------- |
| `CreateDirectory` | `(path: string) -> void`     | Verzeichnis erstellen       |
| `ListDirectory`   | `(path: string) -> string[]` | Verzeichnisinhalt auflisten |

## Validation Builtins

### Format-Validierung

| Funktion             | Signatur                     | Beschreibung              |
| -------------------- | ---------------------------- | ------------------------- |
| `IsValidEmail`       | `(email: string) -> boolean` | E-Mail-Validierung        |
| `IsValidUrl`         | `(url: string) -> boolean`   | URL-Validierung           |
| `IsValidPhoneNumber` | `(phone: string) -> boolean` | Telefonnummer-Validierung |

### Zeichen-Prüfungen

| Funktion         | Signatur                 | Beschreibung              |
| ---------------- | ------------------------ | ------------------------- |
| `IsAlphanumeric` | `(s: string) -> boolean` | Nur Buchstaben und Zahlen |
| `IsAlphabetic`   | `(s: string) -> boolean` | Nur Buchstaben            |
| `IsNumeric`      | `(s: string) -> boolean` | Nur Zahlen                |
| `IsLowercase`    | `(s: string) -> boolean` | Nur Kleinbuchstaben       |
| `IsUppercase`    | `(s: string) -> boolean` | Nur Großbuchstaben        |

### Weitere Validierungen

| Funktion         | Signatur                                               | Beschreibung        |
| ---------------- | ------------------------------------------------------ | ------------------- |
| `IsInRange`      | `(value: number, min: number, max: number) -> boolean` | Wertebereich prüfen |
| `MatchesPattern` | `(text: string, pattern: string) -> boolean`           | Regex-Match         |

## Hashing Builtins

### Hash-Funktionen

| Funktion       | Signatur                   | Beschreibung       |
| -------------- | -------------------------- | ------------------ |
| `HashString`   | `(s: string) -> number`    | String hashen      |
| `HashNumber`   | `(n: number) -> number`    | Number hashen      |
| `SimpleRandom` | `(seed: number) -> number` | Pseudo-Zufallszahl |

### String-Analyse

| Funktion           | Signatur                                    | Beschreibung        |
| ------------------ | ------------------------------------------- | ------------------- |
| `AreAnagrams`      | `(s1: string, s2: string) -> boolean`       | Prüft Anagramme     |
| `IsPalindrome`     | `(s: string) -> boolean`                    | Prüft Palindrom     |
| `CountOccurrences` | `(text: string, pattern: string) -> number` | Vorkommen zählen    |
| `RemoveDuplicates` | `(s: string) -> string`                     | Duplikate entfernen |
| `UniqueCharacters` | `(s: string) -> string`                     | Eindeutige Zeichen  |
| `ReverseWords`     | `(s: string) -> string`                     | Wörter umkehren     |
| `TitleCase`        | `(s: string) -> string`                     | Title Case Format   |

## DeepMind Builtins (Higher-Order Functions)

### Kontrollfluss

| Funktion            | Signatur                                                      | Beschreibung             |
| ------------------- | ------------------------------------------------------------- | ------------------------ |
| `RepeatAction`      | `(times: number, action: () -> void) -> void`                 | Aktion n-mal wiederholen |
| `DelayedSuggestion` | `(action: () -> void, delay: number) -> void`                 | Verzögerte Ausführung    |
| `IfTranced`         | `(cond: boolean, then: () -> void, else: () -> void) -> void` | Bedingte Ausführung      |

### Schleifen

| Funktion      | Signatur                                                 | Beschreibung                  |
| ------------- | -------------------------------------------------------- | ----------------------------- |
| `RepeatUntil` | `(action: () -> void, condition: () -> boolean) -> void` | Wiederhole bis Bedingung wahr |
| `RepeatWhile` | `(condition: () -> boolean, action: () -> void) -> void` | Wiederhole solange wahr       |

### Funktionskomposition

| Funktion  | Signatur                             | Beschreibung                 |
| --------- | ------------------------------------ | ---------------------------- |
| `Compose` | `(f: B -> C, g: A -> B) -> (A -> C)` | Funktionskomposition f(g(x)) |
| `Pipe`    | `(f: A -> B, g: B -> C) -> (A -> C)` | Funktions-Pipeline g(f(x))   |

### Fehlerbehandlung

| Funktion          | Signatur                                                    | Beschreibung |
| ----------------- | ----------------------------------------------------------- | ------------ |
| `TryOrAwaken`     | `(try: () -> void, catch: (error: string) -> void) -> void` | Try-Catch    |
| `EnsureAwakening` | `(main: () -> void, cleanup: () -> void) -> void`           | Try-Finally  |

### Weitere

| Funktion             | Signatur                            | Beschreibung                   |
| -------------------- | ----------------------------------- | ------------------------------ |
| `SequentialTrance`   | `(actions: (() -> void)[]) -> void` | Aktionen sequentiell ausführen |
| `MeasureTranceDepth` | `(action: () -> void) -> number`    | Ausführungszeit messen         |
| `Memoize`            | `(f: A -> R) -> (A -> R)`           | Funktion mit Caching           |

## Verwendungshinweise

### Namenskonventionen

- **PascalCase** für Funktionsnamen (z.B. `CalculateMean`, `ToUpper`)
- **Case-Insensitive** Matching beim Aufruf
- **Typ-Parameter** `T` für generische Funktionen

### Fehlerbehandlung

- Funktionen die fehlschlagen können werfen Runtime-Errors
- Nutze `TryOrAwaken` für Fehlerbehandlung
- Validiere Eingaben mit Validation-Builtins

### Performance

- Array-Operationen erstellen neue Arrays (immutabel)
- Nutze `Memoize` für teure Berechnungen
- `MeasureTranceDepth` für Performance-Profiling

## Siehe auch

- [Detaillierte Array-Funktionen](./array-functions)
- [Detaillierte String-Funktionen](./string-functions)
- [Detaillierte Math-Funktionen](./math-functions)
- [CLI Builtin-Befehl](../cli/commands#builtins)
