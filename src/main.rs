use std::io;

use libs::message::Message;

mod libs;

fn main() -> io::Result<()> {
    let file_path = libs::file_helper::get_cache_path();

    //create a test json vec
    let mut msg_vec = Vec::new();

    for msg_id in 1..10 {
        msg_vec.push(Message {
            msg_id: msg_id,
            msg: "test".to_string(),
            from_user: msg_id % 2 == 0,
        });
    }

    // then, transform it into a vec<&str>
    let mut messages: Vec<String> = Vec::new();
    for message in msg_vec {
        let msg_json: String = serde_json::to_string(&message)?;
        messages.push(msg_json);
    }

    // Create a Vec<&str> from Vec<String>
    let message_refs: Vec<&str> = messages.iter().map(AsRef::as_ref).collect();

    libs::file_helper::save_data(&file_path, message_refs)?;
    println!("Appended strings to file.");

    let strings = libs::file_helper::read_data(&file_path)?;

    for msg in strings {
        let un_json: Message = serde_json::from_str(&msg)?;
        println!("{:?}", un_json.msg);
    }

    Ok(())
}
