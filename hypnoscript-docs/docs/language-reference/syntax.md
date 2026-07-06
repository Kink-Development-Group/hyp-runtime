---
sidebar_position: 1
---

# Syntax

HypnoScript uses a hypnotic syntax that is both intuitive and powerful. Learn the fundamental syntax rules and concepts.

## Basic Structure

### Program Structure

Every HypnoScript program begins with `Focus` and ends with `Relax`:

```hyp
Focus {
    // Program code here
} Relax
```

### Entrance Block

> ⚠️ `entrance` blocks are **only allowed at top level** – directly within `Focus { ... }`. If the block is declared within a function, session, or another block, the parser will abort with the message `'entrance' blocks are only allowed at the top level`.

The `entrance` block is executed at program startup:

```hyp
Focus {
    entrance {
        observe "Program started";
    }
} Relax
```

### Finale Block

Similar to the `entrance` block, `finale { ... }` is exclusively available at the top level and is suitable for cleanup tasks. Here too, the parser enforces strict top-level placement and reports `'finale' blocks are only allowed at the top level` if the block is nested.

```hyp
Focus {
    entrance {
        observe "Setup";
    }

    finale {
        observe "Cleanup";
    }
} Relax
```

### Source File Encoding

`.hyp` files are usually plain UTF-8, but the CLI and test harness decode
other common encodings transparently: UTF-8 with a byte-order mark (BOM),
and UTF-16 (little- or big-endian, with or without BOM). Files saved by
editors that default to UTF-16 — such as some Windows tools — simply work,
no re-encoding required.

## Variables and Assignments

### Induce (Variable Declaration)

Use `induce` to declare variables and assign values. Type annotations are optional but recommended:

```hyp
Focus {
    entrance {
        induce name: string = "HypnoScript";
        induce version: number = 1.0;
        induce isActive: boolean = true;

        observe "Name: " + name;
        observe "Version: " + version;
        observe "Active: " + isActive;
    }
} Relax
```

### SharedTrance (Global Variables)

`sharedTrance` declares a module-level variable that every suggestion shares —
one trance, one mind. The declaration keyword (`induce`, `implant`, `embed`,
`freeze`) is optional; the shorthand `sharedTrance name: type = value;` works
on its own:

```hyp
Focus {
    // Shorthand — no induce needed
    sharedTrance total: number = 0;

    // Classic long form is still valid
    sharedTrance induce sessionName: string = "Deep Dive";

    suggestion addToTotal(n: number) {
        total = total + n;
    }

    entrance {
        addToTotal(5);
        addToTotal(7);
        observe "Total: " + total;  // 12
    }
} Relax
```

`sharedTrance` must be followed by a variable declaration; anything else is a
parse error.

### Data Types

HypnoScript supports various data types:

```hyp
Focus {
    entrance {
        // Strings
        induce text: string = "Hello World";

        // Numbers (only number type)
        induce integer: number = 42;
        induce decimal: number = 3.14159;
        induce large: number = 1_000_000;   // digit separators
        induce scientific: number = 2.5e3;  // exponent notation

        // Boolean
        induce flag: boolean = true;

        // Arrays
        induce numbers: number[] = [1, 2, 3, 4, 5];
        induce names: string[] = ["Alice", "Bob", "Charlie"];

        // Records (defined with tranceify)
        // See Records documentation for details
    }
} Relax
```

### String Escape Sequences

String literals support the usual escapes plus hexadecimal Unicode escapes:

| Escape   | Meaning                                        |
| -------- | ---------------------------------------------- |
| `\n`     | Newline                                        |
| `\t`     | Tab                                            |
| `\r`     | Carriage return                                |
| `\\`     | Backslash                                      |
| `\"`     | Double quote                                   |
| `\$`     | Literal `$` (suppresses interpolation)         |
| `\uXXXX` | Unicode code point (exactly 4 hex digits)      |
| `\xNN`   | Unicode code point (exactly 2 hex digits)      |

```hyp
Focus {
    entrance {
        observe "\u0048\u0079\u0070\u006E\u006F";  // => Hypno
        observe "Hello\x20World\x21";              // => Hello World!
        observe "Spiral: \u25CC";                  // => Spiral: ◌
    }
} Relax
```

Both forms consume a fixed number of hex digits. A truncated escape, a
non-hex digit, or a value that is not a valid Unicode scalar (such as a
surrogate code point `\uD800`–`\uDFFF`) is a **lexer error** — the program
does not even reach the parser.

### String Interpolation

Embed arbitrary expressions in string literals with `${...}`:

```hyp
Focus {
    entrance {
        induce name: string = "Luna";
        induce depth: number = 6;

        observe "Guest ${name} sinks to depth ${depth + 1}.";
        // => Guest Luna sinks to depth 7.

        // Escape with \${ to output a literal ${
        observe "costs \${price}";  // => costs ${price}
    }
} Relax
```

