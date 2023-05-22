use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DummyResponse {
	msg: String,
	appstatus: String,
}

impl DummyResponse {
	pub fn new(message: &str, status: &str) -> Self {
		let msg = message.to_owned();
		let appstatus = status.to_owned();
		DummyResponse { msg, appstatus }
	}
	pub fn new2(msg: String, appstatus: String) -> Self {
		DummyResponse { msg, appstatus }
	}
}
