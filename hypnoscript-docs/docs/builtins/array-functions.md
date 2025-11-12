---
sidebar_position: 2
---

# Array-Funktionen

HypnoScript bietet umfangreiche Array-Funktionen für die Arbeit mit Listen und Sammlungen von Daten.

## Grundlegende Array-Operationen

### ArrayLength(arr)

Gibt die Anzahl der Elemente in einem Array zurück.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce length = ArrayLength(numbers);
observe "Array-Länge: " + length; // 5
```

### ArrayGet(arr, index)

Ruft ein Element an einem bestimmten Index ab.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce first = ArrayGet(fruits, 0); // "Apfel"
induce second = ArrayGet(fruits, 1); // "Banane"
```

### ArraySet(arr, index, value)

Setzt ein Element an einem bestimmten Index.

```hyp
induce numbers = [1, 2, 3, 4, 5];
ArraySet(numbers, 2, 99);
observe numbers; // [1, 2, 99, 4, 5]
```

## Array-Manipulation

### ArraySort(arr)

Sortiert ein Array in aufsteigender Reihenfolge.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce sorted = ArraySort(numbers);
observe sorted; // [1, 1, 2, 3, 4, 5, 6, 9]
```

### ShuffleArray(arr)

Mischt die Elemente eines Arrays zufällig.

```hyp
induce cards = ["Herz", "Karo", "Pik", "Kreuz"];
induce shuffled = ShuffleArray(cards);
observe shuffled; // Zufällige Reihenfolge
```

### ReverseArray(arr)

Kehrt die Reihenfolge der Elemente um.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce reversed = ReverseArray(numbers);
observe reversed; // [5, 4, 3, 2, 1]
```

## Array-Analyse

### SumArray(arr)

Berechnet die Summe aller numerischen Elemente.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce sum = SumArray(numbers);
observe "Summe: " + sum; // 15
```

### AverageArray(arr)

Berechnet den Durchschnitt aller numerischen Elemente.

```hyp
induce grades = [85, 92, 78, 96, 88];
induce average = AverageArray(grades);
observe "Durchschnitt: " + average; // 87.8
```

### MinArray(arr)

Findet das kleinste Element im Array.

```hyp
induce numbers = [42, 17, 89, 3, 56];
induce min = MinArray(numbers);
observe "Minimum: " + min; // 3
```

### MaxArray(arr)

Findet das größte Element im Array.

```hyp
induce numbers = [42, 17, 89, 3, 56];
induce max = MaxArray(numbers);
observe "Maximum: " + max; // 89
```

## Array-Suche

### ArrayContains(arr, value)

Prüft, ob ein Wert im Array enthalten ist.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce hasApple = ArrayContains(fruits, "Apfel"); // true
induce hasGrape = ArrayContains(fruits, "Traube"); // false
```

### ArrayIndexOf(arr, value)

Findet den Index eines Elements im Array.

```hyp
induce colors = ["Rot", "Grün", "Blau", "Gelb"];
induce index = ArrayIndexOf(colors, "Blau");
observe "Index von Blau: " + index; // 2
```

### ArrayLastIndexOf(arr, value)

Findet den letzten Index eines Elements im Array.

```hyp
induce numbers = [1, 2, 3, 2, 4, 2, 5];
induce lastIndex = ArrayLastIndexOf(numbers, 2);
observe "Letzter Index von 2: " + lastIndex; // 5
```

## Array-Filterung

### FilterArray(arr, condition)

Filtert Array-Elemente basierend auf einer Bedingung.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
induce evenNumbers = FilterArray(numbers, "x % 2 == 0");
observe evenNumbers; // [2, 4, 6, 8, 10]
```

### RemoveDuplicates(arr)

Entfernt doppelte Elemente aus dem Array.

```hyp
induce numbers = [1, 2, 2, 3, 3, 4, 5, 5];
induce unique = RemoveDuplicates(numbers);
observe unique; // [1, 2, 3, 4, 5]
```

## Array-Transformation

### MapArray(arr, function)

Wendet eine Funktion auf jedes Element an.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce doubled = MapArray(numbers, "x * 2");
observe doubled; // [2, 4, 6, 8, 10]
```

### ChunkArray(arr, size)

Teilt ein Array in Chunks der angegebenen Größe.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8];
induce chunks = ChunkArray(numbers, 3);
observe chunks; // [[1, 2, 3], [4, 5, 6], [7, 8]]
```

### FlattenArray(arr)

Vereinfacht verschachtelte Arrays.

```hyp
induce nested = [[1, 2], [3, 4], [5, 6]];
induce flat = FlattenArray(nested);
observe flat; // [1, 2, 3, 4, 5, 6]
```

## Array-Erstellung

### Range(start, end, step)

Erstellt ein Array mit Zahlen von start bis end.

```hyp
induce range1 = Range(1, 10); // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
induce range2 = Range(0, 20, 2); // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
induce range3 = Range(10, 1, -1); // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
```

### Repeat(value, count)

Erstellt ein Array mit einem wiederholten Wert.

```hyp
induce zeros = Repeat(0, 5); // [0, 0, 0, 0, 0]
induce stars = Repeat("*", 3); // ["*", "*", "*"]
```

### CreateArray(size, defaultValue)

Erstellt ein Array mit einer bestimmten Größe und Standardwert.

```hyp
induce emptyArray = CreateArray(5); // [null, null, null, null, null]
induce filledArray = CreateArray(3, "Hallo"); // ["Hallo", "Hallo", "Hallo"]
```

## Array-Statistiken

### ArrayVariance(arr)

Berechnet die Varianz der Array-Elemente.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce variance = ArrayVariance(numbers);
observe "Varianz: " + variance;
```

