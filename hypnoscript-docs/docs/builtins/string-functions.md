---
sidebar_position: 3
---

# String Functions

:::tip Complete Reference
See [Builtin Functions Complete Reference](./_complete-reference#string-builtins) for the complete, up-to-date documentation of all string functions.
:::

HypnoScript provides comprehensive string functions for text processing, manipulation, and analysis.

## Basic String Operations

### Length(str)

Returns the length of a string.

```hyp
induce text = "HypnoScript";
induce length = Length(text);
observe "Length: " + length; // 11
```

### Substring(str, start, length)

Extracts a substring from a string.

```hyp
induce text = "HypnoScript";
induce part1 = Substring(text, 0, 5); // "Hypno"
induce part2 = Substring(text, 5, 6); // "Script"
```

### Concat(str1, str2, ...)

Concatenates multiple strings.

```hyp
induce firstName = "John";
induce lastName = "Doe";
induce fullName = Concat(firstName, " ", lastName);
observe fullName; // "John Doe"
```

## String Manipulation

### ToUpper(str)

Converts a string to uppercase.

```hyp
induce text = "HypnoScript";
induce upper = ToUpper(text);
observe upper; // "HYPNOSCRIPT"
```

### ToLower(str)

Converts a string to lowercase.

```hyp
induce text = "HypnoScript";
induce lower = ToLower(text);
observe lower; // "hypnoscript"
```

### Capitalize(str)

Capitalizes the first letter.

```hyp
induce text = "hypnoscript";
induce capitalized = Capitalize(text);
observe capitalized; // "Hypnoscript"
```

### TitleCase(str)

Capitalizes the first letter of each word.

```hyp
induce text = "hypno script programming";
induce titleCase = TitleCase(text);
observe titleCase; // "Hypno Script Programming"
```

## String Analysis

### IsEmpty(str)

Checks if a string is empty.

```hyp
induce empty = "";
induce notEmpty = "Hello";
induce isEmpty1 = IsEmpty(empty); // true
induce isEmpty2 = IsEmpty(notEmpty); // false
```

### IsWhitespace(str)

Checks if a string contains only whitespace.

```hyp
induce whitespace = "   \t\n  ";
induce text = "Hello World";
induce isWhitespace1 = IsWhitespace(whitespace); // true
induce isWhitespace2 = IsWhitespace(text); // false
```

### Contains(str, substring)

Checks if a string contains a substring.

```hyp
induce text = "HypnoScript is a programming language";
induce hasScript = Contains(text, "Script"); // true
induce hasPython = Contains(text, "Python"); // false
```

### StartsWith(str, prefix)

Checks if a string starts with a prefix.

```hyp
induce text = "HypnoScript";
induce startsWithHypno = StartsWith(text, "Hypno"); // true
induce startsWithScript = StartsWith(text, "Script"); // false
```

### EndsWith(str, suffix)

Checks if a string ends with a suffix.

```hyp
induce text = "HypnoScript";
induce endsWithScript = EndsWith(text, "Script"); // true
induce endsWithHypno = EndsWith(text, "Hypno"); // false
```

## String Search

### IndexOf(str, substring)

Finds the first index of a substring.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce index = IndexOf(text, "Script");
observe "Index of 'Script': " + index; // 5
```

### LastIndexOf(str, substring)

Finds the last index of a substring.

```hyp
induce text = "HypnoScript Script Script";
induce lastIndex = LastIndexOf(text, "Script");
observe "Last index of 'Script': " + lastIndex; // 18
```

### CountOccurrences(str, substring)

Counts the occurrences of a substring.

```hyp
induce text = "HypnoScript Script Script";
induce count = CountOccurrences(text, "Script");
observe "Count of 'Script': " + count; // 3
```

## String Transformation

### Reverse(str)

Reverses a string.

```hyp
induce text = "HypnoScript";
induce reversed = Reverse(text);
observe reversed; // "tpircSonpyH"
```

### Trim(str)

Removes whitespace from the beginning and end.

```hyp
induce text = "  HypnoScript  ";
induce trimmed = Trim(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### TrimStart(str)

Removes whitespace from the beginning.

```hyp
induce text = "  HypnoScript";
induce trimmed = TrimStart(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### TrimEnd(str)

Removes whitespace from the end.

```hyp
induce text = "HypnoScript  ";
induce trimmed = TrimEnd(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### Replace(str, oldValue, newValue)

Replaces all occurrences of a substring.

```hyp
induce text = "HypnoScript is a programming language";
induce replaced = Replace(text, "programming language", "language");
observe replaced; // "HypnoScript is a language"
```

### ReplaceAll(str, oldValue, newValue)

Replaces all occurrences (alias for Replace).

```hyp
induce text = "Hello Hello Hello";
induce replaced = ReplaceAll(text, "Hello", "Hi");
observe replaced; // "Hi Hi Hi"
```

## String Formatting

### PadLeft(str, width, char)

Pads a string on the left with characters.

```hyp
induce text = "42";
induce padded = PadLeft(text, 5, "0");
observe padded; // "00042"
```

### PadRight(str, width, char)

Pads a string on the right with characters.

```hyp
induce text = "Hello";
induce padded = PadRight(text, 10, "*");
observe padded; // "Hallo*****"
```

### FormatString(template, ...args)

Formats a string with placeholders.

```hyp
induce name = "Max";
induce age = 30;
induce formatted = FormatString("Hello {0}, you are {1} years old", name, age);
observe formatted; // "Hello Max, you are 30 years old"
```

## String Analysis (Advanced)

### IsPalindrome(str)

Checks if a string is a palindrome.

```hyp
induce palindrome1 = "anna";
induce palindrome2 = "racecar";
induce notPalindrome = "hello";
induce isPal1 = IsPalindrome(palindrome1); // true
induce isPal2 = IsPalindrome(palindrome2); // true
induce isPal3 = IsPalindrome(notPalindrome); // false
```

### IsNumeric(str)

Checks if a string represents a number.

```hyp
induce numeric1 = "123";
induce numeric2 = "3.14";
induce notNumeric = "abc";
induce isNum1 = IsNumeric(numeric1); // true
induce isNum2 = IsNumeric(numeric2); // true
induce isNum3 = IsNumeric(notNumeric); // false
```

### IsAlpha(str)

Checks if a string contains only letters.

```hyp
induce alpha = "HypnoScript";
induce notAlpha = "Hypno123";
induce isAlpha1 = IsAlpha(alpha); // true
induce isAlpha2 = IsAlpha(notAlpha); // false
```

### IsAlphaNumeric(str)

Checks if a string contains only letters and numbers.

```hyp
induce alphanumeric = "Hypno123";
induce notAlphanumeric = "Hypno@123";
induce isAlphaNum1 = IsAlphaNumeric(alphanumeric); // true
induce isAlphaNum2 = IsAlphaNumeric(notAlphanumeric); // false
```

## String Decomposition

### Split(str, delimiter)

Splits a string at a delimiter.

```hyp
induce text = "Apple,Banana,Orange";
induce fruits = Split(text, ",");
observe fruits; // ["Apple", "Banana", "Orange"]
```

### SplitLines(str)

Splits a string at line breaks.

```hyp
induce text = "Line 1\nLine 2\nLine 3";
induce lines = SplitLines(text);
observe lines; // ["Line 1", "Line 2", "Line 3"]
```

### SplitWords(str)

Splits a string into words.

```hyp
induce text = "HypnoScript is a programming language";
induce words = SplitWords(text);
observe words; // ["HypnoScript", "is", "a", "programming", "language"]
```

## String Statistics

### CountWords(str)

Counts the words in a string.

```hyp
induce text = "HypnoScript is a programming language";
induce wordCount = CountWords(text);
observe "Words: " + wordCount; // 5
```

### CountCharacters(str)

Counts the characters in a string.

```hyp
induce text = "Hello World!";
induce charCount = CountCharacters(text);
observe "Characters: " + charCount; // 12
```

### CountLines(str)

Counts the lines in a string.

```hyp
induce text = "Line 1\nLine 2\nLine 3";
induce lineCount = CountLines(text);
observe "Lines: " + lineCount; // 3
```

## String Comparisons

### Compare(str1, str2)

Compares two strings lexicographically.

```hyp
induce str1 = "Apple";
induce str2 = "Banana";
induce comparison = Compare(str1, str2);
observe comparison; // -1 (str1 < str2)
```

### EqualsIgnoreCase(str1, str2)

Compares two strings without case sensitivity.

```hyp
induce str1 = "HypnoScript";
induce str2 = "hypnoscript";
induce equals = EqualsIgnoreCase(str1, str2); // true
```

## String Generation

### Repeat(str, count)

Repeats a string.

```hyp
induce text = "Ha";
induce repeated = Repeat(text, 3);
observe repeated; // "HaHaHa"
```

### GenerateRandomString(length)

Generates a random string.

```hyp
induce random = GenerateRandomString(10);
observe random; // Random 10-character string
```

### GenerateUUID()

Generates a UUID.

```hyp
induce uuid = GenerateUUID();
observe uuid; // "123e4567-e89b-12d3-a456-426614174000"
```

## Practical Examples

### Text Analysis

```hyp
Focus {
    entrance {
        induce text = "HypnoScript is an innovative programming language with hypnotic syntax.";

        observe "Original: " + text;
        observe "Length: " + Length(text);
        observe "Words: " + CountWords(text);
        observe "Characters: " + CountCharacters(text);

        induce upperText = ToUpper(text);
        observe "Uppercase: " + upperText;

        induce titleText = TitleCase(text);
        observe "Title Case: " + titleText;

        induce words = SplitWords(text);
        observe "Words array: " + words;

        induce hasHypno = Contains(text, "Hypno");
        observe "Contains 'Hypno': " + hasHypno;
    }
} Relax;
```

### Email Validation

```hyp
Focus {
    suggestion validateEmail(email) {
        if (IsEmpty(email)) {
            awaken false;
        }

        if (!Contains(email, "@")) {
            return false;
        }

        induce parts = Split(email, "@");
        if (ArrayLength(parts) != 2) {
            return false;
        }

        induce localPart = ArrayGet(parts, 0);
        induce domainPart = ArrayGet(parts, 1);

        if (IsEmpty(localPart) || IsEmpty(domainPart)) {
            return false;
        }

        if (!Contains(domainPart, ".")) {
            return false;
        }

        return true;
    }

    entrance {
        induce emails = ["test@example.com", "invalid-email", "@domain.com", "user@", ""];

        for (induce i = 0; i < ArrayLength(emails); induce i = i + 1) {
            induce email = ArrayGet(emails, i);
            induce isValid = validateEmail(email);
            observe email + " is valid: " + isValid;
        }
    }
} Relax;
```

### Text Formatting

```hyp
Focus {
    entrance {
        induce name = "john doe";
        induce age = 30;
        induce city = "berlin";

        // Format name
        induce formattedName = TitleCase(name);
        observe "Name: " + formattedName; // "John Doe"

        // Format address
        induce address = Concat(formattedName, ", ", ToNumber(age), " years, ", TitleCase(city));
        observe "Address: " + address;

        // Format phone number
        induce phone = "1234567890";
        induce formattedPhone = FormatString("({0}) {1}-{2}",
            Substring(phone, 0, 3),
            Substring(phone, 3, 3),
            Substring(phone, 6, 4));
        observe "Phone: " + formattedPhone; // "(123) 456-7890"
    }
} Relax;
```

## Best Practices

### Efficient String Operations

```hyp
// Build strings
induce parts = ["Hello", "World", "!"];
induce result = Concat(ArrayGet(parts, 0), " ", ArrayGet(parts, 1), ArrayGet(parts, 2));

// String comparisons
if (EqualsIgnoreCase(input, "yes")) {
    // Case-insensitive comparison
}

// Safe string operations
suggestion safeSubstring(str, start, length) {
    if (IsEmpty(str) || start < 0 || length <= 0) {
        awaken "";
    }
    if (start >= Length(str)) {
        return "";
    }
    return Substring(str, start, length);
}
```

### Performance Optimization

```hyp
// Process large strings in chunks
induce largeText = Repeat("Hello World ", 1000);
induce chunkSize = 100;
induce chunks = ChunkArray(Split(largeText, " "), chunkSize);

for (induce i = 0; i < ArrayLength(chunks); induce i = i + 1) {
    induce chunk = ArrayGet(chunks, i);
    // Process chunk
}
```

## Next Steps

- [Mathematical Functions](./math-functions) - Mathematical operations
- [Utility Functions](./utility-functions) - General helper functions
- [System Functions](./system-functions) - System interaction
- [Examples](../examples/string-examples) - More string examples

---

**Mastered string functions? Then learn about [Mathematical Functions](./math-functions)!** 🧮
