---
title: Quick Start
---

# Quick Start Guide

Get up and running with HypnoScript in minutes! This guide will walk you through installing HypnoScript and creating your first script.

## Prerequisites

- **Operating System**: Windows 10/11, Linux, or macOS
- **.NET Runtime**: .NET 8.0 or later
- **Memory**: At least 512MB RAM
- **Disk Space**: 50MB free space

## Installation

### Windows

1. **Using Winget (Recommended)**:

   ```bash
   winget install HypnoScript.HypnoScript
   ```

2. **Manual Installation**:
   - Download the latest release from [GitHub Releases](https://github.com/Kink-Development-Group/hyp-runtime/releases)
   - Extract the ZIP file to a directory of your choice
   - Add the directory to your system PATH

### Linux/macOS

1. **Using Package Manager**:

   ```bash
   # Ubuntu/Debian
   sudo apt-get install hypnoscript

   # macOS (using Homebrew)
   brew install hypnoscript
   ```

2. **Manual Installation**:
   ```bash
   # Download and install
   curl -L https://github.com/Kink-Development-Group/hyp-runtime/releases/latest/download/hypnoscript-linux-x64.tar.gz | tar -xz
   sudo mv hypnoscript /usr/local/bin/
   ```

## Verify Installation

Open a terminal or command prompt and run:

```bash
hyp --version
```

You should see output similar to:

```
HypnoScript CLI v1.0.0
```

## Your First Script

### 1. Create a Simple Script

Create a file named `hello.hyp` with the following content:

```hypno
Focus {
    // Display a welcome message
    Observe("Welcome to HypnoScript!");

    // Define some variables
    induce name: string = "World";
    induce greeting: string = "Hello, " + name + "!";

    // Display the greeting
    Observe(greeting);

    // Perform a simple calculation
    induce number: number = 42;
    induce result: number = number * 2;
    Observe("The answer is: " + result);

    // Use a built-in function
    induce currentTime: string = GetCurrentTime();
    Observe("Current time: " + currentTime);
} Relax
```

### 2. Run Your Script

```bash
hyp run hello.hyp
```

You should see output similar to:

```
Welcome to HypnoScript!
Hello, World!
The answer is: 84
Current time: 2024-01-15 14:30:25
```

## Understanding the Basics

### Script Structure

Every HypnoScript file follows this basic structure:

```hypno
Focus {
    // Your code goes here
    // This is the main execution block
} Relax
```

- `Focus { }` - Marks the beginning of your script execution
- `Relax` - Marks the end of your script execution

### Variables and Types

HypnoScript supports several data types:

```hypno
Focus {
    // String variables
    induce message: string = "Hello, World!";

    // Number variables
    induce count: number = 42;
    induce price: number = 19.99;

    // Boolean variables
    induce isActive: boolean = true;

    // Array variables
    induce numbers: number[] = [1, 2, 3, 4, 5];
    induce names: string[] = ["Alice", "Bob", "Charlie"];

    // Record variables (similar to objects)
    induce user: record = {
        "name": "John Doe",
        "age": 30,
        "email": "john@example.com"
    };
} Relax
```

### Basic Operations

```hypno
Focus {
    // Arithmetic operations
    induce a: number = 10;
    induce b: number = 5;
    induce sum: number = a + b;
    induce difference: number = a - b;
    induce product: number = a * b;
    induce quotient: number = a / b;

    // String operations
    induce firstName: string = "John";
    induce lastName: string = "Doe";
    induce fullName: string = firstName + " " + lastName;

    // Comparison operations
    induce isEqual: boolean = a == b;
    induce isGreater: boolean = a > b;
    induce isLessOrEqual: boolean = a <= b;

    // Logical operations
    induce condition1: boolean = true;
    induce condition2: boolean = false;
    induce bothTrue: boolean = condition1 && condition2;
    induce eitherTrue: boolean = condition1 || condition2;
} Relax
```

## Next Steps

### 1. Explore Built-in Functions

HypnoScript comes with many built-in functions:

```hypno
Focus {
    // String functions
    induce text: string = "Hello, World!";
    induce length: number = Length(text);
    induce upper: string = ToUpperCase(text);
    induce lower: string = ToLowerCase(text);

    // Math functions
    induce number: number = -5.7;
    induce absolute: number = Abs(number);
    induce rounded: number = Round(number);
    induce squareRoot: number = Sqrt(16);

    // Array functions
    induce numbers: number[] = [3, 1, 4, 1, 5];
    induce count: number = Length(numbers);
    induce sorted: number[] = Sort(numbers);
    induce max: number = Max(numbers);
} Relax
```

### 2. Create Functions

```hypno
Focus {
    // Define a simple function
    function Greet(name: string): string {
        return "Hello, " + name + "!";
    }

    // Define a function with multiple parameters
    function CalculateArea(width: number, height: number): number {
        return width * height;
    }

    // Use the functions
    induce greeting: string = Greet("Alice");
    induce area: number = CalculateArea(10, 5);

    Observe(greeting);
    Observe("Area: " + area);
} Relax
```

### 3. Use Control Structures

```hypno
Focus {
    induce score: number = 85;

    // If-else statements
    if (score >= 90) {
        Observe("Excellent!");
    } else if (score >= 80) {
        Observe("Good job!");
    } else if (score >= 70) {
        Observe("Not bad!");
    } else {
        Observe("Keep trying!");
    }

    // Loops
    induce numbers: number[] = [1, 2, 3, 4, 5];

    for (induce i: number = 0; i < Length(numbers); i = i + 1) {
        Observe("Number " + (i + 1) + ": " + numbers[i]);
    }

    // While loop
    induce count: number = 0;
    while (count < 3) {
        Observe("Count: " + count);
        count = count + 1;
    }
} Relax
```

## CLI Commands

HypnoScript CLI provides several useful commands:

```bash
# Run a script
hyp run script.hyp

# Check script for errors (linting)
hyp lint script.hyp

# Measure script performance
hyp benchmark script.hyp

# Generate documentation
hyp docs script.hyp

# Show help
hyp --help

# Show version
hyp --version
```

## Troubleshooting

### Common Issues

1. **"Command not found" error**:

   - Ensure HypnoScript is properly installed
   - Check that the installation directory is in your PATH
   - Try restarting your terminal

2. **Script won't run**:

   - Check for syntax errors using `hyp lint script.hyp`
   - Ensure the file has a `.hyp` extension
   - Verify the script has proper `Focus { } Relax` structure

3. **Permission denied**:
   - On Linux/macOS, ensure the script file is executable
   - Check file permissions: `chmod +x script.hyp`

### Getting Help

- **Documentation**: Explore the [HypnoScript Docs](/intro)
- **GitHub Issues**: Report bugs at [GitHub Issues](https://github.com/Kink-Development-Group/hyp-runtime/issues)
- **Support**: Tausche dich im [GitHub Repository](https://github.com/Kink-Development-Group/hyp-runtime) aus

## What's Next?

Now that you've completed the quick start guide, you can:

1. **Read the Language Reference** - Learn about all HypnoScript features
2. **Explore Examples** - See practical examples and use cases
3. **Try Advanced Features** - Learn about sessions, tranceify, and more
4. **Build Your Own Projects** - Start creating your own HypnoScript applications

Welcome to the HypnoScript community! 🚀
