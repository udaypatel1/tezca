mod args;
mod service;

use args::get_args;
use service::{load_image, compute_ssim};

fn main() {
    
    let args = get_args();

    let img1 = load_image(&args.file1);
    let img2 = load_image(&args.file2);

    let ssim = compute_ssim(&img1, &img2);

    println!("Similarity Index: {:?}", ssim);
}
