use core::fmt;
use std::collections::HashMap;

use rocket::{get, http::Status, post, routes, serde::json::Json};
use rocket_db_pools::{sqlx::types::Uuid, Connection};
use tracing::{error, info};

mod struct_utils;

use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Person {
	id: Uuid,
	// created_at: time::OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
#[serde(crate = "rocket::serde", try_from = "String")]
pub enum IdentityType {
	PassportId,
	SocialSecurityNumber,
}

impl TryFrom<String> for IdentityType {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl fmt::Display for IdentityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Identities {
	id: Uuid,
	id_type: IdentityType,
	id_value: String,
	person: Person,
}


#[get("/person/<id>")]
async fn get(
	id: rocket::serde::uuid::Uuid,
	mut db_connection: Connection<super::db::Db>,
) -> Option<Json<Person>> {
	match sqlx::query_as!(
		Person,
		r#"
			SELECT
				*
			FROM person
			WHERE id = $1
		"#,
		id,
	)
	.fetch_optional(&mut *db_connection)
	.await
	{
		Ok(person) => person.map(Json),
		Err(e) => {
			error!("Error when fetching user w/id '{}': {}", id, e);
			None
		},
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct PersonReq{
	identities: HashMap<IdentityType,String>
}


#[post("/person", data = "<req>")]
fn new_person(req: Json<PersonReq<>>) { 
	info!("debug test {:#?}", req.identities);
}

#[post("/person/new")]
async fn new(
	mut db_connection: Connection<super::db::Db>,
) -> Result<Json<Person>, (Status, String)> {
	let new_person_uuid = Uuid::now_v7();
	match sqlx::query_as!(
		Person,
		"INSERT INTO person (id) VALUES ($1) RETURNING id",
		new_person_uuid,
	)
	.fetch_one(&mut **db_connection)
	.await
	{
		Ok(p) => Ok(Json(p)),
		Err(e) => {
			let error_msg = format!("Unable to generate random new person: {}", e);
			error!(error_msg);
			Err((Status::InternalServerError, error_msg))
		},
	}
}

#[get("/person")]
async fn list(mut db_connection: Connection<super::db::Db>) -> Json<Vec<Person>> {
	match sqlx::query_as!(Person, "SELECT * FROM person")
		.fetch_all(&mut *db_connection)
		.await
	{
		Ok(persons) => Json(persons),
		Err(e) => {
			error!("Error when fetching all users: {}", e);
			Json(Vec::new())
		},
	}
}

pub fn stage() -> rocket::fairing::AdHoc {
	rocket::fairing::AdHoc::on_ignite("Person", |rocket| async {
		rocket.mount("/", routes![get, list, new, new_person])
	})
}
