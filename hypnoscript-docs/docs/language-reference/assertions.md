---
title: Assertions
---

# Assertions

Assertions are powerful tools in HypnoScript for checking conditions and detecting errors early.

## Overview

Assertions allow you to formulate assumptions about the state of your program and automatically verify them. They are particularly useful for debugging, testing, and validating input data.

## Basic Syntax

### Simple Assertion

```hyp
assert condition "Optional message";
```

### Assertion without Message

```hyp
assert condition;
```

## Basic Assertions

### Boolean Assertions

```hyp
Focus {
    entrance {
        induce isLoggedIn = true;
        induce hasPermission = false;

        // Simple boolean assertions
        assert isLoggedIn "User must be logged in";
        assert !hasPermission "User should not have permission";

        // Complex conditions
        induce userAge = 25;
        induce isAdult = userAge >= 18;
        assert isAdult "User must be of legal age";

        observe "All assertions passed!";
    }
} Relax;
```

### Equality Assertions

```hyp
Focus {
    entrance {
        induce expected = 42;
        induce actual = 42;

        // Check equality
        assert actual == expected "Value should be 42";

        // Check inequality
        induce differentValue = 100;
        assert actual != differentValue "Values should be different";

        // String equality
        induce name = "Alice";
        assert name == "Alice" "Name should be Alice";

        observe "Equality assertions passed!";
    }
} Relax;
```

### Numeric Assertions

```hyp
Focus {
    entrance {
        induce value = 50;

        // Greater than
        assert value > 0 "Value should be positive";
        assert value >= 50 "Value should be at least 50";

        // Less than
        assert value < 100 "Value should be less than 100";
        assert value <= 50 "Value should be at most 50";

        // Range check
        assert value >= 0 && value <= 100 "Value should be between 0 and 100";

        observe "Numeric assertions passed!";
    }
} Relax;
```

## Advanced Assertions

### Array Assertions

```hyp
Focus {
    entrance {
        induce numbers = [1, 2, 3, 4, 5];

        // Check array length
        assert ArrayLength(numbers) == 5 "Array should have 5 elements";
        assert ArrayLength(numbers) > 0 "Array should not be empty";

        // Check array contents
        assert ArrayContains(numbers, 3) "Array should contain 3";
        assert !ArrayContains(numbers, 10) "Array should not contain 10";

        // Check array elements
        assert ArrayGet(numbers, 0) == 1 "First element should be 1";
        assert ArrayGet(numbers, ArrayLength(numbers) - 1) == 5 "Last element should be 5";

        observe "Array assertions passed!";
    }
} Relax;
```

### String Assertions

```hyp
Focus {
    entrance {
        induce text = "Hello World";

        // String length
        assert Length(text) > 0 "Text should not be empty";
        assert Length(text) <= 100 "Text should have at most 100 characters";

        // String content
        assert Contains(text, "Hello") "Text should contain 'Hello'";
        assert StartsWith(text, "Hello") "Text should start with 'Hello'";
        assert EndsWith(text, "World") "Text should end with 'World'";

        // String format
        induce email = "user@example.com";
        assert IsValidEmail(email) "Email should be valid";

        observe "String assertions passed!";
    }
} Relax;
```

### Object Assertions

```hyp
Focus {
    entrance {
        record Person {
            name: string;
            age: number;
        }

        induce person = Person {
            name: "Alice",
            age: 30
        };

        // Check object properties
        assert person.name != "" "Name should not be empty";
        assert person.age >= 0 "Age should not be negative";
        assert person.age <= 150 "Age should be realistic";

        // Check object type
        assert person != null "Person should not be null";

        observe "Object assertions passed!";
    }
} Relax;
```

## Specialized Assertions

### Type Assertions

```hyp
Focus {
    entrance {
        induce value = 42;
        induce text = "Hello";
        induce array = [1, 2, 3];

        // Check type
        assert TypeOf(value) == "number" "Value should be of type number";
        assert TypeOf(text) == "string" "Text should be of type string";
        assert TypeOf(array) == "array" "Array should be of type array";

        // Null check
        induce nullableValue = null;
        assert nullableValue == null "Value should be null";

        observe "Type assertions passed!";
    }
} Relax;
```

### Function Assertions

