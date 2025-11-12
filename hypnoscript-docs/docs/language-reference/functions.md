---
sidebar_position: 5
---

# Funktionen

Funktionen in HypnoScript werden mit dem Schlüsselwort `Trance` definiert und ermöglichen die Modularisierung und Wiederverwendung von Code.

## Funktionsdefinition

### Grundlegende Syntax

```hyp
Trance funktionsName(parameter1, parameter2) {
    // Funktionskörper
    return wert; // Optional
}
```

### Einfache Funktion ohne Parameter

```hyp
Focus {
    Trance begruessung() {
        observe "Hallo, HypnoScript!";
    }

    entrance {
        begruessung();
    }
} Relax;
```

### Funktion mit Parametern

```hyp
Focus {
    Trance begruesse(name) {
        observe "Hallo, " + name + "!";
    }

    entrance {
        begruesse("Max");
        begruesse("Anna");
    }
} Relax;
```

### Funktion mit Rückgabewert

```hyp
Focus {
    Trance addiere(a, b) {
        return a + b;
    }

    Trance istGerade(zahl) {
        return zahl % 2 == 0;
    }

    entrance {
        induce summe = addiere(5, 3);
        observe "5 + 3 = " + summe;

        induce check = istGerade(42);
        observe "42 ist gerade: " + check;
    }
} Relax;
```

## Parameter

### Mehrere Parameter

```hyp
Focus {
    Trance rechteckFlaeche(breite, hoehe) {
        return breite * hoehe;
    }

    Trance personInfo(name, alter, stadt) {
        return "Name: " + name + ", Alter: " + alter + ", Stadt: " + stadt;
    }

    entrance {
        induce flaeche = rechteckFlaeche(10, 5);
        observe "Fläche: " + flaeche;

        induce info = personInfo("Max", 30, "Berlin");
        observe info;
    }
} Relax;
```

### Parameter mit Standardwerten

```hyp
Focus {
    Trance begruesse(name, titel = "Herr/Frau") {
        observe titel + " " + name + ", willkommen!";
    }

    entrance {
        begruesse("Mustermann"); // Verwendet Standardtitel
        begruesse("Schmidt", "Dr."); // Überschreibt Standardtitel
    }
} Relax;
```

## Rekursive Funktionen

```hyp
Focus {
    Trance fakultaet(n) {
        if (n <= 1) {
            return 1;
        } else {
            return n * fakultaet(n - 1);
        }
    }

    Trance fibonacci(n) {
        if (n <= 1) {
            return n;
        } else {
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
    }

    entrance {
        induce fact5 = fakultaet(5);
        observe "5! = " + fact5;

        induce fib10 = fibonacci(10);
        observe "Fibonacci(10) = " + fib10;
    }
} Relax;
```

## Funktionen mit Arrays

```hyp
Focus {
    Trance arraySumme(zahlen) {
        induce summe = 0;
        for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
            induce summe = summe + ArrayGet(zahlen, i);
        }
        return summe;
    }

    Trance findeMaximum(zahlen) {
        if (ArrayLength(zahlen) == 0) {
            return null;
        }

        induce max = ArrayGet(zahlen, 0);
        for (induce i = 1; i < ArrayLength(zahlen); induce i = i + 1) {
            induce wert = ArrayGet(zahlen, i);
            if (wert > max) {
                induce max = wert;
            }
        }
        return max;
    }

    Trance filterGerade(zahlen) {
        induce ergebnis = [];
        for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
            induce zahl = ArrayGet(zahlen, i);
            if (zahl % 2 == 0) {
                // Array erweitern (vereinfacht)
                observe "Gerade Zahl gefunden: " + zahl;
            }
        }
        return ergebnis;
    }

    entrance {
        induce testZahlen = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        induce summe = arraySumme(testZahlen);
        observe "Summe: " + summe;

        induce max = findeMaximum(testZahlen);
        observe "Maximum: " + max;

        filterGerade(testZahlen);
    }
} Relax;
```

## Funktionen mit Records

```hyp
Focus {
    Trance erstellePerson(name, alter, stadt) {
        return {
            name: name,
            alter: alter,
            stadt: stadt,
            volljaehrig: alter >= 18
        };
    }

    Trance personInfo(person) {
        return person.name + " (" + person.alter + ") aus " + person.stadt;
    }

    Trance istVolljaehrig(person) {
        return person.volljaehrig;
    }

    entrance {
        induce person1 = erstellePerson("Max", 25, "Berlin");
        induce person2 = erstellePerson("Anna", 16, "Hamburg");

        observe personInfo(person1);
        observe personInfo(person2);

        observe "Max ist volljährig: " + istVolljaehrig(person1);
        observe "Anna ist volljährig: " + istVolljaehrig(person2);
    }
} Relax;
```

