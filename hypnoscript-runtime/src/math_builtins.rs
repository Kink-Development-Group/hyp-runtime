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

    /// Arc sine (inverse sine)
    /// Returns the angle in radians whose sine is x
    /// Range: [-π/2, π/2]
    pub fn asin(x: f64) -> f64 {
        x.asin()
    }

    /// Arc cosine (inverse cosine)
    /// Returns the angle in radians whose cosine is x
    /// Range: [0, π]
    pub fn acos(x: f64) -> f64 {
        x.acos()
    }

    /// Arc tangent (inverse tangent)
    /// Returns the angle in radians whose tangent is x
    /// Range: [-π/2, π/2]
    pub fn atan(x: f64) -> f64 {
        x.atan()
    }

    /// Two-argument arc tangent
    /// Computes the angle in radians between the positive x-axis and the point (x, y)
    /// Range: [-π, π]
    pub fn atan2(y: f64, x: f64) -> f64 {
        y.atan2(x)
    }

    /// Hyperbolic sine
    pub fn sinh(x: f64) -> f64 {
        x.sinh()
    }

    /// Hyperbolic cosine
    pub fn cosh(x: f64) -> f64 {
        x.cosh()
    }

    /// Hyperbolic tangent
    pub fn tanh(x: f64) -> f64 {
        x.tanh()
    }

    /// Inverse hyperbolic sine
    pub fn asinh(x: f64) -> f64 {
        x.asinh()
    }

    /// Inverse hyperbolic cosine
    pub fn acosh(x: f64) -> f64 {
        x.acosh()
    }

    /// Inverse hyperbolic tangent
    pub fn atanh(x: f64) -> f64 {
        x.atanh()
    }

    /// Square root
    pub fn sqrt(x: f64) -> f64 {
        x.sqrt()
    }

    /// Cube root
    pub fn cbrt(x: f64) -> f64 {
        x.cbrt()
    }

    /// Power function
    pub fn pow(base: f64, exponent: f64) -> f64 {
        base.powf(exponent)
    }

    /// Natural logarithm
    pub fn log(x: f64) -> f64 {
        x.ln()
    }

    /// Base-2 logarithm
    pub fn log2(x: f64) -> f64 {
        x.log2()
    }

    /// Base-10 logarithm
    pub fn log10(x: f64) -> f64 {
        x.log10()
    }

    /// Exponential function (e^x)
    pub fn exp(x: f64) -> f64 {
        x.exp()
    }

    /// 2^x
    pub fn exp2(x: f64) -> f64 {
        x.exp2()
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

    /// Hypotenuse (Euclidean distance)
    /// Computes sqrt(x^2 + y^2) without undue overflow or underflow
    pub fn hypot(x: f64, y: f64) -> f64 {
        x.hypot(y)
    }

    /// Convert degrees to radians
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * std::f64::consts::PI / 180.0
    }

    /// Convert radians to degrees
    pub fn radians_to_degrees(radians: f64) -> f64 {
        radians * 180.0 / std::f64::consts::PI
    }

    /// Sign function: returns -1, 0, or 1
    pub fn sign(x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else if x < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    /// Truncate (remove fractional part)
    pub fn trunc(x: f64) -> f64 {
        x.trunc()
    }

    /// Fractional part
    pub fn fract(x: f64) -> f64 {
        x.fract()
    }

    /// Factorial
    pub fn factorial(n: i64) -> i64 {
        if n <= 1 { 1 } else { (2..=n).product() }
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

    #[test]
    fn test_inverse_trig() {
        assert!((MathBuiltins::asin(0.5) - std::f64::consts::PI / 6.0).abs() < 0.0001);
        assert!((MathBuiltins::acos(0.5) - std::f64::consts::PI / 3.0).abs() < 0.0001);
        assert!((MathBuiltins::atan(1.0) - std::f64::consts::PI / 4.0).abs() < 0.0001);
    }

    #[test]
    fn test_hyperbolic() {
        assert!((MathBuiltins::sinh(0.0) - 0.0).abs() < 0.0001);
        assert!((MathBuiltins::cosh(0.0) - 1.0).abs() < 0.0001);
        assert!((MathBuiltins::tanh(0.0) - 0.0).abs() < 0.0001);
    }

    #[test]
    fn test_exp_and_log() {
        assert!((MathBuiltins::exp(1.0) - std::f64::consts::E).abs() < 0.0001);
        assert!((MathBuiltins::log2(8.0) - 3.0).abs() < 0.0001);
        assert!((MathBuiltins::exp2(3.0) - 8.0).abs() < 0.0001);
    }

    #[test]
    fn test_hypot() {
        assert!((MathBuiltins::hypot(3.0, 4.0) - 5.0).abs() < 0.0001);
    }

    #[test]
    fn test_angle_conversion() {
        assert!((MathBuiltins::degrees_to_radians(180.0) - std::f64::consts::PI).abs() < 0.0001);
        assert!((MathBuiltins::radians_to_degrees(std::f64::consts::PI) - 180.0).abs() < 0.0001);
    }

    #[test]
    fn test_sign() {
        assert_eq!(MathBuiltins::sign(42.0), 1.0);
        assert_eq!(MathBuiltins::sign(-42.0), -1.0);
        assert_eq!(MathBuiltins::sign(0.0), 0.0);
    }

    #[test]
    fn test_cbrt() {
        assert!((MathBuiltins::cbrt(27.0) - 3.0).abs() < 0.0001);
        assert!((MathBuiltins::cbrt(-8.0) - (-2.0)).abs() < 0.0001);
    }
}
