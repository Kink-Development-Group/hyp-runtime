---
sidebar_position: 4
---

# Hypnotische Operator-Synonyme

HypnoScript bietet **hypnotische Aliasnamen** für Standard-Operatoren, die die suggestive Natur der Sprache unterstreichen. Jedes Synonym ist **semantisch identisch** zum Standard-Operator, fügt aber eine theatralische, hypnotische Ebene hinzu.

## Philosophie

> _"You are feeling very sleepy... Your code is getting deeper... and deeper..."_

HypnoScript behandelt Code als **Suggestion** an den Computer. Die hypnotischen Operator-Synonyme spiegeln diese Metapher wider und machen Code gleichzeitig:

- 🎭 **Theatralisch** – Operatoren als hypnotische Formeln
- 📖 **Lesbar** – Selbsterklärende Bedeutungen
- 🔄 **Kompatibel** – Mischbar mit Standard-Operatoren
- 🎨 **Ausdrucksstark** – Verstärkt die hypnotische Thematik

## Vergleichsoperatoren

### Gleichheit & Ungleichheit

| Standard | Hypnotisches Synonym      | Bedeutung    | Beispiel                      |
| -------- | ------------------------- | ------------ | ----------------------------- |
| `==`     | `youAreFeelingVerySleepy` | Gleichheit   | `a youAreFeelingVerySleepy b` |
| `!=`     | `youCannotResist`         | Ungleichheit | `x youCannotResist y`         |

**Verwendung:**

```hyp
Focus {
    entrance {
        induce age: number = 25;
        induce name: string = "Luna";

        // Standard-Syntax
        if (age == 25) {
            observe "Age ist 25";
        }

        // Hypnotische Syntax
        if (age youAreFeelingVerySleepy 25) {
            observe "Age ist 25 (hypnotisch)";
        }

        if (name youCannotResist "Max") {
            observe "Name ist nicht Max";
        }
    }
} Relax
```

### Relational (Größer/Kleiner)

| Standard | Hypnotisches Synonym      | Bedeutung           | Beispiel                      |
| -------- | ------------------------- | ------------------- | ----------------------------- |
| `>`      | `lookAtTheWatch`          | Größer als          | `a lookAtTheWatch b`          |
| `<`      | `fallUnderMySpell`        | Kleiner als         | `x fallUnderMySpell y`        |
| `>=`     | `yourEyesAreGettingHeavy` | Größer oder gleich  | `a yourEyesAreGettingHeavy b` |
| `<=`     | `goingDeeper`             | Kleiner oder gleich | `x goingDeeper y`             |

**Verwendung:**

```hyp
Focus {
    entrance {
        induce score: number = 85;
        induce threshold: number = 75;

        // Standard-Syntax
        if (score > threshold) {
            observe "Bestanden!";
        }

        // Hypnotische Syntax
        if (score lookAtTheWatch threshold) {
            observe "Bestanden (hypnotisch)!";
        }

        if (score yourEyesAreGettingHeavy 80) {
            observe "Mindestens 80 Punkte";
        }

        if (score goingDeeper 100) {
            observe "Unter 100 Punkte";
        }
    }
} Relax
```

## Logische Operatoren

| Standard | Hypnotisches Synonym | Bedeutung      | Beispiel                 |
| -------- | -------------------- | -------------- | ------------------------ |
| `&&`     | `underMyControl`     | Logisches UND  | `a underMyControl b`     |
| `\|\|`   | `resistanceIsFutile` | Logisches ODER | `x resistanceIsFutile y` |

**Verwendung:**

```hyp
Focus {
    entrance {
        induce age: number = 22;
        induce hasLicense: boolean = true;

        // Standard-Syntax
        if (age >= 18 && hasLicense == true) {
            observe "Darf fahren!";
        }

        // Hypnotische Syntax
        if (age yourEyesAreGettingHeavy 18 underMyControl hasLicense youAreFeelingVerySleepy true) {
            observe "Darf fahren (hypnotisch)!";
        }

        induce isWeekend: boolean = false;
        induce isHoliday: boolean = true;

        if (isWeekend resistanceIsFutile isHoliday) {
            observe "Frei heute!";
        }
    }
} Relax
```

