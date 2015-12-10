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
	let jsoned = serde_json::to_string(&cu.to_json());
	let _ = File::create("lol").ok().unwrap().write_fmt(format_args!("{:?}", jsoned));
	let decoded = ChatUser::from_json(serde_json::from_str(&*&jsoned.ok().unwrap()).ok().unwrap());
	let _ = File::create("kek").ok().unwrap().write_fmt(format_args!("{:?}", decoded));
}
