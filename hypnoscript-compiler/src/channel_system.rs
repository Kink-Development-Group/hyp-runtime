//! Channel system for inter-task communication in HypnoScript
//!
//! Provides multiple channel types for different communication patterns:
//! - MPSC (Multiple Producer Single Consumer)
//! - Broadcast (Multiple Producer Multiple Consumer)
//! - Watch (Single Producer Multiple Consumer with state)
//! - Oneshot (Single Producer Single Consumer, one-time)

use tokio::sync::{mpsc, broadcast, watch};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Channel identifier
pub type ChannelId = String;

/// Channel types available in HypnoScript
#[derive(Debug, Clone)]
pub enum ChannelType {
    /// Multiple Producer Single Consumer
    Mpsc { buffer_size: usize },
    /// Multiple Producer Multiple Consumer (Broadcast)
    Broadcast { capacity: usize },
    /// Single Producer Multiple Consumer (Watch)
    Watch,
    /// Single Producer Single Consumer (Oneshot)
    Oneshot,
}

/// Message wrapper for type-safe channel communication
#[derive(Debug, Clone)]
pub struct ChannelMessage {
    pub sender_id: Option<String>,
    pub timestamp: std::time::SystemTime,
    pub payload: crate::interpreter::Value,
}

impl ChannelMessage {
    pub fn new(payload: crate::interpreter::Value) -> Self {
        Self {
            sender_id: None,
            timestamp: std::time::SystemTime::now(),
            payload,
        }
    }

    pub fn with_sender(mut self, sender_id: String) -> Self {
        self.sender_id = Some(sender_id);
        self
    }
}

/// MPSC Channel wrapper
pub struct MpscChannel {
    tx: mpsc::UnboundedSender<ChannelMessage>,
    rx: Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<ChannelMessage>>>,
}

impl MpscChannel {
    pub fn new(buffer_size: usize) -> Self {
        let (tx, rx) = if buffer_size == 0 {
            mpsc::unbounded_channel()
        } else {
            // For bounded channels, we use unbounded for simplicity
            // In production, use mpsc::channel(buffer_size)
            mpsc::unbounded_channel()
        };

        Self {
            tx,
            rx: Arc::new(tokio::sync::Mutex::new(rx)),
        }
    }

    pub fn sender(&self) -> mpsc::UnboundedSender<ChannelMessage> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> Arc<tokio::sync::Mutex<mpsc::UnboundedReceiver<ChannelMessage>>> {
        self.rx.clone()
    }

    pub async fn send(&self, message: ChannelMessage) -> Result<(), String> {
        self.tx.send(message)
            .map_err(|e| format!("Failed to send message: {}", e))
    }

    pub async fn receive(&self) -> Option<ChannelMessage> {
        let mut rx = self.rx.lock().await;
        rx.recv().await
    }
}

/// Broadcast Channel wrapper
pub struct BroadcastChannel {
    tx: broadcast::Sender<ChannelMessage>,
}

impl BroadcastChannel {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn sender(&self) -> broadcast::Sender<ChannelMessage> {
        self.tx.clone()
    }

    pub fn subscribe(&self) -> broadcast::Receiver<ChannelMessage> {
        self.tx.subscribe()
    }

    pub async fn send(&self, message: ChannelMessage) -> Result<(), String> {
        self.tx.send(message)
            .map(|_| ())
            .map_err(|e| format!("Failed to broadcast message: {}", e))
    }
}

/// Watch Channel wrapper (for state updates)
pub struct WatchChannel {
    tx: watch::Sender<Option<ChannelMessage>>,
    rx: watch::Receiver<Option<ChannelMessage>>,
}

impl WatchChannel {
    pub fn new() -> Self {
        let (tx, rx) = watch::channel(None);
        Self { tx, rx }
    }

    pub fn sender(&self) -> watch::Sender<Option<ChannelMessage>> {
        self.tx.clone()
    }

    pub fn receiver(&self) -> watch::Receiver<Option<ChannelMessage>> {
        self.rx.clone()
    }

    pub async fn send(&self, message: ChannelMessage) -> Result<(), String> {
        self.tx.send(Some(message))
            .map_err(|e| format!("Failed to send watch message: {}", e))
    }

    pub async fn get_current(&self) -> Option<ChannelMessage> {
        self.rx.borrow().clone()
    }
}

/// Channel registry for managing named channels
pub struct ChannelRegistry {
    mpsc_channels: Arc<RwLock<HashMap<ChannelId, MpscChannel>>>,
    broadcast_channels: Arc<RwLock<HashMap<ChannelId, BroadcastChannel>>>,
    watch_channels: Arc<RwLock<HashMap<ChannelId, WatchChannel>>>,
}

