use ChatUser;
use std::ops::DerefMut;
use json::{FromJsonnable, ToJsonnable};
use time::{now_utc, Tm};
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMessage {
	pub sender: ChatUser,
	pub value: String,
	pub time_posted: Tm,
	pub id: u64,
}


impl ChatMessage {
	pub fn new(by: ChatUser, contents: String) -> ChatMessage {
		ChatMessage{
			sender: by,
			value: contents,
			time_posted: now_utc(),
			id: 0
		}
	}

	/// Can be used with, say, `Rwlock<u64>.write().unwrap()`
	pub fn fill_id<IdFiller: DerefMut<Target=u64>>(&mut self, mut curid: IdFiller) {
		self.id = *curid;
		*curid += 1;
	}
}

impl FromJsonnable for ChatMessage {
	fn from_json(json: Value) -> Result<Self, JsonError> {
		match json {
			Value::Object(map) => {
				let sender = try!(
					match map.get("sender") {
						Some(sender) => ChatUser::from_json(sender.clone()),
						None         => Err(JsonError::missing_field("Missing \"sender\"")),
					});
				let value =
					match map.get("value") {
						Some(value) =>
							match value {
								&Value::String(ref value) => value,
								_                         => return Err(JsonError::invalid_type(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"value\"")),
					};
				let time_posted = try!(
					match map.get("time_posted") {
						Some(time_posted) => Tm::from_json(time_posted.clone()),
						None              => Err(JsonError::missing_field("Missing \"time_posted\"")),
					});
				let id =
					match map.get("id") {
						Some(id) =>
							match id {
								&Value::U64(ref id) => *id,
								_                   => return Err(JsonError::invalid_type(Type::U64)),
							},
						None => 0,
					};

				Ok(ChatMessage{
					sender: sender,
					value: value.clone(),
					time_posted: time_posted,
					id: id,
				})
			},
			_ => Err(JsonError::invalid_type(Type::Struct)),
		}
	}
}

impl ToJsonnable for ChatMessage {
	fn to_json(&self) -> Value {
		let builder = ObjectBuilder::new().insert("sender"     , &self.sender.to_json())
		                                  .insert("value"      , &self.value)
		                                  .insert("time_posted", &self.time_posted.to_json());

		if self.id != 0 {
			builder.insert("id", &self.id)
		} else {
			builder
		}.build()
	}
}
