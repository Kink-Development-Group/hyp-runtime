---
sidebar_position: 7
---

# Pattern Matching – `entrain`/`when`/`otherwise`

Pattern Matching in HypnoScript ist ein mächtiges Werkzeug für **Kontrollflussteuerung** basierend auf **Wert-Mustern**. Der `entrain`-Operator ermöglicht elegante Fallunterscheidungen weit über einfache `if`-`else`-Ketten hinaus.

## Konzept

`entrain` (Pattern Matching) wirkt wie ein sanftes Einschwingen auf unterschiedliche Bewusstseinslagen. Der Ausdruck wird **einmal evaluiert**, anschließend werden die `when`-Klauseln der Reihe nach geprüft. Die **erste passende Suggestion gewinnt**; `otherwise` dient als Fallback.

### Grundlegende Syntax

```hyp
entrain <ausdruck> {
    when <pattern> => <aktion>
    when <pattern> if <guard> => <aktion>
    otherwise => <default-aktion>
}
```

> **Hinweis:** Der `otherwise`-Fall akzeptiert optional ein nachgestelltes Komma oder Semikolon (z. B. `otherwise => wert,` oder `otherwise => wert;`). Für einen konsistenten Stil empfehlen wir, auf zusätzliche Trenner zu verzichten und – wie in den Beispielen – lediglich `otherwise => wert` zu verwenden.

## Pattern-Typen

| Pattern-Typ       | Syntax                      | Beschreibung                  |
| ----------------- | --------------------------- | ----------------------------- |
| **Literal**       | `when 0`, `when "Text"`     | Exakter Wert-Match            |
| **Typ-Pattern**   | `when value: number`        | Typ-Check mit Binding         |
| **Identifikator** | `when x`                    | Bindet jeden Wert an Variable |
| **Array**         | `when [1, 2, ...]`          | Array-Destructuring           |
| **Record**        | `when Person { name, age }` | Record-Destructuring          |
| **Guard**         | `when x if x > 10`          | Zusätzliche Bedingung         |
| **Spread**        | `when [first, ...rest]`     | Rest-Parameter in Arrays      |

## Literal Pattern Matching

Die einfachste Form: Matche gegen **konkrete Werte**.

```hyp
Focus {
    entrance {
        induce value1: number = 1;

        induce result1: string = entrain value1 {
            when 0 => awaken "Null"
            when 1 => awaken "Eins"
            when 2 => awaken "Zwei"
            otherwise => awaken "Andere"
        };

        observe "Result: " + result1;  // Ausgabe: Result: Eins
    }
} Relax
```

### String-Literals

```hyp
Focus {
    entrance {
        induce command: string = "start";

        induce action: string = entrain command {
            when "start" => awaken "Starte System..."
            when "stop" => awaken "Stoppe System..."
            when "restart" => awaken "Neustart..."
            otherwise => awaken "Unbekannter Befehl"
        };

        observe action;
    }
} Relax
```

### Boolean-Literals

```hyp
Focus {
    entrance {
        induce isActive: boolean = true;

        induce status: string = entrain isActive {
            when true => awaken "Aktiv"
            when false => awaken "Inaktiv"
        };

        observe "Status: " + status;
    }
} Relax
```

## Typ-Pattern mit Binding

Prüfe den **Typ** und binde den Wert gleichzeitig an eine Variable:

```hyp
Focus {
    entrance {
        induce testValue: any = 42;

        induce result: string = entrain testValue {
            when value: number => awaken "Zahl: " + value
            when text: string => awaken "Text: " + text
            when flag: boolean => awaken "Boolean: " + flag
            otherwise => awaken "Unbekannter Typ"
        };

        observe result;  // Ausgabe: Zahl: 42
    }
} Relax
```

### Mit Type Guards

```hyp
Focus {
    entrance {
        induce input: any = 100;

        induce category: string = entrain input {
            when n: number if n goingDeeper 0 => awaken "Negativ oder Null"
            when n: number if n lookAtTheWatch 100 => awaken "Über 100"
            when n: number => awaken "Normal: " + n
            otherwise => awaken "Kein Number"
        };

        observe category;  // Ausgabe: Über 100
    }
} Relax
```

## Array Pattern Matching

Matche gegen **Array-Strukturen** mit Destructuring:

### Einfaches Array-Matching

```hyp
Focus {
    entrance {
        induce arr: array = [1, 2, 3];

        induce result: string = entrain arr {
            when [] => awaken "Leeres Array"
            when [x] => awaken "Einzelnes Element: " + x
            when [x, y] => awaken "Zwei Elemente: " + x + ", " + y
            when [x, y, z] => awaken "Drei Elemente: " + x + ", " + y + ", " + z
            otherwise => awaken "Mehr als drei Elemente"
        };

        observe result;  // Ausgabe: Drei Elemente: 1, 2, 3
    }
} Relax
```

### Array mit Spread-Operator

