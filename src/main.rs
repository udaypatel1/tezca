mod cli;
mod algorithms;

use std::time::Instant;
use cli::args::{get_args, Cli};
use algorithms::ssim::{compute_ssim, load_image};

fn main() {
    
    let args = get_args();

    
}
