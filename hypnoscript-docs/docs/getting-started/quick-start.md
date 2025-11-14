---
title: Quick Start
sidebar_position: 2
---

Dieser Leitfaden setzt voraus, dass du HypnoScript gemäß [Installation](./installation) eingerichtet hast. Wir erstellen ein erstes Skript, führen es aus und streifen die wichtigsten Sprachelemente.

## 1. Installation prüfen

```bash
hypnoscript version
```

Der Befehl sollte Versions- und Featureinformationen ausgeben.

## 2. Erstes Skript anlegen

Speichere den folgenden Code als `hello_trance.hyp`:

```hyp
Focus {
    entrance {
        observe "🌀 Willkommen in deiner ersten Hypnose-Session";
    }

    induce name: string = "Hypnotisierte Person";
    observe "Hallo, " + name + "!";

    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce total: number = ArraySum(numbers);
    observe "Summe: " + ToString(total);

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

- `Focus { ... } Relax` markiert Start und Ende des Programms.
- `entrance` eignet sich für Initialisierung.
- `induce` deklariert Variablen mit optionaler Typannotation.
- Hypnotische Operatoren wie `youAreFeelingVerySleepy` (`==`) oder `goingDeeper` (`<=`) sind voll unterstützt.
- `ArraySum` und `ToString` stammen aus der Standardbibliothek.

## 3. Skript ausführen

```bash
hypnoscript run hello_trance.hyp
```

Die Ausgabe sollte die Begrüßung, die Summe und den kleinen While-Loop zeigen.

## 4. Syntax in Kürze

```hyp
Focus {
    freeze PI: number = 3.14159;

    induce toggle: boolean = false;
    oscillate toggle; // toggelt true/false

    suggestion hypnoticEcho(text: string): string {
        awaken text + " ... tiefer ...";
    }

    observe hypnoticEcho("Atme ruhig");

    session Subject {
        expose name: string;
        conceal depth: number;

        suggestion constructor(name: string) {
            this.name = name;
            this.depth = 0;
        }

        expose suggestion deepen() {
            this.depth = this.depth + 1;
            observe this.name + " geht tiefer: " + this.depth;
        }
    }

    induce alice: Subject = Subject("Alice");
    alice.deepen();
} Relax
```

## 5. Kontrollstrukturen

```hyp
if (total lookAtTheWatch 10) {
    observe "größer als 10";
} else if (total youCannotResist 10) {
    observe "ungleich 10";
} else {
    observe "genau 10";
}

while (depth fallUnderMySpell 5) {
    depth = depth + 1;
}

loop {
    observe "Endlosschleife";
    snap; // beendet die Schleife
}

loop (induce i: number = 0; i < 3; i = i + 1) {
    observe "Loop-Iteration " + i;
}

pendulum (induce tick: number = 10; tick underMyControl 15; tick = tick + 1) {
    observe "Pendulum tick " + tick;
}
```

- `snap` ist Synonym für `break`.
- `sink` ist Synonym für `continue`.
- `loop` akzeptiert optional einen Kopf `loop (init; condition; update)` und fällt ohne Klammern auf die klassische Endlosschleife zurück.
- `pendulum` ist ein Alias für die Kopf-Variante und verlangt stets eine Bedingung.
- `deepFocus` kann nach der If-Bedingung stehen: `if (x > 0) deepFocus { ... }`.

## 6. Funktionen und Trigger

```hyp
suggestion add(a: number, b: number): number {
    awaken a + b;
}

trigger onClick = suggestion(label: string) {
    observe "Trigger: " + label;
}

observe ToString(add(2, 3));
onClick("Demo");
```

- `awaken` ist das hypnotische Pendant zu `return`.
- Trigger verhalten sich wie benannte Callback-Funktionen. Sie werden wie normale Funktionen aufgerufen.

## 7. Arrays & Builtins

```hyp
induce arr: number[] = [1, 2, 3];
observe arr[0];              // Direktzugriff
arr[1] = 42;                 // Zuweisung

observe ArrayLength(arr);    // 3
observe ArrayGet(arr, 2);    // 3
observe ArrayJoin(arr, ", ");
```

Weitere nützliche Funktionen:

- Strings: `ToUpper`, `Trim`, `Split`, `Replace`
- Mathe: `Sqrt`, `Clamp`, `Factorial`, `IsPrime`
- System: `GetOperatingSystem`, `GetArgs`
- Dateien: `ReadFile`, `WriteFile`, `ListDirectory`

Alle verfügbaren Builtins listet `hypnoscript builtins` auf.

## 8. Häufige Fragen

| Frage                            | Antwort                                                                                                                                          |
| -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| Warum endet alles mit `Relax`?   | Der Relax-Block markiert das sichere Ausleiten – er ist fester Bestandteil der Grammatik.                                                        |
| Muss ich Typannotationen setzen? | Nein, aber sie verbessern Fehlermeldungen und die Autovervollständigung.                                                                         |
| Gibt es for-Schleifen?           | Ja, `loop (induce i = 0; i < n; i = i + 1) { ... }` bildet eine klassische for-Schleife ab; ohne Kopf bleibt `loop { ... }` eine Endlosschleife. |

## 9. Wie geht es weiter?

- [Core Concepts](./core-concepts) – Konzepte und Toolchain im Überblick
- [CLI Basics](./cli-basics) – Alle Subcommands und Optionen
- [Sprachreferenz](../language-reference/syntax) – Ausführliche Grammatik & Beispiele
- [Builtin-Übersicht](../builtins/overview) – Funktionen nach Kategorien

Viel Spaß beim Experimentieren mit HypnoScript! 🌀
