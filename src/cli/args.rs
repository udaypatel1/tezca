
use clap::{Args, Parser, Subcommand};

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
enum Commands {

	List(ListArgs),
	Algorithm(Algorithm),
}

#[derive(Parser)]
struct Algorithm {
	#[clap(subcommand)]
	algorithm: AlgorithmType,
}

#[derive(Subcommand)]
enum AlgorithmType {
	Ssim(SsimArgs),
}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct SsimArgs {

	#[arg(help = "First image file to compare")]
	pub file1: String,

	#[arg(help = "Second image file to compare")]
	pub file2: String,
}

pub fn get_args() -> Cli {
	Cli::parse()
}
