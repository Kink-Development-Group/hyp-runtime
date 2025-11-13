---
sidebar_position: 3
---

# String-Funktionen

:::tip Vollständige Referenz
Siehe [Builtin-Funktionen Vollständige Referenz](./_complete-reference#string-builtins) für die vollständige, aktuelle Dokumentation aller String-Funktionen.
:::

HypnoScript bietet umfangreiche String-Funktionen für Textverarbeitung, -manipulation und -analyse.

## Grundlegende String-Operationen

### Length(str)

Gibt die Länge eines Strings zurück.

```hyp
induce text = "HypnoScript";
induce length = Length(text);
observe "Länge: " + length; // 11
```

### Substring(str, start, length)

Extrahiert einen Teilstring aus einem String.

```hyp
induce text = "HypnoScript";
induce part1 = Substring(text, 0, 5); // "Hypno"
induce part2 = Substring(text, 5, 6); // "Script"
```

### Concat(str1, str2, ...)

Verkettet mehrere Strings.

```hyp
induce firstName = "Max";
induce lastName = "Mustermann";
induce fullName = Concat(firstName, " ", lastName);
observe fullName; // "Max Mustermann"
```

## String-Manipulation

### ToUpper(str)

Konvertiert einen String zu Großbuchstaben.

```hyp
induce text = "HypnoScript";
induce upper = ToUpper(text);
observe upper; // "HYPNOSCRIPT"
```

### ToLower(str)

Konvertiert einen String zu Kleinbuchstaben.

```hyp
induce text = "HypnoScript";
induce lower = ToLower(text);
observe lower; // "hypnoscript"
```

### Capitalize(str)

Macht den ersten Buchstaben groß.

```hyp
induce text = "hypnoscript";
induce capitalized = Capitalize(text);
observe capitalized; // "Hypnoscript"
```

### TitleCase(str)

Macht jeden Wortanfang groß.

```hyp
induce text = "hypno script programming";
induce titleCase = TitleCase(text);
observe titleCase; // "Hypno Script Programming"
```

## String-Analyse

### IsEmpty(str)

Prüft, ob ein String leer ist.

```hyp
induce empty = "";
induce notEmpty = "Hallo";
induce isEmpty1 = IsEmpty(empty); // true
induce isEmpty2 = IsEmpty(notEmpty); // false
```

### IsWhitespace(str)

Prüft, ob ein String nur Leerzeichen enthält.

```hyp
induce whitespace = "   \t\n  ";
induce text = "Hallo Welt";
induce isWhitespace1 = IsWhitespace(whitespace); // true
induce isWhitespace2 = IsWhitespace(text); // false
```

### Contains(str, substring)

Prüft, ob ein String einen Teilstring enthält.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce hasScript = Contains(text, "Script"); // true
induce hasPython = Contains(text, "Python"); // false
```

### StartsWith(str, prefix)

Prüft, ob ein String mit einem Präfix beginnt.

```hyp
induce text = "HypnoScript";
induce startsWithHypno = StartsWith(text, "Hypno"); // true
induce startsWithScript = StartsWith(text, "Script"); // false
```

### EndsWith(str, suffix)

Prüft, ob ein String mit einem Suffix endet.

```hyp
induce text = "HypnoScript";
induce endsWithScript = EndsWith(text, "Script"); // true
induce endsWithHypno = EndsWith(text, "Hypno"); // false
```

## String-Suche

### IndexOf(str, substring)

Findet den ersten Index eines Teilstrings.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce index = IndexOf(text, "Script");
observe "Index von 'Script': " + index; // 5
```

### LastIndexOf(str, substring)

Findet den letzten Index eines Teilstrings.

```hyp
induce text = "HypnoScript Script Script";
induce lastIndex = LastIndexOf(text, "Script");
observe "Letzter Index von 'Script': " + lastIndex; // 18
```

### CountOccurrences(str, substring)

Zählt die Vorkommen eines Teilstrings.

```hyp
induce text = "HypnoScript Script Script";
induce count = CountOccurrences(text, "Script");
observe "Anzahl 'Script': " + count; // 3
```

## String-Transformation

### Reverse(str)

Kehrt einen String um.

```hyp
induce text = "HypnoScript";
induce reversed = Reverse(text);
observe reversed; // "tpircSonpyH"
```

### Trim(str)

Entfernt Leerzeichen am Anfang und Ende.

```hyp
induce text = "  HypnoScript  ";
induce trimmed = Trim(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### TrimStart(str)

Entfernt Leerzeichen am Anfang.

```hyp
induce text = "  HypnoScript";
induce trimmed = TrimStart(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### TrimEnd(str)

Entfernt Leerzeichen am Ende.

```hyp
induce text = "HypnoScript  ";
induce trimmed = TrimEnd(text);
observe "'" + trimmed + "'"; // "HypnoScript"
```

### Replace(str, oldValue, newValue)

Ersetzt alle Vorkommen eines Teilstrings.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce replaced = Replace(text, "Programmiersprache", "Sprache");
observe replaced; // "HypnoScript ist eine Sprache"
```

### ReplaceAll(str, oldValue, newValue)

Ersetzt alle Vorkommen (Alias für Replace).

```hyp
induce text = "Hallo Hallo Hallo";
induce replaced = ReplaceAll(text, "Hallo", "Hi");
observe replaced; // "Hi Hi Hi"
```

## String-Formatierung

### PadLeft(str, width, char)

Füllt einen String links mit Zeichen auf.

```hyp
induce text = "42";
induce padded = PadLeft(text, 5, "0");
observe padded; // "00042"
```

### PadRight(str, width, char)

Füllt einen String rechts mit Zeichen auf.

```hyp
induce text = "Hallo";
induce padded = PadRight(text, 10, "*");
observe padded; // "Hallo*****"
```

### FormatString(template, ...args)

Formatiert einen String mit Platzhaltern.

```hyp
induce name = "Max";
induce age = 30;
induce formatted = FormatString("Hallo {0}, du bist {1} Jahre alt", name, age);
observe formatted; // "Hallo Max, du bist 30 Jahre alt"
```

## String-Analyse (Erweitert)

### IsPalindrome(str)

Prüft, ob ein String ein Palindrom ist.

```hyp
induce palindrome1 = "anna";
induce palindrome2 = "racecar";
induce notPalindrome = "hello";
induce isPal1 = IsPalindrome(palindrome1); // true
induce isPal2 = IsPalindrome(palindrome2); // true
induce isPal3 = IsPalindrome(notPalindrome); // false
```

### IsNumeric(str)

Prüft, ob ein String eine Zahl darstellt.

```hyp
induce numeric1 = "123";
induce numeric2 = "3.14";
induce notNumeric = "abc";
induce isNum1 = IsNumeric(numeric1); // true
induce isNum2 = IsNumeric(numeric2); // true
induce isNum3 = IsNumeric(notNumeric); // false
```

### IsAlpha(str)

Prüft, ob ein String nur Buchstaben enthält.

```hyp
induce alpha = "HypnoScript";
induce notAlpha = "Hypno123";
induce isAlpha1 = IsAlpha(alpha); // true
induce isAlpha2 = IsAlpha(notAlpha); // false
```

### IsAlphaNumeric(str)

Prüft, ob ein String nur Buchstaben und Zahlen enthält.

```hyp
induce alphanumeric = "Hypno123";
induce notAlphanumeric = "Hypno@123";
induce isAlphaNum1 = IsAlphaNumeric(alphanumeric); // true
induce isAlphaNum2 = IsAlphaNumeric(notAlphanumeric); // false
```

## String-Zerlegung

### Split(str, delimiter)

Teilt einen String an einem Trennzeichen.

```hyp
induce text = "Apfel,Banane,Orange";
induce fruits = Split(text, ",");
observe fruits; // ["Apfel", "Banane", "Orange"]
```

### SplitLines(str)

Teilt einen String an Zeilenumbrüchen.

```hyp
induce text = "Zeile 1\nZeile 2\nZeile 3";
induce lines = SplitLines(text);
observe lines; // ["Zeile 1", "Zeile 2", "Zeile 3"]
```

### SplitWords(str)

Teilt einen String in Wörter.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce words = SplitWords(text);
observe words; // ["HypnoScript", "ist", "eine", "Programmiersprache"]
```

## String-Statistiken

### CountWords(str)

Zählt die Wörter in einem String.

```hyp
induce text = "HypnoScript ist eine Programmiersprache";
induce wordCount = CountWords(text);
observe "Wörter: " + wordCount; // 4
```

### CountCharacters(str)

Zählt die Zeichen in einem String.

```hyp
induce text = "Hallo Welt!";
induce charCount = CountCharacters(text);
observe "Zeichen: " + charCount; // 10
```

### CountLines(str)

Zählt die Zeilen in einem String.

```hyp
induce text = "Zeile 1\nZeile 2\nZeile 3";
induce lineCount = CountLines(text);
observe "Zeilen: " + lineCount; // 3
```

## String-Vergleiche

### Compare(str1, str2)

Vergleicht zwei Strings lexikographisch.

```hyp
induce str1 = "Apfel";
induce str2 = "Banane";
induce comparison = Compare(str1, str2);
observe comparison; // -1 (str1 < str2)
```

### EqualsIgnoreCase(str1, str2)

Vergleicht zwei Strings ohne Berücksichtigung der Groß-/Kleinschreibung.

```hyp
induce str1 = "HypnoScript";
induce str2 = "hypnoscript";
induce equals = EqualsIgnoreCase(str1, str2); // true
```

## String-Generierung

### Repeat(str, count)

Wiederholt einen String.

```hyp
induce text = "Ha";
induce repeated = Repeat(text, 3);
observe repeated; // "HaHaHa"
```

### GenerateRandomString(length)

Generiert einen zufälligen String.

```hyp
induce random = GenerateRandomString(10);
observe random; // Zufälliger 10-Zeichen-String
```

### GenerateUUID()

Generiert eine UUID.

```hyp
induce uuid = GenerateUUID();
observe uuid; // "123e4567-e89b-12d3-a456-426614174000"
```

## Praktische Beispiele

### Text-Analyse

```hyp
Focus {
    entrance {
        induce text = "HypnoScript ist eine innovative Programmiersprache mit hypnotischer Syntax.";

        observe "Original: " + text;
        observe "Länge: " + Length(text);
        observe "Wörter: " + CountWords(text);
        observe "Zeichen: " + CountCharacters(text);

        induce upperText = ToUpper(text);
        observe "Großbuchstaben: " + upperText;

        induce titleText = TitleCase(text);
        observe "Title Case: " + titleText;

        induce words = SplitWords(text);
        observe "Wörter-Array: " + words;

        induce hasHypno = Contains(text, "Hypno");
        observe "Enthält 'Hypno': " + hasHypno;
    }
} Relax;
```

### E-Mail-Validierung

```hyp
Focus {
    Trance validateEmail(email) {
        if (IsEmpty(email)) {
            return false;
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
            observe email + " ist gültig: " + isValid;
        }
    }
} Relax;
```

### Text-Formatierung

```hyp
Focus {
    entrance {
        induce name = "max mustermann";
        induce age = 30;
        induce city = "berlin";

        // Namen formatieren
        induce formattedName = TitleCase(name);
        observe "Name: " + formattedName; // "Max Mustermann"

        // Adresse formatieren
        induce address = Concat(formattedName, ", ", ToNumber(age), " Jahre, ", TitleCase(city));
        observe "Adresse: " + address;

        // Telefonnummer formatieren
        induce phone = "1234567890";
        induce formattedPhone = FormatString("({0}) {1}-{2}",
            Substring(phone, 0, 3),
            Substring(phone, 3, 3),
            Substring(phone, 6, 4));
        observe "Telefon: " + formattedPhone; // "(123) 456-7890"
    }
} Relax;
```

## Best Practices

### Effiziente String-Operationen

```hyp
// Strings zusammenbauen
induce parts = ["Hallo", "Welt", "!"];
induce result = Concat(ArrayGet(parts, 0), " ", ArrayGet(parts, 1), ArrayGet(parts, 2));

// String-Vergleiche
if (EqualsIgnoreCase(input, "ja")) {
    // Case-insensitive Vergleich
}

// Sichere String-Operationen
Trance safeSubstring(str, start, length) {
    if (IsEmpty(str) || start < 0 || length <= 0) {
        return "";
    }
    if (start >= Length(str)) {
        return "";
    }
    return Substring(str, start, length);
}
```

### Performance-Optimierung

```hyp
// Große Strings in Chunks verarbeiten
induce largeText = Repeat("Hallo Welt ", 1000);
induce chunkSize = 100;
induce chunks = ChunkArray(Split(largeText, " "), chunkSize);

for (induce i = 0; i < ArrayLength(chunks); induce i = i + 1) {
    induce chunk = ArrayGet(chunks, i);
    // Chunk verarbeiten
}
```

## Nächste Schritte

- [Mathematische Funktionen](./math-functions) - Mathematische Operationen
- [Utility-Funktionen](./utility-functions) - Allgemeine Hilfsfunktionen
- [System-Funktionen](./system-functions) - System-Interaktion
- [Beispiele](../examples/string-examples) - Weitere String-Beispiele

---

**Beherrschst du String-Funktionen? Dann lerne [Mathematische Funktionen](./math-functions) kennen!** 🧮
