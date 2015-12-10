use time::{at_utc, Tm, Timespec};
use json::{FromJsonnable, ToJsonnable};
use serde::de::{Error, Type};
use serde_json::value::Value;
use serde_json::error::Error as JsonError;
use serde_json::builder::ObjectBuilder;


impl FromJsonnable for Tm {
	fn from_json(json: Value) -> Result<Self, JsonError> {
		match json {
			Value::Object(map) => {
				let sec =
					match map.get("sec") {
						Some(_sec) =>
							match _sec {
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
	fn to_json(&self) -> Value {
		let spec = self.to_timespec();
		ObjectBuilder::new().insert("sec", &spec.sec)
		                    .insert("nsec", &spec.nsec)
		                    .unwrap()
	}
}
