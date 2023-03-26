use std::sync::atomic::{AtomicUsize, Ordering};

use serde_json::{json, Value};

use crate::utils::merge;

pub struct Node {
    key: String,
    id: AtomicUsize,
}

impl Node {
    pub fn new(key: String) -> Node {
        Node {
            key,
            id: AtomicUsize::new(0),
        }
    }

    pub fn respond_to_init_message(&self, request: &Value) -> () {
        let init_response = json!({
            "type": "init_ok",
            "in_reply_to": request["body"]["msg_id"],
        });
        self.respond(request, &init_response)
    }

    pub fn respond_to_echo_message(&self, request: &Value) -> () {
        let echo_response = json!({
            "echo": request["body"]["echo"],
            "type": "echo_ok",
            "in_reply_to": request["body"]["msg_id"],
        });
        self.respond(request, &echo_response)
    }

    pub fn respond_to_generate_message(&self, request: &Value) -> () {
        let generate_response = json!({
            "type": "generate_ok",
            "in_reply_to": request["body"]["msg_id"],
            "id": format!("{}-{}", self.key, self.id.fetch_add(1, Ordering::SeqCst)),
        });
        self.respond(request, &generate_response)
    }

    fn respond(&self, request: &Value, response: &Value) -> () {
        let mut new_body = json!({
            "msg_id": self.id.fetch_add(1, Ordering::SeqCst),
            "in_reply_to": response["body"]["msg_id"]
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
