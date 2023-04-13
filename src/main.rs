use color_eyre::eyre::Result;
use tracing::{info, trace};

mod cli;

async fn yello() -> String {
	String::from("Yello!")
}

#[tokio::main]
async fn main() -> Result<()> {
	color_eyre::install()?;
	let cli = cli::get_cli_args()?;
	trace!("{:#?}", &cli);

	let yo = yello().await;
	info!("{yo:#}");
	Ok(())
}
