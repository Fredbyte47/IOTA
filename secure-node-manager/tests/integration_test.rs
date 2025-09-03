use secure_node_manager::core::node::Node;

#[tokio::test]
async fn test_node_initialization() {
    let node = Node::new("https://testnet.iota.org:443").await;
    assert!(node.is_ok());
}
