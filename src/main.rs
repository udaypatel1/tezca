mod cli;
mod algorithms;

use std::time::Instant;
use cli::args::get_args;
use algorithms::ssim::{compute_ssim, load_image};

fn main() {
    
    let args = get_args();

    let now = Instant::now();

    let img1 = load_image(&args.file1);
    let img2 = load_image(&args.file2);

    let elapsed = now.elapsed();

    let threshold = args.threshold;

    println!("Both file load time elapsed: {:?}", elapsed);

    let now_process = Instant::now();

    let ssim = compute_ssim(&img1, &img2);

    let process_elapsed = now_process.elapsed();

    println!("Algorithm time: {:?}", process_elapsed);
    println!("Similarity Index: {:?}", ssim);
    println!("Above Threshold [{:1}%]: {:?}", threshold, if (ssim * 100.) >= threshold { true } else { false });
}
