---
sidebar_position: 1
---

# Syntax

HypnoScript verwendet eine hypnotische Syntax, die sowohl intuitiv als auch mächtig ist. Lerne die grundlegenden Syntax-Regeln und Konzepte kennen.

## Grundstruktur

### Programm-Struktur

Jedes HypnoScript-Programm beginnt mit `Focus` und endet mit `Relax`:

```hyp
Focus {
    // Programm-Code hier
} Relax
```

### Entrance-Block

> ⚠️ `entrance`-Blöcke sind **nur auf Top-Level** erlaubt – direkt innerhalb von `Focus { ... }`. Wird der Block innerhalb einer Funktion, Session oder eines anderen Blocks deklariert, bricht der Parser mit der Meldung `'entrance' blocks are only allowed at the top level` ab.

Der `entrance`-Block wird beim Programmstart ausgeführt:

```hyp
Focus {
    entrance {
        observe "Programm gestartet";
    }
} Relax
```

### Finale-Block

Analog zum `entrance`-Block steht `finale { ... }` ausschließlich auf oberster Ebene zur Verfügung und eignet sich für Aufräumarbeiten. Auch hier erzwingt der Parser strikte Top-Level-Platzierung und meldet `'finale' blocks are only allowed at the top level`, falls der Block verschachtelt wird.

```hyp
Focus {
    entrance {
        observe "Setup";
    }

    finale {
        observe "Cleanup";
    }
} Relax
```

## Variablen und Zuweisungen

### Induce (Variablendeklaration)

Verwende `induce` um Variablen zu deklarieren und Werte zuzuweisen. Typ-Annotationen sind optional aber empfohlen:

```hyp
Focus {
    entrance {
        induce name: string = "HypnoScript";
        induce version: number = 1.0;
        induce isActive: boolean = true;

        observe "Name: " + name;
        observe "Version: " + version;
        observe "Aktiv: " + isActive;
    }
} Relax
```

### Datentypen

HypnoScript unterstützt verschiedene Datentypen:

```hyp
Focus {
    entrance {
        // Strings
        induce text: string = "Hallo Welt";

        // Zahlen (nur number Typ)
        induce integer: number = 42;
        induce decimal: number = 3.14159;

        // Boolean
        induce flag: boolean = true;

        // Arrays
        induce numbers: number[] = [1, 2, 3, 4, 5];
        induce names: string[] = ["Alice", "Bob", "Charlie"];

        // Records (mit tranceify definiert)
        // Siehe Records-Dokumentation für Details
    }
} Relax
```

## Ausgabe

### Observe (Ausgabe)

Verwende `observe` um Text auszugeben:

```hyp
Focus {
    entrance {
        observe "Einfache Ausgabe";
        observe "Mehrzeilige" + " " + "Ausgabe";

        induce name: string = "HypnoScript";
        observe "Willkommen bei " + name;
    }
} Relax
```

## Kontrollstrukturen

### If-Else

```hyp
Focus {
    entrance {
        induce age: number = 18;

        if (age >= 18) {
            observe "Volljährig";
        } else {
            observe "Minderjährig";
        }

        // Mit else if
        induce score: number = 85;
        if (score >= 90) {
            observe "Ausgezeichnet";
        } else if (score >= 80) {
            observe "Gut";
        } else if (score >= 70) {
            observe "Befriedigend";
        } else {
            observe "Verbesserungsbedarf";
        }
    }
} Relax
```

### While-Schleife

```hyp
Focus {
    entrance {
        induce counter: number = 1;

        while (counter <= 5) {
            observe "Zähler: " + counter;
            counter = counter + 1;
        }
    }
} Relax
```

### Loop-Schleife

`loop` kann wie eine klassische for-Schleife mit Kopf `loop (initialisierung; bedingung; update)` oder als Endlosschleife ohne Kopf verwendet werden. Die Variante `pendulum ( ... )` ist ein Alias für denselben Aufbau, verlangt jedoch immer eine Bedingung und eignet sich für "hin-und-her"-Konstrukte.

```hyp
Focus {
    entrance {
        // Loop-Schleife mit Zähler
        loop (induce i: number = 1; i <= 10; i = i + 1) {
            observe "Iteration " + i;
        }

        // Loop-Schleife über Array mit ArrayLength
        induce fruits: string[] = ["Apfel", "Birne", "Kirsche"];
        loop (induce i: number = 0; i < ArrayLength(fruits); i = i + 1) {
            observe "Frucht " + (i + 1) + ": " + ArrayGet(fruits, i);
        }

        // Pendulum benötigt immer einen Kopf und verhält sich wie loop (...)
        pendulum (induce phase: number = -2; phase <= 2; phase = phase + 1) {
            observe "Phase " + phase;
        }
    }
} Relax
```

## Funktionen

