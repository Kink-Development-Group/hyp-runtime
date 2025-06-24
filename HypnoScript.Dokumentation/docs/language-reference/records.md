---
title: Records
---

# Records

Records sind strukturierte Datentypen in HypnoScript, die es ermöglichen, zusammengehörige Daten in einem Objekt zu gruppieren.

## Übersicht

Records sind unveränderliche (immutable) Datenstrukturen, die mehrere Felder mit verschiedenen Typen enthalten können. Sie sind ideal für die Darstellung von Entitäten, Konfigurationen und strukturierten Daten.

## Syntax

### Record-Deklaration

```hyp
record Person {
    name: string;
    age: number;
    email: string;
    isActive: boolean;
}
```

### Record-Instanziierung

```hyp
induce person = Person {
    name: "Alice Johnson",
    age: 30,
    email: "alice@example.com",
    isActive: true
};
```

### Record mit optionalen Feldern

```hyp
record User {
    id: number;
    username: string;
    email?: string;  // Optionales Feld
    lastLogin?: number;
}
```

## Grundlegende Verwendung

### Einfacher Record

```hyp
Focus {
    entrance {
        // Record definieren
        record Point {
            x: number;
            y: number;
        }

        // Record-Instanz erstellen
        induce point1 = Point {
            x: 10,
            y: 20
        };

        // Auf Felder zugreifen
        observe "X-Koordinate: " + point1.x;
        observe "Y-Koordinate: " + point1.y;
    }
} Relax;
```

### Record mit verschiedenen Datentypen

```hyp
Focus {
    entrance {
        record Product {
            id: number;
            name: string;
            price: number;
            categories: array;
            inStock: boolean;
            metadata: object;
        }

        induce product = Product {
            id: 12345,
            name: "HypnoScript Pro",
            price: 99.99,
            categories: ["Software", "Programming", "Hypnosis"],
            inStock: true,
            metadata: {
                version: "1.0.0",
                releaseDate: "2024-01-15"
            }
        };

        observe "Produkt: " + product.name;
        observe "Preis: " + product.price + " €";
        observe "Kategorien: " + product.categories;
    }
} Relax;
```

## Record-Operationen

### Feldzugriff

```hyp
Focus {
    entrance {
        record Address {
            street: string;
            city: string;
            zipCode: string;
            country: string;
        }

        induce address = Address {
            street: "Musterstraße 123",
            city: "Berlin",
            zipCode: "10115",
            country: "Deutschland"
        };

        // Direkter Feldzugriff
        observe "Straße: " + address.street;
        observe "Stadt: " + address.city;

        // Dynamischer Feldzugriff
        induce fieldName = "zipCode";
        induce fieldValue = address[fieldName];
        observe "PLZ: " + fieldValue;
    }
} Relax;
```

### Record-Kopien mit Änderungen

```hyp
Focus {
    entrance {
        record Config {
            theme: string;
            language: string;
            notifications: boolean;
        }

        induce defaultConfig = Config {
            theme: "dark",
            language: "de",
            notifications: true
        };

        // Kopie mit Änderungen erstellen
        induce userConfig = defaultConfig with {
            theme: "light",
            language: "en"
        };

        observe "Standard-Theme: " + defaultConfig.theme;
        observe "Benutzer-Theme: " + userConfig.theme;
    }
} Relax;
```

### Record-Vergleiche

```hyp
Focus {
    entrance {
        record Vector {
            x: number;
            y: number;
        }

        induce v1 = Vector { x: 1, y: 2 };
        induce v2 = Vector { x: 1, y: 2 };
        induce v3 = Vector { x: 3, y: 4 };

        // Strukturelle Gleichheit
        observe "v1 == v2: " + (v1 == v2);  // true
        observe "v1 == v3: " + (v1 == v3);  // false

        // Tiefenvergleich
        induce areEqual = DeepEquals(v1, v2);
        observe "Tiefenvergleich v1 und v2: " + areEqual;
    }
} Relax;
```

## Erweiterte Record-Features

### Record mit Methoden

