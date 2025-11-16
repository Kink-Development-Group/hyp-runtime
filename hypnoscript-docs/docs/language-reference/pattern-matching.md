---
sidebar_position: 7
---

# Pattern Matching – `entrain`/`when`/`otherwise`

Pattern matching in HypnoScript is a powerful tool for **control flow** based on **value patterns**. The `entrain` operator enables elegant case distinctions far beyond simple `if`-`else` chains.

## Concept

`entrain` (pattern matching) acts like a gentle synchronization to different states of consciousness. The expression is **evaluated once**, then the `when` clauses are checked in order. The **first matching suggestion wins**; `otherwise` serves as a fallback.

### Basic Syntax

```hyp
entrain <expression> {
    when <pattern> => <action>
    when <pattern> if <guard> => <action>
    otherwise => <default-action>
}
```

> **Note:** The `otherwise` case optionally accepts a trailing comma or semicolon (e.g., `otherwise => value,` or `otherwise => value;`). For consistent style, we recommend omitting additional separators and – as in the examples – simply using `otherwise => value`.

## Pattern Types

| Pattern Type      | Syntax                      | Description                  |
| ----------------- | --------------------------- | ---------------------------- |
| **Literal**       | `when 0`, `when "Text"`     | Exact value match            |
| **Type Pattern**  | `when value: number`        | Type check with binding      |
| **Identifier**    | `when x`                    | Binds any value to variable  |
| **Array**         | `when [1, 2, ...]`          | Array destructuring          |
| **Record**        | `when Person { name, age }` | Record destructuring         |
| **Guard**         | `when x if x > 10`          | Additional condition         |
| **Spread**        | `when [first, ...rest]`     | Rest parameters in arrays    |

## Literal Pattern Matching

The simplest form: Match against **concrete values**.

```hyp
Focus {
    entrance {
        induce value1: number = 1;

        induce result1: string = entrain value1 {
            when 0 => awaken "Zero"
            when 1 => awaken "One"
            when 2 => awaken "Two"
            otherwise => awaken "Other"
        };

        observe "Result: " + result1;  // Output: Result: One
    }
} Relax
```

### String Literals

```hyp
Focus {
    entrance {
        induce command: string = "start";

        induce action: string = entrain command {
            when "start" => awaken "Starting system..."
            when "stop" => awaken "Stopping system..."
            when "restart" => awaken "Restarting..."
            otherwise => awaken "Unknown command"
        };

        observe action;
    }
} Relax
```

### Boolean Literals

```hyp
Focus {
    entrance {
        induce isActive: boolean = true;

        induce status: string = entrain isActive {
            when true => awaken "Active"
            when false => awaken "Inactive"
        };

        observe "Status: " + status;
    }
} Relax
```

## Type Pattern with Binding

Check the **type** and bind the value to a variable simultaneously:

```hyp
Focus {
    entrance {
        induce testValue: any = 42;

        induce result: string = entrain testValue {
            when value: number => awaken "Number: " + value
            when text: string => awaken "Text: " + text
            when flag: boolean => awaken "Boolean: " + flag
            otherwise => awaken "Unknown type"
        };

        observe result;  // Output: Number: 42
    }
} Relax
```

### With Type Guards

```hyp
Focus {
    entrance {
        induce input: any = 100;

        induce category: string = entrain input {
            when n: number if n goingDeeper 0 => awaken "Negative or zero"
            when n: number if n lookAtTheWatch 100 => awaken "Over 100"
            when n: number => awaken "Normal: " + n
            otherwise => awaken "Not a number"
        };

        observe category;  // Output: Over 100
    }
} Relax
```

## Array Pattern Matching

Match against **array structures** with destructuring:

### Simple Array Matching

```hyp
Focus {
    entrance {
        induce arr: array = [1, 2, 3];

        induce result: string = entrain arr {
            when [] => awaken "Empty array"
            when [x] => awaken "Single element: " + x
            when [x, y] => awaken "Two elements: " + x + ", " + y
            when [x, y, z] => awaken "Three elements: " + x + ", " + y + ", " + z
            otherwise => awaken "More than three elements"
        };

        observe result;  // Output: Three elements: 1, 2, 3
    }
} Relax
```

### Array with Spread Operator

