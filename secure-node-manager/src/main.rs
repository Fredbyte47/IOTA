use secure_node_manager::core::node::Node;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    log::info!("Starting IOTA Secure Node Manager...");

    let mut node = Node::new("https://api.iota.org:443").await?;
    node.start().await?;

    // ... main event loop would be here ...

    Ok(())
}
