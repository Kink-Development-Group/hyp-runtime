---
title: Assertions
---

# Assertions

Assertions sind mächtige Werkzeuge in HypnoScript, um Bedingungen zu überprüfen und Fehler frühzeitig zu erkennen.

## Übersicht

Assertions ermöglichen es Ihnen, Annahmen über den Zustand Ihres Programms zu formulieren und automatisch zu überprüfen. Sie sind besonders nützlich für Debugging, Testing und die Validierung von Eingabedaten.

## Grundlegende Syntax

### Einfache Assertion

```hyp
assert condition "Optional message";
```

### Assertion ohne Nachricht

```hyp
assert condition;
```

## Grundlegende Assertions

### Wahrheitswert-Assertions

```hyp
Focus {
    entrance {
        induce isLoggedIn = true;
        induce hasPermission = false;

        // Einfache Wahrheitswert-Assertions
        assert isLoggedIn "Benutzer muss eingeloggt sein";
        assert !hasPermission "Benutzer sollte keine Berechtigung haben";

        // Komplexe Bedingungen
        induce userAge = 25;
        induce isAdult = userAge >= 18;
        assert isAdult "Benutzer muss volljährig sein";

        observe "Alle Assertions bestanden!";
    }
} Relax;
```

### Gleichheits-Assertions

```hyp
Focus {
    entrance {
        induce expected = 42;
        induce actual = 42;

        // Gleichheit prüfen
        assert actual == expected "Wert sollte 42 sein";

        // Ungleichheit prüfen
        induce differentValue = 100;
        assert actual != differentValue "Werte sollten unterschiedlich sein";

        // String-Gleichheit
        induce name = "Alice";
        assert name == "Alice" "Name sollte Alice sein";

        observe "Gleichheits-Assertions bestanden!";
    }
} Relax;
```

### Numerische Assertions

```hyp
Focus {
    entrance {
        induce value = 50;

        // Größer-als
        assert value > 0 "Wert sollte positiv sein";
        assert value >= 50 "Wert sollte mindestens 50 sein";

        // Kleiner-als
        assert value < 100 "Wert sollte kleiner als 100 sein";
        assert value <= 50 "Wert sollte maximal 50 sein";

        // Bereich prüfen
        assert value >= 0 && value <= 100 "Wert sollte zwischen 0 und 100 liegen";

        observe "Numerische Assertions bestanden!";
    }
} Relax;
```

## Erweiterte Assertions

### Array-Assertions

```hyp
Focus {
    entrance {
        induce numbers = [1, 2, 3, 4, 5];

        // Array-Länge prüfen
        assert ArrayLength(numbers) == 5 "Array sollte 5 Elemente haben";
        assert ArrayLength(numbers) > 0 "Array sollte nicht leer sein";

        // Array-Inhalt prüfen
        assert ArrayContains(numbers, 3) "Array sollte 3 enthalten";
        assert !ArrayContains(numbers, 10) "Array sollte 10 nicht enthalten";

        // Array-Elemente prüfen
        assert ArrayGet(numbers, 0) == 1 "Erstes Element sollte 1 sein";
        assert ArrayGet(numbers, ArrayLength(numbers) - 1) == 5 "Letztes Element sollte 5 sein";

        observe "Array-Assertions bestanden!";
    }
} Relax;
```

### String-Assertions

```hyp
Focus {
    entrance {
        induce text = "Hello World";

        // String-Länge
        assert Length(text) > 0 "Text sollte nicht leer sein";
        assert Length(text) <= 100 "Text sollte maximal 100 Zeichen haben";

        // String-Inhalt
        assert Contains(text, "Hello") "Text sollte 'Hello' enthalten";
        assert StartsWith(text, "Hello") "Text sollte mit 'Hello' beginnen";
        assert EndsWith(text, "World") "Text sollte mit 'World' enden";

        // String-Format
        induce email = "user@example.com";
        assert IsValidEmail(email) "E-Mail sollte gültig sein";

        observe "String-Assertions bestanden!";
    }
} Relax;
```

