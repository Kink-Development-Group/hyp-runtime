# Builtin Functions Complete Reference

Complete reference of all 110+ builtin functions in HypnoScript (Rust Edition).

## Core Builtins (I/O & Conversion)

### Output Functions

| Function  | Signature                 | Description                   |
| --------- | ------------------------- | ----------------------------- |
| `observe` | `(value: string) -> void` | Standard output with newline  |
| `whisper` | `(value: string) -> void` | Output without newline        |
| `command` | `(value: string) -> void` | Output in uppercase           |
| `drift`   | `(ms: number) -> void`    | Pause/Sleep (in milliseconds) |

### Hypnotic Functions

| Function                | Signature                       | Description                      |
| ----------------------- | ------------------------------- | -------------------------------- |
| `DeepTrance`            | `(duration: number) -> void`    | Deep trance induction with delay |
| `HypnoticCountdown`     | `(from: number) -> void`        | Hypnotic countdown               |
| `TranceInduction`       | `(subjectName: string) -> void` | Complete trance induction        |
| `HypnoticVisualization` | `(scene: string) -> void`       | Hypnotic visualization           |

### Conversion Functions

| Function    | Signature                    | Description                   |
| ----------- | ---------------------------- | ----------------------------- |
| `ToInt`     | `(value: number) -> number`  | Convert to integer (truncate) |
| `ToDouble`  | `(value: string) -> number`  | Parse string to number        |
| `ToString`  | `(value: any) -> string`     | Convert to string             |
| `ToBoolean` | `(value: string) -> boolean` | Parse string to boolean       |

## Math Builtins

### Trigonometric Functions

| Function | Signature               | Description |
| -------- | ----------------------- | ----------- |
| `Sin`    | `(x: number) -> number` | Sine        |
| `Cos`    | `(x: number) -> number` | Cosine      |
| `Tan`    | `(x: number) -> number` | Tangent     |

### Root & Power

| Function | Signature                                    | Description |
| -------- | -------------------------------------------- | ----------- |
| `Sqrt`   | `(x: number) -> number`                      | Square root |
| `Pow`    | `(base: number, exponent: number) -> number` | Power       |

### Logarithms

| Function | Signature               | Description            |
| -------- | ----------------------- | ---------------------- |
| `Log`    | `(x: number) -> number` | Natural logarithm (ln) |
| `Log10`  | `(x: number) -> number` | Base-10 logarithm      |

### Rounding

| Function | Signature               | Description      |
| -------- | ----------------------- | ---------------- |
| `Abs`    | `(x: number) -> number` | Absolute value   |
| `Floor`  | `(x: number) -> number` | Round down       |
| `Ceil`   | `(x: number) -> number` | Round up         |
| `Round`  | `(x: number) -> number` | Round to nearest |

### Min/Max

| Function | Signature                                             | Description |
| -------- | ----------------------------------------------------- | ----------- |
| `Min`    | `(a: number, b: number) -> number`                    | Minimum     |
| `Max`    | `(a: number, b: number) -> number`                    | Maximum     |
| `Clamp`  | `(value: number, min: number, max: number) -> number` | Clamp value |

### Number Theory

| Function    | Signature                          | Description             |
| ----------- | ---------------------------------- | ----------------------- |
| `Factorial` | `(n: number) -> number`            | Factorial               |
| `Gcd`       | `(a: number, b: number) -> number` | Greatest common divisor |
| `Lcm`       | `(a: number, b: number) -> number` | Least common multiple   |
| `IsPrime`   | `(n: number) -> boolean`           | Check if prime          |
| `Fibonacci` | `(n: number) -> number`            | n-th Fibonacci number   |

## String Builtins

### Basic Operations

| Function     | Signature               | Description             |
| ------------ | ----------------------- | ----------------------- |
| `Length`     | `(s: string) -> number` | String length           |
| `ToUpper`    | `(s: string) -> string` | To uppercase            |
| `ToLower`    | `(s: string) -> string` | To lowercase            |
| `Trim`       | `(s: string) -> string` | Remove whitespace       |
| `Reverse`    | `(s: string) -> string` | Reverse string          |
| `Capitalize` | `(s: string) -> string` | Capitalize first letter |

### Search & Replace

