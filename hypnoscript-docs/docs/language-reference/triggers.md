---
sidebar_position: 6
---

# Triggers – Event-Hooks & Callbacks

Triggers sind ein mächtiges Werkzeug in HypnoScript zum Definieren von Event-Hooks, Callbacks und verzögerten Aktionen. Sie kombinieren die Flexibilität von First-Class Functions mit der deklarativen Semantik von Event-Handlern.

## Was sind Triggers?

Ein `trigger` ist eine benannte Callback-Funktion, die auf **Top-Level** (außerhalb von Funktionen, Sessions oder Blöcken) deklariert wird. Triggers sind ideal für:

- 🎯 **Event-Handler** – Reaktion auf Benutzer-Interaktionen oder Systemereignisse
- 🧹 **Cleanup-Aktionen** – Aufräumoperationen nach Programmende
- ⏰ **Verzögerte Ausführung** – Callbacks für asynchrone Operationen
- 🔄 **State-Management** – Zustandsänderungs-Handler in komplexen Sessions

## Grundlegende Syntax

```hyp
trigger triggerName = suggestion(parameter1: type1, parameter2: type2): returnType {
    // Trigger-Code
};
```

### Wichtige Eigenschaften

| Eigenschaft          | Beschreibung                                          |
| -------------------- | ----------------------------------------------------- |
| **Scope**            | Nur Top-Level (Programm- oder Modul-Ebene)            |
| **Deklaration**      | `trigger name = suggestion(...) { ... };`             |
| **First-Class**      | Können als Parameter übergeben und gespeichert werden |
| **Event-Orientiert** | Ideal für Event-Handler und Callbacks                 |

> ✅ Der Rust-Parser erzwingt diese Regel ab sofort strikt: Jeder Versuch, einen `trigger` innerhalb eines Blocks, einer Funktion oder Session zu deklarieren, resultiert in dem Fehler _"Triggers can only be declared at the top level"_.

## Einfache Beispiele

### Cleanup-Trigger

Triggers eignen sich perfekt für Aufräumaktionen am Programmende:

```hyp
Focus {
    induce resourceHandle: number = 42;

    trigger onCleanup = suggestion() {
        observe "Räume Ressource " + resourceHandle + " auf";
        // Ressourcen freigeben
    };

    entrance {
        observe "Programm gestartet";
    }

    finale {
        onCleanup();
        observe "Programm beendet";
    }
} Relax
```

### Event-Handler

Triggers als klassische Event-Handler:

```hyp
Focus {
    trigger onClick = suggestion(buttonId: string) {
        observe "Button geklickt: " + buttonId;
    };

    trigger onSubmit = suggestion(formData: string) {
        observe "Formular abgeschickt: " + formData;
    };

    entrance {
        onClick("btnSave");
        onSubmit("user@example.com");
    }
} Relax
```

## Parametrisierte Triggers

Triggers können beliebige Parameter akzeptieren:

```hyp
Focus {
    trigger onError = suggestion(errorCode: number, message: string) {
        observe "Fehler " + errorCode + ": " + message;
    };

    trigger onSuccess = suggestion(data: string): boolean {
        observe "Erfolg: " + data;
        awaken true;
    };

    entrance {
        onError(404, "Nicht gefunden");
        induce result: boolean = onSuccess("Daten geladen");
    }
} Relax
```

## Integration mit DeepMind/AuraAsync

Triggers glänzen in Kombination mit den Builtin-Funktionen:

### Wiederholte Ausführung

```hyp
Focus {
    induce counter: number = 0;

    trigger onTick = suggestion() {
        counter = counter + 1;
        observe "Tick " + counter;
    };

    entrance {
        // Führe trigger 5x im Abstand von 1000ms aus
        repeatAction(onTick, 5, 1000);
        observe "Finale Zählung: " + counter;
    }
} Relax
```

### Verzögerte Ausführung

```hyp
Focus {
    trigger afterDelay = suggestion(message: string) {
        observe "Verzögerte Nachricht: " + message;
    };

    entrance {
        observe "Starte Verzögerung...";
        delayedSuggestion(afterDelay, 2000, "Hallo nach 2 Sekunden!");
        observe "Verzögerung gestartet";
    }
} Relax
```

## Triggers in Sessions

Während Triggers nur auf Top-Level deklariert werden können, lassen sie sich perfekt mit Sessions kombinieren:

```hyp
// Trigger als Top-Level-Deklaration
trigger onSecondElapsed = suggestion(timer: HypnoTimer) {
    timer.elapsedSeconds = timer.elapsedSeconds + 1;
    observe "Verstrichene Zeit: " + timer.elapsedSeconds + "s";
};

session HypnoTimer {
    expose elapsedSeconds: number;
    conceal timerCallback: suggestion;

    suggestion constructor() {
        this.elapsedSeconds = 0;
        this.timerCallback = onSecondElapsed;
    }

    suggestion start() {
        // Rufe Trigger jede Sekunde auf
        repeatAction(this.timerCallback, 60, 1000);
    }

    suggestion getElapsed(): number {
        awaken this.elapsedSeconds;
    }
}

Focus {
    entrance {
        induce timer = HypnoTimer();
        timer.start();
    }
} Relax
```

