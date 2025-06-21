# HypnoScript - Eine esoterische, TypeScript-inspirierte Sprache mit hypnotischem Flair

**HypnoScript** ist eine vollständig implementierte, esoterische Programmiersprache, die sich an TypeScript/JavaScript anlehnt und dabei alle Klischees rund um Hypnose, Trance und hypnotische Induktion verwendet. Trotz des humorvollen Charakters ist sie Turing-vollständig und unterstützt moderne Sprachfeatures.

## 🎯 Features

### ✅ Vollständig implementiert:

- **Grundlegende Syntax**: `Focus { ... } Relax` Programmstruktur
- **Variablen**: `induce x: number = 5;` mit Typisierung
- **Externe Eingabe**: `induce userInput: string from external;`
- **Kontrollstrukturen**: `if`, `while`, `loop` (for-Schleife)
- **Funktionen**: `suggestion`, `imperative suggestion`, `dominant suggestion`
- **Objektorientierung**: `session` (Klassen) mit Konstruktoren und Methoden
- **Strukturen**: `tranceify` (Records/Structs)
- **Arrays**: `[1, 2, 3]` und Array-Zugriffe `array[index]`
- **Hypnotische Operatoren**: Synonyme für Standardoperatoren
- **Builtin-Funktionen**: Mathematische, String- und hypnotische Funktionen
- **Ein-/Ausgabe**: `observe`, `drift(ms)`
- **Module**: `mindLink` (Import)
- **Globale Variablen**: `sharedTrance`
- **Labels und Goto**: `label:`, `sinkTo label`

### 🧠 Hypnotische Sprachfeatures:

- **Operator-Synonyme**:

  - `youAreFeelingVerySleepy` = `==`
  - `notSoDeep` = `!=`
  - `lookAtTheWatch` = `>`
  - `fallUnderMySpell` = `<`
  - `deeplyGreater` = `>=`
  - `deeplyLess` = `<=`

- **Hypnotische Builtins**:
  - `DeepTrance(duration)`
  - `HypnoticCountdown(from)`
  - `TranceInduction(subjectName)`

## 🏗️ Architektur

Die Implementierung besteht aus mehreren .NET-Projekten:

- **HypnoScript.Core**: Grundlegende Typen und Symbol-Tabellen
- **HypnoScript.LexerParser**: Lexer, Parser und AST
- **HypnoScript.Compiler**: TypeChecker, Interpreter und WASM-Codegenerator
- **HypnoScript.Runtime**: Builtin-Funktionen
- **HypnoScript.CLI**: Kommandozeilen-Interface

## 🚀 Verwendung

### Kompilierung:

```bash
dotnet build
```

### Ausführung:

```bash
dotnet run --project HypnoScript.CLI -- run test_comprehensive.hyp
```

### Debug-Modus:

```bash
dotnet run --project HypnoScript.CLI -- run test_comprehensive.hyp --debug
```

## 📝 Beispiele

### Grundlegendes Programm:

```hypnoscript
Focus {
    entrance {
        observe "Willkommen in HypnoScript!";
    }

    induce greeting: string = "Hello Trance!";
    observe greeting;

    if (true) deepFocus {
        observe "You are feeling very relaxed...";
    }
} Relax
```

### Funktionen und Arrays:

```hypnoscript
Focus {
    suggestion add(a: number, b: number): number {
        awaken a + b;
    }

    induce numbers = [1, 2, 3, 4, 5];
    induce sum = call add(numbers[0], numbers[1]);
    observe "Sum: " + sum;
} Relax
```

### Objektorientierung:

```hypnoscript
Focus {
    session Person {
        expose name: string;

        suggestion constructor(personName: string) {
            this.name = personName;
        }

        suggestion greet() {
            observe "Hello, " + this.name;
        }
    }

    induce person = Person("Alice");
    person.greet();
} Relax
```

### Hypnotische Operatoren:

```hypnoscript
Focus {
    induce x: number = 10;
    induce y: number = 5;

    if (x lookAtTheWatch y) deepFocus {
        observe "10 is greater than 5";
    }

    if (y fallUnderMySpell x) deepFocus {
        observe "5 is less than 10";
    }
} Relax
```

## 🔧 Builtin-Funktionen

### Mathematische Funktionen:

- `Sin(x)`, `Cos(x)`, `Tan(x)`
- `Sqrt(x)`, `Pow(x, y)`
- `Abs(x)`, `Floor(x)`, `Ceiling(x)`, `Round(x)`

### String-Funktionen:

- `Length(str)`, `ToUpper(str)`, `ToLower(str)`
- `Substring(str, start, length)`
- `Contains(str, substring)`, `Replace(str, old, new)`

### Konvertierungsfunktionen:

- `ToInt(value)`, `ToDouble(value)`, `ToString(value)`

### Hypnotische Spezialfunktionen:

- `DeepTrance(duration)`
- `HypnoticCountdown(from)`
- `TranceInduction(subjectName)`

## 🎨 Sprachdesign

HypnoScript kombiniert:

- **TypeScript-ähnliche Syntax** für Vertrautheit
- **Hypnotische Terminologie** für den esoterischen Charme
- **Moderne Sprachfeatures** für praktische Nutzbarkeit
- **Turing-Vollständigkeit** für universelle Berechnungsfähigkeit

## 📊 Status

- ✅ **Lexer**: Vollständig implementiert
- ✅ **Parser**: Vollständig implementiert
- ✅ **AST**: Vollständig implementiert
- ✅ **TypeChecker**: Grundlegend implementiert
- ✅ **Interpreter**: Vollständig implementiert
- ✅ **Builtins**: Umfassend implementiert
- ✅ **CLI**: Funktional
- 🔄 **WASM-Codegenerator**: Grundstruktur vorhanden
- 🔄 **Optimierungen**: Geplant

## 🎯 Nächste Schritte

1. **WASM-Codegenerator vervollständigen**
2. **Performance-Optimierungen**
3. **Standardbibliothek erweitern**
4. **IDE-Integration**
5. **Package Manager**

---

**HypnoScript** - Where programming meets hypnosis! 🧠✨