## Moderne Traum-Semantik

| Standard | Hypnotisches Synonym | Bedeutung              | Beispiel              |
| -------- | -------------------- | ---------------------- | --------------------- |
| `??`     | `lucidFallback`      | Nullish Coalescing     | `x lucidFallback y`   |
| `?.`     | `dreamReach`         | Optional Chaining      | `obj dreamReach prop` |
| `?.[`    | `dreamReach[`        | Optional Array Index   | `arr dreamReach[0]`   |
| `?.(`    | `dreamReach(`        | Optional Function Call | `fn dreamReach(arg)`  |

**Verwendung:**

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

        // Standard-Syntax
        induce defaulted: number = maybeValue ?? 100;

        // Hypnotische Syntax
        induce defaulted2: number = maybeValue lucidFallback 100;

        observe "Defaulted: " + defaulted2;  // 100

        // Optional Chaining
        induce guest: Guest = null;
        induce alias = guest dreamReach profile dreamReach alias lucidFallback "Unknown";

        observe "Alias: " + alias;  // "Unknown"
    }
} Relax
```

## Legacy-Synonyme (Veraltet)

Diese Synonyme existieren aus historischen Gründen, sollten aber **nicht mehr verwendet** werden:

| Veraltet        | Ersetzt durch             | Standard |
| --------------- | ------------------------- | -------- |
| `notSoDeep`     | `youCannotResist`         | `!=`     |
| `deeplyGreater` | `yourEyesAreGettingHeavy` | `>=`     |
| `deeplyLess`    | `goingDeeper`             | `<=`     |

## Kombinierte Beispiele

### Hypnotische Arithmetik mit Guards

```hyp
Focus {
    entrance {
        induce x: number = 7;
        induce y: number = 42;
        induce z: number = 100;

        // Kombiniere mehrere hypnotische Operatoren
        if ((x goingDeeper 100) resistanceIsFutile (y yourEyesAreGettingHeavy 50)) {
            observe "Bedingung erfüllt – trance tiefer!";
        }

        // Komplexer Ausdruck
        if ((x lookAtTheWatch 5) underMyControl (y fallUnderMySpell 50) underMyControl (z youAreFeelingVerySleepy 100)) {
            observe "x > 5 UND y < 50 UND z == 100";
        }
    }
} Relax
```

### Pattern Matching mit Synonymen

```hyp
Focus {
    entrance {
        induce score: number = 85;

        induce grade: string = entrain score {
            when s: number if s yourEyesAreGettingHeavy 90 => awaken "Sehr gut"
            when s: number if s yourEyesAreGettingHeavy 75 => awaken "Gut"
            when s: number if s yourEyesAreGettingHeavy 60 => awaken "Befriedigend"
            otherwise => awaken "Nicht bestanden"
        };

        observe "Note: " + grade;
    }
} Relax
```

### Null-Safety mit Hypnose

```hyp
Focus {
    tranceify User {
        name: string;
        email: string;
    }

    entrance {
        induce maybeUser: User = null;

        // Kombiniere dreamReach und lucidFallback
        induce userName: string = maybeUser dreamReach name lucidFallback "Guest";
        induce userEmail: string = maybeUser dreamReach email lucidFallback "no@email.com";

        observe "User: " + userName;  // "Guest"
        observe "Email: " + userEmail;  // "no@email.com"

        // Mit Vergleich
        if (userName youCannotResist "Guest") {
            observe "Eingeloggter Benutzer!";
        }
    }
} Relax
```

## Stil-Guidelines

### Konsistente Hypnose

Wähle **einen Stil** pro Datei/Modul und bleibe dabei:

```hyp
// ✅ Konsistent hypnotisch
Focus {
    entrance {
        if ((age yourEyesAreGettingHeavy 18) underMyControl (hasLicense youAreFeelingVerySleepy true)) {
            observe "OK";
        }
    }
} Relax

// ✅ Konsistent standard
Focus {
    entrance {
        if ((age >= 18) && (hasLicense == true)) {
            observe "OK";
        }
    }
} Relax

