//! JSON bi-directional serialization.
//!
//! The `FromJsonnable` trait provides an abstract way of handling types, that can be deserialized from JSON
//!
//! The `ToJsonnable` trait provides an abstract way of handling types, that can be serialized to JSON
//!
//!
//! # Examples
//!
//! Consider a user information struct `UserInfo`, that needs to be stored somehow:
//!
//! ```
//! extern crate serde;
//! extern crate serde_json;
//! extern crate chattium_oxide_lib;
//!
//! use serde::de::{Error, Type};
//! use serde_json::value::Value;
//! use serde_json::error::Error as JsonError;
//! use serde_json::builder::ObjectBuilder;
//! use chattium_oxide_lib::json::{ToJsonnable, FromJsonnable};
//!
//! #[derive(Debug, PartialEq)]
//! struct UserInfo {
//! 	name: String,
//! 	id: i64,
//! 	address: (String, String),
//! }
//!
//! impl ToJsonnable for UserInfo {
//! 	fn to_json(&self) -> Value {
//! 		ObjectBuilder::new().insert("name", &self.name)
//! 		                    .insert("id", &self.id)
//! 		                    .insert("address", &self.address)  // Tuples serialize to a JSON array
//! 		                    .unwrap()
//! 	}
//! }
//!
//! impl FromJsonnable for UserInfo {
//! 	// Verbose, but safe, matching algorithm
//! 	fn from_json(json: Value) -> Result<Self, JsonError> {
//! 		match json {
//! 			Value::Object(map) => {
//! 				let name =
//! 					match map.get("name") {
//! 						Some(name) =>
//! 							match name {
//! 								&Value::String(ref name) => name,
//! 								_ => return Err(JsonError::type_mismatch(Type::String)),
//! 							},
//! 						None => return Err(JsonError::missing_field("Missing \"name\"")),
//! 					};
//! 				let id =
//! 					match map.get("id") {
//! 						Some(id) =>
//! 							match id {
//! 								&Value::I64(id) => id,
//! 								&Value::U64(id) => id as i64,
//! 								_ => return Err(JsonError::type_mismatch(Type::I64)),
//! 							},
//! 						None => return Err(JsonError::missing_field("Missing \"id\"")),
//! 					};
//! 				let address =
//! 					match map.get("address") {
//! 						Some(address) =>
//! 							match address {
//! 								&Value::Array(ref address) =>
//! 									match address.len() {
//! 										2 =>
//! 											match (&address[0], &address[1]) {
//! 												(&Value::String(ref laddress), &Value::String(ref raddress)) => (laddress.clone(), raddress.clone()),
//! 												_ => return Err(JsonError::type_mismatch(Type::String)),
//! 											},
//! 										_ => return Err(JsonError::length_mismatch(2)),
//! 									},
//! 								_ => return Err(JsonError::type_mismatch(Type::String)),
//! 							},
//! 						None => return Err(JsonError::missing_field("Missing \"address\"")),
//! 					};
//!
//! 				Ok(UserInfo{
//! 					name: name.clone(),
//! 					id: id,
//! 					address: address,//(address.0.clone(), address.1.clone()),
//! 				})
//! 			},
//! 			_ => Err(JsonError::type_mismatch(Type::Struct)),
//! 		}
//! 	}
//! }
//!
//! fn main() {
//! 	let original = UserInfo{
//! 		name: "user".to_owned(),
//! 		id: 50030,
//! 		address: ("Diagon Alley".to_owned(), "London, UK".to_owned()),
//! 	};
//! 	let serialized = original.to_json_string().unwrap();
//! 	println!("{}", serialized);  // Space-efficient "ugly" format
//! 	let deserialized = UserInfo::from_json_string(&serialized).unwrap();
//! 	assert_eq!(original, deserialized);
//! }
//! ```

mod implementation;
pub use self::implementation::*;

use serde_json;
use serde_json::value::Value;
use serde_json::error::Error as JsonError;


/// A trait for types supporting deserialization from JSON
pub trait FromJsonnable: Sized {
	/// Deserialize a JSON value to `Self`
	///
	/// Returns `Err()` if the Value cannot be deserialized into `Self`
	fn from_json(json: Value) -> Result<Self, JsonError>;

	/// Convenience function for deserializing a JSON string representation directly into `Self`
	fn from_json_string(string: &String) -> Result<Self, JsonError> {
		let value: Value = try!(serde_json::from_str(&*&string));
		Self::from_json(value)
	}
}

/// A trait for types supporting serialization to JSON
pub trait ToJsonnable: Sized {
	/// Serialize `self` to a JSON `Value`.
	///
	/// If `Self` also implements [`FromJsonnable`](trait.FromJsonnable.html), it's highly recommended,
	/// that `self.from_json(self.to_json())` never returns `Err()`.
	fn to_json(&self) -> Value;

	/// Convenience function for converting `self` to a JSON ugly-string representation.
	///
	/// Returns `Err()` if `serde_json` couldn't convert the Value to a String
	fn to_json_string(&self) -> Result<String, JsonError> {
		serde_json::to_string(&self.to_json())
	}
}
