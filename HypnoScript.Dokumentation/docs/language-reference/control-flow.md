---
sidebar_position: 4
---

# Kontrollstrukturen

HypnoScript bietet verschiedene Kontrollstrukturen für bedingte Ausführung und Schleifen.

## If-Else Anweisungen

### Einfache If-Anweisung

```hyp
if (bedingung) {
    // Code wird ausgeführt, wenn bedingung true ist
}
```

### If-Else Anweisung

```hyp
if (bedingung) {
    // Code wenn bedingung true ist
} else {
    // Code wenn bedingung false ist
}
```

### If-Else If-Else Anweisung

```hyp
if (bedingung1) {
    // Code wenn bedingung1 true ist
} else if (bedingung2) {
    // Code wenn bedingung2 true ist
} else {
    // Code wenn alle bedingungen false sind
}
```

### Beispiele

```hyp
Focus {
    entrance {
        induce alter = 18;

        if (alter >= 18) {
            observe "Volljährig";
        } else {
            observe "Minderjährig";
        }

        induce punktzahl = 85;
        if (punktzahl >= 90) {
            observe "Ausgezeichnet";
        } else if (punktzahl >= 80) {
            observe "Gut";
        } else if (punktzahl >= 70) {
            observe "Befriedigend";
        } else {
            observe "Verbesserungsbedarf";
        }
    }
} Relax;
```

## While-Schleifen

### Syntax

```hyp
while (bedingung) {
    // Code wird wiederholt, solange bedingung true ist
}
```

### Beispiele

```hyp
Focus {
    entrance {
        // Einfache While-Schleife
        induce zaehler = 1;
        while (zaehler <= 5) {
            observe "Zähler: " + zaehler;
            induce zaehler = zaehler + 1;
        }

        // While-Schleife mit Array
        induce zahlen = [1, 2, 3, 4, 5];
        induce index = 0;
        while (index < ArrayLength(zahlen)) {
            observe "Zahl " + (index + 1) + ": " + ArrayGet(zahlen, index);
            induce index = index + 1;
        }
    }
} Relax;
```

## For-Schleifen

### Syntax

```hyp
for (initialisierung; bedingung; inkrement) {
    // Code wird wiederholt
}
```

### Beispiele

```hyp
Focus {
    entrance {
        // Standard For-Schleife
        for (induce i = 1; i <= 10; induce i = i + 1) {
            observe "Iteration " + i;
        }

        // For-Schleife über Array
        induce obst = ["Apfel", "Banane", "Orange"];
        for (induce i = 0; i < ArrayLength(obst); induce i = i + 1) {
            observe "Obst " + (i + 1) + ": " + ArrayGet(obst, i);
        }

        // Rückwärts zählen
        for (induce i = 10; i >= 1; induce i = i - 1) {
            observe "Countdown: " + i;
        }
    }
} Relax;
```

## Verschachtelte Kontrollstrukturen

```hyp
Focus {
    entrance {
        induce zahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
            induce zahl = ArrayGet(zahlen, i);

            if (zahl % 2 == 0) {
                observe zahl + " ist gerade";
            } else {
                observe zahl + " ist ungerade";
            }

            if (zahl < 5) {
                observe "  - Kleine Zahl";
            } else if (zahl < 8) {
                observe "  - Mittlere Zahl";
            } else {
                observe "  - Große Zahl";
            }
        }
    }
} Relax;
```

## Break und Continue

### Break

Beendet die aktuelle Schleife sofort:

```hyp
Focus {
    entrance {
        for (induce i = 1; i <= 10; induce i = i + 1) {
            if (i == 5) {
                break; // Schleife wird bei i=5 beendet
            }
            observe "Zahl: " + i;
        }
        observe "Schleife beendet";
    }
} Relax;
```

### Continue

Überspringt den aktuellen Schleifendurchlauf:

```hyp
Focus {
    entrance {
        for (induce i = 1; i <= 10; induce i = i + 1) {
            if (i % 2 == 0) {
                continue; // Gerade Zahlen werden übersprungen
            }
            observe "Ungerade Zahl: " + i;
        }
    }
} Relax;
```

## Best Practices

### Klare Bedingungen

```hyp
// Gut
if (alter >= 18 && punktzahl >= 70) {
    observe "Zugelassen";
}

// Schlecht
if (alter >= 18 && punktzahl >= 70 == true) {
    observe "Zugelassen";
}
```

### Effiziente Schleifen

```hyp
// Gut - Array-Länge einmal berechnen
induce laenge = ArrayLength(zahlen);
for (induce i = 0; i < laenge; induce i = i + 1) {
    // Code
}

// Schlecht - Array-Länge bei jedem Durchlauf berechnen
for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
    // Code
}
```

### Vermeidung von Endlosschleifen

```hyp
// Sicher - mit Break-Bedingung
induce zaehler = 0;
while (true) {
    induce zaehler = zaehler + 1;
    if (zaehler > 100) {
        break;
    }
    // Code
}
```

## Beispiele für komplexe Kontrollstrukturen

### Zahlenraten-Spiel

```hyp
Focus {
    entrance {
        induce zielZahl = 42;
        induce versuche = 0;
        induce maxVersuche = 10;

        while (versuche < maxVersuche) {
            induce versuche = versuche + 1;
            induce rateZahl = 25 + versuche * 2; // Vereinfachte Eingabe

            if (rateZahl == zielZahl) {
                observe "Gewonnen! Die Zahl war " + zielZahl;
                observe "Versuche: " + versuche;
                break;
            } else if (rateZahl < zielZahl) {
                observe "Zu niedrig! Versuch " + versuche;
            } else {
                observe "Zu hoch! Versuch " + versuche;
            }
        }

        if (versuche >= maxVersuche) {
            observe "Verloren! Die Zahl war " + zielZahl;
        }
    }
} Relax;
```

### Array-Verarbeitung mit Bedingungen

```hyp
Focus {
    entrance {
        induce zahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        induce geradeSumme = 0;
        induce ungeradeAnzahl = 0;

        for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
            induce zahl = ArrayGet(zahlen, i);

            if (zahl % 2 == 0) {
                induce geradeSumme = geradeSumme + zahl;
            } else {
                induce ungeradeAnzahl = ungeradeAnzahl + 1;
            }
        }

        observe "Summe der geraden Zahlen: " + geradeSumme;
        observe "Anzahl der ungeraden Zahlen: " + ungeradeAnzahl;
    }
} Relax;
```

## Nächste Schritte

- [Funktionen](./functions) - Funktionsdefinition und -aufruf
- [Sessions](./sessions) - Session-Management
- [Tranceify](./tranceify) - Hypnotische Anwendungen
- [Assertions](./assertions) - Test-Assertions

---

**Beherrschst du die Kontrollstrukturen? Dann lerne [Funktionen](./functions) kennen!** 🔧
