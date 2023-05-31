use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct DummyResponse {
	msg: String,
	appstatus: String,
}

impl DummyResponse {
	pub fn new(msg: String, appstatus: String) -> Self {
		DummyResponse { msg, appstatus }
	}
}
