---
title: Testing Fixtures
---

# Test Fixtures

Test fixtures provide a way to set up test data and environments for consistent, repeatable testing in HypnoScript.

## Overview

Test fixtures are predefined data sets and configurations that help ensure your tests run consistently across different environments and scenarios.

## Creating Test Fixtures

### 1. Basic Test Fixture Structure

```hyp
// test_fixtures.hyp
Session TestData {
  // User data fixtures
  induce testUser: record = {
    "name": "John Doe",
    "email": "john@example.com",
    "age": 30,
    "active": true
  };

  induce adminUser: record = {
    "name": "Admin User",
    "email": "admin@example.com",
    "age": 35,
    "active": true,
    "role": "admin"
  };

  // Array fixtures
  induce numberArray: number[] = [1, 2, 3, 4, 5, 10, 15, 20];
  induce stringArray: string[] = ["apple", "banana", "cherry", "date"];
  induce mixedArray: any[] = [1, "hello", true, 3.14];

  // Configuration fixtures
  induce testConfig: record = {
    "timeout": 5000,
    "retries": 3,
    "debug": true,
    "logLevel": "INFO"
  };
}
```

### 2. Loading Fixtures in Tests

```hyp
// test_with_fixtures.hyp
Focus {
  // Load test fixtures
  MindLink TestData;

  // Use fixture data in tests
  induce user: record = testUser;
  Observe("Testing with user: " + user["name"]);

  // Validate user data
  Assert(IsString(user["name"]), "User name should be a string");
  Assert(IsNumber(user["age"]), "User age should be a number");
  Assert(user["age"] > 0, "User age should be positive");

  // Test with different fixtures
  induce admin: record = adminUser;
  Assert(admin["role"] == "admin", "Admin should have admin role");

  // Test array fixtures
  induce numbers: number[] = numberArray;
  Assert(ArrayLength(numbers) == 8, "Number array should have 8 elements");
  Assert(numbers[0] == 1, "First element should be 1");

  Observe("All fixture tests passed!");
} Relax
```

## Advanced Fixture Patterns

### 1. Dynamic Fixture Generation

```hyp
// dynamic_fixtures.hyp
Focus {
  function GenerateUserFixture(name: string, age: number, role: string): record {
    return {
      "name": name,
      "email": ToLowerCase(name) + "@example.com",
      "age": age,
      "role": role,
      "active": true,
      "createdAt": GetCurrentTime()
    };
  }

  function GenerateNumberArray(size: number, start: number, step: number): number[] {
    induce result: number[] = [];
    induce current: number = start;

    for (induce i: number = 0; i < size; i = i + 1) {
      result = ArrayPush(result, current);
      current = current + step;
    }

    return result;
  }

  // Generate test data dynamically
  induce dynamicUser: record = GenerateUserFixture("Jane Smith", 28, "user");
  induce fibonacci: number[] = GenerateNumberArray(10, 1, 1);

  // Test dynamic fixtures
  Assert(dynamicUser["name"] == "Jane Smith", "Dynamic user name should match");
  Assert(ArrayLength(fibonacci) == 10, "Fibonacci array should have 10 elements");

  Observe("Dynamic fixture generation successful!");
} Relax
```

### 2. Fixture Validation

```hyp
// fixture_validation.hyp
Focus {
  function ValidateUserFixture(user: record): boolean {
    // Check required fields
    if (!HasKey(user, "name") || IsNullOrEmpty(user["name"])) {
      return false;
    }

    if (!HasKey(user, "email") || IsNullOrEmpty(user["email"])) {
      return false;
    }

    if (!HasKey(user, "age") || !IsNumber(user["age"])) {
      return false;
    }

    // Validate email format
    if (!IsValidEmail(user["email"])) {
      return false;
    }

    // Validate age range
    if (user["age"] < 0 || user["age"] > 150) {
      return false;
    }

    return true;
  }

  function ValidateArrayFixture(arr: any[], expectedType: string): boolean {
    if (!IsArray(arr)) {
      return false;
    }

    if (ArrayLength(arr) == 0) {
      return false;
    }

    // Check type consistency
    for (induce i: number = 0; i < ArrayLength(arr); i = i + 1) {
      if (expectedType == "number" && !IsNumber(arr[i])) {
        return false;
      }
      if (expectedType == "string" && !IsString(arr[i])) {
        return false;
      }
    }

    return true;
  }

  // Test fixture validation
  MindLink TestData;

  Assert(ValidateUserFixture(testUser), "Test user fixture should be valid");
  Assert(ValidateUserFixture(adminUser), "Admin user fixture should be valid");
  Assert(ValidateArrayFixture(numberArray, "number"), "Number array fixture should be valid");
  Assert(ValidateArrayFixture(stringArray, "string"), "String array fixture should be valid");

  Observe("Fixture validation tests passed!");
} Relax
```

### 3. Fixture Cleanup and Reset

```hyp
// fixture_cleanup.hyp
Focus {
  function ResetTestEnvironment(): void {
    // Clear any test data
    ClearScreen();
    Observe("Test environment reset");
  }

  function CleanupTestData(): void {
    // Perform cleanup operations
    Observe("Cleaning up test data...");

    // Reset any global state
    // Clear caches
    // Reset configurations

    Observe("Test data cleanup completed");
  }

  // Test with cleanup
  MindLink TestData;

  // Run tests
  induce user: record = testUser;
  Assert(user["name"] == "John Doe", "User name should match fixture");

  // Cleanup after tests
  CleanupTestData();
  ResetTestEnvironment();

  Observe("Test completed with proper cleanup!");
} Relax
```

