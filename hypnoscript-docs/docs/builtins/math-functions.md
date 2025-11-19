---
sidebar_position: 4
---

# Mathematical Functions

HypnoScript provides mathematical functions for calculations, trigonometry, and number theory.

## Available Functions

The following functions are available in the `MathBuiltins` library:

### Trigonometric Functions

#### sin(x: number): number

Calculates the sine (x in radians).

```hyp
Focus {
    induce result: number = sin(0);  // 0
    observe "sin(0) = " + result;
} Relax
```

#### cos(x: number): number

Calculates the cosine (x in radians).

```hyp
Focus {
    induce result: number = cos(0);  // 1
    observe "cos(0) = " + result;
} Relax
```

#### tan(x: number): number

Calculates the tangent (x in radians).

```hyp
Focus {
    induce result: number = tan(0);  // 0
    observe "tan(0) = " + result;
} Relax
```

### Root and Power Functions

#### sqrt(x: number): number

Calculates the square root.

```hyp
Focus {
    induce result: number = sqrt(16);  // 4
    observe "sqrt(16) = " + result;
} Relax
```

#### pow(base: number, exponent: number): number

Calculates a power.

```hyp
Focus {
    induce result: number = pow(2, 3);  // 8
    observe "2^3 = " + result;
} Relax
```

### Logarithms

#### log(x: number): number

Calculates the natural logarithm (ln).

```hyp
Focus {
    induce result: number = log(2.718281828);  // ~1
    observe "ln(e) = " + result;
} Relax
```

#### log10(x: number): number

Calculates the base-10 logarithm.

```hyp
Focus {
    induce result: number = log10(100);  // 2
    observe "log10(100) = " + result;
} Relax
```

### Rounding Functions

#### abs(x: number): number

Returns the absolute value.

```hyp
Focus {
    induce result: number = abs(-5);  // 5
    observe "abs(-5) = " + result;
} Relax
```

#### floor(x: number): number

Rounds down.

```hyp
Focus {
    induce result: number = floor(3.7);  // 3
    observe "floor(3.7) = " + result;
} Relax
```

#### ceil(x: number): number

Rounds up.

```hyp
Focus {
    induce result: number = ceil(3.2);  // 4
    observe "ceil(3.2) = " + result;
} Relax
```

#### round(x: number): number

Rounds to the nearest integer.

```hyp
Focus {
    induce result: number = round(3.5);  // 4
    observe "round(3.5) = " + result;
} Relax
```

### Min/Max

#### min(a: number, b: number): number

Returns the smaller value.

```hyp
Focus {
    induce result: number = min(5, 3);  // 3
    observe "min(5, 3) = " + result;
} Relax
```

#### max(a: number, b: number): number

Returns the larger value.

```hyp
Focus {
    induce result: number = max(5, 3);  // 5
    observe "max(5, 3) = " + result;
} Relax
```

### Advanced Functions

#### factorial(n: number): number

Calculates the factorial.

```hyp
Focus {
    induce result: number = factorial(5);  // 120
    observe "5! = " + result;
} Relax
```

#### gcd(a: number, b: number): number

Calculates the greatest common divisor.

```hyp
Focus {
    induce result: number = gcd(48, 18);  // 6
    observe "gcd(48, 18) = " + result;
} Relax
```

#### lcm(a: number, b: number): number

Calculates the least common multiple.

```hyp
Focus {
    induce result: number = lcm(12, 18);  // 36
    observe "lcm(12, 18) = " + result;
} Relax
```

#### is_prime(n: number): boolean

Checks if a number is prime.

```hyp
Focus {
    induce result: boolean = is_prime(7);  // true
    observe "7 ist Primzahl: " + result;
} Relax
```

#### fibonacci(n: number): number

Calculates the n-th Fibonacci number.

```hyp
Focus {
    induce result: number = fibonacci(10);  // 55
    observe "fibonacci(10) = " + result;
} Relax
```

#### clamp(value: number, min: number, max: number): number

Clamps a value to a range.

```hyp
Focus {
    induce result: number = clamp(15, 0, 10);  // 10
    observe "clamp(15, 0, 10) = " + result;
} Relax
```

## Complete Example

```hyp
Focus {
    entrance {
        observe "=== Mathematical Functions Demo ===";

        // Trigonometry
        induce angle: number = 0;
        observe "sin(0) = " + sin(angle);
        observe "cos(0) = " + cos(angle);

        // Roots and powers
        observe "sqrt(16) = " + sqrt(16);
        observe "pow(2, 10) = " + pow(2, 10);

        // Rounding
        induce pi: number = 3.14159;
        observe "floor(pi) = " + floor(pi);
        observe "ceil(pi) = " + ceil(pi);
        observe "round(pi) = " + round(pi);

        // Min/Max
        observe "min(5, 10) = " + min(5, 10);
        observe "max(5, 10) = " + max(5, 10);

        // Advanced functions
        observe "factorial(5) = " + factorial(5);
        observe "gcd(48, 18) = " + gcd(48, 18);
        observe "fibonacci(10) = " + fibonacci(10);
        observe "is_prime(13): " + is_prime(13);
    }
} Relax
```

