use color_eyre::eyre::Result;
use rocket::{get, routes, serde::json::Json};
use tracing::{debug, info, trace};

mod cli;
mod db;
mod dtos;
mod person;

#[get("/")]
async fn root_index() -> &'static str {
	"Hello, world!\n"
}

#[get("/greet/<subject>")]
async fn greet(subject: &str) -> String {
	format!("Hello, {subject}!\n")
}

#[get("/insult/<subject>")]
async fn offend(subject: &str) -> String {
	format!("Fuck off, {subject}\n")
}

#[get("/insult2/<subject>")]
async fn offend2(subject: &str) -> Json<dtos::DummyResponse> {
	// let out = dtos::DummyResponse::new("my messsage", "my status");
	let out = dtos::DummyResponse::new2(subject.to_owned(), String::from("my status"));
	Json(out)
}

#[rocket::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	let cli = cli::get_cli_args()?;
	trace!("{:#?}", &cli);

	debug!("Starting up rocket...");
	let rocket_result = rocket::build()
		.attach(db::stage()) // Ensure db migrations are run and verified
		.attach(person::stage()) // "api/person"
		.mount("/", routes![root_index, greet, offend, offend2])
		// .mount("/v1/api/", person::stage()) // <- TODO: Figure out how to nest like this!
		.launch()
		.await?;
	info!("Rocket has finished with result: {rocket_result:#?}");
	Ok(())
}
