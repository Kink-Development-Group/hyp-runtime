---
title: Basic Examples
---

This page demonstrates common session scenarios that highlight the latest language and type-checker capabilities around constructors, static members, and visibility.

## Session constructors

Constructors let you hydrate a session with the right defaults. They run automatically immediately after `SessionName(...)` is called.

```hypnoscript
Focus {
    session Account {
        conceal balance: number = 0;

        expose suggestion constructor(initialBalance: number) {
            this.balance = initialBalance;
        }

        expose suggestion deposit(amount: number) {
            this.balance = this.balance + amount;
        }

        expose suggestion current(): number {
            awaken this.balance;
        }
    }

    induce savings = Account(250);
    savings.deposit(50);
    induce balanceNow: number = savings.current();
} Relax
```

The type checker verifies that the constructor receives exactly one numeric argument and that `current()` returns a number.

## Static configuration

Static members (prefixed with `dominant`) belong to the session type instead of an instance. They are ideal for global configuration knobs.

```hypnoscript
Focus {
    session Config {
        dominant expose environment: string = "dev";

        dominant suggestion switch(target: string) {
            Config.environment = target;
        }
    }

    Config.switch("prod");
    induce activeEnv: string = Config.environment;
} Relax
```

When you attempt to rewrite `Config.environment` through an instance (for example `config.environment = ...`) the type checker refuses the script with: `Assign static field 'environment' through session 'Config', not an instance`.

## Visibility enforcement

Encapsulate sensitive state with `conceal`. The compiler catches accidental leaks before runtime.

```hypnoscript
Focus {
    session Vault {
        conceal pin: number = 1337;
        expose suggestion reveal(): number {
            awaken this.pin;
        }
    }

    induce safe = Vault();
    induce pin = safe.reveal();
    induce leaked = safe.pin; // Field 'pin' of session 'Vault' is not visible here
} Relax
```

Only the `reveal()` method can read the concealed field. Every other attempt triggers a type checker diagnostic identical to the inline comment above.