```hyp
Focus {
    entrance {
        record Rectangle {
            width: number;
            height: number;

            // Methoden im Record
            suggestion area(): number {
                awaken this.width * this.height;
            }

            suggestion perimeter(): number {
                awaken 2 * (this.width + this.height);
            }

            suggestion isSquare(): boolean {
                awaken this.width == this.height;
            }
        }

        induce rect = Rectangle {
            width: 10,
            height: 5
        };

        observe "Fläche: " + rect.area();
        observe "Umfang: " + rect.perimeter();
        observe "Ist Quadrat: " + rect.isSquare();
    }
} Relax;
```

### Record mit berechneten Feldern

```hyp
Focus {
    entrance {
        record Circle {
            radius: number;
            diameter: number;  // Berechnet aus radius

            suggestion constructor(r: number) {
                this.radius = r;
                this.diameter = 2 * r;
            }
        }

        induce circle = Circle(5);
        observe "Radius: " + circle.radius;
        observe "Durchmesser: " + circle.diameter;
    }
} Relax;
```

### Record mit Validierung

```hyp
Focus {
    entrance {
        record Email {
            address: string;

            suggestion constructor(email: string) {
                if (IsValidEmail(email)) {
                    this.address = email;
                } else {
                    throw "Ungültige E-Mail-Adresse: " + email;
                }
            }

            suggestion getDomain(): string {
                induce parts = Split(this.address, "@");
                if (ArrayLength(parts) == 2) {
                    awaken parts[1];
                } else {
                    awaken "";
                }
            }
        }

        try {
            induce email = Email("user@example.com");
            observe "E-Mail: " + email.address;
            observe "Domain: " + email.getDomain();
        } catch (error) {
            observe "Fehler: " + error;
        }
    }
} Relax;
```

## Record-Patterns

### Record als Konfiguration

```hyp
Focus {
    entrance {
        record DatabaseConfig {
            host: string;
            port: number;
            username: string;
            password: string;
            database: string;
            ssl: boolean;
            timeout: number;
        }

        induce dbConfig = DatabaseConfig {
            host: "localhost",
            port: 5432,
            username: "admin",
            password: "secret123",
            database: "hypnoscript",
            ssl: true,
            timeout: 30
        };

        // Konfiguration verwenden
        induce connectionString = "postgresql://" + dbConfig.username + ":" +
                                 dbConfig.password + "@" + dbConfig.host + ":" +
                                 dbConfig.port + "/" + dbConfig.database;

        observe "Verbindungsstring: " + connectionString;
    }
} Relax;
```

### Record als API-Response

```hyp
Focus {
    entrance {
        record ApiResponse {
            success: boolean;
            data?: object;
            error?: string;
            timestamp: number;
            requestId: string;
        }

        // Erfolgreiche Antwort
        induce successResponse = ApiResponse {
            success: true,
            data: {
                userId: 123,
                name: "Alice",
                email: "alice@example.com"
            },
            timestamp: GetCurrentTime(),
            requestId: GenerateUUID()
        };

        // Fehlerantwort
        induce errorResponse = ApiResponse {
            success: false,
            error: "Benutzer nicht gefunden",
            timestamp: GetCurrentTime(),
            requestId: GenerateUUID()
        };

        observe "Erfolg: " + successResponse.success;
        observe "Fehler: " + errorResponse.error;
    }
} Relax;
```

### Record für Event-Handling

```hyp
Focus {
    entrance {
        record Event {
            type: string;
            source: string;
            timestamp: number;
            data: object;
            priority: number;
        }

        induce userEvent = Event {
            type: "user.login",
            source: "web-interface",
            timestamp: GetCurrentTime(),
            data: {
                userId: 456,
                ipAddress: "192.168.1.100",
                userAgent: "Mozilla/5.0..."
            },
            priority: 1
        };

        // Event verarbeiten
        if (userEvent.type == "user.login") {
            observe "Benutzer-Login erkannt: " + userEvent.data.userId;
            LogEvent(userEvent);
        }
    }
} Relax;
```

## Record-Arrays und Collections

### Array von Records

```hyp
Focus {
    entrance {
        record Student {
            id: number;
            name: string;
            grade: number;
        }

        induce students = [
            Student { id: 1, name: "Alice", grade: 85 },
            Student { id: 2, name: "Bob", grade: 92 },
            Student { id: 3, name: "Charlie", grade: 78 }
        ];

        // Durch Records iterieren
        for (induce i = 0; i < ArrayLength(students); induce i = i + 1) {
            induce student = students[i];
            observe "Student: " + student.name + " - Note: " + student.grade;
        }

        // Records filtern
        induce topStudents = ArrayFilter(students, function(student) {
            return student.grade >= 90;
        });

        observe "Top-Studenten: " + ArrayLength(topStudents);
    }
} Relax;
```

