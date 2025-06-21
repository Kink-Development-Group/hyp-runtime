---
sidebar_position: 3
---

# Operatoren

HypnoScript unterstützt arithmetische, Vergleichs- und logische Operatoren sowie spezielle Operatoren für Arrays und Records.

## Arithmetische Operatoren

```bash
| Operator | Bedeutung      | Beispiel | Ergebnis |
| -------- | -------------- | -------- | -------- |
| +        | Addition       | 2 + 3    | 5        |
| -        | Subtraktion    | 5 - 2    | 3        |
| \*       | Multiplikation | 4 \* 2   | 8        |
| /        | Division       | 8 / 2    | 4        |
| %        | Modulo         | 7 % 3    | 1        |
| ^        | Potenz         | 2 ^ 3    | 8        |
```

## Vergleichsoperatoren

```bash
| Operator | Bedeutung      | Beispiel | Ergebnis |
| -------- | -------------- | -------- | -------- |
| ==       | Gleich         | 3 == 3   | true     |
| !=       | Ungleich       | 3 != 4   | true     |
| <        | Kleiner        | 2 < 5    | true     |
| >        | Größer         | 5 > 2    | true     |
| <=       | Kleiner gleich | 2 <= 2   | true     |
| >=       | Größer gleich  | 3 >= 2   | true     |
```

## Logische Operatoren

```bash
| Operator | Bedeutung     | Beispiel      | Ergebnis |
| -------- | ------------- | ------------- | -------- | ---- | --- | ----- | ---- |
| &&       | Und           | true && false | false    |
|          |               |               | Oder     | true |     | false | true |
| !        | Nicht         | !true         | false    |
| ^        | Exklusiv-Oder | true ^ false  | true     |
```

## Array- und Record-Operatoren

- Zugriff auf Array-Element: `arr[0]`
- Zugriff auf Record-Feld: `person.name`
- Zuweisung: `arr[1] = 42;`, `person.age = 31;`

## Zuweisungsoperatoren

```hyp
induce x = 5;
x = x + 1; // 6
x += 2;    // 8
x -= 3;    // 5
x *= 2;    // 10
x /= 5;    // 2
```

## Beispiele

```hyp
Focus {
    entrance {
        induce a = 10;
        induce b = 3;
        observe "a + b = " + (a + b);
        observe "a ^ b = " + (a ^ b);
        observe "a == b: " + (a == b);
        observe "a > b: " + (a > b);
        induce arr = [1,2,3];
        observe arr[1]; // 2
        induce person = { name: "Max", age: 30 };
        observe person.name;
    }
} Relax;
```
