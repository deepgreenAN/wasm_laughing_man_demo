//use image::{DynamicImage, ImageBuffer, Rgba};
use rustface::{Detector, FaceInfo, ImageData};

pub fn detect_faces(detector: &mut dyn Detector, gray_vec:&Vec<u8>, width: u32, height: u32) -> Vec<FaceInfo> {
    let mut image = ImageData::new(gray_vec, width, height);
    let faces = detector.detect(&mut image);
    faces
}

// pub fn convert_rgba_to_luma_v1(rgba_vec:Vec<u8>, width: u32, height: u32) -> Vec<u8> {
//     let image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_vec(width, height, rgba_vec).unwrap();
//     DynamicImage::ImageRgba8(image_buffer).to_luma8().into_vec()
// }

pub fn convert_rgba_to_luma_v2(rgba_vec:Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    let mut out_vec: Vec<u8> = vec![0;(width*height) as usize];
    for i in 0..rgba_vec.len() {
        if i % 4 == 0 {  // rのインデックス
            out_vec[i / 4] = (0.299 * (rgba_vec[i] as f64) + 0.587 * (rgba_vec[i+1] as f64) + 0.114*(rgba_vec[i+2] as f64)) as u8;
        }
    }
    out_vec
}