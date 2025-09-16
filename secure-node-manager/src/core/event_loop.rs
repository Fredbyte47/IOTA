use std::sync::{Arc, Weak};
use super::node::Node;

#[derive(Clone, Debug)]
pub struct EventData {
    pub topic: String,
    pub payload: Vec<u8>,
}

pub struct EventHandler {
    callbacks: Vec<Box<dyn Fn(EventData) + Send + 'static>>,
    event_buffer: Vec<EventData>,
    node_ref: Option<Weak<Node>>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            callbacks: Vec::new(),
            event_buffer: Vec::new(),
            node_ref: None,
        }
    }
    
    pub fn set_node_reference(&mut self, node: &Arc<Node>) {
        self.node_ref = Some(Arc::downgrade(node));
        log::debug!("Node reference set (as weak pointer)");
    }

    pub fn add_callback<F>(&mut self, callback: F)
    where
        F: Fn(EventData) + Send + 'static,
    {
        self.callbacks.push(Box::new(callback));
    }

    pub fn push_event(&mut self, event: EventData) {
        self.event_buffer.push(event);
    }

    pub fn log_status_if_node_alive(&self) {
        if let Some(weak_node) = &self.node_ref {
            if let Some(node) = weak_node.upgrade() {
                log::info!("EventHandler status: {}", node.get_status());
            } else {
                log::warn!("Node reference is no longer alive.");
            }
        }
    }
}


// Find me on social media: @ProfessorPasta9
