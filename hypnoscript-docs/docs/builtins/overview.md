---
sidebar_position: 1
---

# Builtin Functions Overview

HypnoScript offers a comprehensive standard library with over **110 built-in functions** in the Rust edition. These functions are available directly in the language and require no additional imports.

## Categories

### 🧠 Core & Hypnotic Functions

Basic I/O, conversion, and hypnotic special functions.

| Function                  | Description                     | Example                             |
| ------------------------- | ------------------------------- | ----------------------------------- |
| `observe(text)`           | Standard output with line break | `observe "Hello World";`            |
| `whisper(text)`           | Output without line break       | `whisper "Part1"; whisper "Part2";` |
| `command(text)`           | Imperative output (uppercase)   | `command "Important!";`             |
| `drift(ms)`               | Pause/Sleep in milliseconds     | `drift(2000);`                      |
| `DeepTrance(duration)`    | Deep trance induction           | `DeepTrance(5000);`                 |
| `HypnoticCountdown(from)` | Hypnotic countdown              | `HypnoticCountdown(10);`            |
| `TranceInduction(name)`   | Complete trance induction       | `TranceInduction("Max");`           |
| `ToInt(value)`            | Convert to integer              | `ToInt(3.14)` → `3`                 |
| `ToString(value)`         | Convert to string               | `ToString(42)` → `"42"`             |
| `ToBoolean(value)`        | Convert to boolean              | `ToBoolean("true")` → `true`        |

### 🔢 Math Functions

Comprehensive mathematical operations and calculations.

| Category           | Functions                                         |
| ------------------ | ------------------------------------------------- |
| **Trigonometry**   | `Sin`, `Cos`, `Tan`                               |
| **Roots & Powers** | `Sqrt`, `Pow`                                     |
| **Logarithms**     | `Log` (ln), `Log10`                               |
| **Rounding**       | `Abs`, `Floor`, `Ceil`, `Round`, `Clamp`          |
| **Min/Max**        | `Min`, `Max`                                      |
| **Number Theory**  | `Factorial`, `Gcd`, `Lcm`, `IsPrime`, `Fibonacci` |

**Example:**

```hyp
induce result: number = Sqrt(16);  // 4.0
induce isPrime: boolean = IsPrime(17);  // true
induce fib: number = Fibonacci(10);  // 55
```

### 📝 String Functions

Functions for string manipulation and analysis.

| Category         | Functions                                                       |
| ---------------- | --------------------------------------------------------------- |
| **Basics**       | `Length`, `ToUpper`, `ToLower`, `Trim`, `Reverse`, `Capitalize` |
| **Search**       | `IndexOf`, `Contains`, `StartsWith`, `EndsWith`                 |
| **Manipulation** | `Replace`, `Split`, `Substring`, `Repeat`                       |
| **Padding**      | `PadLeft`, `PadRight`                                           |
| **Checks**       | `IsEmpty`, `IsWhitespace`                                       |

**Example:**

```hyp
induce text: string = "  Hello World  ";
induce cleaned: string = Trim(text);  // "Hello World"
induce upper: string = ToUpper(cleaned);  // "HELLO WORLD"
induce words: string[] = Split(cleaned, " ");  // ["Hello", "World"]
```

### 📦 Array Functions

Functions for working with arrays and lists.

| Category           | Functions                                         |
| ------------------ | ------------------------------------------------- |
| **Basics**         | `Length`, `IsEmpty`, `Get`, `IndexOf`, `Contains` |
| **Transformation** | `Reverse`, `Sort`, `Distinct`                     |
| **Aggregation**    | `Sum`, `Average`, `Min`, `Max`                    |
| **Slicing**        | `First`, `Last`, `Take`, `Skip`, `Slice`          |
| **More**           | `Join`, `Count`                                   |

**Example:**

```hyp
induce numbers: number[] = [5, 2, 8, 1, 9];
induce sorted: number[] = Sort(numbers);  // [1, 2, 5, 8, 9]
induce sum: number = Sum(numbers);  // 25
induce avg: number = Average(numbers);  // 5.0
```

[→ Detailed Array Functions](./array-functions)

### 📊 Statistics Functions

Functions for statistical calculations and analysis.

| Category             | Functions                                                                                  |
| -------------------- | ------------------------------------------------------------------------------------------ |
| **Central Tendency** | `CalculateMean`, `CalculateMedian`, `CalculateMode`                                        |
| **Dispersion**       | `CalculateVariance`, `CalculateStandardDeviation`, `CalculateRange`, `CalculatePercentile` |
| **Correlation**      | `CalculateCorrelation`, `LinearRegression`                                                 |

**Example:**

```hyp
induce data: number[] = [1, 2, 3, 4, 5];
induce mean: number = CalculateMean(data);  // 3.0
induce stddev: number = CalculateStandardDeviation(data);  // 1.58...
```

