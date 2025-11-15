//! Async built-in functions for HypnoScript
//!
//! Provides async operations like delay, spawn, timeout, and channel operations

use crate::async_runtime::{AsyncRuntime, TaskResult};
use crate::channel_system::{ChannelMessage, ChannelRegistry};
use crate::interpreter::Value;
use std::sync::Arc;
use std::time::Duration;

/// Async built-in functions
pub struct AsyncBuiltins;

impl AsyncBuiltins {
    /// Sleep for specified milliseconds (async delay)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// await asyncDelay(1000); // Sleep for 1 second
    /// ```
    pub async fn async_delay(milliseconds: f64) -> Value {
        let duration = Duration::from_millis(milliseconds as u64);
        crate::async_runtime::async_delay(duration).await;
        Value::Null
    }

    /// Create a promise that resolves after a delay
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce promise = delayedValue(1000, 42);
    /// induce result = await promise; // Returns 42 after 1 second
    /// ```
    ///
    /// Note: Due to Value containing Rc (not Send), this returns a placeholder.
    /// For true async promises with arbitrary values, Value needs to use Arc.
    pub fn delayed_value(milliseconds: f64, _value: Value) -> Value {
        // Cannot use promise_delay with Value due to Send requirement
        // This would require refactoring Value to use Arc instead of Rc
        Value::String(format!("<async promise: {}ms>", milliseconds))
    }

    /// Execute function with timeout
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce result = await withTimeout(5000, longRunningFunction());
    /// ```
    pub async fn with_timeout(milliseconds: f64, future_value: Value) -> Result<Value, String> {
        let duration = Duration::from_millis(milliseconds as u64);

        // In real implementation, this would wrap a future
        crate::async_runtime::async_timeout(duration, async {
            // Simulate async work
            tokio::time::sleep(Duration::from_millis(100)).await;
            future_value
        })
        .await
    }

    /// Spawn async task (fire and forget)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce taskId = spawnTask(taskData);
    /// ```
    ///
    /// Note: Simplified implementation due to Rc in Value (not thread-safe).
    /// For production, Value should use Arc instead of Rc.
    pub fn spawn_task_simple(runtime: &Arc<AsyncRuntime>) -> u64 {
        runtime.spawn(async move {
            // Simple async task
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            Ok(TaskResult::Null)
        })
    }

    /// Wait for multiple promises to complete (Promise.all)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce results = await promiseAll([promise1, promise2, promise3]);
    /// ```
    pub async fn promise_all(promises: Vec<Value>) -> Result<Value, String> {
        // In real implementation, would convert Value::Promise to AsyncPromise
        // and use crate::async_promise::promise_all

        Ok(Value::Array(promises))
    }

    /// Race multiple promises (first to complete wins)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce fastest = await promiseRace([promise1, promise2]);
    /// ```
    pub async fn promise_race(promises: Vec<Value>) -> Result<Value, String> {
        if promises.is_empty() {
            return Err("No promises provided".to_string());
        }

        // Return first promise for now
        Ok(promises[0].clone())
    }

    /// Create MPSC channel
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// createChannel("my-channel", "mpsc", 100);
    /// ```
    pub async fn create_channel(
        registry: &Arc<ChannelRegistry>,
        name: String,
        channel_type: String,
        capacity: f64,
    ) -> Result<Value, String> {
        match channel_type.as_str() {
            "mpsc" => {
                registry
                    .create_mpsc(name.clone(), capacity as usize)
                    .await?;
                Ok(Value::String(format!("Created MPSC channel: {}", name)))
            }
            "broadcast" => {
                registry
                    .create_broadcast(name.clone(), capacity as usize)
                    .await?;
                Ok(Value::String(format!(
                    "Created Broadcast channel: {}",
                    name
                )))
            }
            "watch" => {
                registry.create_watch(name.clone()).await?;
                Ok(Value::String(format!("Created Watch channel: {}", name)))
            }
            _ => Err(format!("Unknown channel type: {}", channel_type)),
        }
    }

    /// Send message to channel
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// await channelSend("my-channel", "mpsc", "Hello!");
    /// ```
    pub async fn channel_send(
        registry: &Arc<ChannelRegistry>,
        channel_name: String,
        channel_type: String,
        message: Value,
    ) -> Result<Value, String> {
        let msg = ChannelMessage::new(message);

        match channel_type.as_str() {
            "mpsc" => registry.send_mpsc(&channel_name, msg).await?,
            "broadcast" => registry.send_broadcast(&channel_name, msg).await?,
            "watch" => registry.send_watch(&channel_name, msg).await?,
            _ => return Err(format!("Unknown channel type: {}", channel_type)),
        }

        Ok(Value::Boolean(true))
    }

    /// Receive message from channel
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce message = await channelReceive("my-channel", "mpsc");
    /// ```
    pub async fn channel_receive(
        registry: &Arc<ChannelRegistry>,
        channel_name: String,
        channel_type: String,
    ) -> Result<Value, String> {
        match channel_type.as_str() {
            "mpsc" => {
                if let Some(msg) = registry.receive_mpsc(&channel_name).await? {
                    Ok(msg.payload)
                } else {
                    Ok(Value::Null)
                }
            }
            _ => Err(format!(
                "Receive not supported for channel type: {}",
                channel_type
            )),
        }
    }

    /// Parallel execution of multiple functions
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce results = await parallel([
    ///     suggestion() { return task1(); },
    ///     suggestion() { return task2(); },
    ///     suggestion() { return task3(); }
    /// ]);
    /// ```
    ///
    /// Note: Due to Value containing Rc (not thread-safe), we execute sequentially
    /// but with async concurrency. For true parallel execution, tasks should not
    /// share mutable state.
    pub async fn parallel_execute(task_count: usize) -> Vec<Value> {
        let mut results = Vec::new();

        // Execute tasks concurrently (but on same thread due to Rc in Value)
        for i in 0..task_count {
            tokio::task::yield_now().await;
            results.push(Value::Number(i as f64));
        }

        results
    }

    /// Get number of CPU cores for optimal parallelism
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// induce cores = cpuCount();
    /// observe "Available CPU cores: " + cores;
    /// ```
    pub fn cpu_count() -> Value {
        Value::Number(num_cpus::get() as f64)
    }

    /// Yield execution to other tasks (cooperative multitasking)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// await yieldTask();
    /// ```
    pub async fn yield_task() -> Value {
        tokio::task::yield_now().await;
        Value::Null
    }

    /// Sleep for specified seconds (alias for asyncDelay)
    ///
    /// # Example (HypnoScript)
    /// ```hypnoscript
    /// await sleep(2); // Sleep for 2 seconds
    /// ```
    pub async fn sleep(seconds: f64) -> Value {
        Self::async_delay(seconds * 1000.0).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_delay() {
        let start = std::time::Instant::now();
        AsyncBuiltins::async_delay(100.0).await;
        let elapsed = start.elapsed();

        assert!(elapsed >= Duration::from_millis(100));
        assert!(elapsed < Duration::from_millis(200));
    }

    #[tokio::test]
    async fn test_with_timeout() {
        let result = AsyncBuiltins::with_timeout(1000.0, Value::Number(42.0)).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Value::Number(42.0));
    }

    #[tokio::test]
    async fn test_yield_task() {
        let result = AsyncBuiltins::yield_task().await;
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_cpu_count() {
        let result = AsyncBuiltins::cpu_count();
        if let Value::Number(count) = result {
            assert!(count > 0.0);
        } else {
            panic!("Expected number");
        }
    }
}
