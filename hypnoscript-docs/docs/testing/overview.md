---
sidebar_position: 1
---

# Test-Framework Übersicht

Das HypnoScript Test-Framework bietet umfassende Testing-Funktionalitäten für Unit-Tests, Integration-Tests und Performance-Tests.

## Grundlagen

### Test-Struktur

Tests in HypnoScript verwenden eine spezielle Syntax mit `Test`-Blöcken:

```hyp
Test "Mein erster Test" {
    entrance {
        induce result = 2 + 2;
        AssertEqual(result, 4);
    }
} Relax;
```

### Test-Ausführung

```bash
# Alle Tests ausführen
dotnet run --project HypnoScript.CLI -- test *.hyp

# Spezifische Test-Datei
dotnet run --project HypnoScript.CLI -- test test_math.hyp

# Tests mit Filter
dotnet run --project HypnoScript.CLI -- test *.hyp --filter "math"

# Parallele Ausführung
dotnet run --project HypnoScript.CLI -- test *.hyp --parallel
```

## Test-Syntax

### Einfache Tests

```hyp
Test "Addition funktioniert" {
    entrance {
        induce a = 5;
        induce b = 3;
        induce result = a + b;
        AssertEqual(result, 8);
    }
} Relax;

Test "String-Verkettung" {
    entrance {
        induce str1 = "Hallo";
        induce str2 = "Welt";
        induce result = str1 + " " + str2;
        AssertEqual(result, "Hallo Welt");
    }
} Relax;
```

### Test mit Setup und Teardown

```hyp
Test "Datei-Operationen" {
    setup {
        WriteFile("test.txt", "Test-Daten");
    }

    entrance {
        induce content = ReadFile("test.txt");
        AssertEqual(content, "Test-Daten");
    }

    teardown {
        if (FileExists("test.txt")) {
            DeleteFile("test.txt");
        }
    }
} Relax;
```

### Test-Gruppen

```hyp
TestGroup "Mathematische Funktionen" {
    Test "Addition" {
        entrance {
            AssertEqual(2 + 2, 4);
        }
    } Relax;

    Test "Subtraktion" {
        entrance {
            AssertEqual(5 - 3, 2);
        }
    } Relax;

    Test "Multiplikation" {
        entrance {
            AssertEqual(4 * 3, 12);
        }
    } Relax;
} Relax;
```

## Assertions

### Grundlegende Assertions

```hyp
Test "Grundlegende Assertions" {
    entrance {
        // Gleichheit
        AssertEqual(5, 5);
        AssertNotEqual(5, 6);

        // Wahrheitswerte
        AssertTrue(true);
        AssertFalse(false);

        // Null-Checks
        AssertNull(null);
        AssertNotNull("nicht null");

        // Leere Checks
        AssertEmpty("");
        AssertNotEmpty("nicht leer");
    }
} Relax;
```

### Erweiterte Assertions

```hyp
Test "Erweiterte Assertions" {
    entrance {
        induce arr = [1, 2, 3, 4, 5];

        // Array-Assertions
        AssertArrayContains(arr, 3);
        AssertArrayNotContains(arr, 6);
        AssertArrayLength(arr, 5);

        // String-Assertions
        induce str = "HypnoScript";
        AssertStringContains(str, "Script");
        AssertStringStartsWith(str, "Hypno");
        AssertStringEndsWith(str, "Script");

        // Numerische Assertions
        AssertGreaterThan(10, 5);
        AssertLessThan(3, 7);
        AssertGreaterThanOrEqual(5, 5);
        AssertLessThanOrEqual(5, 5);

        // Float-Assertions (mit Toleranz)
        AssertFloatEqual(3.14159, 3.14, 0.01);
    }
} Relax;
```

### Exception-Assertions

```hyp
Test "Exception-Tests" {
    entrance {
        // Erwartete Exception
        AssertThrows(function() {
            throw "Test-Exception";
        });

        // Keine Exception
        AssertDoesNotThrow(function() {
            induce x = 1 + 1;
        });

        // Spezifische Exception
        AssertThrowsWithMessage(function() {
            throw "Ungültiger Wert";
        }, "Ungültiger Wert");
    }
} Relax;
```

