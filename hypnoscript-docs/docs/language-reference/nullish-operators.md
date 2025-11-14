---
sidebar_position: 8
---

# Moderne Traum-Semantik – Nullish Operators

HypnoScript bietet moderne, hypnotisch-benannte Operatoren für sicheren Umgang mit `null`- und `undefined`-Werten. Diese Operatoren sind direkte Aliases zu TypeScript/JavaScript-Konzepten, eingebettet in die hypnotische Metaphorik.

## Übersicht

| Konstruktion       | Hypnotisches Synonym | Standard-Operator | Bedeutung                   |
| ------------------ | -------------------- | ----------------- | --------------------------- |
| Nullish Coalescing | `lucidFallback`      | `??`              | Fallback für null/undefined |
| Optional Chaining  | `dreamReach`         | `?.`              | Sicherer Objektzugriff      |
| Optional Array     | `dreamReach[`        | `?.[`             | Sicherer Array-Index        |
| Optional Call      | `dreamReach(`        | `?.(`             | Sicherer Funktionsaufruf    |

## Nullish Coalescing – `lucidFallback` (`??`)

Der `lucidFallback`-Operator (Alias für `??`) gibt den **rechten Operanden** zurück, wenn der linke `null` oder `undefined` ist.

### Syntax

```hyp
wert lucidFallback fallback
wert ?? fallback
```

### Grundlegende Verwendung

```hyp
Focus {
    entrance {
        induce maybeValue: number = null;
        induce defaulted: number = maybeValue lucidFallback 100;
        observe "Wert: " + defaulted;  // Ausgabe: Wert: 100

        induce realValue: number = 42;
        induce result: number = realValue lucidFallback 100;
        observe "Wert: " + result;  // Ausgabe: Wert: 42
    }
} Relax
```

### Unterschied zu `||` (OR)

```hyp
Focus {
    entrance {
        // lucidFallback prüft nur auf null/undefined
        induce zero: number = 0;
        induce empty: string = "";
        induce falseBool: boolean = false;

        observe zero lucidFallback 42;      // 0 (nicht null!)
        observe empty lucidFallback "leer"; // "" (nicht null!)
        observe falseBool lucidFallback true; // false (nicht null!)

        // || prüft auf "falsy" Werte
        observe zero || 42;          // 42 (0 ist falsy)
        observe empty || "leer";     // "leer" ("" ist falsy)
        observe falseBool || true;   // true (false ist falsy)
    }
} Relax
```

### Verschachtelte Fallbacks

```hyp
Focus {
    entrance {
        induce primary: number = null;
        induce secondary: number = null;
        induce tertiary: number = 99;

        induce result: number = primary lucidFallback secondary lucidFallback tertiary;
        observe "Wert: " + result;  // Ausgabe: Wert: 99
    }
} Relax
```

## Optional Chaining – `dreamReach` (`?.`)

Der `dreamReach`-Operator (Alias für `?.`) ermöglicht **sicheren Zugriff** auf verschachtelte Eigenschaften, ohne Fehler bei `null`/`undefined` zu werfen.

### Syntax

```hyp
objekt dreamReach eigenschaft
objekt ?. eigenschaft
```

### Objekt-Zugriff

```hyp
Focus {
    tranceify Guest {
        name: string;
        profile: Profile;
    }

    tranceify Profile {
        alias: string;
        level: number;
    }

    entrance {
        induce guest = Guest {
            name: "Luna",
            profile: Profile {
                alias: "Hypna",
                level: 5
            }
        };

        // Sicherer Zugriff
        induce alias: string = guest dreamReach profile dreamReach alias;
        observe "Alias: " + alias;  // Ausgabe: Alias: Hypna

        // Null-sicherer Zugriff
        induce nullGuest: Guest = null;
        induce safeAlias = nullGuest dreamReach profile dreamReach alias lucidFallback "Unbekannt";
        observe "Alias: " + safeAlias;  // Ausgabe: Alias: Unbekannt
    }
} Relax
```

### Array-Index mit `dreamReach[`

