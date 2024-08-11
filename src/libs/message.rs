use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub msg_id: i128,
    pub msg: String,
    pub from_user: bool,
}