### Suggestion (Funktionsdefinition)

```hyp
Focus {
    // Funktion definieren
    suggestion greet(name: string) {
        observe "Hallo, " + name + "!";
    }

    suggestion add(a: number, b: number): number {
        awaken a + b;
    }

    suggestion factorial(n: number): number {
        if (n <= 1) {
            awaken 1;
        } else {
            awaken n * factorial(n - 1);
        }
    }

    entrance {
        // Funktionen aufrufen
        greet("HypnoScript");

        induce result: number = add(5, 3);
        observe "5 + 3 = " + result;

        induce fact: number = factorial(5);
        observe "5! = " + fact;
    }
} Relax
```

### Funktionen mit Rückgabewerten

```hyp
Focus {
    suggestion calculateArea(width, height) {
        awaken width * height;
    }

    suggestion isEven(number) {
        awaken number % 2 == 0;
    }

    suggestion getMax(a, b) {
        if (a > b) {
            awaken a;
        } else {
            return b;
        }
    }

    entrance {
        induce area = calculateArea(10, 5);
        observe "Fläche: " + area;

        induce check = isEven(42);
        observe "42 ist gerade: " + check;

        induce maximum = getMax(15, 8);
        observe "Maximum: " + maximum;
    }
} Relax;
```

## Arrays

### Array-Operationen

```hyp
Focus {
    entrance {
        // Array erstellen
        induce numbers = [1, 2, 3, 4, 5];

        // Elemente abrufen
        induce first = ArrayGet(numbers, 0);
        observe "Erstes Element: " + first;

        // Elemente setzen
        ArraySet(numbers, 2, 99);
        observe "Nach Änderung: " + numbers;

        // Array-Länge
        induce length = ArrayLength(numbers);
        observe "Array-Länge: " + length;

        // Array durchsuchen
        for (induce i = 0; i < Length(numbers); induce i = i + 1) {
            observe "Element " + i + ": " + ArrayGet(numbers, i);
        }
    }
} Relax;
```

### Array-Funktionen

```hyp
Focus {
    entrance {
        induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];

        // Sortieren
        induce sorted = ArraySort(numbers);
        observe "Sortiert: " + sorted;

        // Summe
        induce sum = ArraySum(numbers);
        observe "Summe: " + sum;

        // Durchschnitt
        induce avg = AverageArray(numbers);
        observe "Durchschnitt: " + avg;

        // Mischen
        induce shuffled = ShuffleArray(numbers);
        observe "Gemischt: " + shuffled;
    }
} Relax;
```

## Records (Objekte)

### Record-Erstellung und -Zugriff

```hyp
Focus {
    entrance {
        // Record erstellen
        induce person = {
            name: "Max Mustermann",
            age: 30,
            city: "Berlin",
            hobbies: ["Programmierung", "Lesen", "Sport"]
        };

        // Eigenschaften abrufen
        observe "Name: " + person.name;
        observe "Alter: " + person.age;
        observe "Stadt: " + person.city;

        // Eigenschaften ändern
        induce person.age = 31;
        observe "Neues Alter: " + person.age;

        // Verschachtelte Records
        induce company = {
            name: "HypnoScript GmbH",
            address: {
                street: "Musterstraße 123",
                city: "Berlin",
                zip: "10115"
            },
            employees: [
                {name: "Alice", role: "Developer"},
                {name: "Bob", role: "Designer"}
            ]
        };

        observe "Firma: " + company.name;
        observe "Adresse: " + company.address.street;
        observe "Erster Mitarbeiter: " + company.employees[0].name;
    }
} Relax;
```

## Sessions

### Session-Erstellung

```hyp
Focus {
    entrance {
        // Session erstellen
        induce session = Session("MeineSession");

        // Session-Variablen setzen
        SessionSet(session, "user", "Max");
        SessionSet(session, "level", 5);
        SessionSet(session, "preferences", {
            theme: "dark",
            language: "de"
        });

        // Session-Variablen abrufen
        induce user = SessionGet(session, "user");
        induce level = SessionGet(session, "level");
        induce prefs = SessionGet(session, "preferences");

        observe "Benutzer: " + user;
        observe "Level: " + level;
        observe "Theme: " + prefs.theme;
    }
} Relax;
```

## Tranceify

### Tranceify für hypnotische Anwendungen

```hyp
Focus {
    entrance {
        // Tranceify-Session starten
        Tranceify("Entspannung") {
            observe "Du entspannst dich jetzt...";
            observe "Atme tief ein...";
            observe "Und aus...";
            observe "Du fühlst dich ruhig und entspannt...";
        }

        // Mit Parametern
        induce clientName = "Anna";
        Tranceify("Induktion", clientName) {
            observe "Hallo " + clientName + ", willkommen zu deiner Sitzung...";
            observe "Du bist in einem sicheren Raum...";
            observe "Du kannst dich vollständig entspannen...";
        }
    }
} Relax;
```

