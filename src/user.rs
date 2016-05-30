use std::net::{SocketAddr, ToSocketAddrs};
use json::{ToJsonnable, FromJsonnable};
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


#[derive(Debug, Clone, Eq)]
pub struct ChatUser {
	/// User's desired name
	pub name: String,
	poster: Option<SocketAddr>,
}


impl ChatUser {
	/// Creates a user defined by the supplied arguments
	pub fn get<Addr: ToSocketAddrs>(name: String, poster: Addr) -> ChatUser {
		ChatUser{
			name: name,
			poster: Self::socket_addr_to_option(poster),
		}
	}

	/// Creates a named, IP-less user.
	/// Used by the client, as it doesn't know its IP, which is filled in server-side using [`fill_ip()`](#method.fill_ip).
	pub fn me(name: String) -> ChatUser {
		ChatUser{
			name: name,
			poster: None,
		}
	}

	/// Server-side function to fill in user's IP, see [`me()`](#method.me).
	pub fn fill_ip<Addr: ToSocketAddrs>(&mut self, poster: Addr) {
		self.poster = Self::socket_addr_to_option(poster);
	}


	fn socket_addr_to_option<Addr: ToSocketAddrs>(poster: Addr) -> Option<SocketAddr> {
		match poster.to_socket_addrs() {
			Ok(mut itr) =>
				match itr.next() {
					Some(addr) => Some(addr),
					None => None,
				},
			Err(_) => None,
		}
	}
}

impl PartialEq for ChatUser {
	/// Name-wise comparison for convenience
	fn eq(&self, other: &ChatUser) -> bool {
		self.name == other.name
	}
}

impl FromJsonnable for ChatUser {
	fn from_json(json: Value) -> Result<ChatUser, JsonError> {
		match json {
			Value::Object(map) => {
				let name =
					match map.get("name") {
						Some(name) =>
							match name {
								&Value::String(ref name) => name,
								_                        => return Err(JsonError::invalid_type(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"name\"")),
					};
				let poster =
					match map.get("poster") {
						Some(poster) =>
							match poster {
								&Value::String(ref poster) => poster,
								_                          => return Err(JsonError::invalid_type(Type::String)),
							},
						None => return Err(JsonError::missing_field("Missing \"poster\"")),
					};

				Ok(ChatUser::get(name.clone(), &poster[..]))
			},
			Value::String(name) => Ok(ChatUser::me(name)),
			_                   => Err(JsonError::invalid_type(Type::Struct)),
		}
	}
}

impl ToJsonnable for ChatUser {
	fn to_json(&self) -> Value {
		match self.poster {
			Some(ref ip) =>
				ObjectBuilder::new().insert("name", &self.name)
			                      .insert("poster", ip.to_string())
			                      .build(),
			None => Value::String(self.name.clone()),
		}
	}
}
