use ChatUser;


#[derive(Debug)]
pub struct ChatMessage {
	pub sender: ChatUser,
	pub value: String,
}
