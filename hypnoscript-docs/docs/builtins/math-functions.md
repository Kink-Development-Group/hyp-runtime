---
sidebar_position: 4
---

# Mathematische Functionen

HypnoScript bietet mathematische Functionen für Berechnungen, Trigonometrie und Zahlentheorie.

## Available Functions

Die folgenden Functionen sind in der `MathBuiltins`-Bibliothek available:

### Trigonometrische Functionen

#### sin(x: number): number

Calculates den Sinus (x in Radiant).

```hyp
Focus {
    induce result: number = sin(0);  // 0
    observe "sin(0) = " + result;
} Relax
```

#### cos(x: number): number

Calculates den Kosinus (x in Radiant).

```hyp
Focus {
    induce result: number = cos(0);  // 1
    observe "cos(0) = " + result;
} Relax
```

#### tan(x: number): number

Calculates den Tangens (x in Radiant).

```hyp
Focus {
    induce result: number = tan(0);  // 0
    observe "tan(0) = " + result;
} Relax
```

### Wurzel- und Potenzfunktionen

#### sqrt(x: number): number

Calculates die Quadratwurzel.

```hyp
Focus {
    induce result: number = sqrt(16);  // 4
    observe "sqrt(16) = " + result;
} Relax
```

#### pow(base: number, exponent: number): number

Calculates eine Potenz.

```hyp
Focus {
    induce result: number = pow(2, 3);  // 8
    observe "2^3 = " + result;
} Relax
```

### Logarithmen

#### log(x: number): number

Calculates den natürlichen Logarithmus (ln).

```hyp
Focus {
    induce result: number = log(2.718281828);  // ~1
    observe "ln(e) = " + result;
} Relax
```

#### log10(x: number): number

Calculates den Logarithmus zur Basis 10.

```hyp
Focus {
    induce result: number = log10(100);  // 2
    observe "log10(100) = " + result;
} Relax
```

### Rundungsfunktionen

#### abs(x: number): number

Returns den absoluten Wert .

```hyp
Focus {
    induce result: number = abs(-5);  // 5
    observe "abs(-5) = " + result;
} Relax
```

#### floor(x: number): number

Rundet ab.

```hyp
Focus {
    induce result: number = floor(3.7);  // 3
    observe "floor(3.7) = " + result;
} Relax
```

#### ceil(x: number): number

Rundet auf.

```hyp
Focus {
    induce result: number = ceil(3.2);  // 4
    observe "ceil(3.2) = " + result;
} Relax
```

#### round(x: number): number

Rundet zur nächsten ganzen Zahl.

```hyp
Focus {
    induce result: number = round(3.5);  // 4
    observe "round(3.5) = " + result;
} Relax
```

### Min/Max

#### min(a: number, b: number): number

Returns den kleineren Wert .

```hyp
Focus {
    induce result: number = min(5, 3);  // 3
    observe "min(5, 3) = " + result;
} Relax
```

#### max(a: number, b: number): number

Returns den größeren Wert .

```hyp
Focus {
    induce result: number = max(5, 3);  // 5
    observe "max(5, 3) = " + result;
} Relax
```

### Advanced Functionen

#### factorial(n: number): number

Calculates die Fakultät.

```hyp
Focus {
    induce result: number = factorial(5);  // 120
    observe "5! = " + result;
} Relax
```

#### gcd(a: number, b: number): number

Calculates den größten gemeinsamen Teiler.

```hyp
Focus {
    induce result: number = gcd(48, 18);  // 6
    observe "gcd(48, 18) = " + result;
} Relax
```

#### lcm(a: number, b: number): number

Calculates das kleinste gemeinsame Vielfache.

```hyp
Focus {
    induce result: number = lcm(12, 18);  // 36
    observe "lcm(12, 18) = " + result;
} Relax
```

#### is_prime(n: number): boolean

Checks, ob eine Zahl eine Primzahl ist.

```hyp
Focus {
    induce result: boolean = is_prime(7);  // true
    observe "7 ist Primzahl: " + result;
} Relax
```

#### fibonacci(n: number): number

Calculates die n-te Fibonacci-Zahl.

```hyp
Focus {
    induce result: number = fibonacci(10);  // 55
    observe "fibonacci(10) = " + result;
} Relax
```

#### clamp(value: number, min: number, max: number): number

