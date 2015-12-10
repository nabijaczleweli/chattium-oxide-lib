use ChatUser;
use json::{FromJsonnable, ToJsonnable};
use time::{now_utc, at_utc, Tm};
use serde::de::Error;
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


#[derive(Debug)]
pub struct ChatMessage {
	pub sender: ChatUser,
	pub value: String,
	pub time_posted: Tm,
}


impl ChatMessage {
	pub fn new(by: ChatUser, contents: String) -> ChatMessage {
		ChatMessage{
			sender: by,
			value: contents,
			time_posted: now_utc(),
		}
	}
}

impl FromJsonnable for ChatMessage {
	fn from_json(json: Value) -> Result<Self, JsonError> {
		Err(JsonError::missing_field(""))
	}
}

impl ToJsonnable for ChatMessage {
	fn to_json(&self) -> Value {
		ObjectBuilder::new().insert("sender", &self.sender.to_json())
		                    .insert("value", &self.value)
		                    .insert("time_posted_s", self.time_posted.to_timespec().sec)
		                    .insert("time_posted_ns", self.time_posted.to_timespec().nsec)
		                    .unwrap()
	}
}
