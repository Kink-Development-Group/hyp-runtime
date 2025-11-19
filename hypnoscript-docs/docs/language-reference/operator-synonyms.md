---
sidebar_position: 4
---

# Hypnotic Operator Synonyms

HypnoScript offers **hypnotic alias names** for standard operators that emphasize the suggestive nature of the language. Each synonym is **semantically identical** to the standard operator but adds a theatrical, hypnotic layer.

## Philosophy

> _"You are feeling very sleepy... Your code is getting deeper... and deeper..."_

HypnoScript treats code as a **suggestion** to the computer. The hypnotic operator synonyms reflect this metaphor and make code simultaneously:

- 🎭 **Theatrical** – Operators as hypnotic formulas
- 📖 **Readable** – Self-explanatory meanings
- 🔄 **Compatible** – Mixable with standard operators
- 🎨 **Expressive** – Reinforces the hypnotic theme

## Comparison Operators

### Equality & Inequality

| Standard | Hypnotic Synonym          | Meaning    | Example                       |
| -------- | ------------------------- | ---------- | ----------------------------- |
| `==`     | `youAreFeelingVerySleepy` | Equality   | `a youAreFeelingVerySleepy b` |
| `!=`     | `youCannotResist`         | Inequality | `x youCannotResist y`         |

**Usage:**

```hyp
Focus {
    entrance {
        induce age: number = 25;
        induce name: string = "Luna";

        // Standard syntax
        if (age == 25) {
            observe "Age is 25";
        }

        // Hypnotic syntax
        if (age youAreFeelingVerySleepy 25) {
            observe "Age is 25 (hypnotic)";
        }

        if (name youCannotResist "Max") {
            observe "Name is not Max";
        }
    }
} Relax
```

### Relational (Greater/Less)

| Standard | Hypnotic Synonym          | Meaning              | Example                       |
| -------- | ------------------------- | -------------------- | ----------------------------- |
| `>`      | `lookAtTheWatch`          | Greater than         | `a lookAtTheWatch b`          |
| `<`      | `fallUnderMySpell`        | Less than            | `x fallUnderMySpell y`        |
| `>=`     | `yourEyesAreGettingHeavy` | Greater or equal     | `a yourEyesAreGettingHeavy b` |
| `<=`     | `goingDeeper`             | Less or equal        | `x goingDeeper y`             |

**Usage:**

```hyp
Focus {
    entrance {
        induce score: number = 85;
        induce threshold: number = 75;

        // Standard syntax
        if (score > threshold) {
            observe "Passed!";
        }

        // Hypnotic syntax
        if (score lookAtTheWatch threshold) {
            observe "Passed (hypnotic)!";
        }

        if (score yourEyesAreGettingHeavy 80) {
            observe "At least 80 points";
        }

        if (score goingDeeper 100) {
            observe "Under 100 points";
        }
    }
} Relax
```

## Logical Operators

| Standard | Hypnotic Synonym     | Meaning        | Example                  |
| -------- | -------------------- | -------------- | ------------------------ |
| `&&`     | `underMyControl`     | Logical AND    | `a underMyControl b`     |
| `\|\|`   | `resistanceIsFutile` | Logical OR     | `x resistanceIsFutile y` |

**Usage:**

```hyp
Focus {
    entrance {
        induce age: number = 22;
        induce hasLicense: boolean = true;

        // Standard syntax
        if (age >= 18 && hasLicense == true) {
            observe "May drive!";
        }

        // Hypnotic syntax
        if (age yourEyesAreGettingHeavy 18 underMyControl hasLicense youAreFeelingVerySleepy true) {
            observe "May drive (hypnotic)!";
        }

        induce isWeekend: boolean = false;
        induce isHoliday: boolean = true;

        if (isWeekend resistanceIsFutile isHoliday) {
            observe "Free today!";
        }
    }
} Relax
```

## Modern Dream Semantics