Begrenzt einen Wert auf einen Bereich.

```hyp
Focus {
    induce result: number = clamp(15, 0, 10);  // 10
    observe "clamp(15, 0, 10) = " + result;
} Relax
```

## Vollständiges Example

```hyp
Focus {
    entrance {
        observe "=== Mathematische Funktionen Demo ===";

        // Trigonometrie
        induce angle: number = 0;
        observe "sin(0) = " + sin(angle);
        observe "cos(0) = " + cos(angle);

        // Wurzeln und Potenzen
        observe "sqrt(16) = " + sqrt(16);
        observe "pow(2, 10) = " + pow(2, 10);

        // Rundung
        induce pi: number = 3.14159;
        observe "floor(pi) = " + floor(pi);
        observe "ceil(pi) = " + ceil(pi);
        observe "round(pi) = " + round(pi);

        // Min/Max
        observe "min(5, 10) = " + min(5, 10);
        observe "max(5, 10) = " + max(5, 10);

        // Erweiterte Funktionen
        observe "factorial(5) = " + factorial(5);
        observe "gcd(48, 18) = " + gcd(48, 18);
        observe "fibonacci(10) = " + fibonacci(10);
        observe "is_prime(13): " + is_prime(13);
    }
} Relax
```

## Hinweise

- Alle Winkelfunktionen (sin, cos, tan) erwarten Radiant als Input
- Die Functionen sind direkt available und müssen nicht importiert werden
- Typ-Konvertierungen erfolgen automatisch zwischen ganzen Zahlen und Fließkommazahlen
  induce atan2_2 = Atan2(1, -1); // 3 _ PI / 4
  induce atan2_3 = Atan2(-1, -1); // -3 _ PI / 4

````

### DegreesToRadians(degrees)

Konvertiert Grad in Radiant.

```hyp
induce rad1 = DegreesToRadians(0); // 0
induce rad2 = DegreesToRadians(90); // PI / 2
induce rad3 = DegreesToRadians(180); // PI
````

### RadiansToDegrees(radians)

Konvertiert Radiant in Grad.

```hyp
induce deg1 = RadiansToDegrees(0); // 0
induce deg2 = RadiansToDegrees(PI / 2); // 90
induce deg3 = RadiansToDegrees(PI); // 180
```

## Logarithmen

### Log(x)

Berechnet den natürlichen Logarithmus.

```hyp
induce log1 = Log(1); // 0
induce log2 = Log(E); // 1
induce log3 = Log(10); // 2.302585092994046
```

### Log10(x)

Berechnet den Logarithmus zur Basis 10.

```hyp
induce log10_1 = Log10(1); // 0
induce log10_2 = Log10(10); // 1
induce log10_3 = Log10(100); // 2
```

### Log2(x)

Berechnet den Logarithmus zur Basis 2.

```hyp
induce log2_1 = Log2(1); // 0
induce log2_2 = Log2(2); // 1
induce log2_3 = Log2(8); // 3
```

### LogBase(x, base)

Berechnet den Logarithmus zur angegebenen Basis.

```hyp
induce logBase1 = LogBase(8, 2); // 3
induce logBase2 = LogBase(100, 10); // 2
induce logBase3 = LogBase(27, 3); // 3
```

## Exponentialfunktionen

### Exp(x)

Berechnet e^x.

```hyp
induce exp1 = Exp(0); // 1
induce exp2 = Exp(1); // E
induce exp3 = Exp(2); // E^2
```

### Exp2(x)

Berechnet 2^x.

```hyp
induce exp2_1 = Exp2(0); // 1
induce exp2_2 = Exp2(1); // 2
induce exp2_3 = Exp2(3); // 8
```

### Exp10(x)

Berechnet 10^x.

```hyp
induce exp10_1 = Exp10(0); // 1
induce exp10_2 = Exp10(1); // 10
induce exp10_3 = Exp10(2); // 100
```

## Hyperbolische Funktionen

### Sinh(x)

Berechnet den hyperbolischen Sinus.

```hyp
induce sinh1 = Sinh(0); // 0
induce sinh2 = Sinh(1); // 1.1752011936438014
```

### Cosh(x)

Berechnet den hyperbolischen Kosinus.

```hyp
induce cosh1 = Cosh(0); // 1
induce cosh2 = Cosh(1); // 1.5430806348152437
```

### Tanh(x)

Berechnet den hyperbolischen Tangens.

```hyp
induce tanh1 = Tanh(0); // 0
induce tanh2 = Tanh(1); // 0.7615941559557649
```

## Ganzzahl-Operationen

### Mod(dividend, divisor)

Berechnet den Modulo (Rest der Division).

```hyp
induce mod1 = Mod(7, 3); // 1
induce mod2 = Mod(10, 5); // 0
induce mod3 = Mod(-7, 3); // -1
```

### Div(dividend, divisor)

Berechnet die ganzzahlige Division.

```hyp
induce div1 = Div(7, 3); // 2
induce div2 = Div(10, 5); // 2
induce div3 = Div(15, 4); // 3
```

### GCD(a, b)

Berechnet den größten gemeinsamen Teiler.

```hyp
induce gcd1 = GCD(12, 18); // 6
induce gcd2 = GCD(7, 13); // 1
induce gcd3 = GCD(0, 5); // 5
```

### LCM(a, b)

Berechnet das kleinste gemeinsame Vielfache.

```hyp
induce lcm1 = LCM(12, 18); // 36
induce lcm2 = LCM(7, 13); // 91
induce lcm3 = LCM(4, 6); // 12
```

### IsPrime(n)

Prüft, ob eine Zahl prim ist.

```hyp
induce isPrime1 = IsPrime(2); // true
induce isPrime2 = IsPrime(17); // true
induce isPrime3 = IsPrime(4); // false
```

### NextPrime(n)

Findet die nächste Primzahl.

```hyp
induce nextPrime1 = NextPrime(10); // 11
induce nextPrime2 = NextPrime(17); // 19
induce nextPrime3 = NextPrime(1); // 2
```

### PrimeFactors(n)

Zerlegt eine Zahl in Primfaktoren.

```hyp
induce factors1 = PrimeFactors(12); // [2, 2, 3]
induce factors2 = PrimeFactors(17); // [17]
induce factors3 = PrimeFactors(100); // [2, 2, 5, 5]
```

## Statistik

### Sum(array)

Berechnet die Summe eines Arrays.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce sum = Sum(numbers); // 15
```

