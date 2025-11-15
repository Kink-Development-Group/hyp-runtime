//! Async Runtime Management for HypnoScript
//!
//! Provides a Tokio-based async runtime with thread pool, event loop,
//! and coordination primitives for true asynchronous execution.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
use tokio::sync::{Mutex, RwLock, broadcast, mpsc};

/// Async runtime manager for HypnoScript
///
/// Provides:
/// - Tokio multi-threaded runtime
/// - Thread pool for parallel execution
/// - Event loop coordination
/// - Channel-based communication
pub struct AsyncRuntime {
    /// Tokio runtime instance
    runtime: Arc<Runtime>,

    /// Broadcast channel for events
    event_tx: broadcast::Sender<RuntimeEvent>,

    /// Message passing channel (MPSC)
    message_tx: mpsc::UnboundedSender<RuntimeMessage>,
    #[allow(dead_code)]
    message_rx: Arc<Mutex<mpsc::UnboundedReceiver<RuntimeMessage>>>,

    /// Shared state for spawned tasks
    tasks: Arc<RwLock<HashMap<TaskId, TaskHandle>>>,

    /// Task ID counter
    next_task_id: Arc<Mutex<u64>>,
}

/// Unique identifier for async tasks
pub type TaskId = u64;

/// Handle to a spawned task
pub struct TaskHandle {
    pub id: TaskId,
    pub handle: tokio::task::JoinHandle<Result<TaskResult, String>>,
}

/// Result of an async task
///
/// Note: Cannot contain full Value types due to Rc (not Send).
/// For thread-safe async, use primitive types or convert to/from Value.
#[derive(Debug, Clone)]
pub enum TaskResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

/// Events broadcast through the runtime
#[derive(Debug, Clone)]
pub enum RuntimeEvent {
    TaskStarted(TaskId),
    TaskCompleted(TaskId, Result<TaskResult, String>),
    TaskCancelled(TaskId),
}

/// Messages sent between runtime components
pub enum RuntimeMessage {
    CancelTask(TaskId),
    Shutdown,
}

impl AsyncRuntime {
    /// Create a new async runtime with default settings
    pub fn new() -> anyhow::Result<Self> {
        Self::with_worker_threads(num_cpus::get())
    }

    /// Create a new async runtime with specified worker threads
    pub fn with_worker_threads(worker_threads: usize) -> anyhow::Result<Self> {
        let runtime = Builder::new_multi_thread()
            .worker_threads(worker_threads)
            .thread_name("hypnoscript-async")
            .enable_all()
            .build()?;

        let (event_tx, _) = broadcast::channel(1000);
        let (message_tx, message_rx) = mpsc::unbounded_channel();

        Ok(Self {
            runtime: Arc::new(runtime),
            event_tx,
            message_tx,
            message_rx: Arc::new(Mutex::new(message_rx)),
            tasks: Arc::new(RwLock::new(HashMap::new())),
            next_task_id: Arc::new(Mutex::new(0)),
        })
    }

    /// Get the Tokio runtime handle
    pub fn handle(&self) -> tokio::runtime::Handle {
        self.runtime.handle().clone()
    }

    /// Spawn an async task on the runtime
    pub fn spawn<F>(&self, future: F) -> TaskId
    where
        F: futures::Future<Output = Result<TaskResult, String>> + Send + 'static,
    {
        let task_id = self.next_task_id();
        let event_tx = self.event_tx.clone();
        let tasks = self.tasks.clone();

        let handle = self.runtime.spawn(async move {
            // Notify task started
            let _ = event_tx.send(RuntimeEvent::TaskStarted(task_id));

            // Execute the future
            let result = future.await;

            // Notify task completed
            let _ = event_tx.send(RuntimeEvent::TaskCompleted(task_id, result.clone()));

            // Remove from tasks map
            tasks.write().await.remove(&task_id);

            result
        });

        // Store task handle
        let task_handle = TaskHandle {
            id: task_id,
            handle,
        };
        self.runtime.block_on(async {
            self.tasks.write().await.insert(task_id, task_handle);
        });

        task_id
    }

    /// Block on a future until completion
    pub fn block_on<F>(&self, future: F) -> F::Output
    where
        F: futures::Future,
    {
        self.runtime.block_on(future)
    }

