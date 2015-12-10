use serde_json::value::Value;
use serde_json::error::Error as JsonError;


pub trait FromJsonnable: Sized {
	fn from_json(json: Value) -> Result<Self, JsonError>;
}

pub trait ToJsonnable: Sized {
	fn to_json(&self) -> Value;
}
