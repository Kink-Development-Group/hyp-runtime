//! Real async Promise/Future implementation for HypnoScript
//!
//! Provides true asynchronous promises that integrate with Tokio runtime.

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::sync::{Mutex, Notify};

/// Async Promise state
#[derive(Debug, Clone)]
pub enum PromiseState<T> {
    Pending,
    Resolved(T),
    Rejected(String),
}

/// A real async Promise that implements Future trait
pub struct AsyncPromise<T> {
    state: Arc<Mutex<PromiseState<T>>>,
    notify: Arc<Notify>,
}

impl<T: Clone> AsyncPromise<T> {
    /// Create a new pending promise
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PromiseState::Pending)),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Create an already resolved promise
    pub fn resolved(value: T) -> Self {
        Self {
            state: Arc::new(Mutex::new(PromiseState::Resolved(value))),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Create an already rejected promise
    pub fn rejected(error: String) -> Self {
        Self {
            state: Arc::new(Mutex::new(PromiseState::Rejected(error))),
            notify: Arc::new(Notify::new()),
        }
    }

    /// Resolve the promise with a value
    pub async fn resolve(&self, value: T) {
        let mut state = self.state.lock().await;
        *state = PromiseState::Resolved(value);
        drop(state);
        self.notify.notify_waiters();
    }

    /// Reject the promise with an error
    pub async fn reject(&self, error: String) {
        let mut state = self.state.lock().await;
        *state = PromiseState::Rejected(error);
        drop(state);
        self.notify.notify_waiters();
    }

    /// Check if promise is resolved
    pub async fn is_resolved(&self) -> bool {
        matches!(*self.state.lock().await, PromiseState::Resolved(_))
    }

    /// Check if promise is rejected
    pub async fn is_rejected(&self) -> bool {
        matches!(*self.state.lock().await, PromiseState::Rejected(_))
    }

    /// Check if promise is pending
    pub async fn is_pending(&self) -> bool {
        matches!(*self.state.lock().await, PromiseState::Pending)
    }

    /// Get the current state (non-blocking snapshot)
    #[allow(dead_code)]
    pub async fn state_snapshot(&self) -> PromiseState<T> {
        self.state.lock().await.clone()
    }
}

impl<T: Clone> Default for AsyncPromise<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Future for AsyncPromise<T> {
    type Output = Result<T, String>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Try to lock the state
        let state = match self.state.try_lock() {
            Ok(guard) => guard.clone(),
            Err(_) => {
                // If we can't lock, register waker and return pending
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };

        match state {
            PromiseState::Resolved(value) => Poll::Ready(Ok(value)),
            PromiseState::Rejected(error) => Poll::Ready(Err(error)),
            PromiseState::Pending => {
                // Register waker for when promise resolves
                let waker = cx.waker().clone();
                let notify = self.notify.clone();

                tokio::spawn(async move {
                    notify.notified().await;
                    waker.wake();
                });

                Poll::Pending
            }
        }
    }
}

impl<T: Clone> Clone for AsyncPromise<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            notify: self.notify.clone(),
        }
    }
}

/// Promise combinator: all promises must resolve
pub async fn promise_all<T: Clone>(promises: Vec<AsyncPromise<T>>) -> Result<Vec<T>, String> {
    let mut results = Vec::new();

    for promise in promises {
        results.push(promise.await?);
    }

    Ok(results)
}

/// Promise combinator: race - first to complete wins
pub async fn promise_race<T: Clone>(promises: Vec<AsyncPromise<T>>) -> Result<T, String> {
    if promises.is_empty() {
        return Err("No promises provided".to_string());
    }

    tokio::select! {
        result = promises[0].clone() => result,
        result = async {
            for promise in promises.iter().skip(1) {
                if let Ok(value) = promise.clone().await {
                    return Ok(value);
                }
            }
            Err("All promises rejected".to_string())
        } => result,
    }
}

/// Promise combinator: any - first successful resolution
pub async fn promise_any<T: Clone>(promises: Vec<AsyncPromise<T>>) -> Result<T, String> {
    if promises.is_empty() {
        return Err("No promises provided".to_string());
    }

    let mut errors = Vec::new();

    for promise in promises {
        match promise.await {
            Ok(value) => return Ok(value),
            Err(e) => errors.push(e),
        }
    }

    Err(format!("All promises rejected: {:?}", errors))
}

/// Promise combinator: allSettled - wait for all to settle (resolve or reject)
pub async fn promise_all_settled<T: Clone>(
    promises: Vec<AsyncPromise<T>>,
) -> Vec<Result<T, String>> {
    let mut results = Vec::new();

    for promise in promises {
        results.push(promise.await);
    }

    results
}

/// Create a promise that resolves after a delay
pub fn promise_delay<T: Clone + Send + 'static>(
    duration: std::time::Duration,
    value: T,
) -> AsyncPromise<T> {
    let promise = AsyncPromise::new();
    let promise_clone = promise.clone();

    tokio::spawn(async move {
        tokio::time::sleep(duration).await;
        promise_clone.resolve(value).await;
    });

    promise
}

/// Create a promise from an async function
pub fn promise_from_async<F, T>(future: F) -> AsyncPromise<T>
where
    F: Future<Output = Result<T, String>> + Send + 'static,
    T: Clone + Send + 'static,
{
    let promise = AsyncPromise::new();
    let promise_clone = promise.clone();

    tokio::spawn(async move {
        match future.await {
            Ok(value) => promise_clone.resolve(value).await,
            Err(error) => promise_clone.reject(error).await,
        }
    });

    promise
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_promise_resolve() {
        let promise = AsyncPromise::new();
        promise.resolve(42).await;

        let result = promise.await;
        assert_eq!(result, Ok(42));
    }

    #[tokio::test]
    async fn test_promise_reject() {
        let promise: AsyncPromise<i32> = AsyncPromise::new();
        promise.reject("Error!".to_string()).await;

        let result = promise.await;
        assert_eq!(result, Err("Error!".to_string()));
    }

    #[tokio::test]
    async fn test_promise_already_resolved() {
        let promise = AsyncPromise::resolved(100);
        let result = promise.await;
        assert_eq!(result, Ok(100));
    }

    #[tokio::test]
    async fn test_promise_delay() {
        let start = std::time::Instant::now();
        let promise = promise_delay(Duration::from_millis(100), "done");

        let result = promise.await;
        let elapsed = start.elapsed();

        assert_eq!(result, Ok("done"));
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[tokio::test]
    async fn test_promise_all() {
        let promises = vec![
            AsyncPromise::resolved(1),
            AsyncPromise::resolved(2),
            AsyncPromise::resolved(3),
        ];

        let result = promise_all(promises).await;
        assert_eq!(result, Ok(vec![1, 2, 3]));
    }

    #[tokio::test]
    async fn test_promise_all_with_rejection() {
        let promises = vec![
            AsyncPromise::resolved(1),
            AsyncPromise::rejected("Error".to_string()),
            AsyncPromise::resolved(3),
        ];

        let result = promise_all(promises).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_promise_race() {
        let promises = vec![
            promise_delay(Duration::from_millis(100), 1),
            promise_delay(Duration::from_millis(50), 2),
            promise_delay(Duration::from_millis(150), 3),
        ];

        let result = promise_race(promises).await;
        assert_eq!(result, Ok(2)); // Should be the fastest
    }

    #[tokio::test]
    async fn test_promise_from_async() {
        let promise = promise_from_async(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(42)
        });

        let result = promise.await;
        assert_eq!(result, Ok(42));
    }
}
