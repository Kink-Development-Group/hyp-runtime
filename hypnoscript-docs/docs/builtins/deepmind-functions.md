---
description: Höhere Kontrollfluss- und Kompositions-Builtins für HypnoScript.
---

# DeepMind-Funktionen

Die DeepMind-Builtins erweitern HypnoScript um mächtige Kontrollfluss- und Functional-Programming-Patterns. Sie
arbeiten Hand in Hand mit `suggestion`-Blöcken und erlauben es, Schleifen, Verzögerungen, Fehlerbehandlung und
Funktionskomposition deklarativ auszudrücken.

## Überblick

| Funktion             | Rückgabewert | Kurzbeschreibung                           |
| -------------------- | ------------ | ------------------------------------------ |
| `RepeatAction`       | `void`       | Aktion eine feste Anzahl an Wiederholungen |
| `DelayedSuggestion`  | `void`       | Aktion nach Millisekunden-Verzögerung      |
| `IfTranced`          | `void`       | Bedingte Ausführung zweier Vorschläge      |
| `RepeatUntil`        | `void`       | Wiederhole Aktion bis Bedingung `true`     |
| `RepeatWhile`        | `void`       | Wiederhole solange Bedingung `true`        |
| `SequentialTrance`   | `void`       | Liste von Aktionen seriell ausführen       |
| `Compose` / `Pipe`   | `suggestion` | Funktionen kombinieren                     |
| `TryOrAwaken`        | `void`       | Fehlerpfad behandeln                       |
| `EnsureAwakening`    | `void`       | Cleanup garantiert ausführen               |
| `MeasureTranceDepth` | `number`     | Laufzeit in Millisekunden messen           |
| `Memoize`            | `suggestion` | Funktionsresultate zwischenspeichern       |

:::tip Namenskonventionen
Alle DeepMind-Builtins verwenden PascalCase (`RepeatAction`) und akzeptieren `suggestion()`-Blöcke als Parameter.
Die Signaturen sind case-insensitive, so dass `repeataction` ebenfalls funktioniert.
:::

## Wiederholung & Timing

### RepeatAction(times, action)

- **Signatur:** `(times: number, action: () -> void) -> void`
- **Beschreibung:** Führt `action` `times`-mal aus. Negative Werte werden ignoriert.

```hyp
RepeatAction(3, suggestion() {
    observe "Affirmation";
});
```

### DelayedSuggestion(action, delayMs)

- **Signatur:** `(action: () -> void, delay: number) -> void`
- **Beschreibung:** Führt `action` nach `delay` Millisekunden aus. Die Ausführung blockiert bis zum Ablauf der Zeit.

```hyp
DelayedSuggestion(suggestion() {
    observe "Willkommen nach 2 Sekunden";
}, 2000);
```

## Bedingte Ausführung

### IfTranced(condition, thenAction, elseAction)

- **Signatur:** `(condition: boolean, then: () -> void, otherwise: () -> void) -> void`
- **Beschreibung:** Evaluierte Bedingung; bei `true` wird `then`, sonst `otherwise` ausgeführt.

```hyp
IfTranced(audienceSize > 10,
    suggestion() { observe "Großgruppe"; },
    suggestion() { observe "Intime Sitzung"; }
);
```

## Komposition & Pipelines

### Compose(f, g)

- **Signatur:** `(f: (B) -> C, g: (A) -> B) -> (A -> C)`
- **Beschreibung:** Erst `g`, dann `f`. Nützlich für wiederverwendbare Datenpipelines.

```hyp
suggestion double(x: number): number { awaken x * 2; }
suggestion addTen(x: number): number { awaken x + 10; }

induce transformer = Compose(double, addTen);
induce result: number = transformer(5); // 30
```

### Pipe(f, g)

- **Signatur:** `(f: (A) -> B, g: (B) -> C) -> (A -> C)`
- **Beschreibung:** Umgekehrte Reihenfolge: zuerst `f`, danach `g`.

```hyp
induce pipeline = Pipe(double, addTen);
observe pipeline(5); // 20
```

## Schleifensteuerung

### RepeatUntil(action, condition)

