use color_eyre::eyre::Result;
use rocket::serde::json::serde_json;
use rocket_db_pools::sqlx::{database::HasValueRef, Database, Decode};
use std::fmt;
use tracing::error;

use super::Person;
// use super::Identities; // TODO: Comment in when in use

#[derive(Debug, PartialEq, Eq)]
struct ParsePersonError;

impl fmt::Display for Person {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let json_string = match serde_json::to_string(self) {
			Ok(json) => json,
			Err(e) => {
				error!("Unable to deserialize person w/id {}, error: {}", self, e);
				String::new()
			},
		};
		write!(f, "{}", json_string)
	}
}

// `'r` is the lifetime of the `Row` being decoded
impl<'row, DB: Database> Decode<'row, DB> for Person
where
	// we want to delegate some of the work to string decoding so let's make sure strings
	// are supported by the database
	&'row str: Decode<'row, DB>,
{
	fn decode(
		value: <DB as HasValueRef<'row>>::ValueRef,
	) -> Result<Person, Box<dyn std::error::Error + 'static + Send + Sync>> {
		// the interface of ValueRef is largely unstable at the moment
		// so this is not directly implementable

		// however, you can delegate to a type that matches the format of the type you want
		// to decode (such as a UTF-8 string)

		let value = <&str as Decode<DB>>::decode(value)?;

		// now you can parse this into your type (assuming there is a `FromStr`)

		Ok(serde_json::from_str(value)?)
	}
}

// TODO: Comment back in when in use
// #[derive(Debug, PartialEq, Eq)]
// struct ParseIdentitiesError;

// impl fmt::Display for Identities {
// 	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
// 		let json_string = match serde_json::to_string(self) {
// 			Ok(json) => json,
// 			Err(e) => {
// 				error!(
// 					"Unable to deserialize identity w/id {}, error: {}",
// 					self.id, e
// 				);
// 				String::new()
// 			},
// 		};
// 		write!(f, "{}", json_string)
// 	}
// }

// // `'r` is the lifetime of the `Row` being decoded
// impl<'row, DB: Database, Identities> Decode<'row, DB> for Identities
// where
// 	// we want to delegate some of the work to string decoding so let's make sure strings
// 	// are supported by the database
// 	&'row str: Decode<'row, DB>,
// {
// 	fn decode(
// 		value: <DB as HasValueRef<'row>>::ValueRef,
// 	) -> Result<Identities, Box<dyn std::error::Error + 'static + Send + Sync>> {
// 		// the interface of ValueRef is largely unstable at the moment
// 		// so this is not directly implementable

// 		// however, you can delegate to a type that matches the format of the type you want
// 		// to decode (such as a UTF-8 string)

// 		let value = <&str as Decode<DB>>::decode(value)?;

// 		// now you can parse this into your type (assuming there is a `FromStr`)

// 		Ok(serde_json::from_str(value)?)
// 	}
// }
