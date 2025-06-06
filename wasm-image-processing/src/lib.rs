mod utils;

use image::imageops;
use image::ImageBuffer;
use image::RgbaImage;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn shrink_by_half(original_image: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let image: RgbaImage = ImageBuffer::from_vec(width, height, original_image).unwrap();

    let output_image =
        imageops::resize(&image, width / 2, height / 2, imageops::FilterType::Nearest);

    output_image.into_vec()
}
