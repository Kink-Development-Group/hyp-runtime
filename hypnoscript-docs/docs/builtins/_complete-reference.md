# Builtin Functions Complete Reference

Complete Reference aller 110+ Builtin Functions in HypnoScript (Rust-Edition).

## Core Builtins (I/O & Konvertierung)

### Output-Functionen

| Function  | Signatur                  | Description                       |
| --------- | ------------------------- | ---------------------------------- |
| `observe` | `(value: string) -> void` | Standard-Output mit Zeilenumbruch |
| `whisper` | `(value: string) -> void` | Output ohne Zeilenumbruch         |
| `command` | `(value: string) -> void` | Output in Großbuchstaben          |
| `drift`   | `(ms: number) -> void`    | Pause/Sleep (in Millisekunden)     |

### Hypnotische Functionen

| Function                | Signatur                        | Description                           |
| ----------------------- | ------------------------------- | -------------------------------------- |
| `DeepTrance`            | `(duration: number) -> void`    | Tiefe Trance-Induktion mit Verzögerung |
| `HypnoticCountdown`     | `(from: number) -> void`        | Hypnotischer Countdown                 |
| `TranceInduction`       | `(subjectName: string) -> void` | Vollständige Trance-Induktion          |
| `HypnoticVisualization` | `(scene: string) -> void`       | Hypnotische Visualisierung             |

### Konvertierungs-Functionen

| Function    | Signatur                     | Description                      |
| ----------- | ---------------------------- | --------------------------------- |
| `ToInt`     | `(value: number) -> number`  | Converts zu Integer (truncate) |
| `ToDouble`  | `(value: string) -> number`  | Parse String zu number            |
| `ToString`  | `(value: any) -> string`     | Converts zu String             |
| `ToBoolean` | `(value: string) -> boolean` | Parse String zu boolean           |

## Math Builtins

### Trigonometrische Functionen

| Function | Signatur                | Description |
| -------- | ----------------------- | ------------ |
| `Sin`    | `(x: number) -> number` | Sinus        |
| `Cos`    | `(x: number) -> number` | Cosinus      |
| `Tan`    | `(x: number) -> number` | Tangens      |

### Wurzel & Potenz

| Function | Signatur                                     | Description  |
| -------- | -------------------------------------------- | ------------- |
| `Sqrt`   | `(x: number) -> number`                      | Quadratwurzel |
| `Pow`    | `(base: number, exponent: number) -> number` | Potenz        |

### Logarithmen

| Function | Signatur                | Description                 |
| -------- | ----------------------- | ---------------------------- |
| `Log`    | `(x: number) -> number` | Natürlicher Logarithmus (ln) |
| `Log10`  | `(x: number) -> number` | Logarithmus zur Basis 10     |

### Rundung

| Function | Signatur                | Description          |
| -------- | ----------------------- | --------------------- |
| `Abs`    | `(x: number) -> number` | Absoluter Wert        |
| `Floor`  | `(x: number) -> number` | Abrunden              |
| `Ceil`   | `(x: number) -> number` | Aufrunden             |
| `Round`  | `(x: number) -> number` | Kaufmännisches Runden |

### Min/Max

| Function | Signatur                                              | Description   |
| -------- | ----------------------------------------------------- | -------------- |
| `Min`    | `(a: number, b: number) -> number`                    | Minimum        |
| `Max`    | `(a: number, b: number) -> number`                    | Maximum        |
| `Clamp`  | `(value: number, min: number, max: number) -> number` | Wert begrenzen |

### Zahlentheorie

| Function    | Signatur                           | Description                     |
| ----------- | ---------------------------------- | -------------------------------- |
| `Factorial` | `(n: number) -> number`            | Fakultät                         |
| `Gcd`       | `(a: number, b: number) -> number` | Größter gemeinsamer Teiler       |
| `Lcm`       | `(a: number, b: number) -> number` | Kleinstes gemeinsames Vielfaches |
| `IsPrime`   | `(n: number) -> boolean`           | Checks ob Primzahl                |
| `Fibonacci` | `(n: number) -> number`            | n-te Fibonacci-Zahl              |