## Hilfsfunktionen

```hyp
Focus {
    Trance validiereAlter(alter) {
        return alter >= 0 && alter <= 150;
    }

    Trance validiereEmail(email) {
        // Einfache E-Mail-Validierung
        return Length(email) > 0 && email != null;
    }

    Trance berechneBMI(gewicht, groesse) {
        if (groesse <= 0) {
            return null;
        }
        return gewicht / (groesse * groesse);
    }

    Trance bmiKategorie(bmi) {
        if (bmi == null) {
            return "Ungültig";
        } else if (bmi < 18.5) {
            return "Untergewicht";
        } else if (bmi < 25) {
            return "Normalgewicht";
        } else if (bmi < 30) {
            return "Übergewicht";
        } else {
            return "Adipositas";
        }
    }

    entrance {
        induce alter = 25;
        induce email = "test@example.com";
        induce gewicht = 70;
        induce groesse = 1.75;

        if (validiereAlter(alter)) {
            observe "Alter ist gültig";
        }

        if (validiereEmail(email)) {
            observe "E-Mail ist gültig";
        }

        induce bmi = berechneBMI(gewicht, groesse);
        induce kategorie = bmiKategorie(bmi);
        observe "BMI: " + bmi + " (" + kategorie + ")";
    }
} Relax;
```

## Mathematische Funktionen

```hyp
Focus {
    Trance potenz(basis, exponent) {
        if (exponent == 0) {
            return 1;
        }

        induce ergebnis = 1;
        for (induce i = 1; i <= exponent; induce i = i + 1) {
            induce ergebnis = ergebnis * basis;
        }
        return ergebnis;
    }

    Trance istPrimzahl(zahl) {
        if (zahl < 2) {
            return false;
        }

        for (induce i = 2; i * i <= zahl; induce i = i + 1) {
            if (zahl % i == 0) {
                return false;
            }
        }
        return true;
    }

    Trance ggT(a, b) {
        while (b != 0) {
            induce temp = b;
            induce b = a % b;
            induce a = temp;
        }
        return a;
    }

    entrance {
        observe "2^10 = " + potenz(2, 10);
        observe "17 ist Primzahl: " + istPrimzahl(17);
        observe "GGT von 48 und 18: " + ggT(48, 18);
    }
} Relax;
```

## Best Practices

### Funktionen benennen

```hyp
// Gut - beschreibende Namen
Trance berechneDurchschnitt(zahlen) { ... }
Trance istGueltigeEmail(email) { ... }
Trance formatiereDatum(datum) { ... }

// Schlecht - unklare Namen
Trance calc(arr) { ... }
Trance check(str) { ... }
Trance format(d) { ... }
```

### Einzelverantwortlichkeit

```hyp
// Gut - eine Funktion, eine Aufgabe
Trance validiereAlter(alter) {
    return alter >= 0 && alter <= 150;
}

Trance berechneAltersgruppe(alter) {
    if (alter < 18) return "Jugendlich";
    if (alter < 65) return "Erwachsen";
    return "Senior";
}

// Schlecht - zu viele Aufgaben in einer Funktion
Trance verarbeitePerson(alter, name, email) {
    // Validierung, Berechnung, Formatierung alles in einer Funktion
}
```

### Fehlerbehandlung

```hyp
Focus {
    Trance sichereDivision(a, b) {
        if (b == 0) {
            observe "Fehler: Division durch Null!";
            return null;
        }
        return a / b;
    }

    Trance arrayElementSicher(arr, index) {
        if (index < 0 || index >= ArrayLength(arr)) {
            observe "Fehler: Index außerhalb des Bereichs!";
            return null;
        }
        return ArrayGet(arr, index);
    }

    entrance {
        induce ergebnis1 = sichereDivision(10, 0);
        induce ergebnis2 = sichereDivision(10, 2);

        induce zahlen = [1, 2, 3];
        induce element1 = arrayElementSicher(zahlen, 5);
        induce element2 = arrayElementSicher(zahlen, 1);
    }
} Relax;
```

## Nächste Schritte

- [Sessions](./sessions) - Session-Management
- [Tranceify](./tranceify) - Hypnotische Anwendungen
- [Arrays](./arrays) - Array-Operationen
- [Records](./records) - Objekt-Programmierung

---

**Beherrschst du Funktionen? Dann lerne [Sessions](./sessions) kennen!** 🧠
