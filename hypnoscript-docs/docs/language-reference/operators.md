# Operatoren

HypnoScript unterstützt Standard-Operatoren sowie hypnotische Synonyme für vergleichende und logische Operatoren. Alle Operatoren sind in der Rust-Implementierung vollständig typsicher.

## Arithmetische Operatoren

| Operator | Bedeutung      | Typ-Anforderung | Beispiel | Ergebnis |
| -------- | -------------- | --------------- | -------- | -------- |
| +        | Addition       | number          | 2 + 3    | 5        |
| -        | Subtraktion    | number          | 5 - 2    | 3        |
| \*       | Multiplikation | number          | 4 \* 2   | 8        |
| /        | Division       | number          | 8 / 2    | 4        |
| %        | Modulo (Rest)  | number          | 7 % 3    | 1        |

**String-Konkatenation:** Der `+` Operator funktioniert auch für Strings:

```hyp
induce text: string = "Hallo " + "Welt";  // "Hallo Welt"
induce mixed: string = "Zahl: " + 42;     // "Zahl: 42"
```

> ⚠️ **Achtung:** Sobald einer der Operanden ein String ist, werden alle anderen Werte implizit in Strings umgewandelt (intern via `to_string()`). Dadurch entstehen z. B. Ergebnisse wie `null + "text" -> "nulltext"` oder `42 + "px" -> "42px"`. Wenn du striktere Typkontrollen erwartest, konvertiere Werte explizit oder prüfe den Typ vor der Verwendung von `+`.

## Vergleichsoperatoren

### Standard-Operatoren (Vergleich)

| Operator | Bedeutung      | Beispiel | Ergebnis |
| -------- | -------------- | -------- | -------- |
| ==       | Gleich         | 3 == 3   | true     |
| !=       | Ungleich       | 3 != 4   | true     |
| >        | Größer         | 5 > 2    | true     |
| <        | Kleiner        | 2 < 5    | true     |
| >=       | Größer gleich  | 3 >= 2   | true     |
| <=       | Kleiner gleich | 2 <= 2   | true     |

### Hypnotische Synonyme (Vergleich)

HypnoScript bietet hypnotische Synonyme für alle Vergleichsoperatoren:

| Hypnotic Synonym        | Standard | Meaning               | Status         |
| ----------------------- | -------- | --------------------- | -------------- |
| youAreFeelingVerySleepy | ==       | Equal                 | ✅ Recommended |
| youCannotResist         | !=       | Not equal             | ✅ Recommended |
| lookAtTheWatch          | >        | Greater than          | ✅ Recommended |
| fallUnderMySpell        | <        | Less than             | ✅ Recommended |
| yourEyesAreGettingHeavy | >=       | Greater than or equal | ✅ Recommended |
| goingDeeper             | <=       | Less than or equal    | ✅ Recommended |

**Legacy Operators** (deprecated, but supported):

| Hypnotic Synonym | Standard | Note                                     |
| ---------------- | -------- | ---------------------------------------- |
| notSoDeep        | !=       | ⚠️ Use `youCannotResist` instead         |
| deeplyGreater    | >=       | ⚠️ Use `yourEyesAreGettingHeavy` instead |
| deeplyLess       | <=       | ⚠️ Use `goingDeeper` instead             |

## Logical Operators

### Standard Operators (Logic)

| Operator | Meaning | Example         | Result |
| -------- | ------- | --------------- | ------ |
| &&       | And     | true && false   | false  |
| \|\|     | Or      | true \|\| false | true   |
| !        | Not     | !true           | false  |

### Hypnotic Synonyms (Logic)

| Hypnotic Synonym   | Standard | Meaning     |
| ------------------ | -------- | ----------- |
| underMyControl     | &&       | Logical AND |
| resistanceIsFutile | \|\|     | Logical OR  |

**Note:** There is no hypnotic synonym for the `!` (Not) operator.

## Operator Precedence

From highest to lowest precedence:

