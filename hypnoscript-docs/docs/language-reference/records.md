---
title: Records
---

# Records

Records are structured data types in HypnoScript that allow you to group related data in an object.

## Overview

Records are immutable data structures that can contain multiple fields with different types. They are ideal for representing entities, configurations, and structured data.

## Syntax

### Record Declaration

```hyp
record Person {
    name: string;
    age: number;
    email: string;
    isActive: boolean;
}
```

### Record Instantiation

```hyp
induce person = Person {
    name: "Alice Johnson",
    age: 30,
    email: "alice@example.com",
    isActive: true
};
```

### Record with Optional Fields

```hyp
record User {
    id: number;
    username: string;
    email?: string;  // Optional field
    lastLogin?: number;
}
```

## Basic Usage

### Simple Record

```hyp
Focus {
    entrance {
        // Define record
        record Point {
            x: number;
            y: number;
        }

        // Create record instance
        induce point1 = Point {
            x: 10,
            y: 20
        };

        // Access fields
        observe "X-Coordinate: " + point1.x;
        observe "Y-Coordinate: " + point1.y;
    }
} Relax;
```

### Record with Different Data Types

```hyp
Focus {
    entrance {
        record Product {
            id: number;
            name: string;
            price: number;
            categories: array;
            inStock: boolean;
            metadata: object;
        }

        induce product = Product {
            id: 12345,
            name: "HypnoScript Pro",
            price: 99.99,
            categories: ["Software", "Programming", "Hypnosis"],
            inStock: true,
            metadata: {
                version: "1.0.0",
                releaseDate: "2025-01-15"
            }
        };

        observe "Product: " + product.name;
        observe "Price: " + product.price + " €";
        observe "Categories: " + product.categories;
    }
} Relax;
```

## Record Operations

### Field Access

```hyp
Focus {
    entrance {
        record Address {
            street: string;
            city: string;
            zipCode: string;
            country: string;
        }

        induce address = Address {
            street: "Musterstraße 123",
            city: "Berlin",
            zipCode: "10115",
            country: "Deutschland"
        };

        // Direct field access
        observe "Street: " + address.street;
        observe "City: " + address.city;

        // Dynamic field access
        induce fieldName = "zipCode";
        induce fieldValue = address[fieldName];
        observe "ZIP: " + fieldValue;
    }
} Relax;
```

### Record Copies with Changes

```hyp
Focus {
    entrance {
        record Config {
            theme: string;
            language: string;
            notifications: boolean;
        }

        induce defaultConfig = Config {
            theme: "dark",
            language: "en",
            notifications: true
        };

        // Create copy with changes
        induce userConfig = defaultConfig with {
            theme: "light",
            language: "de"
        };

        observe "Default theme: " + defaultConfig.theme;
        observe "User theme: " + userConfig.theme;
    }
} Relax;
```

### Record Comparisons

```hyp
Focus {
    entrance {
        record Vector {
            x: number;
            y: number;
        }

        induce v1 = Vector { x: 1, y: 2 };
        induce v2 = Vector { x: 1, y: 2 };
        induce v3 = Vector { x: 3, y: 4 };

        // Structural equality
        observe "v1 == v2: " + (v1 == v2);  // true
        observe "v1 == v3: " + (v1 == v3);  // false

        // Deep comparison
        induce areEqual = DeepEquals(v1, v2);
        observe "Deep comparison v1 and v2: " + areEqual;
    }
} Relax;
```

## Advanced Record Features

### Record with Methods

```hyp
Focus {
    entrance {
        record Rectangle {
            width: number;
            height: number;

            // Methods in record
            suggestion area(): number {
                awaken this.width * this.height;
            }

            suggestion perimeter(): number {
                awaken 2 * (this.width + this.height);
            }

            suggestion isSquare(): boolean {
                awaken this.width == this.height;
            }
        }

        induce rect = Rectangle {
            width: 10,
            height: 5
        };

        observe "Area: " + rect.area();
        observe "Perimeter: " + rect.perimeter();
        observe "Is square: " + rect.isSquare();
    }
} Relax;
```

