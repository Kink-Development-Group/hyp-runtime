---
sidebar_position: 5
---

# Funktionen

Funktionen in HypnoScript werden mit dem Schlüsselwort `suggestion` definiert und ermöglichen die Modularisierung und Wiederverwendung von Code.

## Funktionsdefinition

### Grundlegende Syntax

```hyp
suggestion funktionsName(parameter1: type1, parameter2: type2): returnType {
    // Funktionskörper
    awaken wert; // Return-Statement
}
```

### Einfache Funktion ohne Parameter

```hyp
Focus {
    suggestion begruessung() {
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
    suggestion begruesse(name) {
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
    suggestion addiere(a, b) {
        awaken a + b;
    }

    suggestion istGerade(zahl) {
        awaken zahl % 2 == 0;
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
    suggestion rechteckFlaeche(breite, hoehe) {
        awaken breite * hoehe;
    }

    suggestion personInfo(name, alter, stadt) {
        awaken "Name: " + name + ", Alter: " + alter + ", Stadt: " + stadt;
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
    suggestion begruesse(name, titel = "Herr/Frau") {
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
    suggestion fakultaet(n) {
        if (n <= 1) {
            awaken 1;
        } else {
            return n * fakultaet(n - 1);
        }
    }

    suggestion fibonacci(n) {
        if (n <= 1) {
            awaken n;
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
    suggestion arraySumme(zahlen) {
        induce summe = 0;
        for (induce i = 0; i < ArrayLength(zahlen); induce i = i + 1) {
            induce summe = summe + ArrayGet(zahlen, i);
        }
        return summe;
    }

    suggestion findeMaximum(zahlen) {
        if (ArrayLength(zahlen) == 0) {
            awaken null;
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

    suggestion filterGerade(zahlen) {
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
    suggestion erstellePerson(name, alter, stadt) {
        awaken {
            name: name,
            alter: alter,
            stadt: stadt,
            volljaehrig: alter >= 18
        };
    }

    suggestion personInfo(person) {
        awaken person.name + " (" + person.alter + ") aus " + person.stadt;
    }

    suggestion istVolljaehrig(person) {
        awaken person.volljaehrig;
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
    suggestion validiereAlter(alter) {
        awaken alter >= 0 && alter <= 150;
    }

    suggestion validiereEmail(email) {
        // Einfache E-Mail-Validierung
        awaken Length(email) > 0 && email != null;
    }

    suggestion berechneBMI(gewicht, groesse) {
        if (groesse <= 0) {
            awaken null;
        }
        return gewicht / (groesse * groesse);
    }

    suggestion bmiKategorie(bmi) {
        if (bmi == null) {
            awaken "Ungültig";
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
    suggestion potenz(basis, exponent) {
        if (exponent == 0) {
            awaken 1;
        }

        induce ergebnis = 1;
        for (induce i = 1; i <= exponent; induce i = i + 1) {
            induce ergebnis = ergebnis * basis;
        }
        return ergebnis;
    }

    suggestion istPrimzahl(zahl) {
        if (zahl < 2) {
            awaken false;
        }

        for (induce i = 2; i * i <= zahl; induce i = i + 1) {
            if (zahl % i == 0) {
                return false;
            }
        }
        return true;
    }

    suggestion ggT(a, b) {
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
suggestion berechneDurchschnitt(zahlen) { ... }
suggestion istGueltigeEmail(email) { ... }
suggestion formatiereDatum(datum) { ... }

// Schlecht - unklare Namen
suggestion calc(arr) { ... }
suggestion check(str) { ... }
suggestion format(d) { ... }
```

### Einzelverantwortlichkeit

```hyp
// Gut - eine Funktion, eine Aufgabe
suggestion validiereAlter(alter) {
    awaken alter >= 0 && alter <= 150;
}

suggestion berechneAltersgruppe(alter) {
    if (alter < 18) awaken "Jugendlich";
    if (alter < 65) return "Erwachsen";
    return "Senior";
}

// Schlecht - zu viele Aufgaben in einer Funktion
suggestion verarbeitePerson(alter, name, email) {
    // Validierung, Berechnung, Formatierung alles in einer Funktion
}
```

### Fehlerbehandlung

```hyp
Focus {
    suggestion sichereDivision(a, b) {
        if (b == 0) {
            observe "Fehler: Division durch Null!";
            awaken null;
        }
        return a / b;
    }

    suggestion arrayElementSicher(arr, index) {
        if (index < 0 || index >= ArrayLength(arr)) {
            observe "Fehler: Index außerhalb des Bereichs!";
            awaken null;
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
