use image::DynamicImage;

pub fn new(width: u32, height: u32) -> DynamicImage {
	let image = DynamicImage::new_rgb8(width, height);



	image
}





#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let image = super::new(1024, 1024);
		let result = image.save("testimage.png");
		assert!(result.is_ok(), result);
		
		// match image.save("/usr/testimage.png") {
		//     Ok(_) => (),
		//     Err(e) => { println!("Error! {}", e); },
		// }
	}
}