---
title: Tranceify - Record Types
---

The `tranceify` keyword allows you to define custom record types (similar to structs in other languages) in HypnoScript. This enables structured data management and type-safe field access.

## Syntax

```hypnoscript
tranceify TypeName {
    fieldName1: type;
    fieldName2: type;
    // ... more fields
}
```

## Creating Record Instances

Use the `induce` keyword with a record literal to create instances:

```hypnoscript
induce variableName = TypeName {
    fieldName1: value1,
    fieldName2: value2
};
```

## Field Access

Access record fields using dot notation:

```hypnoscript
induce person = Person { name: "Alice", age: 30 };
observe person.name;  // Alice
observe person.age;   // 30
```

## Examples

### Basic Record Type

```hypnoscript
tranceify Person {
    name: string;
    age: number;
    isInTrance: boolean;
}

induce person1 = Person {
    name: "Alice",
    age: 30,
    isInTrance: true
};

observe "Name: " + person1.name;
observe "Age: " + person1.age;
```

### Complex Record with Multiple Fields

```hypnoscript
tranceify HypnoSession {
    sessionId: number;
    patientName: string;
    tranceDepth: number;
    suggestions: string;
    duration: number;
    isSuccessful: boolean;
}

induce session = HypnoSession {
    sessionId: 42,
    patientName: "Bob",
    tranceDepth: 8.5,
    suggestions: "You are feeling very relaxed",
    duration: 45,
    isSuccessful: true
};

observe "Session ID: " + session.sessionId;
observe "Patient: " + session.patientName;
observe "Success: " + session.isSuccessful;
```

### Records in Arrays

Records can be stored in arrays and accessed like any other value:

```hypnoscript
tranceify Person {
    name: string;
    age: number;
}

induce person1 = Person { name: "Alice", age: 30 };
induce person2 = Person { name: "Bob", age: 25 };
induce person3 = Person { name: "Charlie", age: 35 };

induce people: array = [person1, person2, person3];
observe "Total people: " + Length(people);

observe people[0].name;  // Alice
observe people[1].age;   // 25
```

### Nested Records

Records can contain fields of other record types:

```hypnoscript
tranceify Address {
    street: string;
    city: string;
    zipCode: number;
}

tranceify Employee {
    name: string;
    employeeId: number;
    address: Address;
}

induce emp = Employee {
    name: "Eve",
    employeeId: 1001,
    address: Address {
        street: "Main St 123",
        city: "Hypnoville",
        zipCode: 12345
    }
};

observe emp.name;              // Eve
observe emp.address.city;      // Hypnoville
observe emp.address.street;    // Main St 123
```

### Calculations with Record Fields

Record fields can be used in calculations and expressions:

```hypnoscript
tranceify Rectangle {
    width: number;
    height: number;
}

induce rect = Rectangle {
    width: 10,
    height: 20
};

induce area = rect.width * rect.height;
observe "Rectangle area: " + area;  // 200
```

## Type Checking

HypnoScript's type checker validates:

1. **Field Existence**: All fields in the record type definition must be present in the literal
2. **Field Types**: Each field value must match the declared type
3. **Unknown Fields**: Fields not declared in the type definition are rejected
4. **Field Access**: Accessing non-existent fields produces type errors

### Example Type Errors

```hypnoscript
tranceify Person {
    name: string;
    age: number;
}

// Error: Missing field 'age'
induce p1 = Person {
    name: "Alice"
};

// Error: Type mismatch for field 'age'
induce p2 = Person {
    name: "Bob",
    age: "thirty"  // should be number
};

// Error: Unknown field 'email'
induce p3 = Person {
    name: "Charlie",
    age: 25,
    email: "charlie@example.com"
};

// Error: Field 'address' does not exist
induce p4 = Person { name: "Diana", age: 30 };
observe p4.address;
```

## Best Practices

1. **Naming Convention**: Use PascalCase for record type names (e.g., `Person`, `HypnoSession`)
2. **Field Naming**: Use camelCase for field names (e.g., `firstName`, `sessionId`)
3. **Type Safety**: Always specify field types explicitly
4. **Initialization**: Ensure all required fields are provided when creating instances
5. **Documentation**: Comment complex record types to explain their purpose

## Use Cases

- **Data Structures**: Organize related data into coherent units
- **Configuration Objects**: Define application settings with type safety
- **Domain Models**: Represent business entities (users, sessions, transactions)
- **Message Passing**: Structure data for communication between components
- **API Responses**: Model structured data from external services

## Limitations

- Records are value types and are copied on assignment
- No methods or behaviors can be attached to record types (use `session` for OOP)
- Field visibility is public by default (no access modifiers)
- Record types cannot inherit from other record types

## See Also

- [Session (OOP)](/language-reference/sessions.md) - For object-oriented programming with methods
- [Type System](/language-reference/types.md) - Overview of HypnoScript's type system
- [Variables](/language-reference/variables.md) - Variable declaration and initialization
