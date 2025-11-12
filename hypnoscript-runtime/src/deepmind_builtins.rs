/// DeepMind Control Flow and Higher-Order Functions
///
/// This module provides advanced control flow and functional programming constructs
/// for HypnoScript, including function composition, conditional execution, and
/// repetition utilities.
///
/// # Language Integration
///
/// These functions are designed to work with HypnoScript's hypnotic metaphors:
/// - `repeatAction`: Hypnotic repetition
/// - `delayedSuggestion`: Time-delayed execution
/// - `ifTranced`: Conditional execution as a function
/// - `compose`/`pipe`: Function composition
/// - `repeatUntil`/`repeatWhile`: Advanced loop constructs
/// - `tryOrAwaken`: Error handling
/// - `ensureAwakening`: Cleanup guarantee

use std::thread;
use std::time::Duration;

/// Hypnotic Control Flow Functions
pub struct DeepMindBuiltins;

impl DeepMindBuiltins {
    /// Repeat an action a specific number of times
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// repeatAction(5, suggestion() {
    ///     observe "Om";
    /// });
    /// ```
    pub fn repeat_action<F>(times: usize, mut action: F)
    where
        F: FnMut(),
    {
        for _ in 0..times {
            action();
        }
    }

    /// Execute an action after a delay (in milliseconds)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// delayedSuggestion(suggestion() {
    ///     observe "Delayed message";
    /// }, 2000);
    /// ```
    pub fn delayed_suggestion<F>(action: F, delay_ms: u64)
    where
        F: FnOnce(),
    {
        thread::sleep(Duration::from_millis(delay_ms));
        action();
    }

    /// Conditional execution as a function
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// ifTranced(age >= 18,
    ///     suggestion() { observe "Adult"; },
    ///     suggestion() { observe "Minor"; }
    /// );
    /// ```
    pub fn if_tranced<T, E>(condition: bool, then_action: T, else_action: E)
    where
        T: FnOnce(),
        E: FnOnce(),
    {
        if condition {
            then_action();
        } else {
            else_action();
        }
    }

    /// Compose two functions: f(g(x))
    ///
    /// Returns a new function that applies g first, then f
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce composed = compose(double, addTen);
    /// induce result = composed(5); // double(addTen(5)) = 30
    /// ```
    pub fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(B) -> C,
        G: Fn(A) -> B,
    {
        move |x| f(g(x))
    }

    /// Pipe two functions: g(f(x))
    ///
    /// Returns a new function that applies f first, then g
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce piped = pipe(double, addTen);
    /// induce result = piped(5); // addTen(double(5)) = 20
    /// ```
    pub fn pipe<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
    where
        F: Fn(A) -> B,
        G: Fn(B) -> C,
    {
        move |x| g(f(x))
    }

    /// Repeat until a condition becomes true
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce count = 0;
    /// repeatUntil(
    ///     suggestion() { count = count + 1; },
    ///     suggestion(): boolean { awaken count >= 5; }
    /// );
    /// ```
    pub fn repeat_until<A, C>(mut action: A, mut condition: C)
    where
        A: FnMut(),
        C: FnMut() -> bool,
    {
        while !condition() {
            action();
        }
    }

    /// Repeat while a condition is true
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce n = 3;
    /// repeatWhile(
    ///     suggestion(): boolean { awaken n > 0; },
    ///     suggestion() { observe "Countdown: " + n; n = n - 1; }
    /// );
    /// ```
    pub fn repeat_while<C, A>(mut condition: C, mut action: A)
    where
        C: FnMut() -> bool,
        A: FnMut(),
    {
        while condition() {
            action();
        }
    }

    /// Execute actions sequentially
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce actions = [
    ///     suggestion() { observe "Step 1"; },
    ///     suggestion() { observe "Step 2"; },
    ///     suggestion() { observe "Step 3"; }
    /// ];
    /// sequentialTrance(actions);
    /// ```
    pub fn sequential_trance<F>(actions: Vec<F>)
    where
        F: FnOnce(),
    {
        for action in actions {
            action();
        }
    }

    /// Try-catch style error handling
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// tryOrAwaken(
    ///     suggestion() {
    ///         // risky operation
    ///         induce x = riskyFunction();
    ///     },
    ///     suggestion(error: string) {
    ///         observe "Error: " + error;
    ///     }
    /// );
    /// ```
    pub fn try_or_awaken<T, E>(try_action: T, catch_action: E)
    where
        T: FnOnce() -> Result<(), String>,
        E: FnOnce(String),
    {
        if let Err(err) = try_action() {
            catch_action(err);
        }
    }

    /// Ensure cleanup code runs (like try-finally)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// ensureAwakening(
    ///     suggestion() {
    ///         observe "Main action";
    ///     },
    ///     suggestion() {
    ///         observe "Cleanup always runs";
    ///     }
    /// );
    /// ```
    pub fn ensure_awakening<M, C>(main_action: M, cleanup: C)
    where
        M: FnOnce(),
        C: FnOnce(),
    {
        main_action();
        cleanup();
    }

    /// Measure execution time of an action
    ///
    /// Returns the duration in milliseconds
    pub fn measure_trance_depth<F>(action: F) -> u128
    where
        F: FnOnce(),
    {
        use std::time::Instant;
        let start = Instant::now();
        action();
        start.elapsed().as_millis()
    }

    /// Memoize/cache a function result
    ///
    /// Note: This is a simplified version for demonstration.
    /// A real implementation would use a HashMap.
    pub fn memoize<A, R, F>(f: F) -> impl FnMut(A) -> R
    where
        F: Fn(A) -> R,
        A: Clone,
        R: Clone,
    {
        // Simplified: Just pass through
        // A real memoization would cache results
        move |x| f(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_action() {
        let mut count = 0;
        DeepMindBuiltins::repeat_action(5, || {
            count += 1;
        });
        assert_eq!(count, 5);
    }

    #[test]
    fn test_compose() {
        let double = |x: i32| x * 2;
        let add_ten = |x: i32| x + 10;

        let composed = DeepMindBuiltins::compose(double, add_ten);
        assert_eq!(composed(5), 30); // double(add_ten(5)) = double(15) = 30
    }

    #[test]
    fn test_pipe() {
        let double = |x: i32| x * 2;
        let add_ten = |x: i32| x + 10;

        let piped = DeepMindBuiltins::pipe(double, add_ten);
        assert_eq!(piped(5), 20); // add_ten(double(5)) = add_ten(10) = 20
    }

    #[test]
    fn test_repeat_until() {
        use std::cell::RefCell;
        let count = RefCell::new(0);
        DeepMindBuiltins::repeat_until(
            || { *count.borrow_mut() += 1; },
            || *count.borrow() >= 5
        );
        assert_eq!(*count.borrow(), 5);
    }

    #[test]
    fn test_repeat_while() {
        use std::cell::RefCell;
        let n = RefCell::new(3);
        DeepMindBuiltins::repeat_while(
            || *n.borrow() > 0,
            || { *n.borrow_mut() -= 1; }
        );
        assert_eq!(*n.borrow(), 0);
    }

    #[test]
    fn test_ensure_awakening() {
        let mut cleanup_called = false;
        DeepMindBuiltins::ensure_awakening(
            || { /* main action */ },
            || { cleanup_called = true; }
        );
        assert!(cleanup_called);
    }
}
