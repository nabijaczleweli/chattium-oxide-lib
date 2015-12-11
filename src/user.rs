use std::net::{SocketAddr, ToSocketAddrs};
use json::{ToJsonnable, FromJsonnable};
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


#[derive(Debug, Clone)]
pub struct ChatUser {
	pub name: String,
	poster: SocketAddr,
}


impl ChatUser {
	//TODO: look it up somewhere, maybe?
	/// Creates a user defined by the supplied arguments
	pub fn get<Addr: ToSocketAddrs>(name: String, poster: Addr) -> ChatUser {
		ChatUser{
			name: name,
			poster: poster.to_socket_addrs().ok().unwrap().next().unwrap(),  //TODO: This should probably be handled, eh?
		}
	}
}

impl PartialEq for ChatUser {
	/// Only poster-wise comparison, names might change
	fn eq(&self, other: &ChatUser) -> bool {
		self.poster == other.poster
	}
}

impl Eq for ChatUser {}

impl FromJsonnable for ChatUser {
	fn from_json(json: Value) -> Result<ChatUser, JsonError> {
		match json {
			Value::Object(map) => {
				let name =
					match map.get("name") {
						Some(name) =>
							match name {
								&Value::String(ref name) => name,
								_ => return Err(JsonError::type_mismatch(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"name\"")),
					};
				let poster =
					match map.get("poster") {
						Some(poster) =>
							match poster {
								&Value::String(ref poster) => poster,
								_ => return Err(JsonError::type_mismatch(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"poster\"")),
					};

				Ok(ChatUser::get(name.clone(), &poster[..]))
			},
			_ => Err(JsonError::type_mismatch(Type::Struct)),
		}
	}
}

impl ToJsonnable for ChatUser {
	fn to_json(&self) -> Value {
		ObjectBuilder::new().insert("name", &self.name)
		                    .insert("poster", self.poster.to_string())
		                    .unwrap()
	}
}
