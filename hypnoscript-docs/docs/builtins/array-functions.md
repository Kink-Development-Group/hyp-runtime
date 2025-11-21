---
sidebar_position: 2
---

# Array Functions

:::tip Complete Reference
See [Builtin Functions Complete Reference](./_complete-reference#array-builtins) for the **up-to-date, complete documentation** of all array functions with correct function names.
:::

:::warning Note
This page contains partially outdated function names. The correct reference can be found in the [Complete Reference](./_complete-reference#array-builtins).
:::

HypnoScript provides comprehensive array functions for working with lists and data collections.

## Basic Array Operations

### ArrayLength(arr)

Returns the number of elements in an array.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce length = ArrayLength(numbers);
observe "Array-Länge: " + length; // 5
```

### ArrayGet(arr, index)

Retrieves an element at a specific index.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce first = ArrayGet(fruits, 0); // "Apfel"
induce second = ArrayGet(fruits, 1); // "Banane"
```

### ArraySet(arr, index, value)

Sets an element at a specific index.

```hyp
induce numbers = [1, 2, 3, 4, 5];
ArraySet(numbers, 2, 99);
observe numbers; // [1, 2, 99, 4, 5]
```

## Array Manipulation

### ArraySort(arr)

Sorts an array in ascending order.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce sorted = ArraySort(numbers);
observe sorted; // [1, 1, 2, 3, 4, 5, 6, 9]
```

### ShuffleArray(arr)

Randomly shuffles the elements of an array.

```hyp
induce cards = ["Herz", "Karo", "Pik", "Kreuz"];
induce shuffled = ShuffleArray(cards);
observe shuffled; // Random order
```

### ReverseArray(arr)

Reverses the order of elements.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce reversed = ReverseArray(numbers);
observe reversed; // [5, 4, 3, 2, 1]
```

## Array Analysis

### ArraySum(arr)

Calculates the sum of all numeric elements.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce sum = ArraySum(numbers);
observe "Summe: " + sum; // 15
```

### AverageArray(arr)

Calculates the average of all numeric elements.

```hyp
induce grades = [85, 92, 78, 96, 88];
induce average = AverageArray(grades);
observe "Durchschnitt: " + average; // 87.8
```

### MinArray(arr)

Finds the smallest element in the array.

```hyp
induce numbers = [42, 17, 89, 3, 56];
induce min = MinArray(numbers);
observe "Minimum: " + min; // 3
```

### MaxArray(arr)

Finds the largest element in the array.

```hyp
induce numbers = [42, 17, 89, 3, 56];
induce max = MaxArray(numbers);
observe "Maximum: " + max; // 89
```

## Array Search

### ArrayContains(arr, value)

Checks if a value is contained in the array.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce hasApple = ArrayContains(fruits, "Apfel"); // true
induce hasGrape = ArrayContains(fruits, "Traube"); // false
```

### ArrayIndexOf(arr, value)

Finds the index of an element in the array.

```hyp
induce colors = ["Rot", "Grün", "Blau", "Gelb"];
induce index = ArrayIndexOf(colors, "Blau");
observe "Index von Blau: " + index; // 2
```

### ArrayLastIndexOf(arr, value)

Finds the last index of an element in the array.

```hyp
induce numbers = [1, 2, 3, 2, 4, 2, 5];
induce lastIndex = ArrayLastIndexOf(numbers, 2);
observe "Letzter Index von 2: " + lastIndex; // 5
```

## Array Filtering

### FilterArray(arr, condition)

Filters array elements based on a condition.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
induce evenNumbers = FilterArray(numbers, "x % 2 == 0");
observe evenNumbers; // [2, 4, 6, 8, 10]
```

### RemoveDuplicates(arr)

Removes duplicate elements from the array.

```hyp
induce numbers = [1, 2, 2, 3, 3, 4, 5, 5];
induce unique = RemoveDuplicates(numbers);
observe unique; // [1, 2, 3, 4, 5]
```

## Array Transformation

### MapArray(arr, function)

Applies a function to each element.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce doubled = MapArray(numbers, "x * 2");
observe doubled; // [2, 4, 6, 8, 10]
```

### ChunkArray(arr, size)

Divides an array into chunks of the specified size.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8];
induce chunks = ChunkArray(numbers, 3);
observe chunks; // [[1, 2, 3], [4, 5, 6], [7, 8]]
```

### FlattenArray(arr)

Flattens nested arrays.

```hyp
induce nested = [[1, 2], [3, 4], [5, 6]];
induce flat = FlattenArray(nested);
observe flat; // [1, 2, 3, 4, 5, 6]
```

## Array Creation

### Range(start, end, step)

Creates an array with numbers from start to end.

```hyp
induce range1 = Range(1, 10); // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
induce range2 = Range(0, 20, 2); // [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20]
induce range3 = Range(10, 1, -1); // [10, 9, 8, 7, 6, 5, 4, 3, 2, 1]
```

### Repeat(value, count)

Creates an array with a repeated value.

```hyp
induce zeros = Repeat(0, 5); // [0, 0, 0, 0, 0]
induce stars = Repeat("*", 3); // ["*", "*", "*"]
```

### CreateArray(size, defaultValue)

Creates an array with a specific size and default value.

```hyp
induce emptyArray = CreateArray(5); // [null, null, null, null, null]
induce filledArray = CreateArray(3, "Hallo"); // ["Hallo", "Hallo", "Hallo"]
```

## Array Statistics

### ArrayVariance(arr)

Calculates the variance of array elements.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce variance = ArrayVariance(numbers);
observe "Varianz: " + variance;
```

### ArrayStandardDeviation(arr)

Calculates the standard deviation.

```hyp
induce grades = [85, 92, 78, 96, 88];
induce stdDev = ArrayStandardDeviation(grades);
observe "Standardabweichung: " + stdDev;
```

### ArrayMedian(arr)

Finds the median of the array.

```hyp
induce numbers = [1, 3, 5, 7, 9];
induce median = ArrayMedian(numbers);
observe "Median: " + median; // 5
```

## Array Comparisons

### ArraysEqual(arr1, arr2)

Compares two arrays for equality.

```hyp
induce arr1 = [1, 2, 3];
induce arr2 = [1, 2, 3];
induce arr3 = [1, 2, 4];
induce equal1 = ArraysEqual(arr1, arr2); // true
induce equal2 = ArraysEqual(arr1, arr3); // false
```

### ArrayIntersection(arr1, arr2)

Finds the intersection of two arrays.

```hyp
induce arr1 = [1, 2, 3, 4, 5];
induce arr2 = [3, 4, 5, 6, 7];
induce intersection = ArrayIntersection(arr1, arr2);
observe intersection; // [3, 4, 5]
```

### ArrayUnion(arr1, arr2)

Unites two arrays without duplicates.

```hyp
induce arr1 = [1, 2, 3];
induce arr2 = [3, 4, 5];
induce union = ArrayUnion(arr1, arr2);
observe union; // [1, 2, 3, 4, 5]
```

## Practical Examples

### Number Guessing Game

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
// Calculate array length once
induce length = ArrayLength(arr);
for (induce i = 0; i < length; induce i = i + 1) {
    // Operationen
}

// Process large arrays in chunks
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
suggestion safeArrayGet(arr, index) {
    if (index < 0 || index >= ArrayLength(arr)) {
        awaken null;
    }
    return ArrayGet(arr, index);
}

// Array-Validierung
suggestion isValidArray(arr) {
    awaken arr != null && ArrayLength(arr) > 0;
}
```

## Next Steps

- [String Functions](./string-functions) - String manipulation
- [Mathematical Functions](./math-functions) - Mathematical operations
- [Utility Functions](./utility-functions) - General helper functions
- [Examples](../examples/array-examples) - More array examples

---

**Mastered array functions? Then learn about [String Functions](./string-functions)!** 📝
