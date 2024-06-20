mod args;
mod service;

use std::time::{self, Duration, Instant};

use args::get_args;
use service::{load_image, compute_ssim};

fn main() {
    
    let args = get_args();

    let now = Instant::now();

    let img1 = load_image(&args.file1);
    let img2 = load_image(&args.file2);

    let elapsed = now.elapsed();
    println!("Both file load time elapsed: {:?}", elapsed);

    let nowProcess = Instant::now();

    let ssim = compute_ssim(&img1, &img2);

    let processElapsed = nowProcess.elapsed();

    println!("Algorithm time: {:?}", processElapsed);
    println!("Similarity Index: {:?}", ssim);
}