```hyp
Focus {
    entrance {
        induce numbers: array = [1, 2, 3, 4, 5];
        induce maybeArray: array = null;

        // Normaler Array-Zugriff würde bei null fehlschlagen
        induce value1 = numbers dreamReach[2];
        observe "Value 1: " + value1;  // Ausgabe: Value 1: 3

        // Null-sicherer Array-Zugriff
        induce value2 = maybeArray dreamReach[0] lucidFallback 0;
        observe "Value 2: " + value2;  // Ausgabe: Value 2: 0
    }
} Relax
```

### Funktions-Aufruf mit `dreamReach(`

```hyp
Focus {
    suggestion greet(name: string): string {
        awaken "Hallo, " + name + "!";
    }

    entrance {
        induce maybeFunc: suggestion = greet;
        induce nullFunc: suggestion = null;

        // Sicherer Funktionsaufruf
        induce greeting1 = maybeFunc dreamReach("Luna");
        observe greeting1;  // Ausgabe: Hallo, Luna!

        // Null-sicherer Aufruf
        induce greeting2 = nullFunc dreamReach("Max") lucidFallback "Keine Funktion";
        observe greeting2;  // Ausgabe: Keine Funktion
    }
} Relax
```

## Kombination beider Operatoren

Die wahre Macht zeigt sich bei Kombination von `dreamReach` und `lucidFallback`:

### Sichere Datenextraktion

```hyp
Focus {
    tranceify UserData {
        user: User;
    }

    tranceify User {
        profile: UserProfile;
    }

    tranceify UserProfile {
        settings: Settings;
    }

    tranceify Settings {
        theme: string;
    }

    entrance {
        induce data: UserData = null;

        // Tiefe Navigation mit Fallback
        induce theme: string = data dreamReach user dreamReach profile dreamReach settings dreamReach theme lucidFallback "default";

        observe "Theme: " + theme;  // Ausgabe: Theme: default
    }
} Relax
```

### API-Response-Handling

```hyp
Focus {
    tranceify ApiResponse {
        data: ResponseData;
        error: ErrorInfo;
    }

    tranceify ResponseData {
        items: array;
        meta: Metadata;
    }

    tranceify Metadata {
        total: number;
        page: number;
    }

    entrance {
        // Simuliere API-Response
        induce response: ApiResponse = ApiResponse {
            data: null,
            error: null
        };

        // Sichere Extraktion mit Defaults
        induce items = response dreamReach data dreamReach items lucidFallback [];
        induce total = response dreamReach data dreamReach meta dreamReach total lucidFallback 0;
        induce page = response dreamReach data dreamReach meta dreamReach page lucidFallback 1;

        observe "Items: " + items;
        observe "Total: " + total;
        observe "Page: " + page;
    }
} Relax
```

## Real-World Patterns

### Config-Loading mit Defaults

```hyp
Focus {
    tranceify AppConfig {
        database: DatabaseConfig;
        server: ServerConfig;
    }

    tranceify DatabaseConfig {
        host: string;
        port: number;
        name: string;
    }

    tranceify ServerConfig {
        port: number;
        timeout: number;
    }

    entrance {
        induce config: AppConfig = null;  // Simuliere fehlende Config

        // Lade Config mit sinnvollen Defaults
        induce dbHost = config dreamReach database dreamReach host lucidFallback "localhost";
        induce dbPort = config dreamReach database dreamReach port lucidFallback 5432;
        induce dbName = config dreamReach database dreamReach name lucidFallback "hypnodb";

        induce serverPort = config dreamReach server dreamReach port lucidFallback 8080;
        induce serverTimeout = config dreamReach server dreamReach timeout lucidFallback 30000;

        observe "DB: " + dbHost + ":" + dbPort + "/" + dbName;
        observe "Server: Port " + serverPort + ", Timeout " + serverTimeout + "ms";
    }
} Relax
```

### User-Input Validation

