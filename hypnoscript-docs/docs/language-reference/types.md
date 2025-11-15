---
sidebar_position: 3
---

# Typ-System

HypnoScript setzt auf ein **statisches Typ-System**. Jede Variable, jedes Feld und jeder Rückgabewert besitzt einen klar definierten Typ, der bereits zur Übersetzungszeit geprüft wird. Dadurch werden viele Fehler früh erkannt und Laufzeitüberraschungen vermieden.

## Überblick über die Basistypen

| Typ        | Beschreibung                                                         | Beispielcode                                |
| ---------- | -------------------------------------------------------------------- | ------------------------------------------- |
| `number`   | Gleitkommazahl mit doppelter Genauigkeit                             | `induce temperatur: number = 21.5;`         |
| `string`   | UTF-8 Text, unterstützt Unicode vollumfänglich                       | `induce begruessung: string = "Hallo";`     |
| `boolean`  | Wahrheitswert `true` oder `false`                                    | `induce aktiv: boolean = true;`             |
| `trance`   | Hypnotischer Zustand, wird für Sessions und Suggestionen verwendet   | `induce zustand: trance = induceTrance();`  |
| `array`    | Geordnete Liste mit einheitlichem Elementtyp                         | `induce zahlen: number[] = [1, 2, 3];`      |
| `record`   | Benannter Satz von Feldern mit eigenen Typen                         | `induce klient: Klient = { name, alter };`  |
| `object`   | Dynamisches Objekt, typischerweise für externe Integrationen genutzt | `induce daten: object = loadJson();`        |
| `function` | Funktionsreferenz mit Parametern und Rückgabewert                    | `induce analyseeinheit = suggestion(...)`   |
| `session`  | Laufende HypnoScript-Session                                         | `induce sitzung: session = beginSession();` |
| `unknown`  | Platzhalter, wenn der Typ noch nicht bestimmt werden konnte          | Wird vom Type Checker intern verwendet      |

> 💡 **Hinweis:** `record`, `function` und `array` sind **zusammengesetzte Typen**. Sie tragen zusätzliche Informationen (Feldnamen, Parameterliste, Elementtyp), die beim Type Checking berücksichtigt werden.

Siehe auch [Variablen und Datentypen](./variables.md) für Grundlagen zur Deklaration von Variablen.

## Typannotation und Inferenz

Du kannst Typen explizit angeben oder dem Compiler die Arbeit überlassen:

```hyp
// Explizite Annotation
induce zaehler: number = 0;

// Typinferenz durch den Compiler
induce begruessung = "Willkommen"; // abgeleiteter Typ: string

// Explizite Parameter- und Rückgabetypen bei Funktionen
suggestion verdoppeln(wert: number): number {
    awaken wert * 2;
}
```

Der Compiler versucht stets, den konkretesten Typ abzuleiten. Wenn er keine eindeutige Aussage treffen kann, setzt er intern `unknown` ein und meldet eine Typwarnung oder -fehlermeldung.

## Zusammengesetzte Typen

### Arrays

Arrays sind immer homogen. Der Elementtyp steht hinter dem Array-Namen in eckigen Klammern:

```hyp
induce namen: string[] = ["Sam", "Alex", "Riley"];

induce messwerte: number[];
messwerte = collectValues();
```

Bei Operationen auf Arrays achtet der Type Checker darauf, dass nur passende Elemente eingefügt werden.

### Records

Records kombinieren mehrere Felder zu einem strukturierten Typ:

```hyp
induce Klient = record {
    name: string,
    alter: number,
    aktiv: boolean
};

induce klient: Klient = {
    name: "Mira",
    alter: 29,
    aktiv: true
};
```

Die Struktur eines Records ist **strukturell** – zwei Records sind kompatibel, wenn sie die gleichen Feldnamen und Typen besitzen.

### Funktionen

Funktionen tragen einen vollständigen Signatur-Typ, bestehend aus Parameterliste und Rückgabewert:

```hyp
suggestion hypnoticGreeting(name: string, dauer: number): string {
    observe name;
    observe dauer;
    awaken "Willkommen zurück";
}
```

Funktionstypen können wie jede andere Wertform gespeichert und weitergegeben werden:

```hyp
induce begruessungsFunktion: (string, number) -> string = hypnoticGreeting;
```

## Kompatibilitätsregeln

Der Type Checker nutzt strenge, aber pragmatische Kompatibilitätsregeln:

- **Primitive Typen** müssen exakt übereinstimmen (`number` ist nicht automatisch mit `string` kompatibel).
- **Arrays** sind kompatibel, wenn ihre Elementtypen kompatibel sind.
- **Records** vergleichen Feldanzahl, Feldnamen und Feldtypen.
- **Funktionen** benötigen identische Parameteranzahl sowie kompatible Parameter- und Rückgabetypen.
- **Sessions** und **Trance-Zustände** sind eigene Typen und werden nicht implizit in andere Typen umgewandelt.

Wenn zwei Typen nicht kompatibel sind, meldet der Compiler einen Fehler mit Hinweis auf den erwarteten und den tatsächlich gefundenen Typ.

## Arbeit mit `unknown`

`unknown` dient als Fallback, wenn der Typ nicht eindeutig ermittelt werden kann – beispielsweise bei dynamischen Datenquellen. Ziel sollte es sein, `unknown` so früh wie möglich in einen konkreten Typ zu überführen:

```hyp
induce daten: unknown = loadExternal();

if (isRecord(daten)) {
    induce struktur = cast<Klient>(daten);
    observe struktur.name;
} else {
    warn "Externe Daten konnten nicht interpretiert werden.";
}
```

## Weitere Ressourcen

- [Kontrollstrukturen](./control-flow.md) – Typsichere Entscheidungs- und Schleifenkonstrukte
- [Funktionen](./functions.md) – Signaturen, Rückgabewerte und Inline-Funktionen
- [Records](./records.md) – Detaillierte Einführung in strukturierte Daten

Mit einem klaren Verständnis des Typ-Systems kannst du HypnoScript-Programme schreiben, die sowohl hypnotisch als auch robust sind.```
