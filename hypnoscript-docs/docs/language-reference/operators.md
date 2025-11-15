# Operatoren

HypnoScript unterstützt Standard-Operatoren sowie hypnotische Synonyme für vergleichende und logische Operatoren. Alle Operatoren sind in der Rust-Implementierung vollständig typsicher.

## Arithmetische Operatoren

| Operator | Bedeutung      | Typ-Anforderung | Beispiel | Ergebnis |
| -------- | -------------- | --------------- | -------- | -------- |
| +        | Addition       | number          | 2 + 3    | 5        |
| -        | Subtraktion    | number          | 5 - 2    | 3        |
| \*       | Multiplikation | number          | 4 \* 2   | 8        |
| /        | Division       | number          | 8 / 2    | 4        |
| %        | Modulo (Rest)  | number          | 7 % 3    | 1        |

**String-Konkatenation:** Der `+` Operator funktioniert auch für Strings:

```hyp
induce text: string = "Hallo " + "Welt";  // "Hallo Welt"
induce mixed: string = "Zahl: " + 42;     // "Zahl: 42"
```

> ⚠️ **Achtung:** Sobald einer der Operanden ein String ist, werden alle anderen Werte implizit in Strings umgewandelt (intern via `to_string()`). Dadurch entstehen z. B. Ergebnisse wie `null + "text" -> "nulltext"` oder `42 + "px" -> "42px"`. Wenn du striktere Typkontrollen erwartest, konvertiere Werte explizit oder prüfe den Typ vor der Verwendung von `+`.

## Vergleichsoperatoren

### Standard-Operatoren (Vergleich)

| Operator | Bedeutung      | Beispiel | Ergebnis |
| -------- | -------------- | -------- | -------- |
| ==       | Gleich         | 3 == 3   | true     |
| !=       | Ungleich       | 3 != 4   | true     |
| >        | Größer         | 5 > 2    | true     |
| <        | Kleiner        | 2 < 5    | true     |
| >=       | Größer gleich  | 3 >= 2   | true     |
| <=       | Kleiner gleich | 2 <= 2   | true     |

### Hypnotische Synonyme (Vergleich)

HypnoScript bietet hypnotische Synonyme für alle Vergleichsoperatoren:

| Hypnotisches Synonym    | Standard | Bedeutung      | Status       |
| ----------------------- | -------- | -------------- | ------------ |
| youAreFeelingVerySleepy | ==       | Gleich         | ✅ Empfohlen |
| youCannotResist         | !=       | Ungleich       | ✅ Empfohlen |
| lookAtTheWatch          | >        | Größer         | ✅ Empfohlen |
| fallUnderMySpell        | <        | Kleiner        | ✅ Empfohlen |
| yourEyesAreGettingHeavy | >=       | Größer gleich  | ✅ Empfohlen |
| goingDeeper             | <=       | Kleiner gleich | ✅ Empfohlen |

**Legacy-Operatoren** (veraltet, aber unterstützt):

| Hypnotisches Synonym | Standard | Hinweis                                           |
| -------------------- | -------- | ------------------------------------------------- |
| notSoDeep            | !=       | ⚠️ Verwende stattdessen `youCannotResist`         |
| deeplyGreater        | >=       | ⚠️ Verwende stattdessen `yourEyesAreGettingHeavy` |
| deeplyLess           | <=       | ⚠️ Verwende stattdessen `goingDeeper`             |

## Logische Operatoren

### Standard-Operatoren (Logik)

| Operator | Bedeutung | Beispiel        | Ergebnis |
| -------- | --------- | --------------- | -------- |
| &&       | Und       | true && false   | false    |
| \|\|     | Oder      | true \|\| false | true     |
| !        | Nicht     | !true           | false    |

### Hypnotische Synonyme (Logik)

| Hypnotisches Synonym | Standard | Bedeutung      |
| -------------------- | -------- | -------------- |
| underMyControl       | &&       | Logisches UND  |
| resistanceIsFutile   | \|\|     | Logisches ODER |

**Hinweis:** Es gibt kein hypnotisches Synonym für den `!` (Nicht)-Operator.

## Priorität der Operatoren

Von höchster zu niedrigster Priorität:

1. **Unäre Operatoren:** `!`, `-` (negativ)
2. **Multiplikativ:** `*`, `/`, `%`
3. **Additiv:** `+`, `-`
4. **Vergleich:** `<`, `<=`, `>`, `>=` (und hypnotische Synonyme)
5. **Gleichheit:** `==`, `!=` (und hypnotische Synonyme)
6. **Logisches UND:** `&&` (oder `underMyControl`)
7. **Logisches ODER:** `||` (oder `resistanceIsFutile`)

