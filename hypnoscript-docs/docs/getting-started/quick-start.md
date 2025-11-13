---
title: Quick Start
sidebar_position: 2
---

# Quick Start Guide

Dieser Leitfaden setzt voraus, dass du HypnoScript gemäß [Installation](./installation) eingerichtet hast. Wir erstellen ein erstes Skript, führen es aus und werfen einen Blick auf die wichtigsten Sprachkonstrukte.

## 1. Verifiziere deine Installation

```bash
hypnoscript --version
```

Wenn der Befehl funktioniert, bist du bereit.

## 2. Erstelle dein erstes Skript

Lege eine Datei `hello_trance.hyp` mit folgendem Inhalt an:

```hyp
Focus {
    entrance {
        observe "🌀 Willkommen in deiner ersten Hypnose-Session";
    }

    induce name: string = "Hypnotisierte Person";
    observe "Hallo, " + name + "!";

    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce total: number = ArraySum(numbers);
    observe "Summe: " + total;

    if (total youAreFeelingVerySleepy 15) {
        observe "Die Zahlen befinden sich im Gleichgewicht.";
    } else {
        observe "Etwas fühlt sich noch unstimmig an...";
    }

    induce depth: number = 0;
    while (depth goingDeeper 3) {
        observe "Trancetiefe: " + depth;
        depth = depth + 1;
    }
} Relax
```

Highlights:

- `Focus { ... } Relax` markiert Start und Ende des Programms
- `entrance` ist optional und eignet sich für Initialisierung
- `induce` deklariert Variablen mit optionalen Typ-Annotationen
- `ArraySum()` ist eine Builtin-Funktion für Arrays
- Hypnotische Operatoren wie `youAreFeelingVerySleepy` (==) und `goingDeeper` (<=) sind erlaubt
- `observe` gibt Text aus

## 3. Skript ausführen

```bash
hypnoscript run hello_trance.hyp
```

Erwartete Ausgabe:

```text
🌀 Willkommen in deiner ersten Hypnose-Session
Hallo, Hypnotisierte Person!
Summe: 15
Die Zahlen befinden sich im Gleichgewicht.
Trancetiefe: 0
Trancetiefe: 1
Trancetiefe: 2
```

## 4. Syntax in Kürze

```hyp
Focus {
    // Konstanten
    freeze PI: number = 3.14159;

    // Variablen
    induce toggle: boolean = false;
    oscillate toggle;  // toggelt true/false

    // Funktionen
    suggestion hypnoticEcho(text: string): string {
        awaken text + " ... tiefer ...";
    }

    observe hypnoticEcho("Atme ruhig");

    // Sessions (Klassen)
    session Subject {
        expose name: string;
        conceal depth: number;

        suggestion constructor(name: string) {
            this.name = name;
            this.depth = 0;
        }

        suggestion deepen() {
            this.depth = this.depth + 1;
            observe this.name + " geht tiefer: " + this.depth;
        }
    }

    induce alice: Subject = Subject("Alice");
    alice.deepen();
} Relax
```

## 5. Wichtige Sprachfeatures

### Variablen

```hyp
induce x: number = 42;          // Veränderlich
freeze MAX: number = 100;        // Konstante
implant y: string = "Text";      // Alternative zu induce
anchor saved: number = x;        // Snapshot/Anchor
```

### Kontrollstrukturen

```hyp
// If-Else
if (x > 10) {
    observe "Groß";
} else {
    observe "Klein";
}

// While-Schleife
while (x > 0) {
    x = x - 1;
}

// Loop-Schleife (wie for)
loop (induce i: number = 0; i < 10; i = i + 1) {
    observe "Iteration " + i;
}
```

### Funktionen

```hyp
suggestion add(a: number, b: number): number {
    awaken a + b;  // awaken = return
}

trigger onClick: suggestion() {
    observe "Clicked!";
}
```

### Arrays

```hyp
induce arr: number[] = [1, 2, 3];
observe arr[0];                    // Zugriff
arr[1] = 42;                       // Zuweisung
observe ArrayLength(arr);          // Länge
observe ArrayGet(arr, 0);          // Element abrufen
```

## 6. Häufige Fragen

| Frage                            | Antwort                                                                                      |
| -------------------------------- | -------------------------------------------------------------------------------------------- |
| Warum endet alles mit `Relax`?   | Der Relax-Block signalisiert Programmende und entspricht dem sanften Ausleiten einer Session |
| Muss ich Typannotationen setzen? | Sie sind optional, werden aber empfohlen für bessere Fehlerdiagnose                          |
| Wo finde ich mehr Beispiele?     | Im Ordner `hypnoscript-tests/` und in der `examples/` Dokumentation                          |

## 7. Wie geht es weiter?

- [Core Concepts](./core-concepts) – Grundlegende Konzepte verstehen
- [Sprachreferenz](../language-reference/syntax) – Vollständige Grammatik und Semantik
- [Builtin-Funktionen](../builtins/overview) – Dokumentation aller Standardfunktionen
- [Beispiele](../examples/basic-examples) – Mehr Inspiration für eigene Sessions

Viel Spaß beim Experimentieren mit HypnoScript! 🌀