## String Builtins

### Basis-Operationen

| Function     | Signatur                | Description           |
| ------------ | ----------------------- | ---------------------- |
| `Length`     | `(s: string) -> number` | String-Length           |
| `ToUpper`    | `(s: string) -> string` | In Großbuchstaben      |
| `ToLower`    | `(s: string) -> string` | In Kleinbuchstaben     |
| `Trim`       | `(s: string) -> string` | Whitespace entfernen   |
| `Reverse`    | `(s: string) -> string` | String umkehren        |
| `Capitalize` | `(s: string) -> string` | Ersten Buchstaben groß |

### Suchen & Ersetzen

| Function     | Signatur                                          | Description                                  |
| ------------ | ------------------------------------------------- | --------------------------------------------- |
| `IndexOf`    | `(s: string, pattern: string) -> number`          | Index des Substrings (-1 wenn nicht gefunden) |
| `Replace`    | `(s: string, from: string, to: string) -> string` | Alle Vorkommen ersetzen                       |
| `Contains`   | `(s: string, pattern: string) -> boolean`         | Checks ob enthalten                            |
| `StartsWith` | `(s: string, prefix: string) -> boolean`          | Checks Präfix                                  |
| `EndsWith`   | `(s: string, suffix: string) -> boolean`          | Checks Suffix                                  |

### Manipulation

| Function    | Signatur                                             | Description           |
| ----------- | ---------------------------------------------------- | ---------------------- |
| `Split`     | `(s: string, delimiter: string) -> string[]`         | String aufteilen       |
| `Substring` | `(s: string, start: number, end: number) -> string`  | Teilstring extrahieren |
| `Repeat`    | `(s: string, times: number) -> string`               | String wiederholen     |
| `PadLeft`   | `(s: string, width: number, char: string) -> string` | Links auffüllen        |
| `PadRight`  | `(s: string, width: number, char: string) -> string` | Rechts auffüllen       |

### Prüfungen

| Function       | Signatur                 | Description            |
| -------------- | ------------------------ | ----------------------- |
| `IsEmpty`      | `(s: string) -> boolean` | Checks ob leer           |
| `IsWhitespace` | `(s: string) -> boolean` | Checks ob nur Whitespace |

## Array Builtins

:::note Array-Präfix
Alle Array-Functionen verwenden das Präfix `Array` zur Unterscheidung von String-Functionen (z.B. `ArrayLength` vs. String `Length`).
:::

### Basis-Operationen

| Function        | Signatur                            | Description                                |
| --------------- | ----------------------------------- | ------------------------------------------- |
| `ArrayLength`   | `(arr: T[]) -> number`              | Array-Length                                 |
| `ArrayIsEmpty`  | `(arr: T[]) -> boolean`             | Checks ob leer                               |
| `ArrayGet`      | `(arr: T[], index: number) -> T`    | Element an Index                            |
| `ArrayIndexOf`  | `(arr: T[], element: T) -> number`  | Index des Elements (-1 wenn nicht gefunden) |
| `ArrayContains` | `(arr: T[], element: T) -> boolean` | Checks ob enthalten                          |

### Transformation

| Function        | Signatur                      | Description        |
| --------------- | ----------------------------- | ------------------- |
| `ArrayReverse`  | `(arr: T[]) -> T[]`           | Array umkehren      |
| `ArraySort`     | `(arr: number[]) -> number[]` | Numerisch sortieren |
| `ArrayDistinct` | `(arr: T[]) -> T[]`           | Duplikate entfernen |

### Aggregation

| Function       | Signatur                    | Description |
| -------------- | --------------------------- | ------------ |
| `ArraySum`     | `(arr: number[]) -> number` | Sum        |
| `ArrayAverage` | `(arr: number[]) -> number` | Durchschnitt |
| `ArrayMin`     | `(arr: number[]) -> number` | Minimum      |
| `ArrayMax`     | `(arr: number[]) -> number` | Maximum      |

