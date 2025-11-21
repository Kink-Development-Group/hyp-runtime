---
sidebar_position: 4
---

# Control Flow

HypnoScript provides various control structures for conditional execution and loops.

## If-Else Statements

### Simple If Statement

```hyp
if (condition) {
    // Code is executed if condition is true
}
```

### If-Else Statement

```hyp
if (condition) {
    // Code when condition is true
} else {
    // Code when condition is false
}
```

### If-Else If-Else Statement

```hyp
if (condition1) {
    // Code when condition1 is true
} else if (condition2) {
    // Code when condition2 is true
} else {
    // Code when all conditions are false
}
```

### Examples

```hyp
Focus {
    entrance {
        induce age = 18;

        if (age >= 18) {
            observe "Of legal age";
        } else {
            observe "Minor";
        }

        induce score = 85;
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
} Relax;
```

## While Loops

### Syntax

```hyp
while (condition) {
    // Code is repeated while condition is true
}
```

### Examples

```hyp
Focus {
    entrance {
        // Simple while loop
        induce counter = 1;
        while (counter <= 5) {
            observe "Counter: " + counter;
            induce counter = counter + 1;
        }

        // While loop with array
        induce numbers = [1, 2, 3, 4, 5];
        induce index = 0;
        while (index < ArrayLength(numbers)) {
            observe "Number " + (index + 1) + ": " + ArrayGet(numbers, index);
            induce index = index + 1;
        }
    }
} Relax;
```

## For Loops

### Syntax

```hyp
for (initialization; condition; increment) {
    // Code is repeated
}
```

### Examples

```hyp
Focus {
    entrance {
        // Standard for loop
        for (induce i = 1; i <= 10; induce i = i + 1) {
            observe "Iteration " + i;
        }

        // For loop over array
        induce fruits = ["Apple", "Banana", "Orange"];
        for (induce i = 0; i < ArrayLength(fruits); induce i = i + 1) {
            observe "Fruit " + (i + 1) + ": " + ArrayGet(fruits, i);
        }

        // Count backwards
        for (induce i = 10; i >= 1; induce i = i - 1) {
            observe "Countdown: " + i;
        }
    }
} Relax;
```

## Nested Control Structures

```hyp
Focus {
    entrance {
        induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
            induce number = ArrayGet(numbers, i);

            if (number % 2 == 0) {
                observe number + " is even";
            } else {
                observe number + " is odd";
            }

            if (number < 5) {
                observe "  - Small number";
            } else if (number < 8) {
                observe "  - Medium number";
            } else {
                observe "  - Large number";
            }
        }
    }
} Relax;
```

## Break and Continue

### Break

Exits the current loop immediately:

```hyp
Focus {
    entrance {
        for (induce i = 1; i <= 10; induce i = i + 1) {
            if (i == 5) {
                break; // Loop terminates at i=5
            }
            observe "Number: " + i;
        }
        observe "Loop terminated";
    }
} Relax;
```

### Continue

Skips the current loop iteration:

```hyp
Focus {
    entrance {
        for (induce i = 1; i <= 10; induce i = i + 1) {
            if (i % 2 == 0) {
                continue; // Even numbers are skipped
            }
            observe "Odd number: " + i;
        }
    }
} Relax;
```

## Best Practices

### Clear Conditions

```hyp
// Good
if (age >= 18 && score >= 70) {
    observe "Admitted";
}

// Bad
if (age >= 18 && score >= 70 == true) {
    observe "Admitted";
}
```

### Efficient Loops

```hyp
// Good - calculate array length once
induce length = ArrayLength(numbers);
for (induce i = 0; i < length; induce i = i + 1) {
    // Code
}

// Bad - calculate array length on each iteration
for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
    // Code
}
```

### Avoiding Infinite Loops

```hyp
// Safe - with break condition
induce counter = 0;
while (true) {
    induce counter = counter + 1;
    if (counter > 100) {
        break;
    }
    // Code
}
```

## Examples of Complex Control Structures

### Number Guessing Game

```hyp
Focus {
    entrance {
        induce targetNumber = 42;
        induce attempts = 0;
        induce maxAttempts = 10;

        while (attempts < maxAttempts) {
            induce attempts = attempts + 1;
            induce guessNumber = 25 + attempts * 2; // Simplified input

            if (guessNumber == targetNumber) {
                observe "Won! The number was " + targetNumber;
                observe "Attempts: " + attempts;
                break;
            } else if (guessNumber < targetNumber) {
                observe "Too low! Attempt " + attempts;
            } else {
                observe "Too high! Attempt " + attempts;
            }
        }

        if (attempts >= maxAttempts) {
            observe "Lost! The number was " + targetNumber;
        }
    }
} Relax;
```

### Array Processing with Conditions

```hyp
Focus {
    entrance {
        induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        induce evenSum = 0;
        induce oddCount = 0;

        for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
            induce number = ArrayGet(numbers, i);

            if (number % 2 == 0) {
                induce evenSum = evenSum + number;
            } else {
                induce oddCount = oddCount + 1;
            }
        }

        observe "Sum of even numbers: " + evenSum;
        observe "Count of odd numbers: " + oddCount;
    }
} Relax;
```

## Next Steps

- [Functions](./functions) - Function definition and calling
- [Sessions](./sessions) - Session management
- [Tranceify](./tranceify) - Hypnotic applications
- [Assertions](./assertions) - Test assertions

---

**Mastered control structures? Then learn about [Functions](./functions)!** 🔧
