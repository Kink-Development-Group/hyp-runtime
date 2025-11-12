use std::thread;
use std::time::Duration;

/// Core I/O and hypnotic builtin functions
pub struct CoreBuiltins;

impl CoreBuiltins {
    /// Output a value (observe)
    pub fn observe(value: &str) {
        println!("{}", value);
    }

    /// Wait for specified milliseconds (drift)
    pub fn drift(ms: u64) {
        thread::sleep(Duration::from_millis(ms));
    }

    /// Deep trance induction
    pub fn deep_trance(duration: u64) {
        Self::observe("Entering deep trance...");
        Self::drift(duration);
        Self::observe("Emerging from trance...");
    }

    /// Hypnotic countdown
    pub fn hypnotic_countdown(from: i64) {
        for i in (1..=from).rev() {
            Self::observe(&format!("You are feeling very sleepy... {}", i));
            Self::drift(1000);
        }
        Self::observe("You are now in a deep hypnotic state.");
    }

    /// Trance induction
    pub fn trance_induction(subject_name: &str) {
        Self::observe(&format!(
            "Welcome {}, you are about to enter a deep trance...",
            subject_name
        ));
        Self::drift(2000);
        Self::observe("Take a deep breath and relax...");
        Self::drift(1500);
        Self::observe("With each breath, you feel more and more relaxed...");
        Self::drift(1500);
        Self::observe("Your mind is becoming clear and focused...");
        Self::drift(1000);
    }

    /// Hypnotic visualization
    pub fn hypnotic_visualization(scene: &str) {
        Self::observe(&format!("Imagine yourself in {}...", scene));
        Self::drift(1500);
        Self::observe("The colors are vivid, the sounds are clear...");
        Self::drift(1500);
        Self::observe("You feel completely at peace in this place...");
        Self::drift(1000);
    }

    /// Conversion functions
    pub fn to_int(value: f64) -> i64 {
        value as i64
    }

    pub fn to_double(value: &str) -> Result<f64, String> {
        value.parse::<f64>().map_err(|e| e.to_string())
    }

    pub fn to_string(value: &dyn std::fmt::Display) -> String {
        format!("{}", value)
    }

    pub fn to_boolean(value: &str) -> bool {
        matches!(value.to_lowercase().as_str(), "true" | "1" | "yes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_int() {
        assert_eq!(CoreBuiltins::to_int(42.7), 42);
        assert_eq!(CoreBuiltins::to_int(-5.2), -5);
    }

    #[test]
    fn test_to_double() {
        assert_eq!(CoreBuiltins::to_double("3.14").unwrap(), 3.14);
        assert!(CoreBuiltins::to_double("invalid").is_err());
    }

    #[test]
    fn test_to_boolean() {
        assert!(CoreBuiltins::to_boolean("true"));
        assert!(CoreBuiltins::to_boolean("True"));
        assert!(CoreBuiltins::to_boolean("1"));
        assert!(!CoreBuiltins::to_boolean("false"));
    }
}
