---
sidebar_position: 1
---

# Examples: Utility Functions

This page shows practical examples for using utility functions in HypnoScript. The examples are commented and can be used directly or adapted.

## Dynamic Type Conversion and Validation

```hyp
Focus {
    entrance {
        induce input = "42";
        induce n = ToNumber(input);
        if (IsNumber(n)) {
            observe "Entered number: " + n;
        } else {
            observe "Invalid input!";
        }
    }
} Relax;
```

## Random Selection and Shuffling

```hyp
Focus {
    entrance {
        induce namen = ["Anna", "Ben", "Carla", "Dieter"];
        induce gewinner = Sample(namen, 1);
        observe "Winner: " + gewinner;
        induce gemischt = Shuffle(namen);
        observe "Random order: " + gemischt;
    }
} Relax;
```

## Time Measurement and Sleep

```hyp
Focus {
    entrance {
        induce start = Timestamp();
        Sleep(500); // 0,5 seconds wait
        induce ende = Timestamp();
        observe "Duration: " + (ende - start) + " seconds";
    }
} Relax;
```

## Array Transformations

```hyp
Focus {
    entrance {
        induce zahlen = [1,2,3,4,5,2,3,4];
        induce unique = Unique(zahlen);
        observe "Without duplicates: " + unique;
        induce sortiert = Sort(unique);
        observe "Sorted: " + sortiert;
        induce gepaart = Zip(unique, ["a","b","c","d","e"]);
        observe "Paired: " + gepaart;
    }
} Relax;
```

## Error Handling with Try

```hyp
Focus {
    suggestion safeDivide(a, b) {
        awaken Try(a / b, "Error: Division by zero");
    }
    entrance {
        observe safeDivide(10, 2); // 5
        observe safeDivide(10, 0); // "Error: Division by zero"
    }
} Relax;
```

## JSON Parsing and Generation

```hyp
Focus {
    entrance {
        induce jsonString = '{"name": "Max", "age": 30}';
        induce obj = ParseJSON(jsonString);
        observe "Name: " + obj.name;
        observe "Age: " + obj.age;

        induce arr = [1,2,3];
        induce jsonArr = StringifyJSON(arr);
        observe "JSON array: " + jsonArr;
    }
} Relax;
```

## Range und Repeat

```hyp
Focus {
    entrance {
        induce r = Range(1, 5);
        observe "Range: " + r; // [1,2,3,4,5]
        induce rep = Repeat("A", 3);
        observe "Repeat: " + rep; // ["A","A","A"]
    }
} Relax;
```

## Combined Utility Workflows

```hyp
Focus {
    entrance {
        // Validate and process input
        induce input = "15";
        induce n = ToNumber(input);
        if (IsNumber(n) && n > 10) {
            observe "Input is a number > 10: " + n;
        } else {
            observe "Invalid or too small number!";
        }

        // Random selection from range
        induce zahlen = Range(1, 100);
        induce zufall = Sample(zahlen, 5);
        observe "5 random numbers: " + zufall;

        // Array Transformations kombinieren
        induce arr = [1,2,2,3,4,4,5];
        induce clean = Sort(Unique(arr));
        observe "Sorted & unique: " + clean;
    }
} Relax;
```

---

**See also:**

- [Utility Functions Reference](../builtins/utility-functions)
- [System Functions Examples](./system-examples)

