use tokio::sync::broadcast;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthEvent {
    LoginSuccess { user_id: i32 },
}

pub struct EventBus {
    pub auth_tx: broadcast::Sender<AuthEvent>,
}

impl EventBus {
    pub fn new() -> Self {
        let (auth_tx, _) = broadcast::channel(100);
        Self { auth_tx }
    }
}