### Slicing

| Function     | Signatur                                        | Description                 |
| ------------ | ----------------------------------------------- | ---------------------------- |
| `ArrayFirst` | `(arr: T[]) -> T`                               | Erstes Element               |
| `ArrayLast`  | `(arr: T[]) -> T`                               | Letztes Element              |
| `ArrayTake`  | `(arr: T[], n: number) -> T[]`                  | Erste n Elemente             |
| `ArraySkip`  | `(arr: T[], n: number) -> T[]`                  | Überspringt erste n Elemente |
| `ArraySlice` | `(arr: T[], start: number, end: number) -> T[]` | Teilarray                    |

### Weitere

| Function     | Signatur                                  | Description              |
| ------------ | ----------------------------------------- | ------------------------- |
| `ArrayJoin`  | `(arr: T[], separator: string) -> string` | Array zu String           |
| `ArrayCount` | `(arr: T[], element: T) -> number`        | Häufigkeit eines Elements |

## Statistics Builtins

### Zentrale Tendenz

| Function          | Signatur                    | Description               |
| ----------------- | --------------------------- | -------------------------- |
| `CalculateMean`   | `(arr: number[]) -> number` | Arithmetisches Mittel      |
| `CalculateMedian` | `(arr: number[]) -> number` | Median                     |
| `CalculateMode`   | `(arr: number[]) -> number` | Modus (häufigstes Element) |

### Streuung

| Function                     | Signatur                                        | Description           |
| ---------------------------- | ----------------------------------------------- | ---------------------- |
| `CalculateVariance`          | `(arr: number[]) -> number`                     | Varianz                |
| `CalculateStandardDeviation` | `(arr: number[]) -> number`                     | Standardabweichung     |
| `CalculateRange`             | `(arr: number[]) -> number`                     | Spannweite (Max - Min) |
| `CalculatePercentile`        | `(arr: number[], percentile: number) -> number` | Perzentil berechnen    |

### Korrelation & Regression

| Function               | Signatur                                         | Description                          |
| ---------------------- | ------------------------------------------------ | ------------------------------------- |
| `CalculateCorrelation` | `(x: number[], y: number[]) -> number`           | Korrelationskoeffizient               |
| `LinearRegression`     | `(x: number[], y: number[]) -> (number, number)` | Lineare Regression (slope, intercept) |

## Time Builtins

### Aktuelle Zeit

| Function               | Signatur                     | Description                 |
| ---------------------- | ---------------------------- | ---------------------------- |
| `GetCurrentTime`       | `() -> number`               | Unix Timestamp (Sekunden)    |
| `GetCurrentDate`       | `() -> string`               | Aktuelles Datum (YYYY-MM-DD) |
| `GetCurrentTimeString` | `() -> string`               | Aktuelle Zeit (HH:MM:SS)     |
| `GetCurrentDateTime`   | `() -> string`               | Datum und Zeit               |
| `FormatDateTime`       | `(format: string) -> string` | Formatierte Zeit             |

### Datum-Komponenten

| Function       | Signatur       | Description            |
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

| Function         | Signatur                                  | Description     |
| ---------------- | ----------------------------------------- | ---------------- |
| `IsLeapYear`     | `(year: number) -> boolean`               | Checks Schaltjahr |
| `GetDaysInMonth` | `(year: number, month: number) -> number` | Tage im Monat    |

## System Builtins

### System-Informationen

| Function              | Signatur       | Description          |
| --------------------- | -------------- | --------------------- |
| `GetCurrentDirectory` | `() -> string` | Aktuelles directory |
| `GetOperatingSystem`  | `() -> string` | Operating System        |
| `GetArchitecture`     | `() -> string` | CPU-Architektur       |
| `GetCpuCount`         | `() -> number` | Anzahl CPU-Kerne      |
| `GetHostname`         | `() -> string` | Hostname              |
| `GetUsername`         | `() -> string` | Benutzername          |
| `GetHomeDirectory`    | `() -> string` | Home-directory      |
| `GetTempDirectory`    | `() -> string` | Temp-directory      |