[→ Detailed Statistics Functions](./statistics-functions)

### 🕒 Time & Date

Functions for time and date processing.

| Category         | Functions                                                            |
| ---------------- | -------------------------------------------------------------------- |
| **Current Time** | `GetCurrentTime`, `GetCurrentDate`, `GetCurrentDateTime`             |
| **Components**   | `GetYear`, `GetMonth`, `GetDay`, `GetHour`, `GetMinute`, `GetSecond` |
| **Calculations** | `GetDayOfWeek`, `GetDayOfYear`, `IsLeapYear`, `GetDaysInMonth`       |

**Example:**

```hyp
induce timestamp: number = GetCurrentTime();  // Unix timestamp
induce date: string = GetCurrentDate();  // "2025-01-15"
induce year: number = GetYear();  // 2025
```

[→ Detailed Time/Date Functions](./time-date-functions)

### 💻 System Functions

Functions for system interaction and information.

| Category        | Functions                                                                            |
| --------------- | ------------------------------------------------------------------------------------ |
| **System Info** | `GetOperatingSystem`, `GetArchitecture`, `GetCpuCount`, `GetHostname`, `GetUsername` |
| **Directories** | `GetCurrentDirectory`, `GetHomeDirectory`, `GetTempDirectory`                        |
| **Environment** | `GetEnvVar`, `SetEnvVar`, `GetArgs`                                                  |
| **Process**     | `Exit`                                                                               |

**Example:**

```hyp
induce os: string = GetOperatingSystem();  // "Windows", "Linux", "macOS"
induce cores: number = GetCpuCount();  // 8
induce home: string = GetHomeDirectory();  // "/home/user" or "C:\\Users\\user"
```

[→ Detailed System Functions](./system-functions)

### 📁 File Functions

Functions for filesystem operations.

| Category        | Functions                                                              |
| --------------- | ---------------------------------------------------------------------- |
| **Read/Write**  | `ReadFile`, `WriteFile`, `AppendFile`                                  |
| **Management**  | `DeleteFile`, `CopyFile`, `RenameFile`                                 |
| **Checks**      | `FileExists`, `IsFile`, `IsDirectory`                                  |
| **Information** | `GetFileSize`, `GetFileExtension`, `GetFileName`, `GetParentDirectory` |
| **Directories** | `CreateDirectory`, `ListDirectory`                                     |

**Example:**

```hyp
if (FileExists("config.txt")) {
    induce content: string = ReadFile("config.txt");
    observe "Config: " + content;
} else {
    WriteFile("config.txt", "default config");
}
```

[→ Detailed File Functions](./file-functions)

### 🧩 CLI & Automation

New builtins help build interactive tools and scripts.

| Function         | Description                                               |
| ---------------- | --------------------------------------------------------- |
| `CliPrompt`      | Localized text input with default values                  |
| `CliConfirm`     | Yes/No confirmation with `Y/n` or `J/n` hint              |
| `ParseArguments` | Parses CLI arguments into flags and positional parameters |
| `HasFlag`        | Checks if a flag is set                                   |
| `FlagValue`      | Reads the value of a flag (`--port 8080` → `8080`)        |

**Example:**

```hyp
induce args: string[] = GetArgs();
if (HasFlag(args, "help")) {
    observe "Use --port <PORT>";
    Exit(0);
}

induce port = FlagValue(args, "port") ?? "8080";
induce answer = CliPrompt("Service name", "demo", false, "en-US");
induce confirm = CliConfirm("Start deployment?", true, "en-US");
```

### 🌐 API & Service Functions

Combines HTTP clients with service health tools.

| Function            | Description                                           |
| ------------------- | ----------------------------------------------------- |
| `HttpSend`          | General HTTP client (methods, headers, auth, timeout) |
| `HttpGetJson`       | `GET` with automatic JSON parsing                     |
| `HttpPostJson`      | `POST` JSON → JSON (incl. Content-Type)               |
| `ServiceHealth`     | Creates health report (uptime, latency, P95, SLO)     |
| `RetrySchedule`     | Returns exponential backoff plan with optional jitter |
| `CircuitShouldOpen` | Evaluates error window for circuit breaker            |

**Example:**

```hyp
induce response = HttpGetJson("https://api.example.com/status");
if (response.ok != true) {
    observe "API reports error";
}

induce schedule: number[] = RetrySchedule(5, 250, 2.0, 50, 4000);
observe "Retries every " + schedule[0] + "ms";
```

### 🧾 Data Formats (JSON & CSV)

| Function           | Description                                 |
| ------------------ | ------------------------------------------- |
| `JsonPretty`       | Formats JSON for logs                       |
| `JsonQuery`        | Path query (`data.items[0].name`)           |
| `JsonMerge`        | Recursive merge of two documents            |
| `ParseCsv`         | Reads CSV (delimiter + header configurable) |
| `CsvSelectColumns` | Projects columns by name                    |
| `CsvToString`      | Builds CSV text from table structure        |