### Objekt-Assertions

```hyp
Focus {
    entrance {
        record Person {
            name: string;
            age: number;
        }

        induce person = Person {
            name: "Alice",
            age: 30
        };

        // Objekt-Eigenschaften prüfen
        assert person.name != "" "Name sollte nicht leer sein";
        assert person.age >= 0 "Alter sollte nicht negativ sein";
        assert person.age <= 150 "Alter sollte realistisch sein";

        // Objekt-Typ prüfen
        assert person != null "Person sollte nicht null sein";

        observe "Objekt-Assertions bestanden!";
    }
} Relax;
```

## Spezialisierte Assertions

### Typ-Assertions

```hyp
Focus {
    entrance {
        induce value = 42;
        induce text = "Hello";
        induce array = [1, 2, 3];

        // Typ prüfen
        assert TypeOf(value) == "number" "Wert sollte vom Typ number sein";
        assert TypeOf(text) == "string" "Text sollte vom Typ string sein";
        assert TypeOf(array) == "array" "Array sollte vom Typ array sein";

        // Null-Check
        induce nullableValue = null;
        assert nullableValue == null "Wert sollte null sein";

        observe "Typ-Assertions bestanden!";
    }
} Relax;
```

### Funktions-Assertions

```hyp
Focus {
    entrance {
        // Funktion definieren
        suggestion add(a: number, b: number): number {
            awaken a + b;
        }

        // Funktionsergebnis prüfen
        induce result = call add(2, 3);
        assert result == 5 "2 + 3 sollte 5 ergeben";

        // Funktionsverhalten prüfen
        induce zeroResult = call add(0, 0);
        assert zeroResult == 0 "0 + 0 sollte 0 ergeben";

        // Negative Zahlen
        induce negativeResult = call add(-1, -2);
        assert negativeResult == -3 "-1 + (-2) sollte -3 ergeben";

        observe "Funktions-Assertions bestanden!";
    }
} Relax;
```

### Performance-Assertions

```hyp
Focus {
    entrance {
        // Performance messen
        induce startTime = GetCurrentTime();

        // Operation durchführen
        induce sum = 0;
        for (induce i = 0; i < 1000; induce i = i + 1) {
            sum = sum + i;
        }

        induce endTime = GetCurrentTime();
        induce executionTime = (endTime - startTime) * 1000; // in ms

        // Performance-Assertions
        assert executionTime < 100 "Operation sollte schneller als 100ms sein";
        assert sum == 499500 "Summe sollte korrekt berechnet werden";

        observe "Performance-Assertions bestanden!";
        observe "Ausführungszeit: " + executionTime + " ms";
    }
} Relax;
```

## Assertion-Patterns

### Eingabevalidierung

```hyp
Focus {
    entrance {
        suggestion validateUserInput(username: string, age: number): boolean {
            // Username-Validierung
            assert Length(username) >= 3 "Username sollte mindestens 3 Zeichen haben";
            assert Length(username) <= 20 "Username sollte maximal 20 Zeichen haben";
            assert !Contains(username, " ") "Username sollte keine Leerzeichen enthalten";

            // Alters-Validierung
            assert age >= 13 "Benutzer sollte mindestens 13 Jahre alt sein";
            assert age <= 120 "Alter sollte realistisch sein";

            // Zusätzliche Validierungen
            assert IsValidUsername(username) "Username sollte gültig sein";

            return true;
        }

        // Validierung testen
        try {
            induce isValid = call validateUserInput("alice123", 25);
            assert isValid "Eingabe sollte gültig sein";
            observe "Eingabevalidierung erfolgreich!";
        } catch (error) {
            observe "Validierungsfehler: " + error;
        }
    }
} Relax;
```

### Zustandsvalidierung