// ❌ Gemischt (schwer lesbar)
Focus {
    entrance {
        if ((age yourEyesAreGettingHeavy 18) && (hasLicense == true)) {
            observe "Inkonsistent";
        }
    }
} Relax
```

### Wann hypnotische Syntax verwenden?

| Szenario                    | Empfehlung                             |
| --------------------------- | -------------------------------------- |
| **Produktions-Code**        | Standard-Operatoren (`==`, `>=`, etc.) |
| **Experimentelle Projekte** | Hypnotische Synonyme für Flair         |
| **Hypnose-Thematik**        | Konsequent hypnotisch                  |
| **Tutorials/Demos**         | Standard (vertraut für Einsteiger)     |
| **Code-Golf/Kunst**         | Hypnotisch (maximaler Ausdruck)        |

## Vollständige Referenztabelle

| Kategorie      | Standard | Hypnotisch                | Bedeutung          |
| -------------- | -------- | ------------------------- | ------------------ |
| **Gleichheit** | `==`     | `youAreFeelingVerySleepy` | Gleich             |
|                | `!=`     | `youCannotResist`         | Ungleich           |
| **Relational** | `>`      | `lookAtTheWatch`          | Größer             |
|                | `<`      | `fallUnderMySpell`        | Kleiner            |
|                | `>=`     | `yourEyesAreGettingHeavy` | Größer-gleich      |
|                | `<=`     | `goingDeeper`             | Kleiner-gleich     |
| **Logisch**    | `&&`     | `underMyControl`          | UND                |
|                | `\|\|`   | `resistanceIsFutile`      | ODER               |
| **Nullish**    | `??`     | `lucidFallback`           | Nullish-Coalescing |
|                | `?.`     | `dreamReach`              | Optional-Chaining  |

## Case-Insensitivity

Alle hypnotischen Operatoren sind **case-insensitive**:

```hyp
// Alle Varianten funktionieren
youAreFeelingVerySleepy
YOUAREFEELINGVERYSLEEPY
youarefeelingverysleepy
YouAreFeelingVerySleepy
```

Die **kanonische Form** (in Fehlermeldungen und Dokumentation) ist **camelCase**:

- `youAreFeelingVerySleepy`
- `yourEyesAreGettingHeavy`
- `underMyControl`

## Performance

- **Keine Laufzeit-Overhead**: Synonyme werden zu Standard-Operatoren kompiliert
- **Identische Performance**: `a youAreFeelingVerySleepy b` == `a == b`
- **Keine Größen-Unterschiede**: Binärdatei-Größe unverändert

## Best Practices

### ✅ Do's

```hyp
// ✓ Konsistenter Stil innerhalb einer Datei
if (x yourEyesAreGettingHeavy 10 underMyControl y fallUnderMySpell 20) { ... }

// ✓ Lesbare Kombinationen
induce result = value lucidFallback default;

// ✓ Selbsterklärende Guards
when n: number if n lookAtTheWatch 100 => ...
```

### ❌ Don'ts

```hyp
// ✗ Mische nicht Standard und Hypnotisch
if (x >= 10 underMyControl y < 20) { ... }

// ✗ Übertreibe es nicht
if ((((a youAreFeelingVerySleepy b) underMyControl (c lookAtTheWatch d)) resistanceIsFutile ...)) { ... }

// ✗ Verwende keine veralteten Synonyme
if (x notSoDeep 5) { ... }  // Verwende youCannotResist
```

## Zusammenfassung

Hypnotische Operator-Synonyme sind:

- ✅ **Semantisch identisch** zu Standard-Operatoren
- ✅ **Case-insensitive** (empfohlen: camelCase)
- ✅ **Performance-neutral** (keine Overhead)
- ✅ **Optional** (Standard-Operatoren bleiben gültig)
- ✅ **Ausdrucksstark** (verstärkt hypnotische Thematik)

Nutze sie **konsistent** oder **gar nicht** – Mischungen reduzieren Lesbarkeit!

## Nächste Schritte

- [Operators](./operators) – Vollständige Operator-Referenz
- [Nullish Operators](./nullish-operators) – Moderne Traum-Semantik
- [Pattern Matching](./pattern-matching) – Guards mit Synonymen
- [Syntax](./syntax) – Grundlegende Syntax-Regeln

---

**Bereit für hypnotische Operationen? Nutze Synonyme für maximale Suggestion!** 🌀
