use std::{
    cell::RefCell,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde_json::{json, Value};

use crate::utils::merge;

pub struct Node {
    key: String,
    id: AtomicUsize,
    messages: RefCell<Vec<i64>>,
}

impl Node {
    pub fn new(key: String) -> Node {
        Node {
            key,
            id: AtomicUsize::new(0),
            messages: RefCell::new(Vec::new()),
        }
    }

    pub fn respond_to_init_message(&self, request: &Value) -> () {
        let init_response = json!({
            "type": "init_ok",
        });
        self.respond(request, &init_response)
    }

    pub fn respond_to_echo_message(&self, request: &Value) -> () {
        let echo_response = json!({
            "echo": request["body"]["echo"],
            "type": "echo_ok",
        });
        self.respond(request, &echo_response)
    }

    pub fn respond_to_generate_message(&self, request: &Value) -> () {
        let generate_response = json!({
            "type": "generate_ok",
            "id": format!("{}-{}", self.key, self.id.fetch_add(1, Ordering::SeqCst)),
        });
        self.respond(request, &generate_response)
    }

    pub fn handle_broadcast_message(&self, request: &Value) -> () {
        self.messages
            .borrow_mut()
            .push(request["body"]["message"].as_i64().unwrap());
        let broadcast_response = json!({
            "type": "broadcast_ok",
        });
        self.respond(request, &broadcast_response)
    }

    pub fn handle_read_message(&self, request: &Value) -> () {
        let seen_messages = self.messages.borrow().clone();
        let read_response = json!({
            "type": "read_ok",
            "messages": seen_messages
        });
        self.respond(request, &read_response)
    }

    pub fn handle_topology_message(&self, request: &Value) -> () {
        let topology_response = json!({
            "type": "topology_ok",
        });
        self.respond(request, &topology_response)
    }

    fn respond(&self, request: &Value, response: &Value) -> () {
        let mut new_body = json!({
            "msg_id": self.id.fetch_add(1, Ordering::SeqCst),
            "in_reply_to": request["body"]["msg_id"]
        });
        merge(&mut new_body, response.to_owned());
        let msg = json!({
            "src": self.key,
            "dest": request["src"],
            "body": new_body
        });
        println!("{}", msg)
    }
}
