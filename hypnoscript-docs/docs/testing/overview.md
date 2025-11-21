---
sidebar_position: 1
---

# Test Framework Overview

The HypnoScript test framework provides comprehensive testing functionality for unit tests, integration tests, and performance tests.

## Fundamentals

### Test Structure

Tests in HypnoScript use a special syntax with `Test` blocks:

```hyp
Test "My first test" {
    entrance {
        induce result = 2 + 2;
        AssertEqual(result, 4);
    }
} Relax;
```

### Test Execution

```bash
# Run all tests
dotnet run --project HypnoScript.CLI -- test *.hyp

# Specific test file
dotnet run --project HypnoScript.CLI -- test test_math.hyp

# Tests with filter
dotnet run --project HypnoScript.CLI -- test *.hyp --filter "math"

# Parallel execution
dotnet run --project HypnoScript.CLI -- test *.hyp --parallel
```

## Test Syntax

### Simple Tests

```hyp
Test "Addition works" {
    entrance {
        induce a = 5;
        induce b = 3;
        induce result = a + b;
        AssertEqual(result, 8);
    }
} Relax;

Test "String concatenation" {
    entrance {
        induce str1 = "Hello";
        induce str2 = "World";
        induce result = str1 + " " + str2;
        AssertEqual(result, "Hello World");
    }
} Relax;
```

### Test with Setup and Teardown

```hyp
Test "File operations" {
    setup {
        WriteFile("test.txt", "Test data");
    }

    entrance {
        induce content = ReadFile("test.txt");
        AssertEqual(content, "Test data");
    }

    teardown {
        if (FileExists("test.txt")) {
            DeleteFile("test.txt");
        }
    }
} Relax;
```

### Test Groups

```hyp
TestGroup "Mathematical functions" {
    Test "Addition" {
        entrance {
            AssertEqual(2 + 2, 4);
        }
    } Relax;

    Test "Subtraction" {
        entrance {
            AssertEqual(5 - 3, 2);
        }
    } Relax;

    Test "Multiplication" {
        entrance {
            AssertEqual(4 * 3, 12);
        }
    } Relax;
} Relax;
```

## Assertions

### Basic Assertions

```hyp
Test "Basic assertions" {
    entrance {
        // Equality
        AssertEqual(5, 5);
        AssertNotEqual(5, 6);

        // Boolean values
        AssertTrue(true);
        AssertFalse(false);

        // Null checks
        AssertNull(null);
        AssertNotNull("not null");

        // Empty checks
        AssertEmpty("");
        AssertNotEmpty("not empty");
    }
} Relax;
```

### Advanced Assertions

```hyp
Test "Advanced assertions" {
    entrance {
        induce arr = [1, 2, 3, 4, 5];

        // Array assertions
        AssertArrayContains(arr, 3);
        AssertArrayNotContains(arr, 6);
        AssertArrayLength(arr, 5);

        // String assertions
        induce str = "HypnoScript";
        AssertStringContains(str, "Script");
        AssertStringStartsWith(str, "Hypno");
        AssertStringEndsWith(str, "Script");

        // Numeric assertions
        AssertGreaterThan(10, 5);
        AssertLessThan(3, 7);
        AssertGreaterThanOrEqual(5, 5);
        AssertLessThanOrEqual(5, 5);

        // Float assertions (with tolerance)
        AssertFloatEqual(3.14159, 3.14, 0.01);
    }
} Relax;
```

### Exception Assertions

```hyp
Test "Exception tests" {
    entrance {
        // Expected exception
        AssertThrows(function() {
            throw "Test exception";
        });

        // No exception
        AssertDoesNotThrow(function() {
            induce x = 1 + 1;
        });

        // Specific exception
        AssertThrowsWithMessage(function() {
            throw "Invalid value";
        }, "Invalid value");
    }
} Relax;
```

## Test Fixtures

### Global Fixtures

```hyp
TestFixture "Database fixture" {
    setup {
        // Establish database connection
        induce connection = CreateDatabaseConnection();
        SetGlobalFixture("db", connection);
    }

    teardown {
        // Close database connection
        induce connection = GetGlobalFixture("db");
        CloseDatabaseConnection(connection);
    }
} Relax;

Test "Database test" {
    entrance {
        induce db = GetGlobalFixture("db");
        induce result = ExecuteQuery(db, "SELECT COUNT(*) FROM users");
        AssertGreaterThan(result, 0);
    }
} Relax;
```

### Test-specific Fixtures