| Standard | Hypnotic Synonym | Meaning            | Example               |
| -------- | ---------------- | ------------------ | --------------------- |
| `??`     | `lucidFallback`  | Nullish Coalescing | `x lucidFallback y`   |
| `?.`     | `dreamReach`     | Optional Chaining  | `obj dreamReach prop` |
| `?.[`    | `dreamReach[`    | Optional Array     | `arr dreamReach[0]`   |
| `?.(`    | `dreamReach(`    | Optional Call      | `fn dreamReach(arg)`  |

**Usage:**

```hyp
Focus {
    tranceify Profile {
        alias: string;
        level: number;
    }

    tranceify Guest {
        name: string;
        profile: Profile;
    }

    entrance {
        induce maybeValue: number = null;

        // Standard syntax
        induce defaulted: number = maybeValue ?? 100;

        // Hypnotic syntax
        induce defaulted2: number = maybeValue lucidFallback 100;

        observe "Defaulted: " + defaulted2;  // 100

        // Optional Chaining
        induce guest: Guest = null;
        induce alias = guest dreamReach profile dreamReach alias lucidFallback "Unknown";

        observe "Alias: " + alias;  // "Unknown"
    }
} Relax
```

## Legacy Synonyms (Deprecated)

These synonyms exist for historical reasons but should **no longer be used**:

| Deprecated      | Replaced by               | Standard |
| --------------- | ------------------------- | -------- |
| `notSoDeep`     | `youCannotResist`         | `!=`     |
| `deeplyGreater` | `yourEyesAreGettingHeavy` | `>=`     |
| `deeplyLess`    | `goingDeeper`             | `<=`     |

## Combined Examples

### Hypnotic Arithmetic with Guards

```hyp
Focus {
    entrance {
        induce x: number = 7;
        induce y: number = 42;
        induce z: number = 100;

        // Combine multiple hypnotic operators
        if ((x goingDeeper 100) resistanceIsFutile (y yourEyesAreGettingHeavy 50)) {
            observe "Condition met – go deeper!";
        }

        // Complex expression
        if ((x lookAtTheWatch 5) underMyControl (y fallUnderMySpell 50) underMyControl (z youAreFeelingVerySleepy 100)) {
            observe "x > 5 AND y < 50 AND z == 100";
        }
    }
} Relax
```

### Pattern Matching with Synonyms

```hyp
Focus {
    entrance {
        induce score: number = 85;

        induce grade: string = entrain score {
            when s: number if s yourEyesAreGettingHeavy 90 => awaken "Excellent"
            when s: number if s yourEyesAreGettingHeavy 75 => awaken "Good"
            when s: number if s yourEyesAreGettingHeavy 60 => awaken "Satisfactory"
            otherwise => awaken "Not passed"
        };

        observe "Grade: " + grade;
    }
} Relax
```

### Null-Safety with Hypnosis

```hyp
Focus {
    tranceify User {
        name: string;
        email: string;
    }

    entrance {
        induce maybeUser: User = null;

        // Combine dreamReach and lucidFallback
        induce userName: string = maybeUser dreamReach name lucidFallback "Guest";
        induce userEmail: string = maybeUser dreamReach email lucidFallback "no@email.com";

        observe "User: " + userName;  // "Guest"
        observe "Email: " + userEmail;  // "no@email.com"

        // With comparison
        if (userName youCannotResist "Guest") {
            observe "Logged in user!";
        }
    }
} Relax
```

## Style Guidelines

### Consistent Hypnosis

Choose **one style** per file/module and stick with it:

```hyp
// ✅ Consistently hypnotic
Focus {
    entrance {
        if ((age yourEyesAreGettingHeavy 18) underMyControl (hasLicense youAreFeelingVerySleepy true)) {
            observe "OK";
        }
    }
} Relax

// ✅ Consistently standard
Focus {
    entrance {
        if ((age >= 18) && (hasLicense == true)) {
            observe "OK";
        }
    }
} Relax

// ❌ Mixed (hard to read)
Focus {
    entrance {
        if ((age yourEyesAreGettingHeavy 18) && (hasLicense == true)) {
            observe "Inconsistent";
        }
    }
} Relax
```

