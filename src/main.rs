use maelstrom_challenges::node::Node;
use serde_json::Value;
use std::collections::HashMap;
use std::io::{self, BufRead};

fn register_new_node(message: &Value, nodes: &mut HashMap<String, Node>) -> Option<Node> {
    let key = message["dest"].as_str();
    nodes.insert(
        key.to_owned().unwrap().to_string(),
        Node::new(key.to_owned().unwrap().to_string()),
    )
}

fn handle_message(message: Value, nodes: &mut HashMap<String, Node>) -> () {
    if let Some(message_type) = message["body"]["type"].as_str() {
        if message_type == "init" {
            register_new_node(&message, nodes);
        }
        if let Some(node) = nodes.get(message["dest"].as_str().unwrap()) {
            match message_type {
                "init" => node.respond_to_init_message(&message),
                "echo" => node.respond_to_echo_message(&message),
                "generate" => node.respond_to_generate_message(&message),
                "broadcast" => node.handle_broadcast_message(&message),
                "read" => node.handle_read_message(&message),
                "topology" => node.handle_topology_message(&message),
                "error" => {
                    panic!("Error {}", message)
                }
                _ => {
                    todo!()
                }
            }
        } else {
            panic!("No proper node found for message {}", message)
        }
    }
}

fn main() -> io::Result<()> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(message) = serde_json::from_str(line.unwrap().as_str())? {
            handle_message(message, &mut nodes)
        }
    }
    Ok(())
}