```hyp
Focus {
    entrance {
        record GameState {
            playerHealth: number;
            score: number;
            level: number;
        }

        induce gameState = GameState {
            playerHealth: 100,
            score: 1500,
            level: 3
        };

        // Zustands-Assertions
        assert gameState.playerHealth >= 0 "Spieler-Gesundheit sollte nicht negativ sein";
        assert gameState.playerHealth <= 100 "Spieler-Gesundheit sollte maximal 100 sein";
        assert gameState.score >= 0 "Punktzahl sollte nicht negativ sein";
        assert gameState.level >= 1 "Level sollte mindestens 1 sein";

        // Konsistenz prüfen
        assert gameState.playerHealth > 0 || gameState.level == 1 "Spieler sollte leben oder im ersten Level sein";

        observe "Zustandsvalidierung erfolgreich!";
    }
} Relax;
```

### API-Response-Validierung

```hyp
Focus {
    entrance {
        record ApiResponse {
            status: number;
            data: object;
            message: string;
        }

        // Simulierte API-Antwort
        induce response = ApiResponse {
            status: 200,
            data: {
                userId: 123,
                name: "Alice"
            },
            message: "Success"
        };

        // Response-Validierung
        assert response.status >= 200 && response.status < 300 "Status sollte erfolgreich sein";
        assert response.data != null "Daten sollten vorhanden sein";
        assert Length(response.message) > 0 "Nachricht sollte nicht leer sein";

        // Daten-Validierung
        if (response.data.userId) {
            assert response.data.userId > 0 "User-ID sollte positiv sein";
        }

        if (response.data.name) {
            assert Length(response.data.name) > 0 "Name sollte nicht leer sein";
        }

        observe "API-Response-Validierung erfolgreich!";
    }
} Relax;
```

## Assertion-Frameworks

### Test-Assertions

```hyp
Focus {
    entrance {
        // Test-Setup
        induce testResults = [];

        // Test-Funktionen
        suggestion assertEqual(actual: object, expected: object, message: string) {
            if (actual != expected) {
                ArrayPush(testResults, "FAIL: " + message + " (Expected: " + expected + ", Got: " + actual + ")");
                throw "Assertion failed: " + message;
            } else {
                ArrayPush(testResults, "PASS: " + message);
            }
        }

        suggestion assertTrue(condition: boolean, message: string) {
            if (!condition) {
                ArrayPush(testResults, "FAIL: " + message);
                throw "Assertion failed: " + message;
            } else {
                ArrayPush(testResults, "PASS: " + message);
            }
        }

        // Tests ausführen
        try {
            call assertEqual(2 + 2, 4, "Addition test");
            call assertTrue(Length("Hello") == 5, "String length test");
            call assertEqual(ArrayLength([1, 2, 3]), 3, "Array length test");

            observe "Alle Tests bestanden!";
        } catch (error) {
            observe "Test fehlgeschlagen: " + error;
        }

        // Test-Ergebnisse anzeigen
        observe "Test-Ergebnisse:";
        for (induce i = 0; i < ArrayLength(testResults); induce i = i + 1) {
            observe "  " + testResults[i];
        }
    }
} Relax;
```

### Debug-Assertions

```hyp
Focus {
    entrance {
        induce debugMode = true;

        suggestion debugAssert(condition: boolean, message: string) {
            if (debugMode && !condition) {
                observe "[DEBUG] Assertion failed: " + message;
                observe "[DEBUG] Stack trace: " + GetCallStack();
            }
        }

        // Debug-Assertions verwenden
        induce value = 42;
        call debugAssert(value > 0, "Wert sollte positiv sein");
        call debugAssert(value < 100, "Wert sollte kleiner als 100 sein");

        // Debug-Informationen sammeln
        if (debugMode) {
            induce memoryUsage = GetMemoryUsage();
            call debugAssert(memoryUsage < 1000, "Speichernutzung sollte unter 1GB sein");
        }

        observe "Debug-Assertions abgeschlossen!";
    }
} Relax;
```