| Function     | Signature                                         | Description                          |
| ------------ | ------------------------------------------------- | ------------------------------------ |
| `IndexOf`    | `(s: string, pattern: string) -> number`          | Index of substring (-1 if not found) |
| `Replace`    | `(s: string, from: string, to: string) -> string` | Replace all occurrences              |
| `Contains`   | `(s: string, pattern: string) -> boolean`         | Check if contains                    |
| `StartsWith` | `(s: string, prefix: string) -> boolean`          | Check prefix                         |
| `EndsWith`   | `(s: string, suffix: string) -> boolean`          | Check suffix                         |

### Manipulation

| Function    | Signature                                            | Description       |
| ----------- | ---------------------------------------------------- | ----------------- |
| `Split`     | `(s: string, delimiter: string) -> string[]`         | Split string      |
| `Substring` | `(s: string, start: number, end: number) -> string`  | Extract substring |
| `Repeat`    | `(s: string, times: number) -> string`               | Repeat string     |
| `PadLeft`   | `(s: string, width: number, char: string) -> string` | Pad left          |
| `PadRight`  | `(s: string, width: number, char: string) -> string` | Pad right         |

### Checks

| Function       | Signature                | Description              |
| -------------- | ------------------------ | ------------------------ |
| `IsEmpty`      | `(s: string) -> boolean` | Check if empty           |
| `IsWhitespace` | `(s: string) -> boolean` | Check if only whitespace |

## Array Builtins

:::note Array Prefix
All array functions use the `Array` prefix to distinguish from string functions (e.g., `ArrayLength` vs. string `Length`).
:::

### Basic Operations

| Function        | Signature                           | Description                        |
| --------------- | ----------------------------------- | ---------------------------------- |
| `ArrayLength`   | `(arr: T[]) -> number`              | Array length                       |
| `ArrayIsEmpty`  | `(arr: T[]) -> boolean`             | Check if empty                     |
| `ArrayGet`      | `(arr: T[], index: number) -> T`    | Element at index                   |
| `ArrayIndexOf`  | `(arr: T[], element: T) -> number`  | Index of element (-1 if not found) |
| `ArrayContains` | `(arr: T[], element: T) -> boolean` | Check if contains                  |

### Transformation

| Function        | Signature                     | Description       |
| --------------- | ----------------------------- | ----------------- |
| `ArrayReverse`  | `(arr: T[]) -> T[]`           | Reverse array     |
| `ArraySort`     | `(arr: number[]) -> number[]` | Sort numerically  |
| `ArrayDistinct` | `(arr: T[]) -> T[]`           | Remove duplicates |

### Aggregation

| Function       | Signature                   | Description |
| -------------- | --------------------------- | ----------- |
| `ArraySum`     | `(arr: number[]) -> number` | Sum         |
| `ArrayAverage` | `(arr: number[]) -> number` | Average     |
| `ArrayMin`     | `(arr: number[]) -> number` | Minimum     |
| `ArrayMax`     | `(arr: number[]) -> number` | Maximum     |

### Slicing

| Function     | Signature                                       | Description           |
| ------------ | ----------------------------------------------- | --------------------- |
| `ArrayFirst` | `(arr: T[]) -> T`                               | First element         |
| `ArrayLast`  | `(arr: T[]) -> T`                               | Last element          |
| `ArrayTake`  | `(arr: T[], n: number) -> T[]`                  | First n elements      |
| `ArraySkip`  | `(arr: T[], n: number) -> T[]`                  | Skip first n elements |
| `ArraySlice` | `(arr: T[], start: number, end: number) -> T[]` | Subarray              |

### More

| Function     | Signature                                 | Description          |
| ------------ | ----------------------------------------- | -------------------- |
| `ArrayJoin`  | `(arr: T[], separator: string) -> string` | Array to string      |
| `ArrayCount` | `(arr: T[], element: T) -> number`        | Frequency of element |

## Statistics Builtins

### Central Tendency

| Function          | Signature                   | Description          |
| ----------------- | --------------------------- | -------------------- |
| `CalculateMean`   | `(arr: number[]) -> number` | Arithmetic mean      |
| `CalculateMedian` | `(arr: number[]) -> number` | Median               |
| `CalculateMode`   | `(arr: number[]) -> number` | Mode (most frequent) |

### Dispersion

| Function                     | Signature                                       | Description          |
| ---------------------------- | ----------------------------------------------- | -------------------- |
| `CalculateVariance`          | `(arr: number[]) -> number`                     | Variance             |
| `CalculateStandardDeviation` | `(arr: number[]) -> number`                     | Standard deviation   |
| `CalculateRange`             | `(arr: number[]) -> number`                     | Range (Max - Min)    |
| `CalculatePercentile`        | `(arr: number[], percentile: number) -> number` | Calculate percentile |

