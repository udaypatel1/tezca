
use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "tez")]
#[command(version = "0.1.0")]
#[command(about = "Yet another image comparison CLI utility", long_about = None)]
#[command(author = "Uday Patel")]
pub struct Cli {

	#[clap(subcommand)]
	pub subcommand: Option<Commands>,

}

#[derive(Subcommand)]
#[command(about = "Subcommand Options", long_about = None)]
pub enum Commands {

	Ssim(SsimArgs),
}

#[derive(Args)]
pub struct SsimArgs {

	#[clap(long, short, action = ArgAction::SetFalse, help = "Force match the dimensions of both images")]
	pub force: Option<bool>,

	#[arg(help = "First image file to compare")]
	pub file1: String,

	#[arg(help = "Second image file to compare")]
	pub file2: String,
}

pub fn get_args() -> Cli {
	Cli::parse()
}