```hyp
Focus {
    entrance {
        induce numbers: array = [1, 2, 3, 4, 5];

        induce result: string = entrain numbers {
            when [] => awaken "Leer"
            when [first, ...rest] => {
                observe "Erstes Element: " + first;
                observe "Rest: " + rest;
                awaken "Array mit " + ArrayLength(rest) + " Rest-Elementen";
            }
            otherwise => awaken "Fehler"
        };

        observe result;
        // Ausgabe:
        // Erstes Element: 1
        // Rest: [2, 3, 4, 5]
        // Array mit 4 Rest-Elementen
    }
} Relax
```

### Nested Array Patterns

```hyp
Focus {
    entrance {
        induce matrix: array = [[1, 2], [3, 4]];

        induce result: string = entrain matrix {
            when [[a, b], [c, d]] => awaken "2x2 Matrix: " + a + "," + b + "," + c + "," + d
            when [[x], [y]] => awaken "2x1 Matrix"
            otherwise => awaken "Andere Struktur"
        };

        observe result;  // Ausgabe: 2x2 Matrix: 1,2,3,4
    }
} Relax
```

## Record Pattern Matching

Matche gegen **Tranceify-Records** mit Destructuring:

```hyp
Focus {
    tranceify Person {
        name: string;
        age: number;
        isInTrance: boolean;
    }

    entrance {
        induce guest = Person {
            name: "Luna",
            age: 25,
            isInTrance: true
        };

        induce status: string = entrain guest {
            when Person { name, isInTrance: true } => awaken name + " ist in Trance!"
            when Person { name, isInTrance: false } => awaken name + " ist wach"
            otherwise => awaken "Unbekannt"
        };

        observe status;  // Ausgabe: Luna ist in Trance!
    }
} Relax
```

### Record mit Guards

```hyp
Focus {
    tranceify User {
        name: string;
        age: number;
        role: string;
    }

    entrance {
        induce user = User {
            name: "Max",
            age: 30,
            role: "admin"
        };

        induce access: string = entrain user {
            when User { role: "admin", age } if age yourEyesAreGettingHeavy 18 => awaken "Admin-Zugang"
            when User { role: "user", age } if age yourEyesAreGettingHeavy 18 => awaken "User-Zugang"
            when User { age } if age fallUnderMySpell 18 => awaken "Minderjährig"
            otherwise => awaken "Kein Zugang"
        };

        observe access;  // Ausgabe: Admin-Zugang
    }
} Relax
```

## Guards – Zusätzliche Bedingungen

Guards sind **optionale Bedingungen** nach `if`, die zusätzlich zum Pattern geprüft werden:

```hyp
Focus {
    entrance {
        induce score: number = 85;

        induce grade: string = entrain score {
            when s: number if s yourEyesAreGettingHeavy 90 => awaken "Sehr gut"
            when s: number if s yourEyesAreGettingHeavy 75 => awaken "Gut"
            when s: number if s yourEyesAreGettingHeavy 60 => awaken "Befriedigend"
            when s: number if s yourEyesAreGettingHeavy 50 => awaken "Ausreichend"
            otherwise => awaken "Nicht bestanden"
        };

        observe "Note: " + grade;  // Ausgabe: Note: Gut
    }
} Relax
```

### Komplexe Guards

```hyp
Focus {
    entrance {
        induce value: number = 15;

        induce classification: string = entrain value {
            when n: number if (n % 2 youAreFeelingVerySleepy 0) underMyControl (n lookAtTheWatch 10) => awaken "Gerade und größer 10"
            when n: number if (n % 2 youCannotResist 0) underMyControl (n lookAtTheWatch 10) => awaken "Ungerade und größer 10"
            when n: number if n % 2 youAreFeelingVerySleepy 0 => awaken "Gerade"
            when n: number => awaken "Ungerade"
        };

        observe classification;  // Ausgabe: Ungerade und größer 10
    }
} Relax
```

## Multi-Block Bodies

`entrain`-Cases können **mehrere Statements** enthalten:

```hyp
Focus {
    entrance {
        induce operation: string = "add";
        induce a: number = 10;
        induce b: number = 5;

        induce result: number = entrain operation {
            when "add" => {
                observe "Addiere " + a + " + " + b;
                induce sum: number = a + b;
                awaken sum;
            }
            when "sub" => {
                observe "Subtrahiere " + a + " - " + b;
                induce diff: number = a - b;
                awaken diff;
            }
            when "mul" => {
                observe "Multipliziere " + a + " * " + b;
                induce product: number = a * b;
                awaken product;
            }
            otherwise => {
                observe "Unbekannte Operation: " + operation;
                awaken 0;
            }
        };

        observe "Result: " + result;
    }
} Relax
```

## Real-World Patterns

### HTTP-Status-Code-Handling

