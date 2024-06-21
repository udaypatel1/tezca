use image::{imageops::FilterType, DynamicImage, GenericImageView, GrayImage, Pixel};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{path::Path, sync::mpsc, thread};

pub fn load_image<P: AsRef<Path>>(path: P) -> DynamicImage {

	image::open(path).expect("Failed to load image")
}

fn match_dimensions(image1: &DynamicImage, image2: &DynamicImage) -> (GrayImage, GrayImage) {


	let (width1, height1) = image1.dimensions();
	let (width2, height2) = image2.dimensions();

	if width1 == width2 && height1 == height2 {
		(image1.to_luma8().clone(), image2.to_luma8().clone())
	}
	else if (width1 * height1) > (width2 * height2) {

		let resized_image_1 = image1.resize(width2, height2, FilterType::Nearest);
		(resized_image_1.to_luma8(), image2.to_luma8().clone())
	}
	else {
		let resized_image_2 = image2.resize(width1, height1, FilterType::Nearest);
		(image1.to_luma8().clone(), resized_image_2.to_luma8())
	}
}

fn mean(image: &GrayImage) -> f64 {

	let (width, height) = image.dimensions();
	let pixel_count = (width * height) as f64;

	let sum: f64 = image.par_pixels().map(|pixel| {
		pixel.0[0] as f64
	}).sum();

	sum / pixel_count
}

fn variance(image: &GrayImage, mean: f64) -> f64 {

	let (width, height) = image.dimensions();
	let pixel_count = (width * height) as f64;

	let sum: f64 = image.par_pixels().map(|pixel| {
		let value = pixel.0[0] as f64;
		(value - mean).powi(2)
	}).sum();

	sum / pixel_count
}

fn covariance(image1: &GrayImage, mean1: f64, image2: &GrayImage, mean2: f64) -> f64 {

	let (width, height) = image1.dimensions();
	let pixel_count = (width * height) as f64;
	
	let sum: f64 = image1
					.pixels()
					.zip(image2.pixels())
					.par_bridge()
					.map(|(p1, p2)| {

						let value1 = p1.0[0] as f64;
						let value2: f64 = p2.0[0] as f64;

						(value1 - mean1) * (value2 - mean2)
					})
					.sum();

	sum / pixel_count

}

pub fn compute_ssim(image1: &DynamicImage, image2: &DynamicImage) -> f64 {

	let (image1, image2) = match_dimensions(image1, image2);

	let (mean1, mean2): (f64, f64) = rayon::join(|| mean(&image1), || mean(&image2));
	let (variance1, variance2): (f64, f64) = rayon::join(|| variance(&image1, mean1), || variance(&image2, mean2));

	let covariance = covariance(&image1, mean1, &image2, mean2);

	let k1: f64 = 0.01;
	let k2: f64 = 0.03;

	let c1: f64 = (k1 * 255.0).powi(k1 as i32);
	let c2: f64 = (k2 * 255.0).powi(k2 as i32);

	let ssim = ((2.0 * mean1 * mean2 * c1) * ((2.0 * covariance) + c2)) / ((mean1.powi(2) + mean2.powi(2) + c1) * (variance1 + variance2 + c2));

	ssim
}