## Notes

- All angle functions (sin, cos, tan) expect radians as input
- The functions are directly available and don't need to be imported
- Type conversions happen automatically between integers and floating-point numbers
  induce atan2*2 = Atan2(1, -1); // 3 * PI / 4
  induce atan2*3 = Atan2(-1, -1); // -3 * PI / 4

````

### DegreesToRadians(degrees)

Converts degrees to radians.

```hyp
induce rad1 = DegreesToRadians(0); // 0
induce rad2 = DegreesToRadians(90); // PI / 2
induce rad3 = DegreesToRadians(180); // PI
````

### RadiansToDegrees(radians)

Converts radians to degrees.

```hyp
induce deg1 = RadiansToDegrees(0); // 0
induce deg2 = RadiansToDegrees(PI / 2); // 90
induce deg3 = RadiansToDegrees(PI); // 180
```

## Logarithms

### Log(x)

Calculates the natural logarithm.

```hyp
induce log1 = Log(1); // 0
induce log2 = Log(E); // 1
induce log3 = Log(10); // 2.302585092994046
```

### Log10(x)

Calculates the base-10 logarithm.

```hyp
induce log10_1 = Log10(1); // 0
induce log10_2 = Log10(10); // 1
induce log10_3 = Log10(100); // 2
```

### Log2(x)

Calculates the base-2 logarithm.

```hyp
induce log2_1 = Log2(1); // 0
induce log2_2 = Log2(2); // 1
induce log2_3 = Log2(8); // 3
```

### LogBase(x, base)

Calculates the logarithm with the specified base.

```hyp
induce logBase1 = LogBase(8, 2); // 3
induce logBase2 = LogBase(100, 10); // 2
induce logBase3 = LogBase(27, 3); // 3
```

## Exponential Functions

### Exp(x)

Calculates e^x.

```hyp
induce exp1 = Exp(0); // 1
induce exp2 = Exp(1); // E
induce exp3 = Exp(2); // E^2
```

### Exp2(x)

Calculates 2^x.

```hyp
induce exp2_1 = Exp2(0); // 1
induce exp2_2 = Exp2(1); // 2
induce exp2_3 = Exp2(3); // 8
```

### Exp10(x)

Calculates 10^x.

```hyp
induce exp10_1 = Exp10(0); // 1
induce exp10_2 = Exp10(1); // 10
induce exp10_3 = Exp10(2); // 100
```

## Hyperbolic Functions

### Sinh(x)

Calculates the hyperbolic sine.

```hyp
induce sinh1 = Sinh(0); // 0
induce sinh2 = Sinh(1); // 1.1752011936438014
```

### Cosh(x)

Calculates the hyperbolic cosine.

```hyp
induce cosh1 = Cosh(0); // 1
induce cosh2 = Cosh(1); // 1.5430806348152437
```

### Tanh(x)

Calculates the hyperbolic tangent.

```hyp
induce tanh1 = Tanh(0); // 0
induce tanh2 = Tanh(1); // 0.7615941559557649
```

## Integer Operations

### Mod(dividend, divisor)

Calculates the modulo (remainder of division).

```hyp
induce mod1 = Mod(7, 3); // 1
induce mod2 = Mod(10, 5); // 0
induce mod3 = Mod(-7, 3); // -1
```

### Div(dividend, divisor)

Calculates integer division.

```hyp
induce div1 = Div(7, 3); // 2
induce div2 = Div(10, 5); // 2
induce div3 = Div(15, 4); // 3
```

### GCD(a, b)

Calculates the greatest common divisor.

```hyp
induce gcd1 = GCD(12, 18); // 6
induce gcd2 = GCD(7, 13); // 1
induce gcd3 = GCD(0, 5); // 5
```

### LCM(a, b)

Calculates the least common multiple.

```hyp
induce lcm1 = LCM(12, 18); // 36
induce lcm2 = LCM(7, 13); // 91
induce lcm3 = LCM(4, 6); // 12
```

### IsPrime(n)

Checks if a number is prime.

```hyp
induce isPrime1 = IsPrime(2); // true
induce isPrime2 = IsPrime(17); // true
induce isPrime3 = IsPrime(4); // false
```

### NextPrime(n)

Finds the next prime number.

```hyp
induce nextPrime1 = NextPrime(10); // 11
induce nextPrime2 = NextPrime(17); // 19
induce nextPrime3 = NextPrime(1); // 2
```

### PrimeFactors(n)

Decomposes a number into prime factors.

```hyp
induce factors1 = PrimeFactors(12); // [2, 2, 3]
induce factors2 = PrimeFactors(17); // [17]
induce factors3 = PrimeFactors(100); // [2, 2, 5, 5]
```

## Statistics

### Sum(array)

Calculates the sum of an array.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce sum = Sum(numbers); // 15
```

