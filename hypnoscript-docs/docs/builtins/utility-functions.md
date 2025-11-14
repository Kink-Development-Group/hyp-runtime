---
sidebar_position: 5
---

# Utility-Funktionen

Utility-Funktionen bieten allgemeine Hilfsmittel für Typumwandlung, Vergleiche, Zeit, Zufall, Fehlerbehandlung und mehr.

## Typumwandlung

### ToNumber(value)

Konvertiert einen Wert in eine Zahl (Integer oder Float).

```hyp
induce n1 = ToNumber("42"); // 42
induce n2 = ToNumber("3.14"); // 3.14
induce n3 = ToNumber(true); // 1
induce n4 = ToNumber(false); // 0
```

### ToString(value)

Konvertiert einen Wert in einen String.

```hyp
induce s1 = ToString(42); // "42"
induce s2 = ToString(3.14); // "3.14"
induce s3 = ToString(true); // "true"
```

### ToBoolean(value)

Konvertiert einen Wert in einen booleschen Wert.

```hyp
induce b1 = ToBoolean(1); // true
induce b2 = ToBoolean(0); // false
induce b3 = ToBoolean("true"); // true
induce b4 = ToBoolean(""); // false
```

### ParseJSON(str)

Parst einen JSON-String in ein Objekt/Array.

```hyp
induce obj = ParseJSON('{"name": "Max", "age": 30}');
induce name = obj.name; // "Max"
```

### StringifyJSON(value)

Wandelt ein Objekt/Array in einen JSON-String um.

```hyp
induce arr = [1, 2, 3];
induce json = StringifyJSON(arr); // "[1,2,3]"
```

## Vergleiche & Prüfungen

### IsNull(value)

Prüft, ob ein Wert null ist.

```hyp
induce n = null;
induce isNull = IsNull(n); // true
```

### IsDefined(value)

Prüft, ob ein Wert definiert ist (nicht null).

```hyp
induce x = 42;
induce isDef = IsDefined(x); // true
```

### IsNumber(value)

Prüft, ob ein Wert eine Zahl ist.

```hyp
induce isNum1 = IsNumber(42); // true
induce isNum2 = IsNumber("42"); // false
```

### IsString(value)

Prüft, ob ein Wert ein String ist.

```hyp
induce isStr1 = IsString("Hallo"); // true
induce isStr2 = IsString(42); // false
```

### IsArray(value)

Prüft, ob ein Wert ein Array ist.

```hyp
induce arr = [1,2,3];
induce isArr = IsArray(arr); // true
```

### IsObject(value)

Prüft, ob ein Wert ein Objekt ist.

```hyp
induce obj = ParseJSON('{"a":1}');
induce isObj = IsObject(obj); // true
```

### IsBoolean(value)

Prüft, ob ein Wert ein boolescher Wert ist.

```hyp
induce isBool1 = IsBoolean(true); // true
induce isBool2 = IsBoolean(0); // false
```

### TypeOf(value)

Gibt den Typ eines Wertes als String zurück.

```hyp
induce t1 = TypeOf(42); // "number"
induce t2 = TypeOf("abc"); // "string"
induce t3 = TypeOf([1,2,3]); // "array"
```

## Zeitfunktionen

### Now()

Gibt das aktuelle Datum und die aktuelle Uhrzeit als String zurück.

```hyp
induce now = Now(); // "2025-05-01T12:34:56Z"
```

### Timestamp()

Gibt den aktuellen Unix-Timestamp (Sekunden seit 1970-01-01).

```hyp
induce ts = Timestamp(); // 1714569296
```

### Sleep(ms)

Pausiert die Ausführung für die angegebene Zeit in Millisekunden.

```hyp
Sleep(1000); // 1 Sekunde warten
```

## Zufallsfunktionen

### Shuffle(array)

Mischt die Elemente eines Arrays zufällig.

```hyp
induce arr = [1,2,3,4,5];
induce shuffled = Shuffle(arr);
```

