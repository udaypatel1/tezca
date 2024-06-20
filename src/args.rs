use clap::Parser;

#[derive(Parser)]
#[command(name = "tez")]
#[command(version = "0.1.0")]
#[command(about = "Yet another image comparison CLI utility", long_about = None)]
pub struct Cli {
	#[arg(short, long, default_value_t = 75.0, help = "Threshold for image similarity")]
	pub threshold: f64,

	#[arg(help = "First image file to compare")]
	pub file1: String,

	#[arg(help = "Second image file to compare")]
	pub file2: String,
}

pub fn get_args() -> Cli {
	Cli::parse()
}
