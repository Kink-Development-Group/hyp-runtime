---
sidebar_position: 6
---

# Triggers – Event-Hooks & Callbacks

Triggers are a powerful tool in HypnoScript for defining event hooks, callbacks, and delayed actions. They combine the flexibility of first-class functions with the declarative semantics of event handlers.

## What are Triggers?

A `trigger` is a named callback function that is declared at **top-level** (outside of functions, sessions, or blocks). Triggers are ideal for:

- 🎯 **Event Handlers** – Reacting to user interactions or system events
- 🧹 **Cleanup Actions** – Cleanup operations after program termination
- ⏰ **Delayed Execution** – Callbacks for asynchronous operations
- 🔄 **State Management** – State change handlers in complex sessions

## Basic Syntax

```hyp
trigger triggerName = suggestion(parameter1: type1, parameter2: type2): returnType {
    // Trigger code
};
```

### Key Properties

| Property           | Description                               |
| ------------------ | ----------------------------------------- |
| **Scope**          | Top-level only (program or module level)  |
| **Declaration**    | `trigger name = suggestion(...) { ... };` |
| **First-Class**    | Can be passed as parameters and stored    |
| **Event-Oriented** | Ideal for event handlers and callbacks    |

> ✅ The Rust parser now strictly enforces this rule: Any attempt to declare a `trigger` inside a block, function, or session results in the error _"Triggers can only be declared at the top level"_.

## Simple Examples

### Cleanup Trigger

Triggers are perfect for cleanup actions at program end:

```hyp
Focus {
    induce resourceHandle: number = 42;

    trigger onCleanup = suggestion() {
        observe "Cleaning up resource " + resourceHandle;
        // Release resources
    };

    entrance {
        observe "Program started";
    }

    finale {
        onCleanup();
        observe "Program ended";
    }
} Relax
```

### Event Handler

Triggers as classic event handlers:

```hyp
Focus {
    trigger onClick = suggestion(buttonId: string) {
        observe "Button clicked: " + buttonId;
    };

    trigger onSubmit = suggestion(formData: string) {
        observe "Form submitted: " + formData;
    };

    entrance {
        onClick("btnSave");
        onSubmit("user@example.com");
    }
} Relax
```

## Parameterized Triggers

Triggers can accept arbitrary parameters:

```hyp
Focus {
    trigger onError = suggestion(errorCode: number, message: string) {
        observe "Error " + errorCode + ": " + message;
    };

    trigger onSuccess = suggestion(data: string): boolean {
        observe "Success: " + data;
        awaken true;
    };

    entrance {
        onError(404, "Not found");
        induce result: boolean = onSuccess("Data loaded");
    }
} Relax
```

## Integration with DeepMind/AuraAsync

Triggers shine in combination with builtin functions:

### Repeated Execution

```hyp
Focus {
    induce counter: number = 0;

    trigger onTick = suggestion() {
        counter = counter + 1;
        observe "Tick " + counter;
    };

    entrance {
        // Execute trigger 5 times at 1000ms intervals
        repeatAction(onTick, 5, 1000);
        observe "Final count: " + counter;
    }
} Relax
```

### Delayed Execution

```hyp
Focus {
    trigger afterDelay = suggestion(message: string) {
        observe "Delayed message: " + message;
    };

    entrance {
        observe "Starting delay...";
        delayedSuggestion(afterDelay, 2000, "Hello after 2 seconds!");
        observe "Delay started";
    }
} Relax
```

## Triggers in Sessions

While triggers can only be declared at top-level, they combine perfectly with sessions:

```hyp
// Trigger as top-level declaration
trigger onSecondElapsed = suggestion(timer: HypnoTimer) {
    timer.elapsedSeconds = timer.elapsedSeconds + 1;
    observe "Elapsed time: " + timer.elapsedSeconds + "s";
};

session HypnoTimer {
    expose elapsedSeconds: number;
    conceal timerCallback: suggestion;

    suggestion constructor() {
        this.elapsedSeconds = 0;
        this.timerCallback = onSecondElapsed;
    }

    suggestion start() {
        // Call trigger every second
        repeatAction(this.timerCallback, 60, 1000);
    }

    suggestion getElapsed(): number {
        awaken this.elapsedSeconds;
    }
}

Focus {
    entrance {
        induce timer = HypnoTimer();
        timer.start();
    }
} Relax
```