### Correlation & Regression

| Function               | Signature                                        | Description                          |
| ---------------------- | ------------------------------------------------ | ------------------------------------ |
| `CalculateCorrelation` | `(x: number[], y: number[]) -> number`           | Correlation coefficient              |
| `LinearRegression`     | `(x: number[], y: number[]) -> (number, number)` | Linear regression (slope, intercept) |

## Time Builtins

### Current Time

| Function               | Signature                    | Description               |
| ---------------------- | ---------------------------- | ------------------------- |
| `GetCurrentTime`       | `() -> number`               | Unix timestamp (seconds)  |
| `GetCurrentDate`       | `() -> string`               | Current date (YYYY-MM-DD) |
| `GetCurrentTimeString` | `() -> string`               | Current time (HH:MM:SS)   |
| `GetCurrentDateTime`   | `() -> string`               | Date and time             |
| `FormatDateTime`       | `(format: string) -> string` | Formatted time            |

### Date Components

| Function       | Signature      | Description            |
| -------------- | -------------- | ---------------------- |
| `GetYear`      | `() -> number` | Current year           |
| `GetMonth`     | `() -> number` | Current month (1-12)   |
| `GetDay`       | `() -> number` | Current day (1-31)     |
| `GetHour`      | `() -> number` | Current hour (0-23)    |
| `GetMinute`    | `() -> number` | Current minute (0-59)  |
| `GetSecond`    | `() -> number` | Current second (0-59)  |
| `GetDayOfWeek` | `() -> number` | Day of week (0=Sunday) |
| `GetDayOfYear` | `() -> number` | Day of year (1-366)    |

### Date Calculations

| Function         | Signature                                 | Description     |
| ---------------- | ----------------------------------------- | --------------- |
| `IsLeapYear`     | `(year: number) -> boolean`               | Check leap year |
| `GetDaysInMonth` | `(year: number, month: number) -> number` | Days in month   |

## System Builtins

### System Information

| Function              | Signature      | Description         |
| --------------------- | -------------- | ------------------- |
| `GetCurrentDirectory` | `() -> string` | Current directory   |
| `GetOperatingSystem`  | `() -> string` | Operating system    |
| `GetArchitecture`     | `() -> string` | CPU architecture    |
| `GetCpuCount`         | `() -> number` | Number of CPU cores |
| `GetHostname`         | `() -> string` | Hostname            |
| `GetUsername`         | `() -> string` | Username            |
| `GetHomeDirectory`    | `() -> string` | Home directory      |
| `GetTempDirectory`    | `() -> string` | Temp directory      |

### Environment Variables

| Function    | Signature                               | Description               |
| ----------- | --------------------------------------- | ------------------------- |
| `GetEnvVar` | `(name: string) -> string`              | Read environment variable |
| `SetEnvVar` | `(name: string, value: string) -> void` | Set environment variable  |

### Process

| Function  | Signature                | Description            |
| --------- | ------------------------ | ---------------------- |
| `GetArgs` | `() -> string[]`         | Command line arguments |
| `Exit`    | `(code: number) -> void` | Exit program           |

## File Builtins

### File Operations

| Function     | Signature                                 | Description    |
| ------------ | ----------------------------------------- | -------------- |
| `ReadFile`   | `(path: string) -> string`                | Read file      |
| `WriteFile`  | `(path: string, content: string) -> void` | Write file     |
| `AppendFile` | `(path: string, content: string) -> void` | Append to file |
| `DeleteFile` | `(path: string) -> void`                  | Delete file    |
| `CopyFile`   | `(from: string, to: string) -> void`      | Copy file      |
| `RenameFile` | `(from: string, to: string) -> void`      | Rename file    |

### File Information

| Function             | Signature                   | Description                  |
| -------------------- | --------------------------- | ---------------------------- |
| `FileExists`         | `(path: string) -> boolean` | Check if file exists         |
| `IsFile`             | `(path: string) -> boolean` | Check if path is a file      |
| `IsDirectory`        | `(path: string) -> boolean` | Check if path is a directory |
| `GetFileSize`        | `(path: string) -> number`  | File size in bytes           |
| `GetFileExtension`   | `(path: string) -> string`  | File extension               |
| `GetFileName`        | `(path: string) -> string`  | File name                    |
| `GetParentDirectory` | `(path: string) -> string`  | Parent directory             |

### Directory Operations

