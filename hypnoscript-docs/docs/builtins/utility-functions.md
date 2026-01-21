---
sidebar_position: 5
---

# Utility Functions

Utility functions provide general helpers for type conversion, comparisons, time, randomness, error handling, and more.

## Type Conversion

### ToNumber(value)

Converts a value to a number (integer or float).

```hyp
induce n1 = ToNumber("42"); // 42
induce n2 = ToNumber("3.14"); // 3.14
induce n3 = ToNumber(true); // 1
induce n4 = ToNumber(false); // 0
```

### ToString(value)

Converts a value to a string.

```hyp
induce s1 = ToString(42); // "42"
induce s2 = ToString(3.14); // "3.14"
induce s3 = ToString(true); // "true"
```

### ToBoolean(value)

Converts a value to a boolean.

```hyp
induce b1 = ToBoolean(1); // true
induce b2 = ToBoolean(0); // false
induce b3 = ToBoolean("true"); // true
induce b4 = ToBoolean(""); // false
```

### ParseJSON(str)

Parses a JSON string into an object/array.

```hyp
induce obj = ParseJSON('{"name": "Max", "age": 30}');
induce name = obj.name; // "Max"
```

### StringifyJSON(value)

Converts an object/array into a JSON string.

```hyp
induce arr = [1, 2, 3];
induce json = StringifyJSON(arr); // "[1,2,3]"
```

## Comparisons & Checks

### IsNull(value)

Checks whether a value is null.

```hyp
induce n = null;
induce isNull = IsNull(n); // true
```

### IsDefined(value)

Checks whether a value is defined (not null).

```hyp
induce x = 42;
induce isDef = IsDefined(x); // true
```

### IsNumber(value)

Checks whether a value is a number.

```hyp
induce isNum1 = IsNumber(42); // true
induce isNum2 = IsNumber("42"); // false
```

### IsString(value)

Checks whether a value is a string.

```hyp
induce isStr1 = IsString("Hallo"); // true
induce isStr2 = IsString(42); // false
```

### IsArray(value)

Checks whether a value is an array.

```hyp
induce arr = [1,2,3];
induce isArr = IsArray(arr); // true
```

### IsObject(value)

Checks whether a value is an object.

```hyp
induce obj = ParseJSON('{"a":1}');
induce isObj = IsObject(obj); // true
```

### IsBoolean(value)

Checks whether a value is a boolean.

```hyp
induce isBool1 = IsBoolean(true); // true
induce isBool2 = IsBoolean(0); // false
```

### TypeOf(value)

Returns the type of a value as a string.

```hyp
induce t1 = TypeOf(42); // "number"
induce t2 = TypeOf("abc"); // "string"
induce t3 = TypeOf([1,2,3]); // "array"
```

## Time Functions

### Now()

Returns the current date and time as a string.

```hyp
induce now = Now(); // "2025-05-01T12:34:56Z"
```

### Timestamp()

Returns the current Unix timestamp (seconds since 1970-01-01).

```hyp
induce ts = Timestamp(); // 1714569296
```

### Sleep(ms)

Pauses execution for the specified time in milliseconds.

```hyp
Sleep(1000); // wait 1 second
```

## Random Functions

### Shuffle(array)

Shuffles the elements of an array randomly.

```hyp
induce arr = [1,2,3,4,5];
induce shuffled = Shuffle(arr);
```

### Sample(array, count)

Selects random elements from an array.

```hyp
induce arr = [1,2,3,4,5];
induce sample = Sample(arr, 2); // e.g. [3,5]
```

## Error Handling

### Try(expr, fallback)

Attempts to execute an expression and returns a fallback value on error.

```hyp
induce result = Try(Divide(10, 0), "Error"); // "Error"
```

### Throw(message)

Throws an error with a message.

```hyp
Throw("Invalid value!");
```

## Other Utility Functions

### Range(start, end, step)

Creates an array of numbers in a range.

```hyp
induce r1 = Range(1, 5); // [1,2,3,4,5]
induce r2 = Range(0, 10, 2); // [0,2,4,6,8,10]
```

### Repeat(value, count)

Creates an array with repeated values.

```hyp
induce arr = Repeat("A", 3); // ["A","A","A"]
```

### Zip(array1, array2)

Combines two arrays into an array of pairs.

```hyp
induce a = [1,2,3];
induce b = ["a","b","c"];
induce zipped = Zip(a, b); // [[1,"a"],[2,"b"],[3,"c"]]
```

### Unzip(array)

Splits an array of pairs into two arrays.

```hyp
induce pairs = [[1,"a"],[2,"b"]];
induce [nums, chars] = Unzip(pairs);
```

### ChunkArray(array, size)

Splits an array into chunks of the specified size.

```hyp
induce arr = [1,2,3,4,5,6];
induce chunks = ChunkArray(arr, 2); // [[1,2],[3,4],[5,6]]
```

### Flatten(array)

Flattens a nested array.

```hyp
induce nested = [[1,2],[3,4],[5]];
induce flat = Flatten(nested); // [1,2,3,4,5]
```

### Unique(array)

Removes duplicate values from an array.

```hyp
induce arr = [1,2,2,3,3,3,4];
induce unique = Unique(arr); // [1,2,3,4]
```

### Sort(array, [compareFn])

Sorts an array (optionally with a comparison function).

```hyp
induce arr = [3,1,4,1,5];
induce sorted = Sort(arr); // [1,1,3,4,5]
```

## Best Practices

- Use type checks (IsNumber, IsString, ...) for robust code.
- Use Try for safe error handling.
- Use utility functions for clean, readable, and reusable scripts.

## Examples

### Dynamic Type Conversion

```hyp
Focus {
    entrance {
        induce input = "123";
        induce n = ToNumber(input);
        if (IsNumber(n)) {
            observe "Number: " + n;
        } else {
            observe "Invalid input!";
        }
    }
} Relax;
```

### Random Selection and Shuffling

```hyp
Focus {
    entrance {
        induce names = ["Anna", "Ben", "Carla", "Dieter"];
        induce winner = Sample(names, 1);
        observe "Winner: " + winner;
        induce shuffled = Shuffle(names);
        observe "Random order: " + shuffled;
    }
} Relax;
```

### Timing

```hyp
Focus {
    entrance {
        induce start = Timestamp();
        Sleep(500);
        induce end = Timestamp();
        observe "Duration: " + (end - start) + " seconds";
    }
} Relax;
```

## Next Steps

- [System Functions](./system-functions) – Interaction with the system
- [Examples](../examples/utility-examples) – Practical utility examples

---

**Mastered utility functions? Then explore [System Functions](./system-functions)!** 🖥️
