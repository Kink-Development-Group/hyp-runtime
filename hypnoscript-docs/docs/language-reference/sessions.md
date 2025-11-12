---
title: Sessions
---

# Sessions

Sessions are HypnoScript's object-oriented building blocks. They bundle related state and behaviour while keeping the hypnotic syntax you already know. This page explains how to declare sessions, control visibility, wire constructors, and work with static members.

## Declaring a session

A session groups fields and methods inside a dedicated block:

```hypnoscript
session Account {
    conceal balance: number = 0;

    expose suggestion constructor(initialBalance: number) {
        this.balance = initialBalance;
    }

    expose suggestion deposit(amount: number) {
        this.balance = this.balance + amount;
    }

    conceal suggestion snapshot(): number {
        awaken this.balance;
    }
}
```

Key points:

- `session Name { ... }` declares the type.
- Fields require an explicit visibility keyword (`expose` for public, `conceal` for private). Initialisers are optional.
- Methods use `suggestion`, `imperativeSuggestion`, or `dominant suggestion` depending on the style you prefer. The parser treats `imperativeSuggestion` as an instance method and `dominant suggestion` as static.
- The optional `constructor` keyword after `suggestion` marks a constructor. Constructors cannot be static and always return an instance of the surrounding session. The type checker enforces those rules.

## Field visibility

Visibility determines where a member can be accessed:

- `expose` marks a field or method as public. Public members can be used from any script.
- `conceal` restricts access to the defining session. Both the interpreter and the type checker reject external reads, writes, or calls.

Attempting to reach a concealed member outside its session triggers a type error:

```hypnoscript
Focus {
    session Vault {
        conceal pin: number = 1234;
        expose suggestion reveal(): number {
            awaken this.pin;
        }
    }

    induce vault = Vault();
    induce leak = vault.pin; // Field 'pin' of session 'Vault' is not visible here
} Relax
```

## Methods and constructors

Instance methods rely on `this`, which the runtime binds automatically when you call them on an instance. Mark a constructor with `suggestion constructor(...)` and optionally accept parameters:

```hypnoscript
session Timeline {
    conceal events: number = 0;

    expose suggestion constructor(initial: number) {
        this.events = initial;
    }

    expose suggestion record(amount: number) {
        this.events = this.events + amount;
    }

    conceal suggestion current(): number {
        awaken this.events;
    }
}

Focus {
    induce timeline = Timeline(5);
    timeline.record(3);
    induce count = timeline.current();
} Relax
```

The type checker validates constructor arity and ensures returns inside methods agree with the declared return type.

## Static members

Use the `dominant` modifier to mark members as static. Static fields belong to the session itself, not to individual instances, and must be accessed through the session name:

```hypnoscript
session Config {
    dominant expose version: string = "1.0";

    dominant suggestion setVersion(next: string) {
        Config.version = next;
    }
}

Focus {
    Config.setVersion("2.1");
    induce activeVersion: string = Config.version;
} Relax
```

Static rules enforced by the compiler:

- Assign static fields via the session (`Config.version = ...`), not through an instance.
- Do not call instance methods on the session type (`Config.update()` fails unless `update` is static).
- Constructors are always instance members and cannot be declared `dominant`.

## Summary of type checker guarantees

The extended type checker performs the following validations for sessions:

- Detects duplicate fields, methods, or constructors inside a session.
- Ensures private members stay hidden outside the declaring session.
- Verifies constructors are unique, non-static, and called with the correct number of arguments.
- Differentiates static and instance members for both access and assignment.
- Catches `this` usage in static methods and invalid member assignments (for example, writing to methods).

## Further reading

Head over to [Basic Examples](/examples/basic-examples) for end-to-end snippets that combine constructors, static members, and visibility. The interpreter and runtime design notes in `/docs/reference/interpreter.md` highlight how the execution engine enforces the same constraints at runtime.
