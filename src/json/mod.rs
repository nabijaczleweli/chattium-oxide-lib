mod implementation;
pub use self::implementation::*;

use serde_json;
use serde_json::value::Value;
use serde_json::error::Error as JsonError;


pub trait FromJsonnable: Sized {
	fn from_json(json: Value) -> Result<Self, JsonError>;

	fn from_json_string(string: &String) -> Result<Self, JsonError> {
		let value: Value = try!(serde_json::from_str(&*&string));
		Self::from_json(value)
	}
}

pub trait ToJsonnable: Sized {
	fn to_json(&self) -> Value;

	fn to_json_string(&self) -> Result<String, JsonError> {
		serde_json::to_string(&self.to_json())
	}
}
