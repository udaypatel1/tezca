
use image::{imageops::FilterType, DynamicImage, GenericImageView};
use image_compare::Algorithm;
use std::path::Path;

pub fn load_image<P: AsRef<Path>>(path: P) -> DynamicImage {

	image::open(path).expect("Failed to load image")
}

pub fn match_dimensions(image1: &DynamicImage, image2: &DynamicImage) -> (DynamicImage, DynamicImage) {


	let (width1, height1) = image1.dimensions();
	let (width2, height2) = image2.dimensions();

	if width1 == width2 && height1 == height2 {
		(image1.clone(), image2.clone())
	}
	else if (width1 * height1) > (width2 * height2) {

		let resized_image_1 = image1.resize_exact(width2, height2, FilterType::Nearest);
		(resized_image_1, image2.clone())
	}
	else {
		let resized_image_2 = image2.resize_exact(width1, height1, FilterType::Nearest);
		(image1.clone(), resized_image_2)
	}
}

pub fn compute_ssim(image1: &DynamicImage, image2: &DynamicImage) -> f64 {

	let (image1_resized, image2_resized) = match_dimensions(image1, image2);

	let ssim = image_compare::gray_similarity_structure(&Algorithm::RootMeanSquared, &image1_resized.to_luma8(), &image2_resized.to_luma8()).unwrap().score;

	ssim
}