### ArrayStandardDeviation(arr)

Berechnet die Standardabweichung.

```hyp
induce grades = [85, 92, 78, 96, 88];
induce stdDev = ArrayStandardDeviation(grades);
observe "Standardabweichung: " + stdDev;
```

### ArrayMedian(arr)

Findet den Median des Arrays.

```hyp
induce numbers = [1, 3, 5, 7, 9];
induce median = ArrayMedian(numbers);
observe "Median: " + median; // 5
```

## Array-Vergleiche

### ArraysEqual(arr1, arr2)

Vergleicht zwei Arrays auf Gleichheit.

```hyp
induce arr1 = [1, 2, 3];
induce arr2 = [1, 2, 3];
induce arr3 = [1, 2, 4];
induce equal1 = ArraysEqual(arr1, arr2); // true
induce equal2 = ArraysEqual(arr1, arr3); // false
```

### ArrayIntersection(arr1, arr2)

Findet die Schnittmenge zweier Arrays.

```hyp
induce arr1 = [1, 2, 3, 4, 5];
induce arr2 = [3, 4, 5, 6, 7];
induce intersection = ArrayIntersection(arr1, arr2);
observe intersection; // [3, 4, 5]
```

### ArrayUnion(arr1, arr2)

Vereinigt zwei Arrays ohne Duplikate.

```hyp
induce arr1 = [1, 2, 3];
induce arr2 = [3, 4, 5];
induce union = ArrayUnion(arr1, arr2);
observe union; // [1, 2, 3, 4, 5]
```

## Praktische Beispiele

### Zahlenraten-Spiel

```hyp
Focus {
    entrance {
        induce secretNumber = 42;
        induce guesses = [];
        induce maxGuesses = 10;

        for (induce i = 1; i <= maxGuesses; induce i = i + 1) {
            induce guess = 25 + i * 2; // Vereinfachte Eingabe
            induce guesses = ArrayUnion(guesses, [guess]);

            if (guess == secretNumber) {
                observe "Gewonnen! Versuche: " + ArrayLength(guesses);
                break;
            } else if (guess < secretNumber) {
                observe "Zu niedrig!";
            } else {
                observe "Zu hoch!";
            }
        }

        observe "Alle Versuche: " + guesses;
    }
} Relax;
```

### Notenverwaltung

```hyp
Focus {
    entrance {
        induce grades = [85, 92, 78, 96, 88, 91, 83, 89];

        observe "Noten: " + grades;
        observe "Anzahl: " + ArrayLength(grades);
        observe "Durchschnitt: " + AverageArray(grades);
        observe "Beste Note: " + MaxArray(grades);
        observe "Schlechteste Note: " + MinArray(grades);

        induce sortedGrades = ArraySort(grades);
        observe "Sortiert: " + sortedGrades;

        induce median = ArrayMedian(sortedGrades);
        observe "Median: " + median;
    }
} Relax;
```

### Datenanalyse

```hyp
Focus {
    entrance {
        induce temperatures = [22.5, 24.1, 19.8, 26.3, 23.7, 21.2, 25.9];

        observe "Temperaturen: " + temperatures;
        observe "Durchschnitt: " + AverageArray(temperatures);
        observe "Maximum: " + MaxArray(temperatures);
        observe "Minimum: " + MinArray(temperatures);

        induce variance = ArrayVariance(temperatures);
        induce stdDev = ArrayStandardDeviation(temperatures);
        observe "Varianz: " + variance;
        observe "Standardabweichung: " + stdDev;

        induce warmDays = FilterArray(temperatures, "x > 25");
        observe "Warme Tage (>25°C): " + warmDays;
    }
} Relax;
```

## Best Practices

### Effiziente Array-Operationen

```hyp
// Array-Länge einmal berechnen
induce length = ArrayLength(arr);
for (induce i = 0; i < length; induce i = i + 1) {
    // Operationen
}

// Große Arrays in Chunks verarbeiten
induce largeArray = Range(1, 10000);
induce chunks = ChunkArray(largeArray, 1000);
for (induce i = 0; i < ArrayLength(chunks); induce i = i + 1) {
    induce chunk = ArrayGet(chunks, i);
    // Chunk verarbeiten
}
```

### Fehlerbehandlung

```hyp
// Sichere Array-Zugriffe
Trance safeArrayGet(arr, index) {
    if (index < 0 || index >= ArrayLength(arr)) {
        return null;
    }
    return ArrayGet(arr, index);
}

// Array-Validierung
Trance isValidArray(arr) {
    return arr != null && ArrayLength(arr) > 0;
}
```

## Nächste Schritte

- [String-Funktionen](./string-functions) - String-Manipulation
- [Mathematische Funktionen](./math-functions) - Mathematische Operationen
- [Utility-Funktionen](./utility-functions) - Allgemeine Hilfsfunktionen
- [Beispiele](../examples/array-examples) - Weitere Array-Beispiele

---

**Beherrschst du Array-Funktionen? Dann lerne [String-Funktionen](./string-functions) kennen!** 📝
