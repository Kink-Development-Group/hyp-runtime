---
title: Hello World
sidebar_position: 3
---

# Hello World

Your first HypnoScript program!

## Simple Hello World

Create a file `hello.hyp` with the following content:

```hyp
Focus {
    observe "Hello World!";
} Relax
```

Run the program:

```bash
hypnoscript hello.hyp
```

Output:

```
Hello World!
```

## With Entrance Block

The `entrance` block is executed at program start:

```hyp
Focus {
    entrance {
        observe "Welcome to HypnoScript!";
        observe "This is your first program.";
    }
} Relax
```

## With Variables

```hyp
Focus {
    entrance {
        induce name: string = "Developer";
        observe "Hello, " + name + "!";
        observe "Welcome to HypnoScript.";
    }
} Relax
```

## Interactive Hello World

```hyp
Focus {
    entrance {
        observe "=== HypnoScript Welcome Program ===";

        induce name: string = "World";
        induce version: number = 1.0;

        observe "Hello, " + name + "!";
        observe "HypnoScript Version " + version;
        observe "Ready for hypnotic programming!";
    }
} Relax
```

## With Functions

```hyp
Focus {
    suggestion greet(name: string) {
        observe "Hello, " + name + "!";
        observe "Nice to meet you.";
    }

    entrance {
        greet("HypnoScript Developer");
    }
} Relax
```

## Next Steps

- Learn about [Variables and Data Types](../language-reference/variables.md)
- Understand [Control Structures](../language-reference/control-flow.md)
- Discover [Builtin Functions](../builtins/overview.md)
