extern crate serde;
extern crate serde_json;

use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::io::Write;
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::builder::ObjectBuilder;
use serde_json::error::Error as JsonError;


#[derive(Debug)]
pub struct ChatUser {
	pub name: String,
	pub poster: SocketAddr,
}

#[derive(Debug)]
pub struct ChatMessage {
	pub sender: ChatUser,
	pub value: String,
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

	pub fn to_json(&self) -> Value {
		ObjectBuilder::new().insert("name", &self.name).insert("poster", self.poster.to_string()).unwrap()
	}

	pub fn from_json(json: Value) -> Result<ChatUser, JsonError> {
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


#[test]
fn asdf() {
	let cu = ChatUser::get("keke".to_string(), "127.0.0.1:50030");
	let jsoned = serde_json::to_string(&cu.to_json());
	let _ = File::create("lol").ok().unwrap().write_fmt(format_args!("{:?}", jsoned));
	let decoded = ChatUser::from_json(serde_json::from_str(&*&jsoned.ok().unwrap()).ok().unwrap());
	let _ = File::create("kek").ok().unwrap().write_fmt(format_args!("{:?}", decoded));
}
