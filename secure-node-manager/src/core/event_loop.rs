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
    node_ref: Option<Arc<Node>>,
}

impl EventHandler {
    pub fn new() -> Self {
        EventHandler {
            callbacks: Vec::new(),
            event_buffer: Vec::new(),
            node_ref: None,
        }
    }

    pub fn set_node_reference(&mut self, node: Arc<Node>) {
        // This strong Arc was the source of the memory leak.
        // If the Node also held a reference to this EventHandler,
        // a reference cycle was created, preventing deallocation.
        self.node_ref = Some(node);
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
            // Attempt to promote the Weak pointer to a strong Arc.
            if let Some(node) = weak_node.upgrade() {
                // The node is still alive, we can use it.
                log::info!("EventHandler status: {}", node.get_status());
            } else {
                // The node has been dropped. We can clean up references.
                log::warn!("Node reference is no longer alive.");
            }
        }
    }

    // ... other methods like a run_loop would be here ...
}