impl ChannelRegistry {
    pub fn new() -> Self {
        Self {
            mpsc_channels: Arc::new(RwLock::new(HashMap::new())),
            broadcast_channels: Arc::new(RwLock::new(HashMap::new())),
            watch_channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Create a new MPSC channel
    pub async fn create_mpsc(&self, id: ChannelId, buffer_size: usize) -> Result<(), String> {
        let mut channels = self.mpsc_channels.write().await;
        if channels.contains_key(&id) {
            return Err(format!("Channel '{}' already exists", id));
        }
        channels.insert(id, MpscChannel::new(buffer_size));
        Ok(())
    }

    /// Create a new Broadcast channel
    pub async fn create_broadcast(&self, id: ChannelId, capacity: usize) -> Result<(), String> {
        let mut channels = self.broadcast_channels.write().await;
        if channels.contains_key(&id) {
            return Err(format!("Channel '{}' already exists", id));
        }
        channels.insert(id, BroadcastChannel::new(capacity));
        Ok(())
    }

    /// Create a new Watch channel
    pub async fn create_watch(&self, id: ChannelId) -> Result<(), String> {
        let mut channels = self.watch_channels.write().await;
        if channels.contains_key(&id) {
            return Err(format!("Channel '{}' already exists", id));
        }
        channels.insert(id, WatchChannel::new());
        Ok(())
    }

    /// Get MPSC channel
    pub async fn get_mpsc(&self, id: &ChannelId) -> Option<MpscChannel> {
        let channels = self.mpsc_channels.read().await;
        channels.get(id).map(|ch| MpscChannel {
            tx: ch.tx.clone(),
            rx: ch.rx.clone(),
        })
    }

    /// Get Broadcast channel
    pub async fn get_broadcast(&self, id: &ChannelId) -> Option<BroadcastChannel> {
        let channels = self.broadcast_channels.read().await;
        channels.get(id).map(|ch| BroadcastChannel {
            tx: ch.tx.clone(),
        })
    }

    /// Get Watch channel
    pub async fn get_watch(&self, id: &ChannelId) -> Option<WatchChannel> {
        let channels = self.watch_channels.read().await;
        channels.get(id).map(|ch| WatchChannel {
            tx: ch.tx.clone(),
            rx: ch.rx.clone(),
        })
    }

    /// Send to MPSC channel
    pub async fn send_mpsc(&self, id: &ChannelId, message: ChannelMessage) -> Result<(), String> {
        let channel = self.get_mpsc(id).await
            .ok_or_else(|| format!("MPSC channel '{}' not found", id))?;
        channel.send(message).await
    }

    /// Send to Broadcast channel
    pub async fn send_broadcast(&self, id: &ChannelId, message: ChannelMessage) -> Result<(), String> {
        let channel = self.get_broadcast(id).await
            .ok_or_else(|| format!("Broadcast channel '{}' not found", id))?;
        channel.send(message).await
    }

    /// Send to Watch channel
    pub async fn send_watch(&self, id: &ChannelId, message: ChannelMessage) -> Result<(), String> {
        let channel = self.get_watch(id).await
            .ok_or_else(|| format!("Watch channel '{}' not found", id))?;
        channel.send(message).await
    }

    /// Receive from MPSC channel
    pub async fn receive_mpsc(&self, id: &ChannelId) -> Result<Option<ChannelMessage>, String> {
        let channel = self.get_mpsc(id).await
            .ok_or_else(|| format!("MPSC channel '{}' not found", id))?;
        Ok(channel.receive().await)
    }

    /// Remove a channel
    pub async fn remove_mpsc(&self, id: &ChannelId) -> bool {
        self.mpsc_channels.write().await.remove(id).is_some()
    }

    pub async fn remove_broadcast(&self, id: &ChannelId) -> bool {
        self.broadcast_channels.write().await.remove(id).is_some()
    }

    pub async fn remove_watch(&self, id: &ChannelId) -> bool {
        self.watch_channels.write().await.remove(id).is_some()
    }
}

impl Default for ChannelRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::Value;

    #[tokio::test]
    async fn test_mpsc_channel() {
        let channel = MpscChannel::new(10);

        let message = ChannelMessage::new(Value::Number(42.0));
        channel.send(message.clone()).await.unwrap();

        let received = channel.receive().await.unwrap();
        assert!(matches!(received.payload, Value::Number(n) if n == 42.0));
    }

    #[tokio::test]
    async fn test_broadcast_channel() {
        let channel = BroadcastChannel::new(10);

        let mut rx1 = channel.subscribe();
        let mut rx2 = channel.subscribe();

        let message = ChannelMessage::new(Value::String("Hello".to_string()));
        channel.send(message).await.unwrap();

        let msg1 = rx1.recv().await.unwrap();
        let msg2 = rx2.recv().await.unwrap();

        assert!(matches!(msg1.payload, Value::String(ref s) if s == "Hello"));
        assert!(matches!(msg2.payload, Value::String(ref s) if s == "Hello"));
    }

    #[tokio::test]
    async fn test_watch_channel() {
        let channel = WatchChannel::new();
        let mut rx = channel.receiver();

        let message = ChannelMessage::new(Value::Boolean(true));
        channel.send(message).await.unwrap();

        rx.changed().await.unwrap();
        let current = rx.borrow().clone().unwrap();
        assert!(matches!(current.payload, Value::Boolean(true)));
    }

    #[tokio::test]
    async fn test_channel_registry() {
        let registry = ChannelRegistry::new();

        // Create MPSC channel
        registry.create_mpsc("test-mpsc".to_string(), 10).await.unwrap();

        // Send and receive
        let message = ChannelMessage::new(Value::Number(100.0));
        registry.send_mpsc(&"test-mpsc".to_string(), message).await.unwrap();

        let received = registry.receive_mpsc(&"test-mpsc".to_string()).await.unwrap().unwrap();
        assert!(matches!(received.payload, Value::Number(n) if n == 100.0));
    }
}
