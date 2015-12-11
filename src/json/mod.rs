mod implementation;
pub use self::implementation::*;

use serde_json;
use serde_json::value::Value;
use serde_json::error::Error as JsonError;


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

pub trait ToJsonnable: Sized {
	/// Serialize `self` to a JSON `Value`.
	///
	/// If `Self` also implements [`FromJsonnable`](trait.FromJsonnable.html), it's highly recommended,
	/// that `self.from_json(self.to_json())` never returns `Err()`.
	fn to_json(&self) -> Value;

	/// Convenience function for converting `self` to a JSON ugly-string representation.
	fn to_json_string(&self) -> Result<String, JsonError> {
		serde_json::to_string(&self.to_json())
	}
}