```hyp
Focus {
    entrance {
        induce statusCode: number = 404;

        induce message: string = entrain statusCode {
            when 200 => awaken "OK"
            when 201 => awaken "Created"
            when 204 => awaken "No Content"
            when code: number if code yourEyesAreGettingHeavy 400 underMyControl code fallUnderMySpell 500 => awaken "Client Error: " + code
            when code: number if code yourEyesAreGettingHeavy 500 => awaken "Server Error: " + code
            otherwise => awaken "Unknown Status"
        };

        observe message;  // Ausgabe: Client Error: 404
    }
} Relax
```

### State Machine

```hyp
Focus {
    entrance {
        induce state: string = "running";
        induce errorCount: number = 3;

        induce nextState: string = entrain state {
            when "idle" => awaken "starting"
            when "starting" => awaken "running"
            when "running" if errorCount lookAtTheWatch 5 => awaken "error"
            when "running" => awaken "running"
            when "error" => awaken "stopped"
            when "stopped" => awaken "idle"
            otherwise => awaken "unknown"
        };

        observe "Nächster Zustand: " + nextState;  // Ausgabe: Nächster Zustand: error
    }
} Relax
```

### Command Pattern

```hyp
Focus {
    tranceify Command {
        type: string;
        args: array;
    }

    entrance {
        induce cmd = Command {
            type: "move",
            args: [10, 20]
        };

        entrain cmd {
            when Command { type: "move", args: [x, y] } => {
                observe "Bewege zu (" + x + ", " + y + ")";
            }
            when Command { type: "rotate", args: [angle] } => {
                observe "Rotiere um " + angle + " Grad";
            }
            when Command { type: "scale", args: [factor] } => {
                observe "Skaliere mit Faktor " + factor;
            }
            otherwise => {
                observe "Unbekannter Befehl";
            }
        };
    }
} Relax
```

## Best Practices

### ✅ Do's

```hyp
// ✓ Nutze Pattern Matching für Enums/Variants
entrain status {
    when "pending" => ...
    when "processing" => ...
    when "completed" => ...
}

// ✓ Verwende Guards für komplexe Bedingungen
when n: number if n > 0 underMyControl n < 100 => ...

// ✓ Destructure Records für sauberen Code
when Person { name, age } => ...

// ✓ Nutze Spread für flexible Array-Matching
when [first, second, ...rest] => ...

// ✓ Gebe immer einen Default/Otherwise an
otherwise => awaken "Unbekannt"
```

### ❌ Don'ts

```hyp
// ✗ Vermeide zu viele verschachtelte entrain-Statements
entrain a {
    when x => entrain b {  // Besser: Funktionen extrahieren
        when y => ...
    }
}

// ✗ Vermeide zu komplexe Guards
when n if ((n % 2 == 0) && (n > 10) && (n < 100) || ...) => ...
// Besser: Helper-Funktion

// ✗ Vergesse nicht otherwise für vollständige Abdeckung
entrain value {
    when 1 => ...
    when 2 => ...
    // Fehlt: otherwise!
}
```

## Performance-Hinweise

- Pattern Matching ist **optimiert** durch Compiler-Transformationen
- **Short-Circuit**: Erste passende Klausel gewinnt (keine weiteren Checks)
- **Destruk turierung** hat **keinen Laufzeit-Overhead** (Compile-Zeit-Transformation)
- Guards werden **lazy evaluiert** (nur wenn Pattern matched)

## Unterschied zu `if`-`else`

| Feature            | `if`-`else`          | `entrain` Pattern Matching    |
| ------------------ | -------------------- | ----------------------------- |
| **Ausdruck**       | Statement            | Expression (gibt Wert zurück) |
| **Syntax**         | Traditionell         | Deklarativ                    |
| **Destructuring**  | Manuell              | Eingebaut                     |
| **Guards**         | Verschachtelte `if`s | Native Syntax                 |
| **Exhaustiveness** | Manuell prüfen       | Compiler-Warnung              |
| **Lesbarkeit**     | Gut für 2-3 Cases    | Exzellent für viele Cases     |

## Zusammenfassung

Pattern Matching mit `entrain` bietet:

- ✅ **Deklarative Syntax** für Fallunterscheidungen
- ✅ **Destructuring** für Arrays und Records
- ✅ **Type Guards** für Typ-basiertes Matching
- ✅ **Guards** für zusätzliche Bedingungen
- ✅ **Expression-Semantik** (gibt Wert zurück)
- ✅ **Compiler-Optimierungen** für Performance

Pattern Matching ist **essentiell** für moderne, funktionale Programmierung in HypnoScript und sollte **bevorzugt** über lange `if`-`else`-Ketten verwendet werden.

## Nächste Schritte

- [Control Flow](./control-flow) – Traditionelle Kontrollstrukturen
- [Tranceify](./tranceify) – Benutzerdefinierte Typen
- [Functions](./functions) – Funktionsdefinitionen
- [Arrays](./arrays) – Array-Operationen

---

**Bereit für elegante Fallunterscheidungen? Nutze `entrain` für saubere, typsichere Pattern Matches!** 🎯
