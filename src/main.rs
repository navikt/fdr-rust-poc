use color_eyre::eyre::Result;
use rocket::{get, routes};
use tracing::{debug, info, trace};

mod cli;

#[get("/")]
async fn root_index() -> &'static str {
	"Hello, world!\n"
}

#[get("/greet/<subject>")]
async fn greet(subject: &str) -> String {
	format!("Hello, {subject}!\n")
}

#[rocket::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	let cli = cli::get_cli_args()?;
	trace!("{:#?}", &cli);

	debug!("Starting up rocket...");
	let rocket_result = rocket::build()
		.mount("/", routes![root_index, greet])
		.launch()
		.await?;
	info!("Rocket has finished with result: {rocket_result:#?}");
	Ok(())
}
