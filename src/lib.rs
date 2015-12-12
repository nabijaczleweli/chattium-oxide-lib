//! `chattium-oxide-client` and `chattium-oxide-server` common files
//!
//! chattium-oxide-client: https://github.com/nabijaczleweli/chattium-oxide-client
//! chattium-oxide-server: https://github.com/nabijaczleweli/chattium-oxide-server

extern crate time;
extern crate serde;
extern crate serde_json;

mod user;
mod message;
pub mod json;

pub use self::user::*;
pub use self::message::*;