### environment variablen

| Function    | Signatur                                | Description             |
| ----------- | --------------------------------------- | ------------------------ |
| `GetEnvVar` | `(name: string) -> string`              | environment variable lesen  |
| `SetEnvVar` | `(name: string, value: string) -> void` | environment variable setzen |

### Prozess

| Function  | Signatur                 | Description            |
| --------- | ------------------------ | ----------------------- |
| `GetArgs` | `() -> string[]`         | Kommandozeilenargumente |
| `Exit`    | `(code: number) -> void` | Programm beenden        |

## File Builtins

### File-Operationen

| Function     | Signatur                                  | Description      |
| ------------ | ----------------------------------------- | ----------------- |
| `ReadFile`   | `(path: string) -> string`                | File lesen       |
| `WriteFile`  | `(path: string, content: string) -> void` | File schreiben   |
| `AppendFile` | `(path: string, content: string) -> void` | An File anhängen |
| `DeleteFile` | `(path: string) -> void`                  | File löschen     |
| `CopyFile`   | `(from: string, to: string) -> void`      | File kopieren    |
| `RenameFile` | `(from: string, to: string) -> void`      | File umbenennen  |

### File-Informationen

| Function             | Signatur                    | Description                      |
| -------------------- | --------------------------- | --------------------------------- |
| `FileExists`         | `(path: string) -> boolean` | Checks ob File existiert          |
| `IsFile`             | `(path: string) -> boolean` | Checks ob Pfad eine File ist      |
| `IsDirectory`        | `(path: string) -> boolean` | Checks ob Pfad ein directory ist |
| `GetFileSize`        | `(path: string) -> number`  | Filegröße in Bytes               |
| `GetFileExtension`   | `(path: string) -> string`  | Fileendung                       |
| `GetFileName`        | `(path: string) -> string`  | Filename                         |
| `GetParentDirectory` | `(path: string) -> string`  | Übergeordnetes directory        |

### directory-Operationen

| Function          | Signatur                     | Description                |
| ----------------- | ---------------------------- | --------------------------- |
| `CreateDirectory` | `(path: string) -> void`     | directory erstellen       |
| `ListDirectory`   | `(path: string) -> string[]` | directoryinhalt auflisten |

## Validation Builtins

### Format-Validierung

| Function             | Signatur                     | Description              |
| -------------------- | ---------------------------- | ------------------------- |
| `IsValidEmail`       | `(email: string) -> boolean` | E-Mail-Validierung        |
| `IsValidUrl`         | `(url: string) -> boolean`   | URL-Validierung           |
| `IsValidPhoneNumber` | `(phone: string) -> boolean` | Telefonnummer-Validierung |

### Zeichen-Prüfungen

| Function         | Signatur                 | Description              |
| ---------------- | ------------------------ | ------------------------- |
| `IsAlphanumeric` | `(s: string) -> boolean` | Nur Buchstaben und Zahlen |
| `IsAlphabetic`   | `(s: string) -> boolean` | Nur Buchstaben            |
| `IsNumeric`      | `(s: string) -> boolean` | Nur Zahlen                |
| `IsLowercase`    | `(s: string) -> boolean` | Nur Kleinbuchstaben       |
| `IsUppercase`    | `(s: string) -> boolean` | Nur Großbuchstaben        |

### Weitere Validierungen

| Function         | Signatur                                               | Description        |
| ---------------- | ------------------------------------------------------ | ------------------- |
| `IsInRange`      | `(value: number, min: number, max: number) -> boolean` | Wertebereich prüfen |
| `MatchesPattern` | `(text: string, pattern: string) -> boolean`           | Regex-Match         |

## Hashing Builtins

### Hash-Functionen

| Function       | Signatur                   | Description       |
| -------------- | -------------------------- | ------------------ |
| `HashString`   | `(s: string) -> number`    | String hashen      |
| `HashNumber`   | `(n: number) -> number`    | Number hashen      |
| `SimpleRandom` | `(seed: number) -> number` | Pseudo-Zufallszahl |