```hyp
Focus {
    entrance {
        induce numbers: array = [1, 2, 3, 4, 5];

        induce result: string = entrain numbers {
            when [] => awaken "Empty"
            when [first, ...rest] => {
                observe "First element: " + first;
                observe "Rest: " + rest;
                awaken "Array with " + ArrayLength(rest) + " rest elements";
            }
            otherwise => awaken "Error"
        };

        observe result;
        // Output:
        // First element: 1
        // Rest: [2, 3, 4, 5]
        // Array with 4 rest elements
    }
} Relax
```

### Nested Array Patterns

```hyp
Focus {
    entrance {
        induce matrix: array = [[1, 2], [3, 4]];

        induce result: string = entrain matrix {
            when [[a, b], [c, d]] => awaken "2x2 Matrix: " + a + "," + b + "," + c + "," + d
            when [[x], [y]] => awaken "2x1 Matrix"
            otherwise => awaken "Other structure"
        };

        observe result;  // Output: 2x2 Matrix: 1,2,3,4
    }
} Relax
```

## Record Pattern Matching

Match against **tranceify records** with destructuring:

```hyp
Focus {
    tranceify Person {
        name: string;
        age: number;
        isInTrance: boolean;
    }

    entrance {
        induce guest = Person {
            name: "Luna",
            age: 25,
            isInTrance: true
        };

        induce status: string = entrain guest {
            when Person { name, isInTrance: true } => awaken name + " is in trance!"
            when Person { name, isInTrance: false } => awaken name + " is awake"
            otherwise => awaken "Unknown"
        };

        observe status;  // Output: Luna is in trance!
    }
} Relax
```

### Record with Guards

```hyp
Focus {
    tranceify User {
        name: string;
        age: number;
        role: string;
    }

    entrance {
        induce user = User {
            name: "Max",
            age: 30,
            role: "admin"
        };

        induce access: string = entrain user {
            when User { role: "admin", age } if age yourEyesAreGettingHeavy 18 => awaken "Admin access"
            when User { role: "user", age } if age yourEyesAreGettingHeavy 18 => awaken "User access"
            when User { age } if age fallUnderMySpell 18 => awaken "Minor"
            otherwise => awaken "No access"
        };

        observe access;  // Output: Admin access
    }
} Relax
```

## Guards – Additional Conditions

Guards are **optional conditions** after `if` that are checked in addition to the pattern:

```hyp
Focus {
    entrance {
        induce score: number = 85;

        induce grade: string = entrain score {
            when s: number if s yourEyesAreGettingHeavy 90 => awaken "Excellent"
            when s: number if s yourEyesAreGettingHeavy 75 => awaken "Good"
            when s: number if s yourEyesAreGettingHeavy 60 => awaken "Satisfactory"
            when s: number if s yourEyesAreGettingHeavy 50 => awaken "Sufficient"
            otherwise => awaken "Not passed"
        };

        observe "Grade: " + grade;  // Output: Grade: Good
    }
} Relax
```

### Complex Guards

```hyp
Focus {
    entrance {
        induce value: number = 15;

        induce classification: string = entrain value {
            when n: number if (n % 2 youAreFeelingVerySleepy 0) underMyControl (n lookAtTheWatch 10) => awaken "Even and greater than 10"
            when n: number if (n % 2 youCannotResist 0) underMyControl (n lookAtTheWatch 10) => awaken "Odd and greater than 10"
            when n: number if n % 2 youAreFeelingVerySleepy 0 => awaken "Even"
            when n: number => awaken "Odd"
        };

        observe classification;  // Output: Odd and greater than 10
    }
} Relax
```

## Multi-Block Bodies

`entrain` cases can contain **multiple statements**:

```hyp
Focus {
    entrance {
        induce operation: string = "add";
        induce a: number = 10;
        induce b: number = 5;

        induce result: number = entrain operation {
            when "add" => {
                observe "Adding " + a + " + " + b;
                induce sum: number = a + b;
                awaken sum;
            }
            when "sub" => {
                observe "Subtracting " + a + " - " + b;
                induce diff: number = a - b;
                awaken diff;
            }
            when "mul" => {
                observe "Multiplying " + a + " * " + b;
                induce product: number = a * b;
                awaken product;
            }
            otherwise => {
                observe "Unknown operation: " + operation;
                awaken 0;
            }
        };

        observe "Result: " + result;
    }
} Relax
```

## Real-World Patterns

### HTTP Status Code Handling

