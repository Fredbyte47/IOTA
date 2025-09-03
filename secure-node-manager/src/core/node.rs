use crate::core::event_loop::EventHandler;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug)]
pub struct Node {
    pub api_endpoint: String,
    pub is_running: bool,
}

#[derive(Error, Debug)]
pub enum NodeError {
    #[error("Failed to connect to endpoint: {0}")]
    ConnectionError(String),
}

impl Node {
    pub async fn new(endpoint: &str) -> Result<Self, NodeError> {
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        log::info!("Node configured for endpoint: {}", endpoint);

        Ok(Node {
            api_endpoint: endpoint.to_string(),
            is_running: false,
        })
    }

    pub async fn start(&mut self) -> Result<(), NodeError> {
        self.is_running = true;
        log::info!("Node started successfully.");
        Ok(())
    }

    pub fn get_status(&self) -> String {
        format!("Node at {} is {}", self.api_endpoint, if self.is_running { "RUNNING" } else { "STOPPED" })
    }

    pub fn register_with_event_handler(&self, handler: &mut EventHandler) {
    }
}
