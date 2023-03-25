use serde_json::{json, Value};
use std::io::Write;
use std::io::{self, BufRead};

// {
//     "type":     "init",
//     "msg_id":   1,
//     "node_id":  "n3",
//     "node_ids": ["n1", "n2", "n3"]
//   }
fn respond_to_init_message(raw_message: &Value) -> io::Result<()> {
    let init_response = json!({
        "src": raw_message["dest"],
        "dest": raw_message["src"],
        "body": {
            "type": "init_ok",
            "in_reply_to": raw_message["body"]["msg_id"]
        }
    });
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    print!("{}\n", init_response);
    handle.flush().unwrap();
    Ok(())
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Some(message) = serde_json::from_str(line.unwrap().as_str())? {
            handle_message(message)?;
            ()
        }
    }
    Ok(())
}

fn handle_message(message: Value) -> io::Result<()> {
    match message["body"]["type"].as_str() {
        Some("init") => {
            respond_to_init_message(&message)?;
            return Ok(());
        }
        Some("error") => {
            println!("ERROR: {}", message);
            panic!()
        }
        Some(_) => todo!(),
        None => (),
    }
    Ok(())
}

// let deserialized_message: MessageType = serde_json::from_str(line.unwrap().as_str())?;
// if deserialized_message.type_ == "init" {
//     let response = handle_init_message(message);
//     let response_str = response.unwrap();
//     if let Some(value) = (response_str.as_str()) {
//         println!("{}", value);
//         io::stdout().write_all(value.as_bytes());
//     }
// }