## Imports

### Module importieren

```hyp
import "utils.hyp";
import "math.hyp" as MathUtils;

Focus {
    entrance {
        // Funktionen aus importierten Modulen verwenden
        induce result = MathUtils.calculate(10, 5);
        observe "Ergebnis: " + result;
    }
} Relax;
```

## Assertions

### Assertions für Tests

```hyp
Focus {
    entrance {
        induce expected = 10;
        induce actual = 5 + 5;

        // Assertion - Programm stoppt bei Fehler
        assert actual == expected : "Erwartet 10, aber erhalten " + actual;

        observe "Test erfolgreich!";

        // Weitere Assertions
        induce name = "HypnoScript";
        assert Length(name) > 0 : "Name darf nicht leer sein";
        assert Length(name) <= 50 : "Name zu lang";

        observe "Alle Tests bestanden!";
    }
} Relax;
```

## Kommentare

### Kommentare in HypnoScript

```hyp
Focus {
    // Einzeiliger Kommentar

    entrance {
        induce name = "HypnoScript"; // Inline-Kommentar

        /*
         * Mehrzeiliger Kommentar
         * Kann über mehrere Zeilen gehen
         * Nützlich für längere Erklärungen
         */

        observe "Hallo " + name;
    }
} Relax;
```

## Operatoren

### Arithmetische Operatoren

```hyp
Focus {
    entrance {
        induce a = 10;
        induce b = 3;

        observe "Addition: " + (a + b);        // 13
        observe "Subtraktion: " + (a - b);     // 7
        observe "Multiplikation: " + (a * b);  // 30
        observe "Division: " + (a / b);        // 3.333...
        observe "Modulo: " + (a % b);          // 1
        observe "Potenz: " + (a ^ b);          // 1000
    }
} Relax;
```

### Vergleichsoperatoren

```hyp
Focus {
    entrance {
        induce x = 5;
        induce y = 10;

        observe "Gleich: " + (x == y);         // false
        observe "Ungleich: " + (x != y);       // true
        observe "Kleiner: " + (x < y);         // true
        observe "Größer: " + (x > y);          // false
        observe "Kleiner gleich: " + (x <= y); // true
        observe "Größer gleich: " + (x >= y);  // false
    }
} Relax;
```

### Logische Operatoren

```hyp
Focus {
    entrance {
        induce a = true;
        induce b = false;

        observe "UND: " + (a && b);            // false
        observe "ODER: " + (a || b);           // true
        observe "NICHT: " + (!a);              // false
        observe "XOR: " + (a ^ b);             // true
    }
} Relax;
```

## Best Practices

### Code-Formatierung

```hyp
Focus {
    // Funktionen am Anfang definieren
    suggestion calculateSum(a, b) {
        awaken a + b;
    }

    suggestion validateInput(value) {
        awaken value > 0 && value <= 100;
    }

    entrance {
        // Hauptlogik im entrance-Block
        induce input = 42;

        if (validateInput(input)) {
            induce result = calculateSum(input, 10);
            observe "Ergebnis: " + result;
        } else {
            observe "Ungültige Eingabe";
        }
    }
} Relax;
```

### Namenskonventionen

- **Variablen**: camelCase (`userName`, `totalCount`)
- **Funktionen**: camelCase (`calculateArea`, `validateInput`)
- **Konstanten**: UPPER_SNAKE_CASE (`MAX_RETRY_COUNT`)
- **Sessions**: PascalCase (`UserSession`, `GameState`)

### Fehlerbehandlung

```hyp
Focus {
    entrance {
        induce input = "abc";

        // Typprüfung
        if (IsNumber(input)) {
            induce number = ToNumber(input);
            observe "Zahl: " + number;
        } else {
            observe "Fehler: Keine gültige Zahl";
        }

        // Array-Zugriff prüfen
        induce array = [1, 2, 3];
        induce index = 5;

        if (index >= 0 && index < Length(array)) {
            induce value = ArrayGet(array, index);
            observe "Wert: " + value;
        } else {
            observe "Fehler: Index außerhalb des Bereichs";
        }
    }
} Relax;
```

## Nächste Schritte

- [Variablen und Datentypen](./variables) - Detaillierte Informationen zu Variablen
- [Operatoren](./operators) - Alle verfügbaren Operatoren
- [Kontrollstrukturen](./control-flow) - If, While, For und mehr
- [Funktionen](./functions) - Funktionsdefinition und -aufruf
- [Beispiele](../examples/basic-examples) - Praktische Beispiele

---

**Beherrschst du die Grundlagen? Dann lerne mehr über [Variablen und Datentypen](./variables)!** 📚