### String-Analyse

| Function           | Signatur                                    | Description        |
| ------------------ | ------------------------------------------- | ------------------- |
| `AreAnagrams`      | `(s1: string, s2: string) -> boolean`       | Checks Anagramme     |
| `IsPalindrome`     | `(s: string) -> boolean`                    | Checks Palindrom     |
| `CountOccurrences` | `(text: string, pattern: string) -> number` | Vorkommen zählen    |
| `RemoveDuplicates` | `(s: string) -> string`                     | Duplikate entfernen |
| `UniqueCharacters` | `(s: string) -> string`                     | Eindeutige Zeichen  |
| `ReverseWords`     | `(s: string) -> string`                     | Wörter umkehren     |
| `TitleCase`        | `(s: string) -> string`                     | Title Case Format   |

## DeepMind Builtins (Higher-Order Functions)

### Kontrollfluss

| Function            | Signatur                                                      | Description             |
| ------------------- | ------------------------------------------------------------- | ------------------------ |
| `RepeatAction`      | `(times: number, action: () -> void) -> void`                 | Aktion n-mal wiederholen |
| `DelayedSuggestion` | `(action: () -> void, delay: number) -> void`                 | Verzögerte Ausführung    |
| `IfTranced`         | `(cond: boolean, then: () -> void, else: () -> void) -> void` | Bedingte Ausführung      |

### Schleifen

| Function      | Signatur                                                 | Description                  |
| ------------- | -------------------------------------------------------- | ----------------------------- |
| `RepeatUntil` | `(action: () -> void, condition: () -> boolean) -> void` | Wiederhole bis Bedingung wahr |
| `RepeatWhile` | `(condition: () -> boolean, action: () -> void) -> void` | Wiederhole solange wahr       |

### Functionskomposition

| Function  | Signatur                             | Description                 |
| --------- | ------------------------------------ | ---------------------------- |
| `Compose` | `(f: B -> C, g: A -> B) -> (A -> C)` | Functionskomposition f(g(x)) |
| `Pipe`    | `(f: A -> B, g: B -> C) -> (A -> C)` | Functions-Pipeline g(f(x))   |

### Fehlerbehandlung

| Function          | Signatur                                                    | Description |
| ----------------- | ----------------------------------------------------------- | ------------ |
| `TryOrAwaken`     | `(try: () -> void, catch: (error: string) -> void) -> void` | Try-Catch    |
| `EnsureAwakening` | `(main: () -> void, cleanup: () -> void) -> void`           | Try-Finally  |

### Weitere

| Function             | Signatur                            | Description                   |
| -------------------- | ----------------------------------- | ------------------------------ |
| `SequentialTrance`   | `(actions: (() -> void)[]) -> void` | Aktionen sequentiell ausführen |
| `MeasureTranceDepth` | `(action: () -> void) -> number`    | Ausführungszeit messen         |
| `Memoize`            | `(f: A -> R) -> (A -> R)`           | Function mit Caching           |

## Usageshinweise

### Namenskonventionen

- **PascalCase** für Functionsnamen (z.B. `CalculateMean`, `ToUpper`)
- **Case-Insensitive** Matching beim Aufruf
- **Typ-Parameters** `T` für generische Functionen

### Fehlerbehandlung

- Functionen die fehlschlagen können werfen Runtime-Errors
- Use `TryOrAwaken` für Fehlerbehandlung
- Validiere Inputn mit Validation-Builtins

### Performance

- Array-Operationen erstellen neue Arrays (immutabel)
- Use `Memoize` für teure Berechnungen
- `MeasureTranceDepth` für Performance-Profiling

## See auch

- [Detaillierte Array-Functionen](./array-functions)
- [Detaillierte String-Functionen](./string-functions)
- [Detaillierte Math-Functionen](./math-functions)
- [CLI Builtin-Command](../cli/commands#builtins)