### Average(array)

Berechnet den Durchschnitt eines Arrays.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce avg = Average(numbers); // 3
```

### Median(array)

Berechnet den Median eines Arrays.

```hyp
induce numbers1 = [1, 2, 3, 4, 5];
induce median1 = Median(numbers1); // 3

induce numbers2 = [1, 2, 3, 4];
induce median2 = Median(numbers2); // 2.5
```

### Mode(array)

Berechnet den Modus eines Arrays.

```hyp
induce numbers = [1, 2, 2, 3, 4, 2, 5];
induce mode = Mode(numbers); // 2
```

### Variance(array)

Berechnet die Varianz eines Arrays.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce variance = Variance(numbers); // 2.5
```

### StandardDeviation(array)

Berechnet die Standardabweichung eines Arrays.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce stdDev = StandardDeviation(numbers); // 1.5811388300841898
```

### Min(array)

Findet das Minimum in einem Array.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce min = Min(numbers); // 1
```

### Max(array)

Findet das Maximum in einem Array.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce max = Max(numbers); // 9
```

### Range(array)

Berechnet die Spannweite eines Arrays.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce range = Range(numbers); // 8
```

## Zufallszahlen

### Random()

Generiert eine Zufallszahl zwischen 0 und 1.

```hyp
induce random1 = Random(); // 0.123456789
induce random2 = Random(); // 0.987654321
```

### RandomRange(min, max)

Generiert eine Zufallszahl in einem Bereich.

```hyp
induce random1 = RandomRange(1, 10); // Zufällige Ganzzahl zwischen 1 und 10
induce random2 = RandomRange(0.0, 1.0); // Zufällige Dezimalzahl zwischen 0 und 1
```

### RandomInt(min, max)

Generiert eine zufällige Ganzzahl.

```hyp
induce random1 = RandomInt(1, 10); // Zufällige Ganzzahl zwischen 1 und 10
induce random2 = RandomInt(-100, 100); // Zufällige Ganzzahl zwischen -100 und 100
```

### RandomChoice(array)