```hyp
Test "With fixture" {
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

## Test Parameterization

### Parameterized Tests

```hyp
Test "Addition with parameters" {
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

### Data-driven Tests

```hyp
Test "String tests with data" {
    dataSource "test_data.json"

    entrance {
        induce [input, expected] = GetTestData();
        induce result = ToUpper(input);
        AssertEqual(result, expected);
    }
} Relax;
```

## Performance Tests

### Benchmark Tests

```hyp
Benchmark "Array sorting" {
    entrance {
        induce arr = Range(1, 1000);
        induce shuffled = Shuffle(arr);

        induce startTime = Timestamp();
        induce sorted = Sort(shuffled);
        induce endTime = Timestamp();

        induce duration = endTime - startTime;
        AssertLessThan(duration, 1.0); // Maximum 1 second

        // Store performance metrics
        RecordMetric("sort_duration", duration);
        RecordMetric("array_size", Length(arr));
    }
} Relax;
```

### Load Tests

```hyp
LoadTest "API performance" {
    iterations 100
    concurrent 10

    entrance {
        induce startTime = Timestamp();
        induce response = HttpGet("https://api.example.com/data");
        induce endTime = Timestamp();

        induce responseTime = (endTime - startTime) * 1000; // in ms
        AssertLessThan(responseTime, 500); // Maximum 500ms

        RecordMetric("response_time", responseTime);
        RecordMetric("response_size", Length(response));
    }
} Relax;
```

## Test Reporting

### Different Report Formats

```bash
# Text report (default)
dotnet run --project HypnoScript.CLI -- test *.hyp

# JSON report
dotnet run --project HypnoScript.CLI -- test *.hyp --format json --output test-results.json

# XML report (for CI/CD)
dotnet run --project HypnoScript.CLI -- test *.hyp --format xml --output test-results.xml

# HTML report
dotnet run --project HypnoScript.CLI -- test *.hyp --format html --output test-report.html
```

### Coverage Reporting

```bash
# Enable code coverage
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage

# Coverage with threshold
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-threshold 80

# Generate coverage report
dotnet run --project HypnoScript.CLI -- test *.hyp --coverage --coverage-report html
```

## Test Configuration

### Test Configuration in hypnoscript.config.json

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

### Test Organization

```hyp
// test_math.hyp
TestGroup "Basic mathematical operations" {
    Test "Addition" {
        entrance {
            AssertEqual(2 + 2, 4);
        }
    } Relax;

    Test "Subtraction" {
        entrance {
            AssertEqual(5 - 3, 2);
        }
    } Relax;
} Relax;

TestGroup "Advanced mathematics" {
    Test "Exponentiation" {
        entrance {
            AssertEqual(Pow(2, 3), 8);
        }
    } Relax;

    Test "Square root" {
        entrance {
            AssertFloatEqual(Sqrt(16), 4, 0.001);
        }
    } Relax;
} Relax;
```

### Test Naming

```hyp
// Good test names
Test "should_return_sum_when_adding_two_numbers" { ... } Relax;
Test "should_throw_exception_when_dividing_by_zero" { ... } Relax;
Test "should_validate_email_format_correctly" { ... } Relax;

// Bad test names
Test "test1" { ... } Relax;
Test "math" { ... } Relax;
Test "function" { ... } Relax;
```

### Test Isolation

```hyp
Test "Isolated test" {
    setup {
        // Each test gets its own data
        induce testFile = "test_" + Timestamp() + ".txt";
        WriteFile(testFile, "Test data");
        SetTestData("file", testFile);
    }

    entrance {
        induce file = GetTestData("file");
        induce content = ReadFile(file);
        AssertEqual(content, "Test data");
    }

    teardown {
        // Cleanup
        induce file = GetTestData("file");
        if (FileExists(file)) {
            DeleteFile(file);
        }
    }
} Relax;
```

### Mocking and Stubbing

```hyp
Test "With mock" {
    entrance {
        // Create mock function
        MockFunction("HttpGet", function(url) {
            return '{"status": "success", "data": "mocked"}';
        });

        induce response = HttpGet("https://api.example.com");
        induce data = ParseJSON(response);

        AssertEqual(data.status, "success");
        AssertEqual(data.data, "mocked");

        // Remove mock
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

## Next Steps

- [Test Assertions](./assertions) - Detailed assertion reference
- [Test Fixtures](./fixtures) - Advanced fixture features
- [Performance Testing](./performance) - Performance test guide
- [Test Reporting](./reporting) - Report configuration

---

**Mastered the test framework? Learn about [Test Assertions](./assertions) next!** ✅