**Example:**

```hyp
induce payload = JsonPretty(ReadFile("response.json"));
induce table = ParseCsv(ReadFile("data.csv"));
induce namesOnly = CsvSelectColumns(table, ["name"]);
WriteFile("names.csv", CsvToString(namesOnly));
```

### ✅ Validation

Functions for data validation.

| Category      | Functions                                                                   |
| ------------- | --------------------------------------------------------------------------- |
| **Format**    | `IsValidEmail`, `IsValidUrl`, `IsValidPhoneNumber`                          |
| **Character** | `IsAlphanumeric`, `IsAlphabetic`, `IsNumeric`, `IsLowercase`, `IsUppercase` |
| **More**      | `IsInRange`, `MatchesPattern`                                               |

**Example:**

```hyp
induce email: string = "user@example.com";
if (IsValidEmail(email)) {
    observe "Valid email!";
}
```

[→ Detailed Validation Functions](./validation-functions)

### 🔐 Hashing & String Analysis

Functions for hashing and advanced string operations.

| Category           | Functions                                                           |
| ------------------ | ------------------------------------------------------------------- |
| **Hashing**        | `HashString`, `HashNumber`, `SimpleRandom`                          |
| **Analysis**       | `AreAnagrams`, `IsPalindrome`, `CountOccurrences`                   |
| **Transformation** | `RemoveDuplicates`, `UniqueCharacters`, `ReverseWords`, `TitleCase` |

**Example:**

```hyp
induce hash: number = HashString("password");
induce isPalin: boolean = IsPalindrome("anna");  // true
induce titleText: string = TitleCase("hello world");  // "Hello World"
```

[→ Detailed Hashing Functions](./hashing-encoding)

### 🧠 DeepMind (Higher-Order Functions)

Advanced functional programming and control flow.

| Category           | Functions                                                        |
| ------------------ | ---------------------------------------------------------------- |
| **Loops**          | `RepeatAction`, `RepeatUntil`, `RepeatWhile`                     |
| **Delay**          | `DelayedSuggestion`                                              |
| **Composition**    | `Compose`, `Pipe`                                                |
| **Error Handling** | `TryOrAwaken`, `EnsureAwakening`                                 |
| **More**           | `IfTranced`, `SequentialTrance`, `MeasureTranceDepth`, `Memoize` |

**Example:**

```hyp
// Repeat action 5 times
RepeatAction(5, suggestion() {
    observe "Repeated!";
});

// Function composition
suggestion double(x: number): number {
    awaken x * 2;
}

suggestion addTen(x: number): number {
    awaken x + 10;
}

induce composed = Compose(double, addTen);
induce result: number = composed(5);  // double(addTen(5)) = 30
```

[→ Detailed DeepMind Functions](./deepmind-functions)

## Usage

All Builtin Functions can be used directly in HypnoScript code without imports:

```hyp
Focus {
    entrance {
        observe "=== Builtin Functions Demo ===";
    }

    // Array functions
    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce sum: number = Sum(numbers);
    observe "Sum: " + sum;

    // String functions
    induce text: string = "Hello World";
    induce reversed: string = Reverse(text);
    observe "Reversed: " + reversed;

    // Mathematical functions
    induce sqrt: number = Sqrt(16);
    observe "Square root of 16: " + sqrt;

    // System functions
    induce os: string = GetOperatingSystem();
    observe "Operating system: " + os;

    // Validation
    induce isValid: boolean = IsValidEmail("test@example.com");
    observe "Email valid: " + isValid;

    // Statistics
    induce mean: number = CalculateMean([1, 2, 3, 4, 5]);
    observe "Mean: " + mean;

    finale {
        observe "=== Demo completed ===";
    }
} Relax
```

## CLI Command

List all Builtin Functions in the terminal:

```bash
hypnoscript builtins
```

## Complete Reference

For a complete alphabetical list of all 110+ functions see:

[→ Complete Builtin Reference](./_complete-reference)

## Category Index

- [Math Functions](./math-functions)
- [String Functions](./string-functions)
- [Array Functions](./array-functions)
- [Statistics Functions](./statistics-functions)
- [Time/Date Functions](./time-date-functions)
- [System Functions](./system-functions)
- [File Functions](./file-functions)
- [Validation Functions](./validation-functions)
- [Hashing Functions](./hashing-encoding)
- [DeepMind Functions](./deepmind-functions)
- [Hypnotic Functions](./hypnotic-functions)

## Next Steps

- [Examples](../examples/basic-examples) - Practical examples
- [Language Reference](../language-reference/syntax) - Language syntax
- [CLI Commands](../cli/commands) - Command-line commands