```hyp
Focus {
    entrance {
        // Define function
        suggestion add(a: number, b: number): number {
            awaken a + b;
        }

        // Check function result
        induce result = call add(2, 3);
        assert result == 5 "2 + 3 should equal 5";

        // Check function behavior
        induce zeroResult = call add(0, 0);
        assert zeroResult == 0 "0 + 0 should equal 0";

        // Negative numbers
        induce negativeResult = call add(-1, -2);
        assert negativeResult == -3 "-1 + (-2) should equal -3";

        observe "Function assertions passed!";
    }
} Relax;
```

### Performance Assertions

```hyp
Focus {
    entrance {
        // Measure performance
        induce startTime = GetCurrentTime();

        // Perform operation
        induce sum = 0;
        for (induce i = 0; i < 1000; induce i = i + 1) {
            sum = sum + i;
        }

        induce endTime = GetCurrentTime();
        induce executionTime = (endTime - startTime) * 1000; // in ms

        // Performance assertions
        assert executionTime < 100 "Operation should be faster than 100ms";
        assert sum == 499500 "Sum should be calculated correctly";

        observe "Performance assertions passed!";
        observe "Execution time: " + executionTime + " ms";
    }
} Relax;
```

## Assertion Patterns

### Input Validation

```hyp
Focus {
    entrance {
        suggestion validateUserInput(username: string, age: number): boolean {
            // Username validation
            assert Length(username) >= 3 "Username should have at least 3 characters";
            assert Length(username) <= 20 "Username should have at most 20 characters";
            assert !Contains(username, " ") "Username should not contain spaces";

            // Age validation
            assert age >= 13 "User should be at least 13 years old";
            assert age <= 120 "Age should be realistic";

            // Additional validations
            assert IsValidUsername(username) "Username should be valid";

            return true;
        }

        // Test validation
        try {
            induce isValid = call validateUserInput("alice123", 25);
            assert isValid "Input should be valid";
            observe "Input validation successful!";
        } catch (error) {
            observe "Validation error: " + error;
        }
    }
} Relax;
```

### State Validation

```hyp
Focus {
    entrance {
        record GameState {
            playerHealth: number;
            score: number;
            level: number;
        }

        induce gameState = GameState {
            playerHealth: 100,
            score: 1500,
            level: 3
        };

        // State assertions
        assert gameState.playerHealth >= 0 "Player health should not be negative";
        assert gameState.playerHealth <= 100 "Player health should be at most 100";
        assert gameState.score >= 0 "Score should not be negative";
        assert gameState.level >= 1 "Level should be at least 1";

        // Consistency check
        assert gameState.playerHealth > 0 || gameState.level == 1 "Player should be alive or in first level";

        observe "State validation successful!";
    }
} Relax;
```

### API Response Validation

```hyp
Focus {
    entrance {
        record ApiResponse {
            status: number;
            data: object;
            message: string;
        }

        // Simulated API response
        induce response = ApiResponse {
            status: 200,
            data: {
                userId: 123,
                name: "Alice"
            },
            message: "Success"
        };

        // Response validation
        assert response.status >= 200 && response.status < 300 "Status should be successful";
        assert response.data != null "Data should be present";
        assert Length(response.message) > 0 "Message should not be empty";

        // Data validation
        if (response.data.userId) {
            assert response.data.userId > 0 "User ID should be positive";
        }

        if (response.data.name) {
            assert Length(response.data.name) > 0 "Name should not be empty";
        }

        observe "API response validation successful!";
    }
} Relax;
```

## Assertion Frameworks

### Test Assertions

```hyp
Focus {
    entrance {
        // Test setup
        induce testResults = [];

        // Test functions
        suggestion assertEqual(actual: object, expected: object, message: string) {
            if (actual != expected) {
                ArrayPush(testResults, "FAIL: " + message + " (Expected: " + expected + ", Got: " + actual + ")");
                throw "Assertion failed: " + message;
            } else {
                ArrayPush(testResults, "PASS: " + message);
            }
        }

        suggestion assertTrue(condition: boolean, message: string) {
            if (!condition) {
                ArrayPush(testResults, "FAIL: " + message);
                throw "Assertion failed: " + message;
            } else {
                ArrayPush(testResults, "PASS: " + message);
            }
        }

        // Run tests
        try {
            call assertEqual(2 + 2, 4, "Addition test");
            call assertTrue(Length("Hello") == 5, "String length test");
            call assertEqual(ArrayLength([1, 2, 3]), 3, "Array length test");

            observe "All tests passed!";
        } catch (error) {
            observe "Test failed: " + error;
        }

        // Display test results
        observe "Test results:";
        for (induce i = 0; i < ArrayLength(testResults); induce i = i + 1) {
            observe "  " + testResults[i];
        }
    }
} Relax;
```

