---
sidebar_position: 3
---

# Type System

HypnoScript relies on a **static type system**. Every variable, every field, and every return value has a clearly defined type that is checked at compile time. This allows many errors to be caught early and avoids runtime surprises.

## Overview of Base Types

| Type       | Description                                              | Example Code                                |
| ---------- | -------------------------------------------------------- | ------------------------------------------- |
| `number`   | Double-precision floating-point number                   | `induce temperature: number = 21.5;`        |
| `string`   | UTF-8 text, fully supports Unicode                       | `induce greeting: string = "Hello";`        |
| `boolean`  | Truth value `true` or `false`                            | `induce active: boolean = true;`            |
| `trance`   | Hypnotic state, used for sessions and suggestions        | `induce state: trance = induceTrance();`    |
| `array`    | Ordered list with uniform element type                   | `induce numbers: number[] = [1, 2, 3];`     |
| `record`   | Named set of fields with their own types                 | `induce client: Client = { name, age };`    |
| `object`   | Dynamic object, typically used for external integrations | `induce data: object = loadJson();`         |
| `function` | Function reference with parameters and return value      | `induce analyzeUnit = suggestion(...)`      |
| `session`  | Running HypnoScript session                              | `induce meeting: session = beginSession();` |
| `unknown`  | Placeholder when the type could not yet be determined    | Used internally by the type checker         |

> 💡 **Note:** `record`, `function`, and `array` are **composite types**. They carry additional information (field names, parameter list, element type) that is considered during type checking.

See also [Variables and Data Types](./variables.md) for basics on declaring variables.

## Type Annotation and Inference

You can specify types explicitly or let the compiler do the work:

```hyp
// Explicit annotation
induce counter: number = 0;

// Type inference by the compiler
induce greeting = "Welcome"; // inferred type: string

// Explicit parameter and return types for functions
suggestion double(value: number): number {
    awaken value * 2;
}
```

The compiler always attempts to infer the most specific type. If it cannot make a clear statement, it internally uses `unknown` and reports a type warning or error.

## Composite Types

### Arrays

Arrays are always homogeneous. The element type is specified in square brackets after the array name:

```hyp
induce names: string[] = ["Sam", "Alex", "Riley"];

induce measurements: number[];
measurements = collectValues();
```

For operations on arrays, the type checker ensures that only matching elements are inserted.

### Records

Records combine multiple fields into a structured type:

```hyp
induce Client = record {
    name: string,
    age: number,
    active: boolean
};

induce client: Client = {
    name: "Mira",
    age: 29,
    active: true
};
```

The structure of a record is **structural** – two records are compatible if they have the same field names and types.

### Functions

Functions carry a complete signature type, consisting of parameter list and return value:

```hyp
suggestion hypnoticGreeting(name: string, duration: number): string {
    observe name;
    observe duration;
    awaken "Welcome back";
}
```

Function types can be stored and passed like any other value form:

```hyp
induce greetingFunction: (string, number) -> string = hypnoticGreeting;
```

## Compatibility Rules

The type checker uses strict but pragmatic compatibility rules:

- **Primitive types** must match exactly (`number` is not automatically compatible with `string`).
- **Arrays** are compatible if their element types are compatible.
- **Records** compare field count, field names, and field types.
- **Functions** require identical parameter count and compatible parameter and return types.
- **Sessions** and **trance states** are their own types and are not implicitly converted to other types.

If two types are not compatible, the compiler reports an error with information about the expected and the actually found type.

## Working with `unknown`

`unknown` serves as a fallback when the type cannot be determined unambiguously – for example, with dynamic data sources. The goal should be to convert `unknown` into a concrete type as early as possible:

```hyp
induce data: unknown = loadExternal();

if (isRecord(data)) {
    induce structure = cast<Client>(data);
    observe structure.name;
} else {
    warn "External data could not be interpreted.";
}
```

## Further Resources

- [Control Flow](./control-flow.md) – Type-safe decision and loop constructs
- [Functions](./functions.md) – Signatures, return values, and inline functions
- [Records](./records.md) – Detailed introduction to structured data

With a clear understanding of the type system, you can write HypnoScript programs that are both hypnotic and robust.```