| Function          | Signature                    | Description      |
| ----------------- | ---------------------------- | ---------------- |
| `CreateDirectory` | `(path: string) -> void`     | Create directory |
| `ListDirectory`   | `(path: string) -> string[]` | List directory   |

## Validation Builtins

### Format Validation

| Function             | Signature                    | Description               |
| -------------------- | ---------------------------- | ------------------------- |
| `IsValidEmail`       | `(email: string) -> boolean` | Email validation          |
| `IsValidUrl`         | `(url: string) -> boolean`   | URL-Validierung           |
| `IsValidPhoneNumber` | `(phone: string) -> boolean` | Telefonnummer-Validierung |

### Zeichen-Prüfungen

| Function         | Signatur                 | Description               |
| ---------------- | ------------------------ | ------------------------- |
| `IsAlphanumeric` | `(s: string) -> boolean` | Nur Buchstaben und Zahlen |
| `IsAlphabetic`   | `(s: string) -> boolean` | Nur Buchstaben            |
| `IsNumeric`      | `(s: string) -> boolean` | Nur Zahlen                |
| `IsLowercase`    | `(s: string) -> boolean` | Nur Kleinbuchstaben       |
| `IsUppercase`    | `(s: string) -> boolean` | Nur Großbuchstaben        |

### Weitere Validierungen

| Function         | Signatur                                               | Description         |
| ---------------- | ------------------------------------------------------ | ------------------- |
| `IsInRange`      | `(value: number, min: number, max: number) -> boolean` | Wertebereich prüfen |
| `MatchesPattern` | `(text: string, pattern: string) -> boolean`           | Regex-Match         |

## Hashing Builtins

### Hash-Functionen

| Function       | Signatur                   | Description        |
| -------------- | -------------------------- | ------------------ |
| `HashString`   | `(s: string) -> number`    | String hashen      |
| `HashNumber`   | `(n: number) -> number`    | Number hashen      |
| `SimpleRandom` | `(seed: number) -> number` | Pseudo-Zufallszahl |

### String-Analyse

| Function           | Signatur                                    | Description         |
| ------------------ | ------------------------------------------- | ------------------- |
| `AreAnagrams`      | `(s1: string, s2: string) -> boolean`       | Checks Anagramme    |
| `IsPalindrome`     | `(s: string) -> boolean`                    | Checks Palindrom    |
| `CountOccurrences` | `(text: string, pattern: string) -> number` | Vorkommen zählen    |
| `RemoveDuplicates` | `(s: string) -> string`                     | Duplikate entfernen |
| `UniqueCharacters` | `(s: string) -> string`                     | Eindeutige Zeichen  |
| `ReverseWords`     | `(s: string) -> string`                     | Wörter umkehren     |
| `TitleCase`        | `(s: string) -> string`                     | Title Case Format   |

## DeepMind Builtins (Higher-Order Functions)

### Kontrollfluss

| Function            | Signatur                                                      | Description              |
| ------------------- | ------------------------------------------------------------- | ------------------------ |
| `RepeatAction`      | `(times: number, action: () -> void) -> void`                 | Aktion n-mal wiederholen |
| `DelayedSuggestion` | `(action: () -> void, delay: number) -> void`                 | Verzögerte Ausführung    |
| `IfTranced`         | `(cond: boolean, then: () -> void, else: () -> void) -> void` | Bedingte Ausführung      |

### Schleifen

| Function      | Signatur                                                 | Description                   |
| ------------- | -------------------------------------------------------- | ----------------------------- |
| `RepeatUntil` | `(action: () -> void, condition: () -> boolean) -> void` | Wiederhole bis Bedingung wahr |
| `RepeatWhile` | `(condition: () -> boolean, action: () -> void) -> void` | Wiederhole solange wahr       |

### Functionskomposition

| Function  | Signatur                             | Description                  |
| --------- | ------------------------------------ | ---------------------------- |
| `Compose` | `(f: B -> C, g: A -> B) -> (A -> C)` | Functionskomposition f(g(x)) |
| `Pipe`    | `(f: A -> B, g: B -> C) -> (A -> C)` | Functions-Pipeline g(f(x))   |

### Fehlerbehandlung

| Function          | Signatur                                                    | Description |
| ----------------- | ----------------------------------------------------------- | ----------- |
| `TryOrAwaken`     | `(try: () -> void, catch: (error: string) -> void) -> void` | Try-Catch   |
| `EnsureAwakening` | `(main: () -> void, cleanup: () -> void) -> void`           | Try-Finally |

### Weitere

| Function             | Signatur                            | Description                    |
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
