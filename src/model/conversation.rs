
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversation{
    pub messages : Vec<Message>,
}
impl Conversation {
    pub fn new() -> Conversation {
       Conversation { messages: Vec::new() } 
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub user: bool,
    pub text: String,
}