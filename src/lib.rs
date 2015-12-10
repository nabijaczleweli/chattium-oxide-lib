extern crate time;
extern crate serde;
extern crate serde_json;

mod user;
mod message;
pub mod json;

pub use self::user::*;
pub use self::message::*;


#[test]
fn asdf() {
	use std::fs::File;
	use std::io::Write;
	use json::*;

	let cu = ChatUser::get("keke".to_string(), "127.0.0.1:50030");
	let cm = ChatMessage::new(cu, "top-kek".to_string());
	let jsoned = cm.to_json_string();
	let _ = File::create("lol").ok().unwrap().write_fmt(format_args!("{:?}", jsoned));
	let decoded = ChatMessage::from_json_string(&*&jsoned.ok().unwrap());
	let _ = File::create("kek").ok().unwrap().write_fmt(format_args!("{:?}", decoded));
}
