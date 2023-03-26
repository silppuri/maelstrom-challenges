use serde_json::{json, Value};
use std::io::Write;
use std::io::{self, BufRead};

use std::sync::atomic::{AtomicUsize, Ordering};

static MESSAGE_ID: AtomicUsize = AtomicUsize::new(0);

fn respond(request: &Value, response: &Value) -> io::Result<()> {
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
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("{}\n", msg);
    return handle.flush();
}

fn respond_to_init_message(request: &Value) -> io::Result<()> {
    let init_response = json!({
        "type": "init_ok",
        "in_reply_to": request["body"]["msg_id"],
    });
    respond(request, &init_response)?;
    Ok(())
}

fn respond_to_echo_message(request: &Value) -> io::Result<()> {
    let echo_response = json!({
        "echo": request["body"]["echo"],
        "type": "echo_ok",
        "in_reply_to": request["body"]["msg_id"],
    });
    respond(request, &echo_response)?;
    Ok(())
}

fn handle_message(message: Value) -> io::Result<()> {
    match message["body"]["type"].as_str() {
        Some("init") => respond_to_init_message(&message),
        Some("echo") => respond_to_echo_message(&message),
        Some("error") => {
            println!("ERROR: {}", message);
            panic!()
        }
        Some(_) => {
            todo!()
        }
        None => Ok(()),
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(message) = serde_json::from_str(line.unwrap().as_str())? {
            handle_message(message)?
        }
    }
    Ok(())
}

fn merge(a: &mut Value, b: Value) {
    if let Value::Object(a) = a {
        if let Value::Object(b) = b {
            for (k, v) in b {
                if v.is_null() {
                    a.remove(&k);
                } else {
                    merge(a.entry(k).or_insert(Value::Null), v);
                }
            }

            return;
        }
    }

    *a = b;
}
