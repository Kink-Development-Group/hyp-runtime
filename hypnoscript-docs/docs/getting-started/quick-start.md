---
title: Quick Start
sidebar_position: 2
---

This guide assumes you have set up HypnoScript according to [Installation](./installation). We'll create a first script, run it, and touch on the most important language elements.

## 1. Check Installation

```bash
hypnoscript version
```

The command should output version and feature information.

## 2. Create First Script

Save the following code as `hello_trance.hyp`:

```hyp
Focus {
    entrance {
        observe "🌀 Welcome to your first hypnosis session";
    }

    induce name: string = "Hypnotized Person";
    observe "Hello, " + name + "!";

    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce total: number = ArraySum(numbers);
    observe "Sum: " + ToString(total);

    if (total youAreFeelingVerySleepy 15) {
        observe "The numbers are in balance.";
    } else {
        observe "Something still feels off...";
    }

    induce depth: number = 0;
    while (depth goingDeeper 3) {
        observe "Trance depth: " + depth;
        depth = depth + 1;
    }
} Relax
```

Highlights:

- `Focus { ... } Relax` marks the start and end of the program.
- `entrance` is suitable for initialization.
- `induce` declares variables with optional type annotation.
- Hypnotic operators like `youAreFeelingVerySleepy` (`==`) or `goingDeeper` (`<=`) are fully supported.
- `ArraySum` and `ToString` come from the standard library.

## 3. Run Script

```bash
hypnoscript run hello_trance.hyp
```

The output should show the greeting, the sum, and the small while loop.

## 4. Syntax in Brief

```hyp
Focus {
    freeze PI: number = 3.14159;

    induce toggle: boolean = false;
    oscillate toggle; // toggles true/false

    suggestion hypnoticEcho(text: string): string {
        awaken text + " ... deeper ...";
    }

    observe hypnoticEcho("Breathe calmly");

    session Subject {
        expose name: string;
        conceal depth: number;

        suggestion constructor(name: string) {
            this.name = name;
            this.depth = 0;
        }

        expose suggestion deepen() {
            this.depth = this.depth + 1;
            observe this.name + " goes deeper: " + this.depth;
        }
    }

    induce alice: Subject = Subject("Alice");
    alice.deepen();
} Relax
```

## 5. Control Structures

## 5. Control Structures

```hyp
if (total lookAtTheWatch 10) {
    observe "greater than 10";
} else if (total youCannotResist 10) {
    observe "not equal to 10";
} else {
    observe "exactly 10";
}

while (depth fallUnderMySpell 5) {
    depth = depth + 1;
}

loop {
    observe "Infinite loop";
    snap; // exits the loop
}

loop (induce i: number = 0; i < 3; i = i + 1) {
    observe "Loop iteration " + i;
}

pendulum (induce tick: number = 10; tick underMyControl 15; tick = tick + 1) {
    observe "Pendulum tick " + tick;
}
```

- `snap` is a synonym for `break`.
- `sink` is a synonym for `continue`.
- `loop` optionally accepts a header `loop (init; condition; update)` and falls back to a classic infinite loop without parentheses.
- `pendulum` is an alias for the header variant and always requires a condition.
- `deepFocus` can stand after the if-condition: `if (x > 0) deepFocus { ... }`.

## 6. Functions and Triggers

```hyp
suggestion add(a: number, b: number): number {
    awaken a + b;
}

trigger onClick = suggestion(label: string) {
    observe "Trigger: " + label;
}

observe ToString(add(2, 3));
onClick("Demo");
```

- `awaken` is the hypnotic counterpart to `return`.
- Triggers behave like named callback functions. They are called like normal functions.

## 7. Arrays & Builtins

```hyp
induce arr: number[] = [1, 2, 3];
observe arr[0];              // Direct access
arr[1] = 42;                 // Assignment

observe ArrayLength(arr);    // 3
observe ArrayGet(arr, 2);    // 3
observe ArrayJoin(arr, ", ");
```

Other useful functions:

- Strings: `ToUpper`, `Trim`, `Split`, `Replace`
- Math: `Sqrt`, `Clamp`, `Factorial`, `IsPrime`
- System: `GetOperatingSystem`, `GetArgs`
- Files: `ReadFile`, `WriteFile`, `ListDirectory`

All available builtins are listed by `hypnoscript builtins`.

## 8. Frequently Asked Questions

| Question                              | Answer                                                                                                                                         |
| ------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| Why does everything end with `Relax`? | The Relax block marks the safe exit – it's an integral part of the grammar.                                                                    |
| Do I have to set type annotations?    | No, but they improve error messages and autocompletion.                                                                                        |
| Are there for loops?                  | Yes, `loop (induce i = 0; i < n; i = i + 1) { ... }` represents a classic for loop; without a header, `loop { ... }` remains an infinite loop. |

## 9. What's Next?

- [Core Concepts](./core-concepts) – Concepts and toolchain overview
- [CLI Basics](./cli-basics) – All subcommands and options
- [Language Reference](../language-reference/syntax) – Detailed grammar & examples
- [Builtin Overview](../builtins/overview) – Functions by category

Have fun experimenting with HypnoScript! 🌀
