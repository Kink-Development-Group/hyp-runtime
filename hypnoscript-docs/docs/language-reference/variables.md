---
sidebar_position: 2
---

# Variablen und Datentypen

In HypnoScript werden Variablen mit dem Schlüsselwort `induce` deklariert. Die Sprache ist dynamisch typisiert, unterstützt aber verschiedene primitive und komplexe Datentypen.

## Variablen deklarieren

```hyp
induce name = "HypnoScript";
induce zahl = 42;
induce pi = 3.1415;
induce aktiv = true;
induce liste = [1, 2, 3];
induce person = { name: "Max", age: 30 };
```

## Unterstützte Datentypen

| Typ     | Beispiel                   | Beschreibung                     |
| ------- | -------------------------- | -------------------------------- |
| String  | "Hallo Welt"               | Zeichenkette                     |
| Integer | 42                         | Ganzzahl                         |
| Double  | 3.1415                     | Gleitkommazahl                   |
| Boolean | true, false                | Wahrheitswert                    |
| Array   | [1, 2, 3]                  | Liste von Werten                 |
| Record  | \{ name: "Max", age: 30 \} | Objekt mit Schlüssel/Wert-Paaren |
| Null    | null                       | Leerer Wert                      |

## Typumwandlung

Viele Builtins unterstützen automatische Typumwandlung. Für explizite Umwandlung:

```hyp
induce zahl = "42";
induce alsZahl = ToNumber(zahl); // 42
induce alsString = ToString(alsZahl); // "42"
```

## Variablen-Sichtbarkeit

- Variablen sind im aktuellen Block und in Unterblöcken sichtbar.
- Funktionsparameter sind nur innerhalb der Funktion sichtbar.

## Konstanten

Konstanten werden wie Variablen behandelt, aber per Konvention in Großbuchstaben geschrieben:

```hyp
induce MAX_COUNT = 100;
```

## Best Practices

- Verwende sprechende Namen (z.B. `benutzerName`, `maxWert`)
- Nutze Arrays und Records für strukturierte Daten
- Initialisiere Variablen immer mit einem Wert

## Beispiele

```hyp
Focus {
    entrance {
        induce greeting = "Hallo";
        induce count = 5;
        induce values = [1, 2, 3, 4, 5];
        induce user = { name: "Anna", age: 28 };
        observe greeting + ", " + user.name + "!";
        observe "Werte: " + values;
    }
} Relax;
```