### Record als Dictionary-Wert

```hyp
Focus {
    entrance {
        record ProductInfo {
            name: string;
            price: number;
            category: string;
        }

        induce productCatalog = {
            "PROD001": ProductInfo { name: "Laptop", price: 999.99, category: "Electronics" },
            "PROD002": ProductInfo { name: "Mouse", price: 29.99, category: "Electronics" },
            "PROD003": ProductInfo { name: "Book", price: 19.99, category: "Books" }
        };

        // Produkt nach ID suchen
        induce productId = "PROD001";
        if (productCatalog[productId]) {
            induce product = productCatalog[productId];
            observe "Produkt gefunden: " + product.name + " - " + product.price + " €";
        }
    }
} Relax;
```

## Best Practices

### Record-Design

```hyp
Focus {
    entrance {
        // ✅ GUT: Klare, spezifische Records
        record UserProfile {
            userId: number;
            displayName: string;
            email: string;
            preferences: object;
        }

        // ❌ SCHLECHT: Zu generische Records
        record Data {
            field1: object;
            field2: object;
            field3: object;
        }

        // ✅ GUT: Immutable Records verwenden
        induce user = UserProfile {
            userId: 123,
            displayName: "Alice",
            email: "alice@example.com",
            preferences: {
                theme: "dark",
                language: "de"
            }
        };

        // ✅ GUT: Kopien für Änderungen erstellen
        induce updatedUser = user with {
            displayName: "Alice Johnson"
        };
    }
} Relax;
```

### Performance-Optimierung

```hyp
Focus {
    entrance {
        // ✅ GUT: Records für kleine, häufig verwendete Daten
        record Point {
            x: number;
            y: number;
        }

        // ✅ GUT: Sessions für komplexe Objekte mit Verhalten
        session ComplexObject {
            expose data: object;

            suggestion processData() {
                // Komplexe Verarbeitung
            }
        }

        // ✅ GUT: Records für Konfigurationen
        record AppConfig {
            debug: boolean;
            logLevel: string;
            maxConnections: number;
        }
    }
} Relax;
```

### Fehlerbehandlung

```hyp
Focus {
    entrance {
        record ValidationResult {
            isValid: boolean;
            errors: array;
            warnings: array;
        }

        suggestion validateEmail(email: string): ValidationResult {
            induce errors = [];
            induce warnings = [];

            if (Length(email) == 0) {
                ArrayPush(errors, "E-Mail darf nicht leer sein");
            } else if (!IsValidEmail(email)) {
                ArrayPush(errors, "Ungültiges E-Mail-Format");
            }

            if (Length(email) > 100) {
                ArrayPush(warnings, "E-Mail ist sehr lang");
            }

            return ValidationResult {
                isValid: ArrayLength(errors) == 0,
                errors: errors,
                warnings: warnings
            };
        }

        induce result = validateEmail("test@example.com");
        if (result.isValid) {
            observe "E-Mail ist gültig";
        } else {
            observe "E-Mail-Fehler: " + result.errors;
        }
    }
} Relax;
```

## Fehlerbehandlung

Records können bei ungültigen Operationen Fehler werfen:

```hyp
Focus {
    entrance {
        try {
            record Person {
                name: string;
                age: number;
            }

            induce person = Person {
                name: "Alice",
                age: 30
            };

            // Ungültiger Feldzugriff
            induce invalidField = person.nonexistentField;
        } catch (error) {
            observe "Record-Fehler: " + error;
        }

        try {
            // Ungültige Record-Erstellung
            induce invalidPerson = Person {
                name: "Bob",
                age: "ungültig"  // Sollte number sein
            };
        } catch (error) {
            observe "Validierungsfehler: " + error;
        }
    }
} Relax;
```

## Nächste Schritte

- [Sessions](./sessions) - Objektorientierte Programmierung mit Sessions
- [Arrays](./arrays) - Array-Operationen und Collections
- [Functions](./functions) - Funktionsdefinitionen und -aufrufe

---

**Records gemeistert? Dann lerne [Sessions](./sessions) kennen!** ✅