Wählt ein zufälliges Element aus einem Array.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce randomFruit = RandomChoice(fruits); // Zufälliges Obst
```

### RandomSample(array, count)

Wählt zufällige Elemente aus einem Array.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
induce sample = RandomSample(numbers, 3); // 3 zufällige Zahlen
```

## Mathematische Konstanten

### PI

Die Kreiszahl π.

```hyp
induce pi = PI; // 3.141592653589793
```

### E

Die Eulersche Zahl e.

```hyp
induce e = E; // 2.718281828459045
```

### PHI

Der Goldene Schnitt φ.

```hyp
induce phi = PHI; // 1.618033988749895
```

### SQRT2

Die Quadratwurzel von 2.

```hyp
induce sqrt2 = SQRT2; // 1.4142135623730951
```

### SQRT3

Die Quadratwurzel von 3.

```hyp
induce sqrt3 = SQRT3; // 1.7320508075688772
```

## Praktische Beispiele

### Geometrische Berechnungen

```hyp
Focus {
    entrance {
        // Kreis-Berechnungen
        induce radius = 5;
        induce area = PI * Pow(radius, 2);
        induce circumference = 2 * PI * radius;

        observe "Kreis mit Radius " + radius + ":";
        observe "Fläche: " + Round(area, 2);
        observe "Umfang: " + Round(circumference, 2);

        // Dreieck-Berechnungen
        induce a = 3;
        induce b = 4;
        induce c = Sqrt(Pow(a, 2) + Pow(b, 2)); // Pythagoras

        observe "Rechtwinkliges Dreieck:";
        observe "Seite a: " + a;
        observe "Seite b: " + b;
        observe "Hypotenuse c: " + Round(c, 2);

        // Volumen einer Kugel
        induce sphereRadius = 3;
        induce volume = (4.0 / 3.0) * PI * Pow(sphereRadius, 3);
        observe "Kugel-Volumen: " + Round(volume, 2);
    }
} Relax;
```

### Statistische Analyse

```hyp
Focus {
    entrance {
        induce scores = [85, 92, 78, 96, 88, 91, 87, 94, 82, 89];

        observe "Prüfungsergebnisse: " + scores;
        observe "Anzahl: " + ArrayLength(scores);
        observe "Durchschnitt: " + Round(Average(scores), 2);
        observe "Median: " + Median(scores);
        observe "Minimum: " + Min(scores);
        observe "Maximum: " + Max(scores);
        observe "Spannweite: " + Range(scores);
        observe "Standardabweichung: " + Round(StandardDeviation(scores), 2);
        observe "Varianz: " + Round(Variance(scores), 2);

        // Notenverteilung
        induce excellent = 0;
        induce good = 0;
        induce average = 0;
        induce poor = 0;

        for (induce i = 0; i < ArrayLength(scores); induce i = i + 1) {
            induce score = ArrayGet(scores, i);
            if (score >= 90) {
                induce excellent = excellent + 1;
            } else if (score >= 80) {
                induce good = good + 1;
            } else if (score >= 70) {
                induce average = average + 1;
            } else {
                induce poor = poor + 1;
            }
        }

        observe "Notenverteilung:";
        observe "Ausgezeichnet (90+): " + excellent;
        observe "Gut (80-89): " + good;
        observe "Durchschnittlich (70-79): " + average;
        observe "Schwach (<70): " + poor;
    }
} Relax;
```

### Finanzmathematik

```hyp
Focus {
    suggestion calculateCompoundInterest(principal, rate, time, compounds) {
        awaken principal * Pow(1 + rate / compounds, compounds * time);
    }

    suggestion calculateLoanPayment(principal, rate, years) {
        induce monthlyRate = rate / 12 / 100;
        induce numberOfPayments = years * 12;
        awaken principal * (monthlyRate * Pow(1 + monthlyRate, numberOfPayments)) /
               (Pow(1 + monthlyRate, numberOfPayments) - 1);
    }

    entrance {
        // Zinseszins
        induce principal = 10000;
        induce rate = 5; // 5% pro Jahr
        induce time = 10; // 10 Jahre
        induce compounds = 12; // Monatlich

        induce finalAmount = calculateCompoundInterest(principal, rate / 100, time, compounds);
        observe "Zinseszins-Berechnung:";
        observe "Anfangskapital: €" + principal;
        observe "Zinssatz: " + rate + "%";
        observe "Laufzeit: " + time + " Jahre";
        observe "Endkapital: €" + Round(finalAmount, 2);
        observe "Gewinn: €" + Round(finalAmount - principal, 2);

        // Kreditberechnung
        induce loanAmount = 200000;
        induce loanRate = 3.5; // 3.5% pro Jahr
        induce loanYears = 30;

        induce monthlyPayment = calculateLoanPayment(loanAmount, loanRate, loanYears);
        induce totalPayment = monthlyPayment * loanYears * 12;
        induce totalInterest = totalPayment - loanAmount;

        observe "Kreditberechnung:";
        observe "Kreditsumme: €" + loanAmount;
        observe "Zinssatz: " + loanRate + "%";
        observe "Laufzeit: " + loanYears + " Jahre";
        observe "Monatliche Rate: €" + Round(monthlyPayment, 2);
        observe "Gesamtzinsen: €" + Round(totalInterest, 2);
        observe "Gesamtrückzahlung: €" + Round(totalPayment, 2);
    }
} Relax;
```