Interpolations may contain any expression — variables, arithmetic, function
calls, even nested strings. The result is converted to a string automatically.

### Null and Nullable Types

`null` is a first-class literal. A type only admits `null` when it is declared
nullable — with a `?` suffix or the hypnotic `lucid` modifier:

```hyp
Focus {
    entrance {
        induce maybe: number? = null;        // nullable via '?'
        induce dreamy: lucid string = null;  // nullable via 'lucid'

        // lucidFallback (??) supplies a default; the result is non-nullable
        induce certain: number = maybe lucidFallback 42;

        // null works as an entrain pattern
        induce state: string = entrain maybe {
            when null => "empty";
            otherwise => "filled";
        };
        observe state;
    }
} Relax
```

The type checker enforces null safety in both directions: assigning `null` (or
a nullable value) to a non-nullable type is a type error.

### Array Type Annotations

Use the `[]` suffix for typed arrays, including nested and nullable forms:

```hyp
Focus {
    tranceify Plan {
        title: string;
        tags: string[];
    }
    entrance {
        induce names: string[] = ["Alice", "Bob"];
        induce matrix: number[][] = [[1, 2], [3, 4]];
        induce optionalTags: string[]? = null;
    }
} Relax
```

## Output

### Observe (Output)

Use `observe` to output text:

```hyp
Focus {
    entrance {
        observe "Simple output";
        observe "Multi-line" + " " + "output";

        induce name: string = "HypnoScript";
        observe "Welcome to " + name;
    }
} Relax
```

## Control Structures

### If-Else

```hyp
Focus {
    entrance {
        induce age: number = 18;

        if (age >= 18) {
            observe "Of legal age";
        } else {
            observe "Minor";
        }

        // With else if
        induce score: number = 85;
        if (score >= 90) {
            observe "Excellent";
        } else if (score >= 80) {
            observe "Good";
        } else if (score >= 70) {
            observe "Satisfactory";
        } else {
            observe "Needs improvement";
        }
    }
} Relax
```

### DeepFocus (Conditional Block)

`deepFocus (condition) { ... }` is a standalone conditional statement — an
`if` with hypnotic emphasis, marking a block the program sinks into only when
the condition holds. There is no `else` branch:

```hyp
Focus {
    entrance {
        induce depth: number = 7;

        deepFocus (depth > 5) {
            observe "You are in deep trance now...";
        }
    }
} Relax
```

`deepFocus` may also follow an `if` condition as a block marker:

```hyp
Focus {
    entrance {
        induce n: number = 0;
        if (n <= 0) deepFocus {
            observe "Fully grounded.";
        }
    }
} Relax
```

The condition must be a boolean — the type checker reports
`DeepFocus condition must be boolean` otherwise.

### While Loop

```hyp
Focus {
    entrance {
        induce counter: number = 1;

        while (counter <= 5) {
            observe "Counter: " + counter;
            counter = counter + 1;
        }
    }
} Relax
```

### Loop Statement

`loop` can be used like a classic for-loop with a header `loop (initialization; condition; update)` or as an infinite loop without a header. The variant `pendulum ( ... )` is an alias for the same structure, but always requires a condition and is suitable for "back-and-forth" constructs.

```hyp
Focus {
    entrance {
        // Loop statement with counter
        loop (induce i: number = 1; i <= 10; i = i + 1) {
            observe "Iteration " + i;
        }

        // Loop statement over array with ArrayLength
        induce fruits: string[] = ["Apple", "Pear", "Cherry"];
        loop (induce i: number = 0; i < ArrayLength(fruits); i = i + 1) {
            observe "Fruit " + (i + 1) + ": " + ArrayGet(fruits, i);
        }

        // Pendulum always requires a header and behaves like loop (...)
        pendulum (induce phase: number = -2; phase <= 2; phase = phase + 1) {
            observe "Phase " + phase;
        }
    }
} Relax
```

### Labeled Loops

Label a loop to `snap` (break) or `sink` (continue) it from inside nested
loops:

```hyp
Focus {
    entrance {
        outer: loop (induce i: number = 0; i < 3; i = i + 1) {
            loop (induce j: number = 0; j < 3; j = j + 1) {
                if (i + j == 3) {
                    snap outer;   // break out of BOTH loops
                }
                if (j == 1) {
                    sink outer;   // continue the OUTER loop
                }
            }
        }
    }
} Relax
```

The keyword form `label outer: loop (...) { ... }` is also accepted.

### Timed Pauses (drift)

`drift(ms);` (synonym: `pauseReality(ms);`) pauses execution for the given
number of milliseconds:

