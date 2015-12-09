extern crate rustc_serialize;

use std::collections::btree_map::BTreeMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::fs::File;
use std::io::Write;
use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use rustc_serialize::json::{self, ToJson, Json};


#[derive(Debug)]
pub struct ChatUser {
	pub name: String,
	pub poster: SocketAddr,
}

#[derive(RustcDecodable)]
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
}

impl Decodable for ChatUser {
	fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
		let name = try!(d.read_str());
		let poster: &str = &*&try!(d.read_str());
		File::create("lel").ok().unwrap().write_fmt(format_args!("{}\n{}", name, poster));
		Ok(ChatUser::get(name, poster))
	}
}


impl ToJson for ChatUser {
	fn to_json(&self) -> Json {
		let mut d = BTreeMap::new();
		// All standard types implement `to_json()`, so use it
		d.insert("name".to_string(), self.name.to_json());
		d.insert("poster".to_string(), self.poster.to_string().to_json());
		Json::Object(d)
	}
}

/*impl Encodable for ChatUser {
	fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
		try!(s.emit_str(&*&self.name));
		try!(s.emit_str(&*&format!("{}", self.poster)));
		Ok(())
	}
}*/


#[test]
fn asdf() {
	let cu = ChatUser::get("keke".to_string(), "127.0.0.1:50030");
	let jsoned = cu.to_json().to_string();
	let _ = File::create("lol").ok().unwrap().write_fmt(format_args!("{}", jsoned));
	let decoded = json::decode::<ChatUser>(&jsoned);
	let _ = File::create("kek").ok().unwrap().write_fmt(format_args!("{:?}", decoded));
}
