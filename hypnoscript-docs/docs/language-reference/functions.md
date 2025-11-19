---
sidebar_position: 5
---

# Functions

Functions in HypnoScript are defined with the keyword `suggestion` and enable modularization and code reuse.

## Function Definition

### Basic Syntax

```hyp
suggestion functionName(parameter1: type1, parameter2: type2): returnType {
    // Function body
    awaken value; // Return statement
}
```

### Simple Function without Parameters

```hyp
Focus {
    suggestion greeting() {
        observe "Hello, HypnoScript!";
    }

    entrance {
        greeting();
    }
} Relax;
```

### Function with Parameters

```hyp
Focus {
    suggestion greet(name) {
        observe "Hello, " + name + "!";
    }

    entrance {
        greet("Max");
        greet("Anna");
    }
} Relax;
```

### Function with Return Value

```hyp
Focus {
    suggestion add(a, b) {
        awaken a + b;
    }

    suggestion isEven(number) {
        awaken number % 2 == 0;
    }

    entrance {
        induce sum = add(5, 3);
        observe "5 + 3 = " + sum;

        induce check = isEven(42);
        observe "42 is even: " + check;
    }
} Relax;
```

## Parameters

### Multiple Parameters

```hyp
Focus {
    suggestion rectangleArea(width, height) {
        awaken width * height;
    }

    suggestion personInfo(name, age, city) {
        awaken "Name: " + name + ", Age: " + age + ", City: " + city;
    }

    entrance {
        induce area = rectangleArea(10, 5);
        observe "Area: " + area;

        induce info = personInfo("Max", 30, "Berlin");
        observe info;
    }
} Relax;
```

### Parameters with Default Values

```hyp
Focus {
    suggestion greet(name, title = "Mr./Ms.") {
        observe title + " " + name + ", welcome!";
    }

    entrance {
        greet("Mustermann"); // Uses default title
        greet("Schmidt", "Dr."); // Overrides default title
    }
} Relax;
```

## Recursive Functions

```hyp
Focus {
    suggestion factorial(n) {
        if (n <= 1) {
            awaken 1;
        } else {
            return n * factorial(n - 1);
        }
    }

    suggestion fibonacci(n) {
        if (n <= 1) {
            awaken n;
        } else {
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
    }

    entrance {
        induce fact5 = factorial(5);
        observe "5! = " + fact5;

        induce fib10 = fibonacci(10);
        observe "Fibonacci(10) = " + fib10;
    }
} Relax;
```

## Functions with Arrays

```hyp
Focus {
    suggestion arraySum(numbers) {
        induce sum = 0;
        for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
            induce sum = sum + ArrayGet(numbers, i);
        }
        return sum;
    }

    suggestion findMaximum(numbers) {
        if (ArrayLength(numbers) == 0) {
            awaken null;
        }

        induce max = ArrayGet(numbers, 0);
        for (induce i = 1; i < ArrayLength(numbers); induce i = i + 1) {
            induce value = ArrayGet(numbers, i);
            if (value > max) {
                induce max = value;
            }
        }
        return max;
    }

    suggestion filterEven(numbers) {
        induce result = [];
        for (induce i = 0; i < ArrayLength(numbers); induce i = i + 1) {
            induce number = ArrayGet(numbers, i);
            if (number % 2 == 0) {
                // Extend array (simplified)
                observe "Even number found: " + number;
            }
        }
        return result;
    }

    entrance {
        induce testNumbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        induce sum = arraySum(testNumbers);
        observe "Sum: " + sum;

        induce max = findMaximum(testNumbers);
        observe "Maximum: " + max;

        filterEven(testNumbers);
    }
} Relax;
```

## Functions with Records

```hyp
Focus {
    suggestion createPerson(name, age, city) {
        awaken {
            name: name,
            age: age,
            city: city,
            ofLegalAge: age >= 18
        };
    }

    suggestion personInfo(person) {
        awaken person.name + " (" + person.age + ") from " + person.city;
    }

    suggestion isOfLegalAge(person) {
        awaken person.ofLegalAge;
    }

    entrance {
        induce person1 = createPerson("Max", 25, "Berlin");
        induce person2 = createPerson("Anna", 16, "Hamburg");

        observe personInfo(person1);
        observe personInfo(person2);

        observe "Max is of legal age: " + isOfLegalAge(person1);
        observe "Anna is of legal age: " + isOfLegalAge(person2);
    }
} Relax;
```

