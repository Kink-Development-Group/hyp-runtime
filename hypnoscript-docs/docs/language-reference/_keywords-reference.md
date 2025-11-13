# Schlüsselwörter-Referenz

Vollständige Referenz aller Schlüsselwörter in HypnoScript basierend auf der Rust-Implementierung.

## Programmstruktur

| Schlüsselwort | Beschreibung                            | Beispiel                        |
| ------------- | --------------------------------------- | ------------------------------- |
| `Focus`       | Programmstart (erforderlich)            | `Focus { ... } Relax`           |
| `Relax`       | Programmende (erforderlich)             | `Focus { ... } Relax`           |
| `entrance`    | Initialisierungsblock (optional)        | `entrance { observe "Start"; }` |
| `finale`      | Cleanup/Destruktor-Block (optional)     | `finale { observe "Ende"; }`    |
| `deepFocus`   | Erweiterter if-Block mit tieferer Scope | `if (x > 5) deepFocus { ... }`  |

## Variablendeklarationen

| Schlüsselwort | Beschreibung                     | Mutabilität   | Beispiel                             |
| ------------- | -------------------------------- | ------------- | ------------------------------------ |
| `induce`      | Standard-Variablendeklaration    | Veränderbar   | `induce x: number = 42;`             |
| `implant`     | Alternative Variablendeklaration | Veränderbar   | `implant y: string = "text";`        |
| `freeze`      | Konstanten-Deklaration           | Unveränderbar | `freeze PI: number = 3.14159;`       |
| `anchor`      | State Snapshot/Backup erstellen  | Unveränderbar | `anchor saved = currentValue;`       |
| `from`        | Eingabe-Quellangabe              | -             | `induce x: number from external;`    |
| `external`    | Externe Eingabequelle            | -             | `induce name: string from external;` |

## Kontrollstrukturen

| Schlüsselwort | Beschreibung             | Äquivalent | Beispiel                                         |
| ------------- | ------------------------ | ---------- | ------------------------------------------------ |
| `if`          | Bedingte Anweisung       | if         | `if (x > 5) { ... }`                             |
| `else`        | Alternative Verzweigung  | else       | `if (x > 5) { ... } else { ... }`                |
| `while`       | While-Schleife           | while      | `while (x > 0) { x = x - 1; }`                   |
| `loop`        | For-ähnliche Schleife    | for        | `loop (induce i = 0; i < 10; i = i + 1) { ... }` |
| `snap`        | Schleife abbrechen       | break      | `while (true) { snap; }`                         |
| `sink`        | Zum nächsten Durchlauf   | continue   | `while (x < 10) { sink; }`                       |
| `sinkTo`      | Goto (zu Label springen) | goto       | `sinkTo myLabel;`                                |
| `oscillate`   | Boolean-Variable togglen | -          | `oscillate isActive;`                            |

**Hinweis:** `break` und `continue` werden auch als Synonyme für `snap` und `sink` akzeptiert.

## Funktionen

| Schlüsselwort          | Beschreibung                    | Beispiel                                               |
| ---------------------- | ------------------------------- | ------------------------------------------------------ |
| `suggestion`           | Funktionsdeklaration            | `suggestion add(a: number, b: number): number { ... }` |
| `trigger`              | Event-Handler/Callback-Funktion | `trigger onClick = suggestion() { ... };`              |
| `imperativeSuggestion` | Imperative Funktion (Modifier)  | `imperativeSuggestion doSomething() { ... }`           |
| `dominantSuggestion`   | Statische Funktion (Modifier)   | `dominantSuggestion helperFunc() { ... }`              |
| `awaken`               | Return-Statement                | `awaken x + y;`                                        |
| `call`                 | Expliziter Funktionsaufruf      | `call myFunction();`                                   |

**Hinweis:** `return` wird auch als Synonym für `awaken` akzeptiert.

## Objektorientierung

### Sessions (Klassen)

| Schlüsselwort | Beschreibung         | Beispiel                                       |
| ------------- | -------------------- | ---------------------------------------------- |
| `session`     | Klassendeklaration   | `session Person { ... }`                       |
| `constructor` | Konstruktor-Methode  | `suggestion constructor(name: string) { ... }` |
| `expose`      | Public-Sichtbarkeit  | `expose name: string;`                         |
| `conceal`     | Private-Sichtbarkeit | `conceal age: number;`                         |
| `dominant`    | Statischer Member    | `dominant counter: number = 0;`                |

### Strukturen