### Sample(array, count)

Wählt zufällige Elemente aus einem Array.

```hyp
induce arr = [1,2,3,4,5];
induce sample = Sample(arr, 2); // z.B. [3,5]
```

## Fehlerbehandlung

### Try(expr, fallback)

Versucht, einen Ausdruck auszuführen, und gibt im Fehlerfall einen Fallback-Wert zurück.

```hyp
induce result = Try(Divide(10, 0), "Fehler"); // "Fehler"
```

### Throw(message)

Löst einen Fehler mit einer Nachricht aus.

```hyp
Throw("Ungültiger Wert!");
```

## Sonstige Utility-Funktionen

### Range(start, end, step)

Erzeugt ein Array von Zahlen im Bereich.

```hyp
induce r1 = Range(1, 5); // [1,2,3,4,5]
induce r2 = Range(0, 10, 2); // [0,2,4,6,8,10]
```

### Repeat(value, count)

Erzeugt ein Array mit wiederholten Werten.

```hyp
induce arr = Repeat("A", 3); // ["A","A","A"]
```

### Zip(array1, array2)

Verbindet zwei Arrays zu einem Array von Paaren.

```hyp
induce a = [1,2,3];
induce b = ["a","b","c"];
induce zipped = Zip(a, b); // [[1,"a"],[2,"b"],[3,"c"]]
```

### Unzip(array)

Teilt ein Array von Paaren in zwei Arrays.

```hyp
induce pairs = [[1,"a"],[2,"b"]];
induce [nums, chars] = Unzip(pairs);
```

### ChunkArray(array, size)

Teilt ein Array in Blöcke der angegebenen Größe.

```hyp
induce arr = [1,2,3,4,5,6];
induce chunks = ChunkArray(arr, 2); // [[1,2],[3,4],[5,6]]
```

### Flatten(array)

Macht ein verschachteltes Array flach.

```hyp
induce nested = [[1,2],[3,4],[5]];
induce flat = Flatten(nested); // [1,2,3,4,5]
```

### Unique(array)

Entfernt doppelte Werte aus einem Array.

```hyp
induce arr = [1,2,2,3,3,3,4];
induce unique = Unique(arr); // [1,2,3,4]
```

### Sort(array, [compareFn])

Sortiert ein Array (optional mit Vergleichsfunktion).

```hyp
induce arr = [3,1,4,1,5];
induce sorted = Sort(arr); // [1,1,3,4,5]
```

## Best Practices

- Nutze Typprüfungen (IsNumber, IsString, ...) für robusten Code.
- Verwende Try für sichere Fehlerbehandlung.
- Nutze Utility-Funktionen für saubere, lesbare und wiederverwendbare Skripte.

## Beispiele

### Dynamische Typumwandlung

```hyp
Focus {
    entrance {
        induce input = "123";
        induce n = ToNumber(input);
        if (IsNumber(n)) {
            observe "Zahl: " + n;
        } else {
            observe "Ungültige Eingabe!";
        }
    }
} Relax;
```

### Zufällige Auswahl und Mischen

```hyp
Focus {
    entrance {
        induce names = ["Anna", "Ben", "Carla", "Dieter"];
        induce winner = Sample(names, 1);
        observe "Gewinner: " + winner;
        induce shuffled = Shuffle(names);
        observe "Zufällige Reihenfolge: " + shuffled;
    }
} Relax;
```

### Zeitmessung

```hyp
Focus {
    entrance {
        induce start = Timestamp();
        Sleep(500);
        induce end = Timestamp();
        observe "Dauer: " + (end - start) + " Sekunden";
    }
} Relax;
```

## Nächste Schritte

- [System-Funktionen](./system-functions) – Interaktion mit dem System
- [Beispiele](../examples/utility-examples) – Praktische Utility-Beispiele

---

**Utility-Funktionen gemeistert? Dann lerne [System-Funktionen](./system-functions) kennen!** 🖥️