```hyp
Focus {
    entrance {
        observe "Sinking deeper...";
        drift(500);
        observe "...and deeper.";
    }
} Relax
```

The `HYPNO_TIME_SCALE` environment variable scales every themed pause:
`HYPNO_TIME_SCALE=0` skips all pauses (useful for tests), `0.5` halves them.

## Functions

### Suggestion (Function Definition)

```hyp
Focus {
    // Define function
    suggestion greet(name: string) {
        observe "Hello, " + name + "!";
    }

    suggestion add(a: number, b: number): number {
        awaken a + b;
    }

    suggestion factorial(n: number): number {
        if (n <= 1) {
            awaken 1;
        } else {
            awaken n * factorial(n - 1);
        }
    }

    entrance {
        // Call functions
        greet("HypnoScript");

        induce result: number = add(5, 3);
        observe "5 + 3 = " + result;

        induce fact: number = factorial(5);
        observe "5! = " + fact;
    }
} Relax
```

### Imperative Suggestions

`imperativeSuggestion` declares a function in commanding style. Since 1.3.0
the two-word form `imperative suggestion` is accepted as well — both are
interchangeable, at the top level and inside sessions:

```hyp
Focus {
    // One-word form
    imperativeSuggestion obey(command: string) {
        observe "You will " + command + ".";
    }

    // Two-word form — identical meaning
    imperative suggestion comply(command: string) {
        observe "You must " + command + ".";
    }

    session Hypnotist {
        // Works as an instance method too
        expose imperative suggestion command(target: string) {
            observe "Sleep now, " + target + "!";
        }
    }

    entrance {
        obey("relax");
        comply("focus");
    }
} Relax
```

### Functions with Return Values

```hyp
Focus {
    suggestion calculateArea(width, height) {
        awaken width * height;
    }

    suggestion isEven(number) {
        awaken number % 2 == 0;
    }

    suggestion getMax(a, b) {
        if (a > b) {
            awaken a;
        } else {
            return b;
        }
    }

    entrance {
        induce area = calculateArea(10, 5);
        observe "Area: " + area;

        induce check = isEven(42);
        observe "42 is even: " + check;

        induce maximum = getMax(15, 8);
        observe "Maximum: " + maximum;
    }
} Relax;
```

## Arrays

### Array Operations

```hyp
Focus {
    entrance {
        // Create array
        induce numbers = [1, 2, 3, 4, 5];

        // Get elements
        induce first = ArrayGet(numbers, 0);
        observe "First element: " + first;

        // Set elements
        ArraySet(numbers, 2, 99);
        observe "After modification: " + numbers;

        // Array length
        induce length = ArrayLength(numbers);
        observe "Array length: " + length;

        // Iterate through array
        for (induce i = 0; i < Length(numbers); induce i = i + 1) {
            observe "Element " + i + ": " + ArrayGet(numbers, i);
        }
    }
} Relax;
```

### Array Functions

```hyp
Focus {
    entrance {
        induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];

        // Sort
        induce sorted = ArraySort(numbers);
        observe "Sorted: " + sorted;

        // Sum
        induce sum = ArraySum(numbers);
        observe "Sum: " + sum;

        // Average
        induce avg = AverageArray(numbers);
        observe "Average: " + avg;

        // Shuffle
        induce shuffled = ShuffleArray(numbers);
        observe "Shuffled: " + shuffled;
    }
} Relax;
```

## Records (Objects)

### Record Creation and Access

```hyp
Focus {
    entrance {
        // Create record
        induce person = {
            name: "John Doe",
            age: 30,
            city: "Berlin",
            hobbies: ["Programming", "Reading", "Sports"]
        };

        // Get properties
        observe "Name: " + person.name;
        observe "Age: " + person.age;
        observe "City: " + person.city;

        // Modify properties
        induce person.age = 31;
        observe "New age: " + person.age;

        // Nested records
        induce company = {
            name: "HypnoScript LLC",
            address: {
                street: "Example Street 123",
                city: "Berlin",
                zip: "10115"
            },
            employees: [
                {name: "Alice", role: "Developer"},
                {name: "Bob", role: "Designer"}
            ]
        };

        observe "Company: " + company.name;
        observe "Address: " + company.address.street;
        observe "First employee: " + company.employees[0].name;
    }
} Relax;
```

## Sessions

### Session Creation

```hyp
Focus {
    entrance {
        // Create session
        induce session = Session("MySession");

        // Set session variables
        SessionSet(session, "user", "Max");
        SessionSet(session, "level", 5);
        SessionSet(session, "preferences", {
            theme: "dark",
            language: "en"
        });

        // Get session variables
        induce user = SessionGet(session, "user");
        induce level = SessionGet(session, "level");
        induce prefs = SessionGet(session, "preferences");

        observe "User: " + user;
        observe "Level: " + level;
        observe "Theme: " + prefs.theme;
    }
} Relax;
```