1. **Unary Operators:** `!`, `-` (negative)
2. **Multiplicative:** `*`, `/`, `%`
3. **Additive:** `+`, `-`
4. **Comparison:** `<`, `<=`, `>`, `>=` (and hypnotic synonyms)
5. **Equality:** `==`, `!=` (and hypnotic synonyms)
6. **Logical AND:** `&&` (or `underMyControl`)
7. **Logical OR:** `||` (or `resistanceIsFutile`)

Use parentheses `( )` for explicit grouping.

## Array Access and Assignment

Arrays are indexed with square brackets `[ ]` (0-based):

```hyp
induce arr: number[] = [10, 20, 30];
observe arr[0];       // Output: 10
observe arr[2];       // Output: 30

arr[1] = 42;          // Assignment
observe arr[1];       // Output: 42
```

For advanced array operations see [Array Builtin Functions](../builtins/array-functions).

## Assignment Operator

The simple assignment operator `=` is used for assignments:

```hyp
induce x: number = 5;
x = x + 1;  // 6
x = 10;     // Reassignment
```

**Important:** Compound assignment operators (`+=`, `-=`, `*=`, etc.) are **not implemented**.

Use instead:

````hyp
// WRONG: x += 5;
// CORRECT:
x = x + 5;

## Examples

### Standard Operators

```hyp
Focus {
    entrance {
        induce a: number = 10;
        induce b: number = 3;

        observe "a + b = " + (a + b);    // 13
        observe "a - b = " + (a - b);    // 7
        observe "a * b = " + (a * b);    // 30
        observe "a / b = " + (a / b);    // 3.333...
        observe "a % b = " + (a % b);    // 1

        observe "a == b: " + (a == b);   // false
        observe "a > b: " + (a > b);     // true
        observe "a <= 10: " + (a <= 10); // true
    }
} Relax
````

### Hypnotic Synonyms

```hyp
Focus {
    entrance {
        induce x: number = 10;
        induce y: number = 10;

        if (x youAreFeelingVerySleepy y) {
            observe "x equals y!";
        }

        if (x lookAtTheWatch 5 underMyControl y yourEyesAreGettingHeavy 8) {
            observe "Both conditions are true!";
        }

        if (x fallUnderMySpell 20 resistanceIsFutile y youAreFeelingVerySleepy 10) {
            observe "At least one condition is true!";
        }
    }
} Relax
```

### Array Operations

```hyp
Focus {
    entrance {
        induce numbers: number[] = [1, 2, 3, 4, 5];

        observe "First element: " + numbers[0];
        observe "Array length: " + ArrayLength(numbers);

        numbers[2] = 99;
        observe "Modified element: " + numbers[2];
    }
} Relax
```

### Operator Combinations

```hyp
Focus {
    entrance {
        induce x: number = 10;
        induce y: number = 20;
        induce z: number = 5;

        // Complex expressions with precedence
        induce result1: number = x + y * z;     // 110 (multiplication first)
        induce result2: number = (x + y) * z;   // 150 (parentheses first)

        observe "result1 = " + result1;
        observe "result2 = " + result2;

        // Combining logical operators
        if (x lookAtTheWatch 5 underMyControl y lookAtTheWatch 15) {
            observe "x > 5 AND y > 15";
        }

        if (x fallUnderMySpell 5 resistanceIsFutile y yourEyesAreGettingHeavy 20) {
            observe "x < 5 OR y >= 20";
        }

        // Negation
        induce isActive: boolean = true;
        if (!isActive) {
            observe "Not active";
        } else {
            observe "Active";
        }
    }
} Relax
```

## Best Practices

1. **Use parentheses** for complex expressions for better readability
2. **Use hypnotic operators** consistently for thematic consistency
3. **Avoid legacy operators** (`notSoDeep`, `deeplyGreater`, `deeplyLess`)
4. **Type consistency** matters: Only compare values of the same type
5. **Explicit conversion** when needed with builtin functions (`ToInt`, `ToDouble`, `ToString`)

## See Also

- [Variables](./variables) - Variable declaration and assignment
- [Control Flow](./control-flow) - if, while, loop
- [Builtin Functions](../builtins/overview) - Available standard functions
- [Syntax](./syntax) - Complete language syntax
