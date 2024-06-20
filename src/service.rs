use image::{DynamicImage, GenericImageView, Pixel};
use std::{path::Path, sync::mpsc, thread, time::Duration};

pub fn load_image<P: AsRef<Path>>(path: P) -> DynamicImage {

	image::open(path).expect("Failed to load image")
}

fn mean(image: &DynamicImage) -> f64 {

	let (width, height) = image.dimensions();
	let pixel_count = (width * height) as f64;
	let mut sum = 0.0;

	for pixel in image.pixels() {

		sum += pixel.2.to_luma().0[0] as f64;
	}

	sum / pixel_count
}

fn variance(image: &DynamicImage, mean: f64) -> f64 {

	let (width, height) = image.dimensions();
	let pixel_count = (width * height) as f64;
	let mut sum = 0.0;

	for pixel in image.pixels() {

		let value = pixel.2.to_luma().0[0] as f64;
		sum += (value - mean).powi(2);
	}

	sum / pixel_count
}

fn covariance(image1: &DynamicImage, mean1: f64, image2: &DynamicImage, mean2: f64) -> f64 {

	let (width, height) = image1.dimensions();
	let pixel_count = (width * height) as f64;
	let mut sum = 0.0;

	for (p1, p2) in image1.pixels().zip(image2.pixels()) {
		let value1 = p1.2.to_luma().0[0] as f64;
		let value2 = p2.2.to_luma().0[0] as f64;

		sum += (value1 - mean1) * (value2 - mean2);
	}

	sum / pixel_count

}

pub fn compute_ssim(image1: &DynamicImage, image2: &DynamicImage) -> f64 {

	let (img1_clone1, img1_clone2) = (image1.clone(), image1.clone());
	let (img2_clone1, img2_clone2) = (image2.clone(), image2.clone());

	let (mean_var_tx1, mean_var_rx1) = mpsc::channel();
	let(mean_var_tx2, mean_var_rx2) = mpsc::channel();

	let handle_image1 = thread::spawn(move || {
		
		let mean1 = mean(&img1_clone1);
		let variance1 = variance(&img1_clone2, mean1);

		let _ = mean_var_tx1.send((mean1, variance1));
	});

	let handle_image2 = thread::spawn(move || {
		
		let mean2 = mean(&img2_clone1);
		let variance2 = variance(&img2_clone2, mean2);

		let _ = mean_var_tx2.send((mean2, variance2));
	});

	let (mean1, variance1) = mean_var_rx1.recv().unwrap();
	let (mean2, variance2) = mean_var_rx2.recv().unwrap();

	handle_image1.join().unwrap();
	handle_image2.join().unwrap();

	let covariance = covariance(image1, mean1, image2, mean2);

	let K1: f64 = 0.01;
	let K2: f64 = 0.03;

	let C1: f64 = (K1 * 255.0).powi(K1 as i32);
	let C2: f64 = (K2 * 255.0).powi(K2 as i32);

	let ssim = ((2.0 * mean1 * mean2 * C1) * (2.0 * covariance + C2)) / ((mean1.powi(2) + mean2.powi(2) + C1) * (variance1 + variance2 + C2));

	ssim
}