### When to Use Hypnotic Syntax?

| Scenario                 | Recommendation                         |
| ------------------------ | -------------------------------------- |
| **Production code**      | Standard operators (`==`, `>=`, etc.)  |
| **Experimental projects**| Hypnotic synonyms for flair            |
| **Hypnosis theme**       | Consistently hypnotic                  |
| **Tutorials/Demos**      | Standard (familiar to beginners)       |
| **Code golf/Art**        | Hypnotic (maximum expression)          |

## Complete Reference Table

| Category       | Standard | Hypnotic                  | Meaning            |
| -------------- | -------- | ------------------------- | ------------------ |
| **Equality**   | `==`     | `youAreFeelingVerySleepy` | Equal              |
|                | `!=`     | `youCannotResist`         | Not equal          |
| **Relational** | `>`      | `lookAtTheWatch`          | Greater            |
|                | `<`      | `fallUnderMySpell`        | Less               |
|                | `>=`     | `yourEyesAreGettingHeavy` | Greater-or-equal   |
|                | `<=`     | `goingDeeper`             | Less-or-equal      |
| **Logical**    | `&&`     | `underMyControl`          | AND                |
|                | `\|\|`   | `resistanceIsFutile`      | OR                 |
| **Nullish**    | `??`     | `lucidFallback`           | Nullish-Coalescing |
|                | `?.`     | `dreamReach`              | Optional-Chaining  |

## Case-Insensitivity

All hypnotic operators are **case-insensitive**:

```hyp
// All variants work
youAreFeelingVerySleepy
YOUAREFEELINGVERYSLEEPY
youarefeelingverysleepy
YouAreFeelingVerySleepy
```

The **canonical form** (in error messages and documentation) is **camelCase**:

- `youAreFeelingVerySleepy`
- `yourEyesAreGettingHeavy`
- `underMyControl`

## Performance

- **No runtime overhead**: Synonyms are compiled to standard operators
- **Identical performance**: `a youAreFeelingVerySleepy b` == `a == b`
- **No size differences**: Binary file size unchanged

## Best Practices

### ✅ Do's

```hyp
// ✓ Consistent style within a file
if (x yourEyesAreGettingHeavy 10 underMyControl y fallUnderMySpell 20) { ... }

// ✓ Readable combinations
induce result = value lucidFallback default;

// ✓ Self-explanatory guards
when n: number if n lookAtTheWatch 100 => ...
```

### ❌ Don'ts

```hyp
// ✗ Don't mix standard and hypnotic
if (x >= 10 underMyControl y < 20) { ... }

// ✗ Don't overdo it
if ((((a youAreFeelingVerySleepy b) underMyControl (c lookAtTheWatch d)) resistanceIsFutile ...)) { ... }

// ✗ Don't use deprecated synonyms
if (x notSoDeep 5) { ... }  // Use youCannotResist
```

## Summary

Hypnotic operator synonyms are:

- ✅ **Semantically identical** to standard operators
- ✅ **Case-insensitive** (recommended: camelCase)
- ✅ **Performance-neutral** (no overhead)
- ✅ **Optional** (standard operators remain valid)
- ✅ **Expressive** (reinforces hypnotic theme)

Use them **consistently** or **not at all** – mixing reduces readability!

## Next Steps

- [Operators](./operators) – Complete operator reference
- [Nullish Operators](./nullish-operators) – Modern dream semantics
- [Pattern Matching](./pattern-matching) – Guards with synonyms
- [Syntax](./syntax) – Basic syntax rules

---

**Ready for hypnotic operations? Use synonyms for maximum suggestion!** 🌀