### Average(array)

Calculates the average of an array.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce avg = Average(numbers); // 3
```

### Median(array)

Calculates the median of an array.

```hyp
induce numbers1 = [1, 2, 3, 4, 5];
induce median1 = Median(numbers1); // 3

induce numbers2 = [1, 2, 3, 4];
induce median2 = Median(numbers2); // 2.5
```

### Mode(array)

Calculates the mode of an array.

```hyp
induce numbers = [1, 2, 2, 3, 4, 2, 5];
induce mode = Mode(numbers); // 2
```

### Variance(array)

Calculates the variance of an array.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce variance = Variance(numbers); // 2.5
```

### StandardDeviation(array)

Calculates the standard deviation of an array.

```hyp
induce numbers = [1, 2, 3, 4, 5];
induce stdDev = StandardDeviation(numbers); // 1.5811388300841898
```

### Min(array)

Finds the minimum in an array.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce min = Min(numbers); // 1
```

### Max(array)

Finds the maximum in an array.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce max = Max(numbers); // 9
```

### Range(array)

Calculates the range of an array.

```hyp
induce numbers = [3, 1, 4, 1, 5, 9, 2, 6];
induce range = Range(numbers); // 8
```

## Random Numbers

### Random()

Generates a random number between 0 and 1.

```hyp
induce random1 = Random(); // 0.123456789
induce random2 = Random(); // 0.987654321
```

### RandomRange(min, max)

Generates a random number in a range.

```hyp
induce random1 = RandomRange(1, 10); // Random integer between 1 and 10
induce random2 = RandomRange(0.0, 1.0); // Random decimal between 0 and 1
```

### RandomInt(min, max)

Generates a random integer.

```hyp
induce random1 = RandomInt(1, 10); // Random integer between 1 and 10
induce random2 = RandomInt(-100, 100); // Random integer between -100 and 100
```

### RandomChoice(array)

Selects a random element from an array.

```hyp
induce fruits = ["Apfel", "Banane", "Orange"];
induce randomFruit = RandomChoice(fruits); // Random fruit
```

### RandomSample(array, count)

Selects random elements from an array.

```hyp
induce numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
induce sample = RandomSample(numbers, 3); // 3 random numbers
```

## Mathematical Constants

### PI

The mathematical constant π.

```hyp
induce pi = PI; // 3.141592653589793
```

### E

Euler's number e.

```hyp
induce e = E; // 2.718281828459045
```

### PHI

The golden ratio φ.

```hyp
induce phi = PHI; // 1.618033988749895
```

### SQRT2

The square root of 2.

```hyp
induce sqrt2 = SQRT2; // 1.4142135623730951
```

### SQRT3

The square root of 3.

```hyp
induce sqrt3 = SQRT3; // 1.7320508075688772
```

## Practical Examples

### Geometric Calculations

```hyp
Focus {
    entrance {
        // Circle calculations
        induce radius = 5;
        induce area = PI * Pow(radius, 2);
        induce circumference = 2 * PI * radius;

        observe "Circle with radius " + radius + ":";
        observe "Area: " + Round(area, 2);
        observe "Circumference: " + Round(circumference, 2);

        // Triangle calculations
        induce a = 3;
        induce b = 4;
        induce c = Sqrt(Pow(a, 2) + Pow(b, 2)); // Pythagoras

        observe "Right triangle:";
        observe "Side a: " + a;
        observe "Side b: " + b;
        observe "Hypotenuse c: " + Round(c, 2);

        // Volume of a sphere
        induce sphereRadius = 3;
        induce volume = (4.0 / 3.0) * PI * Pow(sphereRadius, 3);
        observe "Sphere volume: " + Round(volume, 2);
    }
} Relax;
```

### Statistical Analysis

```hyp
Focus {
    entrance {
        induce scores = [85, 92, 78, 96, 88, 91, 87, 94, 82, 89];

        observe "Exam results: " + scores;
        observe "Count: " + ArrayLength(scores);
        observe "Average: " + Round(Average(scores), 2);
        observe "Median: " + Median(scores);
        observe "Minimum: " + Min(scores);
        observe "Maximum: " + Max(scores);
        observe "Range: " + Range(scores);
        observe "Standard deviation: " + Round(StandardDeviation(scores), 2);
        observe "Variance: " + Round(Variance(scores), 2);

        // Grade distribution
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

## Next Steps

- [Utility Functions](./utility-functions) - General helper functions
- [System Functions](./system-functions) - System interaction
- [Statistics Functions](./statistics-functions) - Advanced statistics
- [Examples](../examples/math-examples) - More mathematical examples

---

**Mastered mathematical functions? Then learn about [Utility Functions](./utility-functions)!** 🔧