### Record with Computed Fields

```hyp
Focus {
    entrance {
        record Circle {
            radius: number;
            diameter: number;  // Computed from radius

            suggestion constructor(r: number) {
                this.radius = r;
                this.diameter = 2 * r;
            }
        }

        induce circle = Circle(5);
        observe "Radius: " + circle.radius;
        observe "Diameter: " + circle.diameter;
    }
} Relax;
```

### Record with Validation

```hyp
Focus {
    entrance {
        record Email {
            address: string;

            suggestion constructor(email: string) {
                if (IsValidEmail(email)) {
                    this.address = email;
                } else {
                    throw "Invalid email address: " + email;
                }
            }

            suggestion getDomain(): string {
                induce parts = Split(this.address, "@");
                if (ArrayLength(parts) == 2) {
                    awaken parts[1];
                } else {
                    awaken "";
                }
            }
        }

        try {
            induce email = Email("user@example.com");
            observe "Email: " + email.address;
            observe "Domain: " + email.getDomain();
        } catch (error) {
            observe "Error: " + error;
        }
    }
} Relax;
```

## Record Patterns

### Record as Configuration

```hyp
Focus {
    entrance {
        record DatabaseConfig {
            host: string;
            port: number;
            username: string;
            password: string;
            database: string;
            ssl: boolean;
            timeout: number;
        }

        induce dbConfig = DatabaseConfig {
            host: "localhost",
            port: 5432,
            username: "admin",
            password: "secret123",
            database: "hypnoscript",
            ssl: true,
            timeout: 30
        };

        // Use configuration
        induce connectionString = "postgresql://" + dbConfig.username + ":" +
                                 dbConfig.password + "@" + dbConfig.host + ":" +
                                 dbConfig.port + "/" + dbConfig.database;

        observe "Connection string: " + connectionString;
    }
} Relax;
```

### Record as API Response

```hyp
Focus {
    entrance {
        record ApiResponse {
            success: boolean;
            data?: object;
            error?: string;
            timestamp: number;
            requestId: string;
        }

        // Successful response
        induce successResponse = ApiResponse {
            success: true,
            data: {
                userId: 123,
                name: "Alice",
                email: "alice@example.com"
            },
            timestamp: GetCurrentTime(),
            requestId: GenerateUUID()
        };

        // Error response
        induce errorResponse = ApiResponse {
            success: false,
            error: "User not found",
            timestamp: GetCurrentTime(),
            requestId: GenerateUUID()
        };

        observe "Success: " + successResponse.success;
        observe "Error: " + errorResponse.error;
    }
} Relax;
```

### Record for Event Handling

```hyp
Focus {
    entrance {
        record Event {
            type: string;
            source: string;
            timestamp: number;
            data: object;
            priority: number;
        }

        induce userEvent = Event {
            type: "user.login",
            source: "web-interface",
            timestamp: GetCurrentTime(),
            data: {
                userId: 456,
                ipAddress: "192.168.1.100",
                userAgent: "Mozilla/5.0..."
            },
            priority: 1
        };

        // Process event
        if (userEvent.type == "user.login") {
            observe "User login detected: " + userEvent.data.userId;
            LogEvent(userEvent);
        }
    }
} Relax;
```

## Record Arrays and Collections

### Array of Records

```hyp
Focus {
    entrance {
        record Student {
            id: number;
            name: string;
            grade: number;
        }

        induce students = [
            Student { id: 1, name: "Alice", grade: 85 },
            Student { id: 2, name: "Bob", grade: 92 },
            Student { id: 3, name: "Charlie", grade: 78 }
        ];

        // Iterate through records
        for (induce i = 0; i < ArrayLength(students); induce i = i + 1) {
            induce student = students[i];
            observe "Student: " + student.name + " - Grade: " + student.grade;
        }

        // Filter records
        induce topStudents = ArrayFilter(students, function(student) {
            return student.grade >= 90;
        });

        observe "Top students: " + ArrayLength(topStudents);
    }
} Relax;
```

