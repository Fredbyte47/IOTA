pub mod core;

/// Prelude for importing common types.
pub mod prelude {
    pub use crate::core::node::Node;
    pub use crate::core::event_loop::EventHandler;
}