## Helper Functions

```hyp
Focus {
    suggestion validateAge(age) {
        awaken age >= 0 && age <= 150;
    }

    suggestion validateEmail(email) {
        // Simple email validation
        awaken Length(email) > 0 && email != null;
    }

    suggestion calculateBMI(weight, height) {
        if (height <= 0) {
            awaken null;
        }
        return weight / (height * height);
    }

    suggestion bmiCategory(bmi) {
        if (bmi == null) {
            awaken "Invalid";
        } else if (bmi < 18.5) {
            return "Underweight";
        } else if (bmi < 25) {
            return "Normal weight";
        } else if (bmi < 30) {
            return "Overweight";
        } else {
            return "Obesity";
        }
    }

    entrance {
        induce age = 25;
        induce email = "test@example.com";
        induce weight = 70;
        induce height = 1.75;

        if (validateAge(age)) {
            observe "Age is valid";
        }

        if (validateEmail(email)) {
            observe "Email is valid";
        }

        induce bmi = calculateBMI(weight, height);
        induce category = bmiCategory(bmi);
        observe "BMI: " + bmi + " (" + category + ")";
    }
} Relax;
```

## Mathematical Functions

```hyp
Focus {
    suggestion power(base, exponent) {
        if (exponent == 0) {
            awaken 1;
        }

        induce result = 1;
        for (induce i = 1; i <= exponent; induce i = i + 1) {
            induce result = result * base;
        }
        return result;
    }

    suggestion isPrime(number) {
        if (number < 2) {
            awaken false;
        }

        for (induce i = 2; i * i <= number; induce i = i + 1) {
            if (number % i == 0) {
                return false;
            }
        }
        return true;
    }

    suggestion gcd(a, b) {
        while (b != 0) {
            induce temp = b;
            induce b = a % b;
            induce a = temp;
        }
        return a;
    }

    entrance {
        observe "2^10 = " + power(2, 10);
        observe "17 is prime: " + isPrime(17);
        observe "GCD of 48 and 18: " + gcd(48, 18);
    }
} Relax;
```

## Best Practices

### Naming Functions

```hyp
// Good - descriptive names
suggestion calculateAverage(numbers) { ... }
suggestion isValidEmail(email) { ... }
suggestion formatDate(date) { ... }

// Bad - unclear names
suggestion calc(arr) { ... }
suggestion check(str) { ... }
suggestion format(d) { ... }
```

### Single Responsibility

```hyp
// Good - one function, one task
suggestion validateAge(age) {
    awaken age >= 0 && age <= 150;
}

suggestion calculateAgeGroup(age) {
    if (age < 18) awaken "Youth";
    if (age < 65) return "Adult";
    return "Senior";
}

// Bad - too many tasks in one function
suggestion processPerson(age, name, email) {
    // Validation, calculation, formatting all in one function
}
```

### Error Handling

```hyp
Focus {
    suggestion safeDivision(a, b) {
        if (b == 0) {
            observe "Error: Division by zero!";
            awaken null;
        }
        return a / b;
    }

    suggestion arrayElementSafe(arr, index) {
        if (index < 0 || index >= ArrayLength(arr)) {
            observe "Error: Index out of range!";
            awaken null;
        }
        return ArrayGet(arr, index);
    }

    entrance {
        induce result1 = safeDivision(10, 0);
        induce result2 = safeDivision(10, 2);

        induce numbers = [1, 2, 3];
        induce element1 = arrayElementSafe(numbers, 5);
        induce element2 = arrayElementSafe(numbers, 1);
    }
} Relax;
```

## Next Steps

- [Sessions](./sessions) - Session management
- [Tranceify](./tranceify) - Hypnotic applications
- [Arrays](./arrays) - Array operations
- [Records](./records) - Object programming

---

**Mastered functions? Then learn about [Sessions](./sessions)!** 🧠
