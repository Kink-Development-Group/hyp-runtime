use std::f64;

/// Mathematical builtin functions
pub struct MathBuiltins;

impl MathBuiltins {
    /// Sine function
    pub fn sin(x: f64) -> f64 {
        x.sin()
    }

    /// Cosine function
    pub fn cos(x: f64) -> f64 {
        x.cos()
    }

    /// Tangent function
    pub fn tan(x: f64) -> f64 {
        x.tan()
    }

    /// Square root
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    /// Power function
    pub fn pow(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }

    /// Natural logarithm
    pub fn log(x: f64) -> f64 {
        x.ln()
    }

    /// Base-10 logarithm
    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    /// Absolute value
    pub fn abs(x: f64) -> f64 {
        x.abs()
    }

    /// Floor function
    pub fn floor(x: f64) -> f64 {
        x.floor()
    }

    /// Ceiling function
    pub fn ceil(x: f64) -> f64 {
        x.ceil()
    }

    /// Round function
    pub fn round(x: f64) -> f64 {
        x.round()
    }

    /// Minimum of two values
    pub fn min(a: f64, b: f64) -> f64 {
        a.min(b)
    }

    /// Maximum of two values
    pub fn max(a: f64, b: f64) -> f64 {
        a.max(b)
    }

    /// Factorial
    pub fn factorial(n: i64) -> i64 {
        if n <= 1 {
            1
        } else {
            (2..=n).product()
        }
    }

    /// Greatest Common Divisor
    pub fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a.abs()
    }

    /// Least Common Multiple
    pub fn lcm(a: i64, b: i64) -> i64 {
        if a == 0 || b == 0 {
            0
        } else {
            (a * b).abs() / Self::gcd(a, b)
        }
    }

    /// Check if number is prime
    pub fn is_prime(n: i64) -> bool {
        if n <= 1 {
            return false;
        }
        if n <= 3 {
            return true;
        }
        if n % 2 == 0 || n % 3 == 0 {
            return false;
        }
        let mut i = 5;
        while i * i <= n {
            if n % i == 0 || n % (i + 2) == 0 {
                return false;
            }
            i += 6;
        }
        true
    }

    /// Fibonacci number
    pub fn fibonacci(n: i64) -> i64 {
        if n <= 1 {
            n
        } else {
            let mut a = 0;
            let mut b = 1;
            for _ in 2..=n {
                let temp = a + b;
                a = b;
                b = temp;
            }
            b
        }
    }

    /// Clamp value between min and max
    pub fn clamp(value: f64, min: f64, max: f64) -> f64 {
        if value < min {
            min
        } else if value > max {
            max
        } else {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial() {
        assert_eq!(MathBuiltins::factorial(5), 120);
        assert_eq!(MathBuiltins::factorial(0), 1);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(MathBuiltins::gcd(48, 18), 6);
        assert_eq!(MathBuiltins::gcd(100, 50), 50);
    }

    #[test]
    fn test_is_prime() {
        assert!(MathBuiltins::is_prime(7));
        assert!(MathBuiltins::is_prime(13));
        assert!(!MathBuiltins::is_prime(4));
        assert!(!MathBuiltins::is_prime(1));
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(MathBuiltins::fibonacci(0), 0);
        assert_eq!(MathBuiltins::fibonacci(1), 1);
        assert_eq!(MathBuiltins::fibonacci(10), 55);
    }
}