## Unterschied zu normalen Funktionen

| Aspekt          | `suggestion`                            | `trigger`                                   |
| --------------- | --------------------------------------- | ------------------------------------------- |
| **Deklaration** | `suggestion name(params): type { ... }` | `trigger name = suggestion(params) { ... }` |
| **Scope**       | Block-Level (lokal/global)              | **Nur Top-Level**                           |
| **Semantik**    | Wiederverwendbare Funktion              | Event-Handler/Callback                      |
| **Verwendung**  | Allgemeine Logik                        | Ereignisgesteuert                           |
| **Konvention**  | Algorithmen, Berechnungen               | Reaktionen, Cleanup, Events                 |

## Lokale Callbacks in Sessions

Für Callbacks innerhalb von Sessions oder Funktionen verwende **anonyme suggestion-Expressions**:

```hyp
session TaskManager {
    conceal taskCallback: suggestion;

    suggestion constructor() {
        // Anonyme suggestion-Expression als lokaler Callback
        this.taskCallback = suggestion() {
            observe "Task ausgeführt!";
        };
    }

    suggestion executeTask() {
        this.taskCallback();
    }
}
```

## Best Practices

### ✅ Do's

```hyp
// ✓ Benenne Triggers mit 'on'-Präfix für Klarheit
trigger onAwaken = suggestion() { ... };
trigger onError = suggestion(code: number) { ... };

// ✓ Verwende Triggers für Event-Handler
trigger onClick = suggestion(id: string) { ... };

// ✓ Kombiniere mit finale-Blöcken für garantierte Ausführung
finale {
    onCleanup();
}

// ✓ Nutze Triggers mit DeepMind-Funktionen
repeatAction(onUpdate, 10, 500);
```

### ❌ Don'ts

```hyp
// ✗ Vermeide Trigger innerhalb von Funktionen
suggestion myFunction() {
    trigger localTrigger = suggestion() { ... };  // FEHLER!
}

// ✗ Vermeide Trigger in Sessions
session MySession {
    trigger classTrigger = suggestion() { ... };  // FEHLER!
}

// ✗ Verwende stattdessen anonyme Expressions für lokale Callbacks
this.callback = suggestion() { observe "Lokaler Callback"; };
```

## Erweiterte Patterns

### Chain of Triggers

```hyp
Focus {
    trigger step1 = suggestion() {
        observe "Schritt 1 abgeschlossen";
        step2();
    };

    trigger step2 = suggestion() {
        observe "Schritt 2 abgeschlossen";
        step3();
    };

    trigger step3 = suggestion() {
        observe "Alle Schritte abgeschlossen!";
    };

    entrance {
        step1();  // Startet die Kette
    }
} Relax
```

### Conditional Triggers

```hyp
Focus {
    induce debugMode: boolean = true;

    trigger onDebug = suggestion(message: string) {
        if (debugMode) {
            observe "[DEBUG] " + message;
        }
    };

    entrance {
        onDebug("Programm gestartet");
        debugMode = false;
        onDebug("Diese Nachricht wird nicht angezeigt");
    }
} Relax
```

### Trigger Registry Pattern

```hyp
Focus {
    induce eventRegistry: array = [];

    trigger registerEvent = suggestion(eventName: string) {
        observe "Event registriert: " + eventName;
        // eventRegistry.push(eventName);  // Wenn Array-Push verfügbar
    };

    trigger onAppStart = suggestion() {
        registerEvent("app_started");
    };

    trigger onAppStop = suggestion() {
        registerEvent("app_stopped");
    };

    entrance {
        onAppStart();
    }

    finale {
        onAppStop();
    }
} Relax
```

## Zusammenfassung

Triggers sind **First-Class Event-Handler** in HypnoScript, die:

- ✅ Nur auf **Top-Level** deklariert werden
- ✅ Perfekt für **Event-Handling** und **Callbacks** geeignet sind
- ✅ Mit **DeepMind/AuraAsync** kombiniert werden können
- ✅ Als **Parameter** übergeben und **gespeichert** werden können
- ✅ Durch **Naming-Conventions** (`on*`) klar erkennbar sind

Für lokale Callbacks innerhalb von Funktionen oder Sessions verwende anonyme `suggestion()`-Expressions.

## Nächste Schritte

- [Functions](./functions) – Allgemeine Funktionsdefinition
- [Sessions](./sessions) – Objektorientierte Programmierung
- [Async & Await](./async-await) – Asynchrone Programmierung
- [Pattern Matching](./pattern-matching) – Erweiterte Kontrollstrukturen

---

**Bereit für Event-basierte Programmierung? Nutze Triggers für elegante Event-Flows!** 🎯