```hyp
Focus {
    entrance {
        induce statusCode: number = 404;

        induce message: string = entrain statusCode {
            when 200 => awaken "OK"
            when 201 => awaken "Created"
            when 204 => awaken "No Content"
            when code: number if code yourEyesAreGettingHeavy 400 underMyControl code fallUnderMySpell 500 => awaken "Client Error: " + code
            when code: number if code yourEyesAreGettingHeavy 500 => awaken "Server Error: " + code
            otherwise => awaken "Unknown Status"
        };

        observe message;  // Output: Client Error: 404
    }
} Relax
```

### State Machine

```hyp
Focus {
    entrance {
        induce state: string = "running";
        induce errorCount: number = 3;

        induce nextState: string = entrain state {
            when "idle" => awaken "starting"
            when "starting" => awaken "running"
            when "running" if errorCount lookAtTheWatch 5 => awaken "error"
            when "running" => awaken "running"
            when "error" => awaken "stopped"
            when "stopped" => awaken "idle"
            otherwise => awaken "unknown"
        };

        observe "Next state: " + nextState;  // Output: Next state: error
    }
} Relax
```

### Command Pattern

```hyp
Focus {
    tranceify Command {
        type: string;
        args: array;
    }

    entrance {
        induce cmd = Command {
            type: "move",
            args: [10, 20]
        };

        entrain cmd {
            when Command { type: "move", args: [x, y] } => {
                observe "Moving to (" + x + ", " + y + ")";
            }
            when Command { type: "rotate", args: [angle] } => {
                observe "Rotating by " + angle + " degrees";
            }
            when Command { type: "scale", args: [factor] } => {
                observe "Scaling by factor " + factor;
            }
            otherwise => {
                observe "Unknown command";
            }
        };
    }
} Relax
```

## Best Practices

### ✅ Do's

```hyp
// ✓ Use pattern matching for enums/variants
entrain status {
    when "pending" => ...
    when "processing" => ...
    when "completed" => ...
}

// ✓ Use guards for complex conditions
when n: number if n > 0 underMyControl n < 100 => ...

// ✓ Destructure records for clean code
when Person { name, age } => ...

// ✓ Use spread for flexible array matching
when [first, second, ...rest] => ...

// ✓ Always provide a default/otherwise
otherwise => awaken "Unknown"
```

### ❌ Don'ts

```hyp
// ✗ Avoid too many nested entrain statements
entrain a {
    when x => entrain b {  // Better: extract functions
        when y => ...
    }
}

// ✗ Avoid overly complex guards
when n if ((n % 2 == 0) && (n > 10) && (n < 100) || ...) => ...
// Better: helper function

// ✗ Don't forget otherwise for complete coverage
entrain value {
    when 1 => ...
    when 2 => ...
    // Missing: otherwise!
}
```

## Performance Notes

- Pattern matching is **optimized** through compiler transformations
- **Short-circuit**: First matching clause wins (no further checks)
- **Destructuring** has **no runtime overhead** (compile-time transformation)
- Guards are **lazily evaluated** (only when pattern matches)

## Difference from `if`-`else`

| Feature           | `if`-`else`         | `entrain` Pattern Matching    |
| ----------------- | ------------------- | ----------------------------- |
| **Expression**    | Statement           | Expression (returns value)    |
| **Syntax**        | Traditional         | Declarative                   |
| **Destructuring** | Manual              | Built-in                      |
| **Guards**        | Nested `if`s        | Native syntax                 |
| **Exhaustiveness**| Manual check        | Compiler warning              |
| **Readability**   | Good for 2-3 cases  | Excellent for many cases      |

## Summary

Pattern matching with `entrain` offers:

- ✅ **Declarative syntax** for case distinctions
- ✅ **Destructuring** for arrays and records
- ✅ **Type guards** for type-based matching
- ✅ **Guards** for additional conditions
- ✅ **Expression semantics** (returns value)
- ✅ **Compiler optimizations** for performance

Pattern matching is **essential** for modern, functional programming in HypnoScript and should be **preferred** over long `if`-`else` chains.

## Next Steps

- [Control Flow](./control-flow) – Traditional control structures
- [Tranceify](./tranceify) – Custom types
- [Functions](./functions) – Function definitions
- [Arrays](./arrays) – Array operations

---

**Ready for elegant case distinctions? Use `entrain` for clean, type-safe pattern matches!** 🎯