    /// Subscribe to runtime events
    pub fn subscribe(&self) -> broadcast::Receiver<RuntimeEvent> {
        self.event_tx.subscribe()
    }

    /// Send a message through the runtime channel
    pub fn send_message(&self, message: RuntimeMessage) -> Result<(), String> {
        self.message_tx
            .send(message)
            .map_err(|e| format!("Failed to send message: {}", e))
    }

    /// Get next task ID
    fn next_task_id(&self) -> TaskId {
        let mut id = self.runtime.block_on(self.next_task_id.lock());
        let current = *id;
        *id += 1;
        current
    }

    /// Cancel a running task
    pub fn cancel_task(&self, task_id: TaskId) -> Result<(), String> {
        self.runtime.block_on(async {
            let mut tasks = self.tasks.write().await;
            if let Some(task) = tasks.remove(&task_id) {
                task.handle.abort();
                let _ = self.event_tx.send(RuntimeEvent::TaskCancelled(task_id));
                Ok(())
            } else {
                Err(format!("Task {} not found", task_id))
            }
        })
    }

    /// Wait for a task to complete
    pub async fn await_task(&self, task_id: TaskId) -> Result<TaskResult, String> {
        let _handle = {
            let tasks = self.tasks.read().await;
            tasks
                .get(&task_id)
                .ok_or_else(|| format!("Task {} not found", task_id))?
                .handle
                .abort_handle()
        };

        // Note: This is a simplified version. In production, we'd need a better approach
        // to avoid the ownership issues with JoinHandle
        Err("Task awaiting not yet fully implemented".to_string())
    }

    /// Shutdown the runtime
    pub fn shutdown(self) {
        // Cancel all running tasks
        self.runtime.block_on(async {
            let tasks = self.tasks.write().await;
            for (_, task) in tasks.iter() {
                task.handle.abort();
            }
        });

        // Send shutdown message
        let _ = self.send_message(RuntimeMessage::Shutdown);
    }
}

impl Default for AsyncRuntime {
    fn default() -> Self {
        Self::new().expect("Failed to create async runtime")
    }
}

impl Drop for AsyncRuntime {
    fn drop(&mut self) {
        // Cleanup happens automatically
    }
}

/// Async delay utility
pub async fn async_delay(duration: std::time::Duration) {
    tokio::time::sleep(duration).await;
}

/// Async timeout wrapper
pub async fn async_timeout<F, T>(duration: std::time::Duration, future: F) -> Result<T, String>
where
    F: futures::Future<Output = T>,
{
    tokio::time::timeout(duration, future)
        .await
        .map_err(|_| "Operation timed out".to_string())
}

/// Spawn a task on the global runtime
pub fn spawn_task<F>(future: F) -> tokio::task::JoinHandle<F::Output>
where
    F: futures::Future + Send + 'static,
    F::Output: Send + 'static,
{
    tokio::spawn(future)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_runtime_creation() {
        let runtime = AsyncRuntime::new();
        assert!(runtime.is_ok());
    }

    #[test]
    fn test_block_on() {
        let runtime = AsyncRuntime::new().unwrap();
        let result = runtime.block_on(async { 42 });
        assert_eq!(result, 42);
    }

    #[test]
    fn test_spawn_task() {
        let runtime = AsyncRuntime::new().unwrap();
        let _task_id = runtime.spawn(async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            Ok(TaskResult::Number(42.0))
        });

        // Give task time to complete
        std::thread::sleep(Duration::from_millis(50));
    }

    #[test]
    fn test_async_delay() {
        let runtime = AsyncRuntime::new().unwrap();
        let start = std::time::Instant::now();
        runtime.block_on(async_delay(Duration::from_millis(100)));
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(100));
    }

    #[test]
    fn test_async_timeout() {
        let runtime = AsyncRuntime::new().unwrap();

        // Should complete
        let result = runtime.block_on(async_timeout(Duration::from_millis(100), async { 42 }));
        assert_eq!(result, Ok(42));

        // Should timeout
        let result = runtime.block_on(async_timeout(Duration::from_millis(10), async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            42
        }));
        assert!(result.is_err());
    }
}