## Test-Fixtures

### Globale Fixtures

```hyp
TestFixture "Datenbank-Fixture" {
    setup {
        // Datenbank-Verbindung aufbauen
        induce connection = CreateDatabaseConnection();
        SetGlobalFixture("db", connection);
    }

    teardown {
        // Datenbank-Verbindung schließen
        induce connection = GetGlobalFixture("db");
        CloseDatabaseConnection(connection);
    }
} Relax;

Test "Datenbank-Test" {
    entrance {
        induce db = GetGlobalFixture("db");
        induce result = ExecuteQuery(db, "SELECT COUNT(*) FROM users");
        AssertGreaterThan(result, 0);
    }
} Relax;
```

### Test-spezifische Fixtures

```hyp
Test "Mit Fixture" {
    fixture {
        induce testData = [1, 2, 3, 4, 5];
        return testData;
    }

    entrance {
        induce data = GetFixture();
        AssertArrayLength(data, 5);
        AssertArrayContains(data, 3);
    }
} Relax;
```

## Test-Parameterisierung

### Parameterisierte Tests

```hyp
Test "Addition mit Parametern" {
    parameters {
        [2, 3, 5],
        [5, 7, 12],
        [0, 0, 0],
        [-1, 1, 0]
    }

    entrance {
        induce [a, b, expected] = GetTestParameters();
        induce result = a + b;
        AssertEqual(result, expected);
    }
} Relax;
```

### Daten-getriebene Tests

```hyp
Test "String-Tests mit Daten" {
    dataSource "test_data.json"

    entrance {
        induce [input, expected] = GetTestData();
        induce result = ToUpper(input);
        AssertEqual(result, expected);
    }
} Relax;
```

## Performance-Tests

### Benchmark-Tests

```hyp
Benchmark "Array-Sortierung" {
    entrance {
        induce arr = Range(1, 1000);
        induce shuffled = Shuffle(arr);

        induce startTime = Timestamp();
        induce sorted = Sort(shuffled);
        induce endTime = Timestamp();

        induce duration = endTime - startTime;
        AssertLessThan(duration, 1.0); // Maximal 1 Sekunde

        // Performance-Metriken speichern
        RecordMetric("sort_duration", duration);
        RecordMetric("array_size", Length(arr));
    }
} Relax;
```

### Load-Tests

```hyp
LoadTest "API-Performance" {
    iterations 100
    concurrent 10

    entrance {
        induce startTime = Timestamp();
        induce response = HttpGet("https://api.example.com/data");
        induce endTime = Timestamp();

        induce responseTime = (endTime - startTime) * 1000; // in ms
        AssertLessThan(responseTime, 500); // Maximal 500ms

        RecordMetric("response_time", responseTime);
        RecordMetric("response_size", Length(response));
    }
} Relax;
```

## Test-Reporting

### Verschiedene Report-Formate

```bash
# Text-Report (Standard)
dotnet run --project HypnoScript.CLI -- test *.hyp

# JSON-Report
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json

# XML-Report (für CI/CD)
dotnet run --project HypnoScript.CLI -- test *.hyp --format xml --output test-results.xml

# HTML-Report
dotnet run --project HypnoScript.CLI -- test *.hyp --format html --output test-report.html
```

### Coverage-Reporting

```bash
# Code-Coverage aktivieren
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage

# Coverage mit Schwellenwert
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-threshold 80

# Coverage-Report generieren
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-report html
```

## Test-Konfiguration

### Test-Konfiguration in hypnoscript.config.json

```json
{
  "testFramework": {
    "autoRun": true,
    "reportFormat": "detailed",
    "parallelExecution": true,
    "timeout": 30000,
    "coverage": {
      "enabled": true,
      "threshold": 80,
      "excludePatterns": ["**/test/**", "**/vendor/**"]
    },
    "fixtures": {
      "autoSetup": true,
      "autoTeardown": true
    },
    "assertions": {
      "strictMode": true,
      "floatTolerance": 0.001
    }
  }
}
```