## Difference from Regular Functions

| Aspect          | `suggestion`                            | `trigger`                                   |
| --------------- | --------------------------------------- | ------------------------------------------- |
| **Declaration** | `suggestion name(params): type { ... }` | `trigger name = suggestion(params) { ... }` |
| **Scope**       | Block-level (local/global)              | **Top-level only**                          |
| **Semantics**   | Reusable function                       | Event handler/Callback                      |
| **Usage**       | General logic                           | Event-driven                                |
| **Convention**  | Algorithms, calculations                | Reactions, cleanup, events                  |

## Local Callbacks in Sessions

For callbacks within sessions or functions, use **anonymous suggestion expressions**:

```hyp
session TaskManager {
    conceal taskCallback: suggestion;

    suggestion constructor() {
        // Anonymous suggestion expression as local callback
        this.taskCallback = suggestion() {
            observe "Task executed!";
        };
    }

    suggestion executeTask() {
        this.taskCallback();
    }
}
```

## Best Practices

### ✅ Do's

```hyp
// ✓ Name triggers with 'on' prefix for clarity
trigger onAwaken = suggestion() { ... };
trigger onError = suggestion(code: number) { ... };

// ✓ Use triggers for event handlers
trigger onClick = suggestion(id: string) { ... };

// ✓ Combine with finale blocks for guaranteed execution
finale {
    onCleanup();
}

// ✓ Use triggers with DeepMind functions
repeatAction(onUpdate, 10, 500);
```

### ❌ Don'ts

```hyp
// ✗ Avoid triggers inside functions
suggestion myFunction() {
    trigger localTrigger = suggestion() { ... };  // ERROR!
}

// ✗ Avoid triggers in sessions
session MySession {
    trigger classTrigger = suggestion() { ... };  // ERROR!
}

// ✗ Use anonymous expressions for local callbacks instead
this.callback = suggestion() { observe "Local callback"; };
```

## Advanced Patterns

### Chain of Triggers

```hyp
Focus {
    trigger step1 = suggestion() {
        observe "Step 1 completed";
        step2();
    };

    trigger step2 = suggestion() {
        observe "Step 2 completed";
        step3();
    };

    trigger step3 = suggestion() {
        observe "All steps completed!";
    };

    entrance {
        step1();  // Starts the chain
    }
} Relax
```

### Conditional Triggers

```hyp
Focus {
    induce debugMode: boolean = true;

    trigger onDebug = suggestion(message: string) {
        if (debugMode) {
            observe "[DEBUG] " + message;
        }
    };

    entrance {
        onDebug("Program started");
        debugMode = false;
        onDebug("This message will not be displayed");
    }
} Relax
```

### Trigger Registry Pattern

```hyp
Focus {
    induce eventRegistry: array = [];

    trigger registerEvent = suggestion(eventName: string) {
        observe "Event registered: " + eventName;
        // eventRegistry.push(eventName);  // If array push is available
    };

    trigger onAppStart = suggestion() {
        registerEvent("app_started");
    };

    trigger onAppStop = suggestion() {
        registerEvent("app_stopped");
    };

    entrance {
        onAppStart();
    }

    finale {
        onAppStop();
    }
} Relax
```

## Summary

Triggers are **first-class event handlers** in HypnoScript that:

- ✅ Can only be declared at **top-level**
- ✅ Are perfect for **event handling** and **callbacks**
- ✅ Can be combined with **DeepMind/AuraAsync**
- ✅ Can be **passed as parameters** and **stored**
- ✅ Are clearly recognizable through **naming conventions** (`on*`)

For local callbacks within functions or sessions, use anonymous `suggestion()` expressions.

## Next Steps

- [Functions](./functions) – General function definition
- [Sessions](./sessions) – Object-oriented programming
- [Async & Await](./async-await) – Asynchronous programming
- [Pattern Matching](./pattern-matching) – Advanced control structures

---

**Ready for event-based programming? Use triggers for elegant event flows!** 🎯