Verwende Klammern `( )` für explizite Gruppierung.

## Array-Zugriff und Zuweisung

Arrays werden mit eckigen Klammern `[ ]` indiziert (0-basiert):

```hyp
induce arr: number[] = [10, 20, 30];
observe arr[0];       // Ausgabe: 10
observe arr[2];       // Ausgabe: 30

arr[1] = 42;          // Zuweisung
observe arr[1];       // Ausgabe: 42
```

Für erweiterte Array-Operationen siehe [Array Builtin-Funktionen](../builtins/array-functions).

## Zuweisungsoperator

Der einfache Zuweisungsoperator `=` wird für Zuweisungen verwendet:

```hyp
induce x: number = 5;
x = x + 1;  // 6
x = 10;     // Neuzuweisung
```

**Wichtig:** Zusammengesetzte Zuweisungsoperatoren (`+=`, `-=`, `*=`, etc.) sind **nicht implementiert**.

Verwende stattdessen:

````hyp
// FALSCH: x += 5;
// RICHTIG:
x = x + 5;

## Beispiele

### Standard-Operatoren

```hyp
Focus {
    entrance {
        induce a: number = 10;
        induce b: number = 3;

        observe "a + b = " + (a + b);    // 13
        observe "a - b = " + (a - b);    // 7
        observe "a * b = " + (a * b);    // 30
        observe "a / b = " + (a / b);    // 3.333...
        observe "a % b = " + (a % b);    // 1

        observe "a == b: " + (a == b);   // false
        observe "a > b: " + (a > b);     // true
        observe "a <= 10: " + (a <= 10); // true
    }
} Relax
````

### Hypnotische Synonyme

```hyp
Focus {
    entrance {
        induce x: number = 10;
        induce y: number = 10;

        if (x youAreFeelingVerySleepy y) {
            observe "x ist gleich y!";
        }

        if (x lookAtTheWatch 5 underMyControl y yourEyesAreGettingHeavy 8) {
            observe "Beide Bedingungen sind wahr!";
        }

        if (x fallUnderMySpell 20 resistanceIsFutile y youAreFeelingVerySleepy 10) {
            observe "Mindestens eine Bedingung ist wahr!";
        }
    }
} Relax
```

### Array-Operationen

```hyp
Focus {
    entrance {
        induce numbers: number[] = [1, 2, 3, 4, 5];

        observe "Erstes Element: " + numbers[0];
        observe "Array-Länge: " + ArrayLength(numbers);

        numbers[2] = 99;
        observe "Geändertes Element: " + numbers[2];
    }
} Relax
```

### Operatorkombinationen

```hyp
Focus {
    entrance {
        induce x: number = 10;
        induce y: number = 20;
        induce z: number = 5;

        // Komplexe Ausdrücke mit Prioritäten
        induce result1: number = x + y * z;     // 110 (Multiplikation zuerst)
        induce result2: number = (x + y) * z;   // 150 (Klammern zuerst)

        observe "result1 = " + result1;
        observe "result2 = " + result2;

        // Logische Operatoren kombinieren
        if (x lookAtTheWatch 5 underMyControl y lookAtTheWatch 15) {
            observe "x > 5 UND y > 15";
        }

        if (x fallUnderMySpell 5 resistanceIsFutile y yourEyesAreGettingHeavy 20) {
            observe "x < 5 ODER y >= 20";
        }

        // Negation
        induce isActive: boolean = true;
        if (!isActive) {
            observe "Nicht aktiv";
        } else {
            observe "Aktiv";
        }
    }
} Relax
```

## Best Practices

1. **Verwende Klammern** bei komplexen Ausdrücken für bessere Lesbarkeit
2. **Nutze hypnotische Operatoren** konsequent für thematische Konsistenz
3. **Vermeide Legacy-Operatoren** (`notSoDeep`, `deeplyGreater`, `deeplyLess`)
4. **Typ-Konsistenz** beachten: Vergleiche nur Werte gleichen Typs
5. **Explizite Konvertierung** wenn nötig mit Builtin-Funktionen (`ToInt`, `ToDouble`, `ToString`)

## Siehe auch

- [Variablen](./variables) - Variablendeklaration und -zuweisung
- [Kontrollstrukturen](./control-flow) - if, while, loop
- [Builtin-Funktionen](../builtins/overview) - Verfügbare Standardfunktionen
- [Syntax](./syntax) - Vollständige Sprachsyntax
