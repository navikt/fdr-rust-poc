use std::ops::Deref;

use clap::{Args, CommandFactory, Parser, Subcommand};
use clap_verbosity_flag::{InfoLevel, Verbosity};
use color_eyre::eyre::Result;

#[derive(Debug, Parser)]
pub struct Cli {
	#[clap(flatten)]
	verbosity: clap_verbosity_flag::Verbosity<InfoLevel>,

	#[command(subcommand)]
	command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
	/// Print completion script to stdout for a shell
	Completions {
		/// The shell to generate the completions for
		#[arg(value_enum)]
		shell: clap_complete_command::Shell,
	},

	/// Parameters to assign an Issue or PR to a GH Project's field
	Item(ProgramArgsClap),
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

pub fn get_cli_args() -> Result<ProgramArgs> {
	let cli = Cli::parse();
	match cli.command {
		Commands::Completions { shell } => {
			// A shell completion script has been requested
			shell.generate(&mut Cli::command(), &mut std::io::stdout());
			std::process::exit(0);
		},
		Commands::Item(args) => {
			set_module_log_level(&cli.verbosity);
			Ok(ProgramArgs(args))
		},
	}
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
	tracing_subscriber::fmt()
		.with_max_level(convert_filter(verbosity.log_level_filter()))
		.json()
		.init();
}
