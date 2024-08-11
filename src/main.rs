use std::io;

use libs::message::Message;

mod libs;

fn main() -> io::Result<()> {
    let file_path = libs::file_helper::get_cache_path();
    let data = vec!["Hello", "world", "this", "is", "Rust"];
    libs::file_helper::save_data(&file_path, data)?;
    println!("Appended strings to file.");

    let strings = libs::file_helper::read_data(&file_path)?;
    for s in strings {
        println!("{}", s);
    }

    let msg = Message {
        msg_id: 1,
        msg: "Hello 测试文档".to_string(),
        from_user: true,
    };
    let data = serde_json::to_string(&msg)?;
    // println!("{:?}", data);

    let un_json: Message = serde_json::from_str(&data)?;
    println!("{:?}", un_json);

    Ok(())
}