## Best Practices

### Assertion-Strategien

```hyp
Focus {
    entrance {
        // ✅ GUT: Spezifische Assertions
        induce userAge = 25;
        assert userAge >= 18 "Benutzer muss volljährig sein";

        // ✅ GUT: Aussagekräftige Nachrichten
        induce result = 42;
        assert result == 42 "Berechnung sollte 42 ergeben, nicht " + result;

        // ✅ GUT: Frühe Validierung
        suggestion processUser(user: object) {
            assert user != null "Benutzer-Objekt darf nicht null sein";
            assert user.name != "" "Benutzername darf nicht leer sein";

            // Verarbeitung...
        }

        // ❌ SCHLECHT: Zu allgemeine Assertions
        assert true "Alles ist gut";

        // ❌ SCHLECHT: Fehlende Nachrichten
        assert userAge >= 18;
    }
} Relax;
```

### Performance-Considerations

```hyp
Focus {
    entrance {
        // ✅ GUT: Einfache Assertions für Performance-kritische Pfade
        induce criticalValue = 100;
        assert criticalValue > 0; // Schnelle Prüfung

        // ✅ GUT: Komplexe Assertions nur im Debug-Modus
        induce debugMode = true;
        if (debugMode) {
            induce complexValidation = ValidateComplexData();
            assert complexValidation "Komplexe Validierung fehlgeschlagen";
        }

        // ✅ GUT: Assertions für invariante Bedingungen
        induce loopCount = 0;
        while (loopCount < 10) {
            assert loopCount >= 0 "Schleifenzähler sollte nicht negativ sein";
            loopCount = loopCount + 1;
        }
    }
} Relax;
```

## Fehlerbehandlung

### Assertion-Fehler abfangen

```hyp
Focus {
    entrance {
        induce assertionErrors = [];

        suggestion safeAssert(condition: boolean, message: string) {
            try {
                assert condition message;
                awaken true;
            } catch (error) {
                ArrayPush(assertionErrors, error);
                return false;
            }
        }

        // Sichere Assertions verwenden
        induce test1 = call safeAssert(2 + 2 == 4, "Mathematik funktioniert");
        induce test2 = call safeAssert(2 + 2 == 5, "Diese Assertion sollte fehlschlagen");
        induce test3 = call safeAssert(Length("Hello") == 5, "String-Länge ist korrekt");

        // Ergebnisse auswerten
        observe "Erfolgreiche Assertions: " + (test1 && test3);
        observe "Fehlgeschlagene Assertions: " + (!test2);

        if (ArrayLength(assertionErrors) > 0) {
            observe "Assertion-Fehler:";
            for (induce i = 0; i < ArrayLength(assertionErrors); induce i = i + 1) {
                observe "  " + assertionErrors[i];
            }
        }
    }
} Relax;
```

### Assertion-Level

```hyp
Focus {
    entrance {
        induce assertionLevel = "strict"; // "strict", "normal", "relaxed"

        suggestion levelAssert(condition: boolean, message: string, level: string) {
            if (level == "strict" ||
                (level == "normal" && assertionLevel != "relaxed") ||
                (level == "relaxed" && assertionLevel == "relaxed")) {
                assert condition message;
            }
        }

        // Level-spezifische Assertions
        call levelAssert(true, "Immer prüfen", "strict");
        call levelAssert(2 + 2 == 4, "Normale Prüfung", "normal");
        call levelAssert(Length("test") == 4, "Entspannte Prüfung", "relaxed");

        observe "Level-spezifische Assertions abgeschlossen!";
    }
} Relax;
```

## Nächste Schritte

- [Testing Overview](../testing/overview) - Umfassender Testing-Guide
- [Functions](./functions) - Funktionsdefinitionen und -aufrufe
- [Error Handling](../error-handling/overview) - Fehlerbehandlung

---

**Assertions gemeistert? Dann lerne [Testing Overview](../testing/overview) kennen!** ✅