- **Signatur:** `(action: () -> void, condition: () -> boolean) -> void`
- **Beschreibung:** Führt `action` aus, solange `condition()` `false` liefert. Bedingung wird nach jedem Durchlauf geprüft.

```hyp
induce counter: number = 0;
RepeatUntil(
    suggestion() { counter = counter + 1; },
    suggestion(): boolean { awaken counter >= 5; }
);
```

### RepeatWhile(condition, action)

- **Signatur:** `(condition: () -> boolean, action: () -> void) -> void`
- **Beschreibung:** Prüft `condition()` vor jedem Durchlauf; bei `true` läuft `action`, sonst endet die Schleife.

```hyp
induce energy: number = 3;
RepeatWhile(
    suggestion(): boolean { awaken energy > 0; },
    suggestion() {
        observe "Noch Energie: " + energy;
        energy = energy - 1;
    }
);
```

## Sequenzen & Fehlerbehandlung

### SequentialTrance(actions)

- **Signatur:** `(actions: (() -> void)[]) -> void`
- **Beschreibung:** Führt eine Liste von `suggestion`-Blöcken nacheinander aus.

```hyp
SequentialTrance([
    suggestion() { observe "Phase 1"; },
    suggestion() { observe "Phase 2"; },
    suggestion() { observe "Phase 3"; }
]);
```

### TryOrAwaken(tryAction, catchAction)

- **Signatur:** `(try: () -> Result<void, string>, catch: (error: string) -> void) -> void`
- **Beschreibung:** Führt `try` aus und ruft bei Fehlern `catch` mit der Fehlermeldung auf.

```hyp
TryOrAwaken(
    suggestion(): Result<void, string> {
        if (audienceSize < 0) {
            awaken Err("Negative Audience");
        }
        observe "Session startet";
        awaken Ok(());
    },
    suggestion(error: string) {
        observe "Fehler: " + error;
    }
);
```

### EnsureAwakening(mainAction, cleanup)

- **Signatur:** `(main: () -> void, cleanup: () -> void) -> void`
- **Beschreibung:** Führt `main` aus und garantiert, dass `cleanup` anschließend aufgerufen wird.

```hyp
EnsureAwakening(
    suggestion() {
        observe "Datei öffnen";
    },
    suggestion() {
        observe "Datei schließen";
    }
);
```

## Messung & Memoisierung

### MeasureTranceDepth(action)

- **Signatur:** `(action: () -> void) -> number`
- **Beschreibung:** Führt `action` aus und gibt die Dauer in Millisekunden zurück.

```hyp
induce duration: number = MeasureTranceDepth(suggestion() {
    RepeatAction(1000, suggestion() { observe "Tick"; });
});
observe "Laufzeit: " + duration + " ms";
```

### Memoize(f)

- **Signatur:** `(f: (A) -> R) -> (A -> R)`
- **Beschreibung:** Liefert eine Wrapper-Funktion. In der aktuellen Runtime-Version wird das Ergebnis nicht dauerhaft
  zwischengespeichert, aber das Interface bleibt stabil für zukünftige Optimierungen.

```hyp
suggestion square(x: number): number { awaken x * x; }
induce memoSquare = Memoize(square);

observe memoSquare(4); // 16
observe memoSquare(4); // 16 (zukünftig aus Cache)
```

## Tipps für den Einsatz

- `RepeatAction`, `RepeatUntil` und `RepeatWhile` blockieren synchron; nutze `DelayedSuggestion` für einfache
  Zeitsteuerung.
- Kombiniere `Compose` und `Pipe` mit Array- oder String-Builtins, um filter-map-reduce-Ketten lesbar zu halten.
- `TryOrAwaken` erwartet einen `Result`-ähnlichen Rückgabewert. Gib `Ok(())` für Erfolg und `Err("Message")` für Fehler
  zurück.
- `MeasureTranceDepth` eignet sich für schnelle Performance-Messungen ohne zusätzliches Werkzeug.

## Siehe auch

- [Builtin-Übersicht](./overview)
- [Vollständige Referenz – DeepMind](./_complete-reference#deepmind-builtins-higher-order-functions)
- [CLI Builtins anzeigen](../cli/commands#builtins)
