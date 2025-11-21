---
sidebar_position: 2
---

# Variables and Data Types

:::tip Complete Reference
See [Keywords Reference](./_keywords-reference#variables-and-constants) for the complete documentation of all variable keywords (induce, implant, freeze, anchor, oscillate).
:::

In HypnoScript, variables are declared with the keyword `induce`. The language supports static type checking with various primitive and complex data types.

## Declaring Variables

```hyp
induce name = "HypnoScript";
induce number = 42;
induce pi = 3.1415;
induce active = true;
induce list = [1, 2, 3];
induce person = { name: "Max", age: 30 };
```

## Supported Data Types

| Type    | Example                    | Description                 |
| ------- | -------------------------- | --------------------------- |
| String  | "Hello World"              | String of characters        |
| Integer | 42                         | Whole number                |
| Double  | 3.1415                     | Floating-point number       |
| Boolean | true, false                | Truth value                 |
| Array   | [1, 2, 3]                  | List of values              |
| Record  | \{ name: "Max", age: 30 \} | Object with key/value pairs |
| Null    | null                       | Empty value                 |

## Type Conversion

Many builtins support automatic type conversion. For explicit conversion:

```hyp
induce number = "42";
induce asNumber = ToNumber(number); // 42
induce asString = ToString(asNumber); // "42"
```

## Variable Visibility

- Variables are visible in the current block and in sub-blocks.
- Function parameters are only visible within the function.

## Constants

Constants are treated like variables, but by convention written in uppercase:

```hyp
induce MAX_COUNT = 100;
```

## Best Practices

- Use descriptive names (e.g. `userName`, `maxValue`)
- Use arrays and records for structured data
- Always initialize variables with a value

## Examples

```hyp
Focus {
    entrance {
        induce greeting = "Hello";
        induce count = 5;
        induce values = [1, 2, 3, 4, 5];
        induce user = { name: "Anna", age: 28 };
        observe greeting + ", " + user.name + "!";
        observe "Values: " + values;
    }
} Relax;
```