### Wissenschaftliche Berechnungen

```hyp
Focus {
    entrance {
        // Physikalische Berechnungen
        induce mass = 10; // kg
        induce velocity = 20; // m/s
        induce kineticEnergy = 0.5 * mass * Pow(velocity, 2);

        observe "Kinetische Energie:";
        observe "Masse: " + mass + " kg";
        observe "Geschwindigkeit: " + velocity + " m/s";
        observe "Energie: " + Round(kineticEnergy, 2) + " J";

        // Chemische Berechnungen
        induce temperature = 25; // Celsius
        induce kelvin = temperature + 273.15;
        observe "Temperaturumrechnung:";
        observe "Celsius: " + temperature + "°C";
        observe "Kelvin: " + Round(kelvin, 2) + " K";

        // Trigonometrische Anwendungen
        induce angle = 30; // Grad
        induce radians = DegreesToRadians(angle);
        induce sinValue = Sin(radians);
        induce cosValue = Cos(radians);
        induce tanValue = Tan(radians);

        observe "Trigonometrie (" + angle + "°):";
        observe "Sinus: " + Round(sinValue, 4);
        observe "Kosinus: " + Round(cosValue, 4);
        observe "Tangens: " + Round(tanValue, 4);

        // Logarithmische Skalen
        induce ph = 7; // pH-Wert
        induce hConcentration = Pow(10, -ph);
        observe "pH-Berechnung:";
        observe "pH-Wert: " + ph;
        observe "H+-Konzentration: " + hConcentration + " mol/L";
    }
} Relax;
```

## Best Practices

### Numerische Genauigkeit

```hyp
// Vermeide Gleitkomma-Vergleiche
if (Abs(a - b) < 0.0001) {
    // a und b sind praktisch gleich
}

// Verwende Round für Outputn
observe "Ergebnis: " + Round(result, 4);

// Große Zahlen
induce largeNumber = 123456789;
induce formatted = FormatString("{0:N0}", largeNumber);
observe "Zahl: " + formatted; // 123,456,789
```

### Performance-Optimierung

```hyp
// Caching von Konstanten
induce PI_OVER_180 = PI / 180;

suggestion degreesToRadians(degrees) {
    awaken degrees * PI_OVER_180;
}

// Vermeide wiederholte Berechnungen
suggestion calculateDistance(x1, y1, x2, y2) {
    induce dx = x2 - x1;
    induce dy = y2 - y1;
    awaken Sqrt(dx * dx + dy * dy);
}
```

### Fehlerbehandlung

```hyp
suggestion safeDivision(numerator, denominator) {
    if (denominator == 0) {
        observe "Fehler: Division durch Null!";
        awaken 0;
    }
    return numerator / denominator;
}

suggestion safeLog(x) {
    if (x <= 0) {
        observe "Fehler: Logarithmus nur für positive Zahlen!";
        awaken 0;
    }
    return Log(x);
}
```

## Nächste Schritte

- [Utility-Funktionen](./utility-functions) - Allgemeine Hilfsfunktionen
- [System-Funktionen](./system-functions) - System-Interaktion
- [Statistik-Funktionen](./statistics-functions) - Erweiterte Statistik
- [Beispiele](../examples/math-examples) - Weitere mathematische Beispiele

---

**Beherrschst du mathematische Funktionen? Dann lerne [Utility-Funktionen](./utility-functions) kennen!** 🔧
