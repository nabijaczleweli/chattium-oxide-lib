use time::{at_utc, Tm, Timespec};
use json::{FromJsonnable, ToJsonnable};
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


impl FromJsonnable for Tm {
	// Deserialize via Timespec
	fn from_json(json: Value) -> Result<Self, JsonError> {
		match json {
			Value::Object(map) => {
				let sec =
					match map.get("sec") {
						Some(sec) =>
							match sec {
								&Value::I64(sec) => sec,
								&Value::U64(sec) => sec as i64,  // The types get weird here
								_ => return Err(JsonError::type_mismatch(Type::I64)),
							},
						None => return Err(JsonError::missing_field("Missing \"sec\"")),
					};
				let nsec =
					match map.get("nsec") {
						Some(nsec) =>
							match nsec {
								&Value::I64(nsec) => nsec as i32,
								&Value::U64(nsec) => nsec as i32,
								_ => return Err(JsonError::type_mismatch(Type::I32)),
							},
						None => return Err(JsonError::missing_field("Missing \"nsec\"")),
					};

				Ok(at_utc(Timespec::new(sec, nsec)))
			},
			_ => Err(JsonError::type_mismatch(Type::Struct)),
		}
	}
}

impl ToJsonnable for Tm {
	// Serialize via Timespec
	fn to_json(&self) -> Value {
		let spec = self.to_timespec();
		ObjectBuilder::new().insert("sec", &spec.sec)
		                    .insert("nsec", &spec.nsec)
		                    .unwrap()
	}
}

impl<T: FromJsonnable> FromJsonnable for Vec<T> {
	fn from_json(json: Value) -> Result<Self, JsonError> {
		match json {
			Value::Array(arr) => {
				let mut elems: Vec<T> = Vec::with_capacity(arr.len());
				for elem in arr {
					match T::from_json(elem) {
						Ok(elem) => elems.push(elem),
						Err(e) => return Err(e),
					}
				}
				Ok(elems)
			},
			_ => Err(JsonError::type_mismatch(Type::Seq)),
		}
	}
}

impl<T: ToJsonnable> ToJsonnable for Vec<T> {
	fn to_json(&self) -> Value {
		Value::Array(self.iter().map(|ref elem| elem.to_json()).collect())
	}
}
