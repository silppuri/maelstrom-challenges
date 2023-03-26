use maelstrom_challenges::utils::merge;
use serde_json::{json, Value};
use std::io::{self, BufRead};

use std::sync::atomic::{AtomicUsize, Ordering};

static MESSAGE_ID: AtomicUsize = AtomicUsize::new(0);
static RANDOM_ID: AtomicUsize = AtomicUsize::new(0);

fn respond(request: &Value, response: &Value) -> () {
    let mut new_body = json!({
        "msg_id": MESSAGE_ID.fetch_add(1, Ordering::SeqCst),
        "in_reply_to": response["body"]["msg_id"]
    });
    merge(&mut new_body, response.to_owned());
    let msg = json!({
        "src": request["dest"],
        "dest": request["src"],
        "body": new_body
    });
    println!("{}", msg)
}

fn respond_to_init_message(request: &Value) -> () {
    eprintln!("INIT MESSAGE: {}", request);
    let init_response = json!({
        "type": "init_ok",
        "in_reply_to": request["body"]["msg_id"],
    });
    respond(request, &init_response)
}

fn respond_to_echo_message(request: &Value) -> () {
    let echo_response = json!({
        "echo": request["body"]["echo"],
        "type": "echo_ok",
        "in_reply_to": request["body"]["msg_id"],
    });
    respond(request, &echo_response)
}

fn respond_to_generate_message(request: &Value) -> () {
    let generate_response = json!({
        "type": "generate_ok",
        "in_reply_to": request["body"]["msg_id"],
        "id": format!("{}-{}", request["dest"].as_str().unwrap(), RANDOM_ID.fetch_add(1, Ordering::SeqCst)),
    });
    respond(request, &generate_response)
}

fn handle_message(message: Value) -> () {
    match message["body"]["type"].as_str() {
        Some("init") => respond_to_init_message(&message),
        Some("echo") => respond_to_echo_message(&message),
        Some("error") => {
            println!("ERROR: {}", message);
            panic!()
        }
        Some("generate") => respond_to_generate_message(&message),
        Some(_) => {
            todo!()
        }
        None => (),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(message) = serde_json::from_str(line.unwrap().as_str())? {
            handle_message(message)
        }
    }
    Ok(())
}