| Schlüsselwort | Beschreibung              | Beispiel                                    |
| ------------- | ------------------------- | ------------------------------------------- |
| `tranceify`   | Record/Struct-Deklaration | `tranceify Point { x: number; y: number; }` |

## Ein-/Ausgabe

| Schlüsselwort | Beschreibung         | Verhalten                   | Beispiel                            |
| ------------- | -------------------- | --------------------------- | ----------------------------------- |
| `observe`     | Standard-Ausgabe     | Mit Zeilenumbruch           | `observe "Hallo Welt";`             |
| `whisper`     | Ausgabe ohne Umbruch | Ohne Zeilenumbruch          | `whisper "Teil1"; whisper "Teil2";` |
| `command`     | Imperative Ausgabe   | Großbuchstaben, mit Umbruch | `command "Wichtig!";`               |
| `drift`       | Pause/Sleep          | Verzögerung in ms           | `drift(2000);`                      |

## Module und Globals

| Schlüsselwort  | Beschreibung      | Beispiel                                  |
| -------------- | ----------------- | ----------------------------------------- |
| `mindLink`     | Import/Include    | `mindLink "utilities.hyp";`               |
| `sharedTrance` | Globale Variable  | `sharedTrance config: string = "global";` |
| `label`        | Label-Deklaration | `label myLabel;`                          |

## Datentypen

| Schlüsselwort | Beschreibung          | Beispiel                         |
| ------------- | --------------------- | -------------------------------- |
| `number`      | Numerischer Typ       | `induce x: number = 42;`         |
| `string`      | String-Typ            | `induce text: string = "hello";` |
| `boolean`     | Boolean-Typ           | `induce flag: boolean = true;`   |
| `trance`      | Spezieller Trance-Typ | `induce state: trance;`          |

## Literale

| Schlüsselwort | Beschreibung    | Beispiel                            |
| ------------- | --------------- | ----------------------------------- |
| `true`        | Boolean-Literal | `induce isActive: boolean = true;`  |
| `false`       | Boolean-Literal | `induce isActive: boolean = false;` |

## Testing/Debugging

| Schlüsselwort | Beschreibung | Beispiel        |
| ------------- | ------------ | --------------- |
| `assert`      | Assertion    | `assert x > 0;` |

## Hypnotische Operatoren

### Vergleichsoperatoren

| Hypnotisch                | Standard | Bedeutung      |
| ------------------------- | -------- | -------------- |
| `youAreFeelingVerySleepy` | `==`     | Gleich         |
| `youCannotResist`         | `!=`     | Ungleich       |
| `lookAtTheWatch`          | `>`      | Größer         |
| `fallUnderMySpell`        | `<`      | Kleiner        |
| `yourEyesAreGettingHeavy` | `>=`     | Größer gleich  |
| `goingDeeper`             | `<=`     | Kleiner gleich |

### Legacy-Operatoren (veraltet, aber unterstützt)

| Hypnotisch      | Standard | Hinweis                            |
| --------------- | -------- | ---------------------------------- |
| `notSoDeep`     | `!=`     | Verwende `youCannotResist`         |
| `deeplyGreater` | `>=`     | Verwende `yourEyesAreGettingHeavy` |
| `deeplyLess`    | `<=`     | Verwende `goingDeeper`             |

### Logische Operatoren

| Hypnotisch           | Standard | Bedeutung      |
| -------------------- | -------- | -------------- |
| `underMyControl`     | `&&`     | Logisches UND  |
| `resistanceIsFutile` | `\|\|`   | Logisches ODER |

## Verwendungshinweise

### Case-Insensitivity

Alle Schlüsselwörter sind **case-insensitive** beim Lexing, werden aber zu ihrer kanonischen Form normalisiert:

```hyp
// Alle folgenden sind äquivalent:
Focus { ... } Relax
focus { ... } relax
FOCUS { ... } RELAX
```

### Standard-Synonyme

Für bessere Lesbarkeit unterstützt HypnoScript Standard-Synonyme:

- `return` → `awaken`
- `break` → `snap`
- `continue` → `sink`

### Empfehlungen

1. **Verwende kanonische Formen** für bessere Lesbarkeit
2. **Nutze hypnotische Operatoren** für thematische Konsistenz
3. **Vermeide Legacy-Operatoren** (`notSoDeep`, `deeplyGreater`, `deeplyLess`)
4. **Bevorzuge `induce`** gegenüber `implant` für Standardvariablen
5. **Nutze `freeze`** für unveränderbare Werte statt `induce`