## Tranceify

### Tranceify for Hypnotic Applications

```hyp
Focus {
    entrance {
        // Start tranceify session
        Tranceify("Relaxation") {
            observe "You are relaxing now...";
            observe "Breathe in deeply...";
            observe "And out...";
            observe "You feel calm and relaxed...";
        }

        // With parameters
        induce clientName = "Anna";
        Tranceify("Induction", clientName) {
            observe "Hello " + clientName + ", welcome to your session...";
            observe "You are in a safe space...";
            observe "You can completely relax...";
        }
    }
} Relax;
```

## Imports

### Importing Modules

```hyp
import "utils.hyp";
import "math.hyp" as MathUtils;

Focus {
    entrance {
        // Use functions from imported modules
        induce result = MathUtils.calculate(10, 5);
        observe "Result: " + result;
    }
} Relax;
```

## Assertions

### Assertions for Tests

```hyp
Focus {
    entrance {
        induce expected = 10;
        induce actual = 5 + 5;

        // Assertion - program stops on error
        assert actual == expected : "Expected 10, but got " + actual;

        observe "Test successful!";

        // More assertions
        induce name = "HypnoScript";
        assert Length(name) > 0 : "Name must not be empty";
        assert Length(name) <= 50 : "Name too long";

        observe "All tests passed!";
    }
} Relax;
```

## Comments

### Comments in HypnoScript

```hyp
Focus {
    // Single-line comment

    entrance {
        induce name = "HypnoScript"; // Inline comment

        /*
         * Multi-line comment
         * Can span multiple lines
         * Useful for longer explanations
         */

        observe "Hello " + name;
    }
} Relax;
```

## Operators

### Arithmetic Operators

```hyp
Focus {
    entrance {
        induce a = 10;
        induce b = 3;

        observe "Addition: " + (a + b);        // 13
        observe "Subtraction: " + (a - b);     // 7
        observe "Multiplication: " + (a * b);  // 30
        observe "Division: " + (a / b);        // 3.333...
        observe "Modulo: " + (a % b);          // 1
        observe "Power: " + (a ^ b);          // 1000
    }
} Relax;
```

### Comparison Operators

```hyp
Focus {
    entrance {
        induce x = 5;
        induce y = 10;

        observe "Equal: " + (x == y);         // false
        observe "Not equal: " + (x != y);       // true
        observe "Less than: " + (x < y);         // true
        observe "Greater than: " + (x > y);          // false
        observe "Less than or equal: " + (x <= y); // true
        observe "Greater than or equal: " + (x >= y);  // false
    }
} Relax;
```

### Logical Operators

```hyp
Focus {
    entrance {
        induce a = true;
        induce b = false;

        observe "AND: " + (a && b);            // false
        observe "OR: " + (a || b);           // true
        observe "NOT: " + (!a);              // false
        observe "XOR: " + (a ^ b);             // true
    }
} Relax;
```

## Best Practices

### Code Formatting

```hyp
Focus {
    // Define functions at the beginning
    suggestion calculateSum(a, b) {
        awaken a + b;
    }

    suggestion validateInput(value) {
        awaken value > 0 && value <= 100;
    }

    entrance {
        // Main logic in entrance block
        induce input = 42;

        if (validateInput(input)) {
            induce result = calculateSum(input, 10);
            observe "Result: " + result;
        } else {
            observe "Invalid input";
        }
    }
} Relax;
```

### Naming Conventions

- **Variables**: camelCase (`userName`, `totalCount`)
- **Functions**: camelCase (`calculateArea`, `validateInput`)
- **Constants**: UPPER_SNAKE_CASE (`MAX_RETRY_COUNT`)
- **Sessions**: PascalCase (`UserSession`, `GameState`)

### Error Handling

```hyp
Focus {
    entrance {
        induce input = "abc";

        // Type checking
        if (IsNumber(input)) {
            induce number = ToNumber(input);
            observe "Number: " + number;
        } else {
            observe "Error: Not a valid number";
        }

        // Array access check
        induce array = [1, 2, 3];
        induce index = 5;

        if (index >= 0 && index < Length(array)) {
            induce value = ArrayGet(array, index);
            observe "Value: " + value;
        } else {
            observe "Error: Index out of range";
        }
    }
} Relax;
```

## Next Steps

- [Variables and Data Types](./variables) - Detailed information about variables
- [Operators](./operators) - All available operators
- [Control Flow](./control-flow) - If, While, For and more
- [Functions](./functions) - Function definition and calling
- [Examples](../examples/basic-examples) - Practical examples

---

**Mastered the basics? Then learn more about [Variables and Data Types](./variables)!** 📚