### Record as Dictionary Value

```hyp
Focus {
    entrance {
        record ProductInfo {
            name: string;
            price: number;
            category: string;
        }

        induce productCatalog = {
            "PROD001": ProductInfo { name: "Laptop", price: 999.99, category: "Electronics" },
            "PROD002": ProductInfo { name: "Mouse", price: 29.99, category: "Electronics" },
            "PROD003": ProductInfo { name: "Book", price: 19.99, category: "Books" }
        };

        // Search product by ID
        induce productId = "PROD001";
        if (productCatalog[productId]) {
            induce product = productCatalog[productId];
            observe "Product found: " + product.name + " - " + product.price + " €";
        }
    }
} Relax;
```

## Best Practices

### Record Design

```hyp
Focus {
    entrance {
        // ✅ GOOD: Clear, specific records
        record UserProfile {
            userId: number;
            displayName: string;
            email: string;
            preferences: object;
        }

        // ❌ BAD: Too generic records
        record Data {
            field1: object;
            field2: object;
            field3: object;
        }

        // ✅ GOOD: Use immutable records
        induce user = UserProfile {
            userId: 123,
            displayName: "Alice",
            email: "alice@example.com",
            preferences: {
                theme: "dark",
                language: "en"
            }
        };

        // ✅ GOOD: Create copies for changes
        induce updatedUser = user with {
            displayName: "Alice Johnson"
        };
    }
} Relax;
```

### Performance Optimization

```hyp
Focus {
    entrance {
        // ✅ GOOD: Records for small, frequently used data
        record Point {
            x: number;
            y: number;
        }

        // ✅ GOOD: Sessions for complex objects with behavior
        session ComplexObject {
            expose data: object;

            suggestion processData() {
                // Complex processing
            }
        }

        // ✅ GOOD: Records for configurations
        record AppConfig {
            debug: boolean;
            logLevel: string;
            maxConnections: number;
        }
    }
} Relax;
```

### Error Handling

```hyp
Focus {
    entrance {
        record ValidationResult {
            isValid: boolean;
            errors: array;
            warnings: array;
        }

        suggestion validateEmail(email: string): ValidationResult {
            induce errors = [];
            induce warnings = [];

            if (Length(email) == 0) {
                ArrayPush(errors, "Email must not be empty");
            } else if (!IsValidEmail(email)) {
                ArrayPush(errors, "Invalid email format");
            }

            if (Length(email) > 100) {
                ArrayPush(warnings, "Email is very long");
            }

            return ValidationResult {
                isValid: ArrayLength(errors) == 0,
                errors: errors,
                warnings: warnings
            };
        }

        induce result = validateEmail("test@example.com");
        if (result.isValid) {
            observe "Email is valid";
        } else {
            observe "Email errors: " + result.errors;
        }
    }
} Relax;
```

## Error Handling

Records can throw errors on invalid operations:

```hyp
Focus {
    entrance {
        try {
            record Person {
                name: string;
                age: number;
            }

            induce person = Person {
                name: "Alice",
                age: 30
            };

            // Invalid field access
            induce invalidField = person.nonexistentField;
        } catch (error) {
            observe "Record error: " + error;
        }

        try {
            // Invalid record creation
            induce invalidPerson = Person {
                name: "Bob",
                age: "invalid"  // Should be number
            };
        } catch (error) {
            observe "Validation error: " + error;
        }
    }
} Relax;
```

## Next Steps

- [Sessions](./sessions) - Object-oriented programming with sessions
- [Arrays](./arrays) - Array operations and collections
- [Functions](./functions) - Function definitions and calls

---

**Mastered records? Then learn about [Sessions](./sessions)!** ✅