## Fixture Categories

### 1. Data Fixtures

```hyp
// data_fixtures.hyp
Session DataFixtures {
  // User data
  induce users: record[] = [
    {"id": 1, "name": "Alice", "email": "alice@example.com"},
    {"id": 2, "name": "Bob", "email": "bob@example.com"},
    {"id": 3, "name": "Charlie", "email": "charlie@example.com"}
  ];

  // Product data
  induce products: record[] = [
    {"id": "P001", "name": "Laptop", "price": 999.99, "category": "Electronics"},
    {"id": "P002", "name": "Book", "price": 19.99, "category": "Books"},
    {"id": "P003", "name": "Coffee", "price": 4.99, "category": "Food"}
  ];

  // Configuration data
  induce settings: record = {
    "theme": "dark",
    "language": "en",
    "timezone": "UTC",
    "notifications": true
  };
}
```

### 2. State Fixtures

```hyp
// state_fixtures.hyp
Session StateFixtures {
  // Application state
  induce appState: record = {
    "isLoggedIn": true,
    "currentUser": "admin",
    "permissions": ["read", "write", "delete"],
    "sessionTimeout": 3600
  };

  // Form state
  induce formState: record = {
    "isValid": true,
    "isSubmitted": false,
    "errors": [],
    "values": {
      "username": "testuser",
      "email": "test@example.com",
      "password": "********"
    }
  };
}
```

### 3. Error Fixtures

```hyp
// error_fixtures.hyp
Session ErrorFixtures {
  // Common error scenarios
  induce validationErrors: record[] = [
    {"field": "email", "message": "Invalid email format", "code": "EMAIL_INVALID"},
    {"field": "password", "message": "Password too short", "code": "PASSWORD_SHORT"},
    {"field": "age", "message": "Age must be positive", "code": "AGE_INVALID"}
  ];

  induce networkErrors: record[] = [
    {"code": 404, "message": "Resource not found", "type": "NOT_FOUND"},
    {"code": 500, "message": "Internal server error", "type": "SERVER_ERROR"},
    {"code": 403, "message": "Access forbidden", "type": "FORBIDDEN"}
  ];
}
```

## Best Practices

### 1. Fixture Organization

```hyp
// Organize fixtures by domain
Session UserFixtures {
  // User-related test data
}

Session ProductFixtures {
  // Product-related test data
}

Session ConfigFixtures {
  // Configuration test data
}
```

### 2. Fixture Naming Conventions

```hyp
// Use descriptive names
induce validUserFixture: record = {...};
induce invalidUserFixture: record = {...};
induce adminUserFixture: record = {...};

// Use consistent naming patterns
induce testData_Users: record[] = {...};
induce testData_Products: record[] = {...};
induce testData_Config: record = {...};
```

### 3. Fixture Documentation

```hyp
// Document your fixtures
Session WellDocumentedFixtures {
  // User fixture for testing authentication
  // Contains valid user credentials and profile data
  induce testUser: record = {
    "username": "testuser",
    "password": "testpass123",
    "email": "test@example.com",
    "profile": {
      "firstName": "Test",
      "lastName": "User",
      "age": 25
    }
  };

  // Admin user fixture for testing authorization
  // Contains admin privileges and elevated permissions
  induce adminUser: record = {
    "username": "admin",
    "password": "adminpass123",
    "email": "admin@example.com",
    "role": "admin",
    "permissions": ["read", "write", "delete", "admin"]
  };
}
```

### 4. Fixture Reusability

```hyp
// Create reusable fixture components
function CreateBaseUser(name: string, email: string): record {
  return {
    "name": name,
    "email": email,
    "createdAt": GetCurrentTime(),
    "isActive": true
  };
}

function CreateUserWithRole(name: string, email: string, role: string): record {
  induce baseUser: record = CreateBaseUser(name, email);
  baseUser["role"] = role;
  return baseUser;
}
```

## Integration with Test Framework

### 1. Using Fixtures in Test Commands

```bash
# Run tests with specific fixtures
dotnet run -- test test_with_fixtures.hyp --verbose

# Run tests with fixture validation
dotnet run -- test fixture_validation.hyp --debug
```

### 2. Fixture Loading in Tests

```hyp
// test_integration.hyp
Focus {
  // Load multiple fixture sessions
  MindLink TestData;
  MindLink DataFixtures;
  MindLink ErrorFixtures;

  // Test with combined fixtures
  induce user: record = testUser;
  induce products: record[] = products;
  induce errors: record[] = validationErrors;

  // Comprehensive testing
  Assert(ValidateUserFixture(user), "User fixture should be valid");
  Assert(ArrayLength(products) > 0, "Products fixture should not be empty");
  Assert(ArrayLength(errors) > 0, "Error fixtures should be available");

  Observe("Integration test with fixtures completed successfully!");
} Relax
```

## Conclusion

Test fixtures are essential for creating reliable, maintainable tests in HypnoScript. By following these patterns and best practices, you can create comprehensive test suites that are easy to understand, maintain, and extend.

Remember to:

- Keep fixtures simple and focused
- Use descriptive names and documentation
- Validate fixture data
- Organize fixtures logically
- Reuse fixture components when possible
- Clean up after tests

This approach will help you build robust test suites that catch issues early and provide confidence in your code quality.
