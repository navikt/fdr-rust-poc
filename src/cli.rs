use std::{io::IsTerminal, ops::Deref};

use clap::{Args, Parser};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::eyre::Result;

use tracing_subscriber::{filter, prelude::*};

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(flatten)]
	verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,
}

#[derive(Debug, Args)]
pub struct ProgramArgsClap {}

#[derive(Debug)]
pub struct ProgramArgs(ProgramArgsClap);
impl Deref for ProgramArgs {
	type Target = ProgramArgsClap;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

pub fn get_cli_args() -> Result<Cli> {
	let cli = Cli::parse();
	set_module_log_level(&cli.verbosity);
	Ok(cli)
}

/// Function to help convert log level from `clap-verbosity-flag` crate to `tracing_subscriber`
const fn convert_filter(filter: log::LevelFilter) -> tracing_subscriber::filter::LevelFilter {
	use tracing_subscriber::filter as trace;
	match filter {
		log::LevelFilter::Off => trace::LevelFilter::OFF,
		log::LevelFilter::Error => trace::LevelFilter::ERROR,
		log::LevelFilter::Warn => trace::LevelFilter::WARN,
		log::LevelFilter::Info => trace::LevelFilter::INFO,
		log::LevelFilter::Debug => trace::LevelFilter::DEBUG,
		log::LevelFilter::Trace => trace::LevelFilter::TRACE,
	}
}

pub fn set_module_log_level<T: clap_verbosity_flag::LogLevel>(verbosity: &Verbosity<T>) {
	use tracing_subscriber::fmt as fmt_layer;
	let (json_logging, plain_logging) = match std::io::stdout().is_terminal() {
		true => (None, Some(fmt_layer::layer().compact())),
		false => (Some(fmt_layer::layer().json()), None),
	};
	tracing_subscriber::Registry::default()
		.with(json_logging)
		.with(plain_logging)
		.with(filter::Targets::new().with_default(convert_filter(verbosity.log_level_filter())))
		.init();
}