## Best Practices

### Test-Organisation

```hyp
// test_math.hyp
TestGroup "Mathematische Grundoperationen" {
    Test "Addition" {
        entrance {
            AssertEqual(2 + 2, 4);
        }
    } Relax;

    Test "Subtraktion" {
        entrance {
            AssertEqual(5 - 3, 2);
        }
    } Relax;
} Relax;

TestGroup "Erweiterte Mathematik" {
    Test "Potenzierung" {
        entrance {
            AssertEqual(Pow(2, 3), 8);
        }
    } Relax;

    Test "Wurzel" {
        entrance {
            AssertFloatEqual(Sqrt(16), 4, 0.001);
        }
    } Relax;
} Relax;
```

### Test-Naming

```hyp
// Gute Test-Namen
Test "should_return_sum_when_adding_two_numbers" { ... } Relax;
Test "should_throw_exception_when_dividing_by_zero" { ... } Relax;
Test "should_validate_email_format_correctly" { ... } Relax;

// Schlechte Test-Namen
Test "test1" { ... } Relax;
Test "math" { ... } Relax;
Test "function" { ... } Relax;
```

### Test-Isolation

```hyp
Test "Isolierter Test" {
    setup {
        // Jeder Test bekommt seine eigenen Daten
        induce testFile = "test_" + Timestamp() + ".txt";
        WriteFile(testFile, "Test-Daten");
        SetTestData("file", testFile);
    }

    entrance {
        induce file = GetTestData("file");
        induce content = ReadFile(file);
        AssertEqual(content, "Test-Daten");
    }

    teardown {
        // Aufräumen
        induce file = GetTestData("file");
        if (FileExists(file)) {
            DeleteFile(file);
        }
    }
} Relax;
```

### Mocking und Stubbing

```hyp
Test "Mit Mock" {
    entrance {
        // Mock-Funktion erstellen
        MockFunction("HttpGet", function(url) {
            return '{"status": "success", "data": "mocked"}';
        });

        induce response = HttpGet("https://api.example.com");
        induce data = ParseJSON(response);

        AssertEqual(data.status, "success");
        AssertEqual(data.data, "mocked");

        // Mock entfernen
        UnmockFunction("HttpGet");
    }
} Relax;
```

## CI/CD Integration

### GitHub Actions

```yaml
name: HypnoScript Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup .NET
        uses: actions/setup-dotnet@v3
        with:
          dotnet-version: '8.0.x'

      - name: Run tests
        run: dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json

      - name: Upload test results
        uses: actions/upload-artifact@v3
        with:
          name: test-results
          path: test-results.json

      - name: Check coverage
        run: dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-threshold 80
```

### Jenkins Pipeline

```groovy
pipeline {
    agent any

    stages {
        stage('Test') {
            steps {
                sh 'dotnet run --project HypnoScript.CLI -- test *.hyp --format xml --output test-results.xml'
            }
            post {
                always {
                    publishTestResults testResultsPattern: 'test-results.xml'
                }
            }
        }

        stage('Coverage') {
            steps {
                sh 'dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-report html'
            }
            post {
                always {
                    publishHTML([
                        allowMissing: false,
                        alwaysLinkToLastBuild: true,
                        keepAll: true,
                        reportDir: 'coverage',
                        reportFiles: 'index.html',
                        reportName: 'Coverage Report'
                    ])
                }
            }
        }
    }
}
```

## Nächste Schritte

- [Test-Assertions](./assertions) - Detaillierte Assertion-Referenz
- [Test-Fixtures](./fixtures) - Erweiterte Fixture-Funktionen
- [Performance-Testing](./performance) - Performance-Test-Guide
- [Test-Reporting](./reporting) - Report-Konfiguration

---

**Test-Framework gemeistert? Dann lerne [Test-Assertions](./assertions) kennen!** ✅
