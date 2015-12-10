use ChatUser;
use json::{FromJsonnable, ToJsonnable};
use time::{now_utc, Tm};
use serde::de::{Error, Type};
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
		match json {
			Value::Object(map) => {
				let sender = try!(
					match map.get("sender") {
						Some(sender) =>
							match sender {
								&Value::Object(_) => ChatUser::from_json(sender.clone()),
								_ => Err(JsonError::type_mismatch(Type::String)),
							},
						None => Err(JsonError::missing_field("Missing \"sender\"")),
					});
				let value =
					match map.get("value") {
						Some(value) =>
							match value {
								&Value::String(ref value) => value,
								_ => return Err(JsonError::type_mismatch(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"value\"")),
					};
				let time_posted = try!(
					match map.get("time_posted") {
						Some(time_posted) =>
							match time_posted {
								&Value::Object(_) => Tm::from_json(time_posted.clone()),
								_ => Err(JsonError::type_mismatch(Type::String)),
							},
						None => Err(JsonError::missing_field("Missing \"time_posted\"")),
					});

				Ok(ChatMessage{
					sender: sender,
					value: value.clone(),
					time_posted: time_posted,
				})
			},
			_ => Err(JsonError::type_mismatch(Type::Struct)),
		}
	}
}

impl ToJsonnable for ChatMessage {
	fn to_json(&self) -> Value {
		ObjectBuilder::new().insert("sender", &self.sender.to_json())
		                    .insert("value", &self.value)
		                    .insert("time_posted", &self.time_posted.to_json())
		                    .unwrap()
	}
}