```hyp
Focus {
    tranceify FormData {
        email: string;
        age: number;
        newsletter: boolean;
    }

    entrance {
        induce formData: FormData = null;  // Simuliere leeres Formular

        // Validiere und setze Defaults
        induce email = formData dreamReach email lucidFallback "";
        induce age = formData dreamReach age lucidFallback 0;
        induce newsletter = formData dreamReach newsletter lucidFallback false;

        // Validierung mit hypnotischen Operators
        induce isValid = (Length(email) lookAtTheWatch 0) underMyControl (age yourEyesAreGettingHeavy 18);

        if (isValid) {
            observe "Gültige Eingabe: " + email;
        } else {
            observe "Ungültige Eingabe!";
        }
    }
} Relax
```

## Best Practices

### ✅ Do's

```hyp
// ✓ Verwende lucidFallback für null-Checks
induce value = maybeNull lucidFallback defaultValue;

// ✓ Nutze dreamReach für verschachtelte Objekte
induce deep = obj dreamReach prop1 dreamReach prop2;

// ✓ Kombiniere beide für sichere Datenextraktion
induce safe = obj dreamReach prop lucidFallback fallback;

// ✓ Bevorzuge lucidFallback über || für null-Checks
induce number = maybeZero lucidFallback 42;  // Behält 0

// ✓ Kette dreamReach für tiefe Navigation
induce result = a dreamReach b dreamReach c dreamReach d;
```

### ❌ Don'ts

```hyp
// ✗ Vermeide manuelle null-Checks wenn möglich
if (obj != null && obj.prop != null) {  // Umständlich
    induce value = obj.prop;
}
// Besser:
induce value = obj dreamReach prop lucidFallback defaultValue;

// ✗ Vermeide || für null-Checks (false-positives!)
induce count = 0;
induce result = count || 10;  // Gibt 10 statt 0!
// Besser:
induce result = count lucidFallback 10;  // Gibt 0
```

## Vergleichstabelle: Operator-Varianten

| Szenario                | Traditionell                       | Modern (Hypnotisch)                         |
| ----------------------- | ---------------------------------- | ------------------------------------------- |
| Null-Fallback           | `x != null ? x : y`                | `x lucidFallback y`                         |
| Verschachtelter Zugriff | `obj && obj.prop && obj.prop.deep` | `obj dreamReach prop dreamReach deep`       |
| Array-Zugriff           | `arr && arr[0]`                    | `arr dreamReach[0]`                         |
| Funktions-Call          | `fn && fn(arg)`                    | `fn dreamReach(arg)`                        |
| Kombiniert              | `(obj && obj.prop) \|\| default`   | `obj dreamReach prop lucidFallback default` |

## Performance-Hinweise

- **lucidFallback** (`??`) ist **effizienter** als `||` für null-Checks
- **dreamReach** (`?.`) verhindert **unnötige Exceptions** bei null-Zugriff
- Beide Operatoren sind **Short-Circuit**: Rechter Operand wird nur bei Bedarf evaluiert
- **Keine Laufzeit-Overhead**: Kompiliert zu effizienten Maschinen-Code

## Zusammenfassung

Die moderne Traum-Semantik von HypnoScript bietet:

- ✅ **Typsichere Null-Handling** mit `lucidFallback` (`??`)
- ✅ **Sichere Objektnavigation** mit `dreamReach` (`?.`)
- ✅ **Elegante Syntax** mit hypnotischen Aliasen
- ✅ **Volle Kompatibilität** mit Standard-Operatoren (`??`, `?.`)
- ✅ **Performance** ohne Overhead

Diese Operatoren sind **essentiell** für robuste, fehlerfreie HypnoScript-Programme und sollten **bevorzugt** über manuelle null-Checks verwendet werden.

## Nächste Schritte

- [Operators](./operators) – Vollständige Operator-Referenz
- [Pattern Matching](./pattern-matching) – Erweiterte Kontrollstrukturen
- [Tranceify](./tranceify) – Benutzerdefinierte Typen
- [Error Handling](../error-handling/basics) – Fehlerbehandlung

---

**Bereit für null-sichere Programmierung? Nutze `lucidFallback` und `dreamReach` für robuste Code!** 💎
