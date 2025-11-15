---
sidebar_position: 1
---

# Beispiele: Utility-Funktionen

Diese Seite zeigt praxisnahe Beispiele für den Einsatz von Utility-Funktionen in HypnoScript. Die Beispiele sind kommentiert und können direkt übernommen oder angepasst werden.

## Dynamische Typumwandlung und Validierung

```hyp
Focus {
    entrance {
        induce input = "42";
        induce n = ToNumber(input);
        if (IsNumber(n)) {
            observe "Eingegebene Zahl: " + n;
        } else {
            observe "Ungültige Eingabe!";
        }
    }
} Relax;
```

## Zufällige Auswahl und Mischen

```hyp
Focus {
    entrance {
        induce namen = ["Anna", "Ben", "Carla", "Dieter"];
        induce gewinner = Sample(namen, 1);
        observe "Gewinner: " + gewinner;
        induce gemischt = Shuffle(namen);
        observe "Zufällige Reihenfolge: " + gemischt;
    }
} Relax;
```

## Zeitmessung und Sleep

```hyp
Focus {
    entrance {
        induce start = Timestamp();
        Sleep(500); // 0,5 Sekunden warten
        induce ende = Timestamp();
        observe "Dauer: " + (ende - start) + " Sekunden";
    }
} Relax;
```

## Array-Transformationen

```hyp
Focus {
    entrance {
        induce zahlen = [1,2,3,4,5,2,3,4];
        induce unique = Unique(zahlen);
        observe "Ohne Duplikate: " + unique;
        induce sortiert = Sort(unique);
        observe "Sortiert: " + sortiert;
        induce gepaart = Zip(unique, ["a","b","c","d","e"]);
        observe "Gepaart: " + gepaart;
    }
} Relax;
```

## Fehlerbehandlung mit Try

```hyp
Focus {
    suggestion safeDivide(a, b) {
        awaken Try(a / b, "Fehler: Division durch Null");
    }
    entrance {
        observe safeDivide(10, 2); // 5
        observe safeDivide(10, 0); // "Fehler: Division durch Null"
    }
} Relax;
```

## JSON-Parsing und -Erzeugung

```hyp
Focus {
    entrance {
        induce jsonString = '{"name": "Max", "age": 30}';
        induce obj = ParseJSON(jsonString);
        observe "Name: " + obj.name;
        observe "Alter: " + obj.age;

        induce arr = [1,2,3];
        induce jsonArr = StringifyJSON(arr);
        observe "JSON-Array: " + jsonArr;
    }
} Relax;
```

## Range und Repeat

```hyp
Focus {
    entrance {
        induce r = Range(1, 5);
        observe "Range: " + r; // [1,2,3,4,5]
        induce rep = Repeat("A", 3);
        observe "Repeat: " + rep; // ["A","A","A"]
    }
} Relax;
```

## Kombinierte Utility-Workflows

```hyp
Focus {
    entrance {
        // Eingabe validieren und verarbeiten
        induce input = "15";
        induce n = ToNumber(input);
        if (IsNumber(n) && n > 10) {
            observe "Eingabe ist eine Zahl > 10: " + n;
        } else {
            observe "Ungültige oder zu kleine Zahl!";
        }

        // Zufällige Auswahl aus Range
        induce zahlen = Range(1, 100);
        induce zufall = Sample(zahlen, 5);
        observe "5 zufällige Zahlen: " + zufall;

        // Array-Transformationen kombinieren
        induce arr = [1,2,2,3,4,4,5];
        induce clean = Sort(Unique(arr));
        observe "Sortiert & eindeutig: " + clean;
    }
} Relax;
```

---

**Siehe auch:**

- [Utility-Funktionen Referenz](../builtins/utility-functions)
- [System-Funktionen Beispiele](./system-examples)