### Debug Assertions

```hyp
Focus {
    entrance {
        induce debugMode = true;

        suggestion debugAssert(condition: boolean, message: string) {
            if (debugMode && !condition) {
                observe "[DEBUG] Assertion failed: " + message;
                observe "[DEBUG] Stack trace: " + GetCallStack();
            }
        }

        // Use debug assertions
        induce value = 42;
        call debugAssert(value > 0, "Value should be positive");
        call debugAssert(value < 100, "Value should be less than 100");

        // Collect debug information
        if (debugMode) {
            induce memoryUsage = GetMemoryUsage();
            call debugAssert(memoryUsage < 1000, "Memory usage should be under 1GB");
        }

        observe "Debug assertions completed!";
    }
} Relax;
```

## Best Practices

### Assertion Strategies

```hyp
Focus {
    entrance {
        // ✅ GOOD: Specific assertions
        induce userAge = 25;
        assert userAge >= 18 "User must be of legal age";

        // ✅ GOOD: Meaningful messages
        induce result = 42;
        assert result == 42 "Calculation should equal 42, not " + result;

        // ✅ GOOD: Early validation
        suggestion processUser(user: object) {
            assert user != null "User object must not be null";
            assert user.name != "" "Username must not be empty";

            // Processing...
        }

        // ❌ BAD: Too general assertions
        assert true "Everything is fine";

        // ❌ BAD: Missing messages
        assert userAge >= 18;
    }
} Relax;
```

### Performance Considerations

```hyp
Focus {
    entrance {
        // ✅ GOOD: Simple assertions for performance-critical paths
        induce criticalValue = 100;
        assert criticalValue > 0; // Fast check

        // ✅ GOOD: Complex assertions only in debug mode
        induce debugMode = true;
        if (debugMode) {
            induce complexValidation = ValidateComplexData();
            assert complexValidation "Complex validation failed";
        }

        // ✅ GOOD: Assertions for invariant conditions
        induce loopCount = 0;
        while (loopCount < 10) {
            assert loopCount >= 0 "Loop counter should not be negative";
            loopCount = loopCount + 1;
        }
    }
} Relax;
```

## Error Handling

### Catching Assertion Errors

```hyp
Focus {
    entrance {
        induce assertionErrors = [];

        suggestion safeAssert(condition: boolean, message: string) {
            try {
                assert condition message;
                awaken true;
            } catch (error) {
                ArrayPush(assertionErrors, error);
                return false;
            }
        }

        // Use safe assertions
        induce test1 = call safeAssert(2 + 2 == 4, "Math works");
        induce test2 = call safeAssert(2 + 2 == 5, "This assertion should fail");
        induce test3 = call safeAssert(Length("Hello") == 5, "String length is correct");

        // Evaluate results
        observe "Successful assertions: " + (test1 && test3);
        observe "Failed assertions: " + (!test2);

        if (ArrayLength(assertionErrors) > 0) {
            observe "Assertion errors:";
            for (induce i = 0; i < ArrayLength(assertionErrors); induce i = i + 1) {
                observe "  " + assertionErrors[i];
            }
        }
    }
} Relax;
```

### Assertion Levels

```hyp
Focus {
    entrance {
        induce assertionLevel = "strict"; // "strict", "normal", "relaxed"

        suggestion levelAssert(condition: boolean, message: string, level: string) {
            if (level == "strict" ||
                (level == "normal" && assertionLevel != "relaxed") ||
                (level == "relaxed" && assertionLevel == "relaxed")) {
                assert condition message;
            }
        }

        // Level-specific assertions
        call levelAssert(true, "Always check", "strict");
        call levelAssert(2 + 2 == 4, "Normal check", "normal");
        call levelAssert(Length("test") == 4, "Relaxed check", "relaxed");

        observe "Level-specific assertions completed!";
    }
} Relax;
```

## Next Steps

- [Testing Overview](../testing/overview) - Comprehensive testing guide
- [Functions](./functions) - Function definitions and calls
- [Error Handling](../error-handling/overview) - Error handling

---

**Mastered assertions? Then learn about [Testing Overview](../testing/overview)!** ✅
