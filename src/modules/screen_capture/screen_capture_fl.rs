use image::{DynamicImage, ImageBuffer};
use scrap::{Capturer, Display};
use std::thread;
use std::time::{Duration, Instant};

use crate::helpers::constraint::constraint::FPS_LIMIT;
pub fn capture_screen() -> Vec<u8> {
    // Only send data of primary display
    let display = Display::primary().expect("Failed to find primary display");
    let (width, height): (u32, u32) = (display.width() as u32, display.height() as u32);
    let mut capturer = Capturer::new(display).expect("Failed to start screen capturing");

    let frame_time = Duration::from_secs_f64(1.0 / FPS_LIMIT as f64);

    loop {
        let start_time = Instant::now();
        match capturer.frame() {
            Ok(buffer) => {
                println!("Captured screen");
                // print_image_size(&buffer.to_vec());
                let buffer = image_compress(buffer.to_vec(), width, height);
                // save_rgb_image_from_bytes(buffer, width, height);
                // handle_admin_binary_events(&mut window, Bytes::from(buffer)).unwrap();
                // return buffer.to_vec();
                return buffer;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Failed to capture screen: {:?}", e);
                thread::sleep(Duration::from_millis(100));
            }
        }

        let elapsed = start_time.elapsed();
        if elapsed < frame_time {
            thread::sleep(frame_time - elapsed);
        }
    }
}

fn print_image_size(pixel_data: &Vec<u8>) {
    let image_size_bytes = pixel_data.len();
    let image_size_kb = image_size_bytes as f64 / 1024.0;
    let image_size_mb = image_size_kb / 1024.0;

    println!("Vec<u8> size: {} bytes", image_size_bytes);
    println!("Vec<u8> size: {:.2} KB", image_size_kb);
    println!("Vec<u8> size: {:.2} MB", image_size_mb);
}

fn save_rgb_image_from_bytes(bytes: Vec<u8>, width: u32, height: u32) {
    if bytes.len() != (width * height * 3) as usize {
        panic!("Byte vector size does not match expected size: width * height * 3");
    }

    let rgb_img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, bytes)
        .expect("Failed to create image buffer from raw bytes");

    // Convert the ImageBuffer to a DynamicImage
    let dynamic_img = DynamicImage::ImageRgb8(rgb_img);
    dynamic_img
        .save("output_2.png")
        .expect("Failed to save image");
}

fn image_compress(bytes: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    // Create an ImageBuffer from the raw RGB data
    let rgb_img: ImageBuffer<image::Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, bytes)
        .expect("Failed to create image buffer from raw bytes");

    // Convert the ImageBuffer to a DynamicImage
    let dynamic_img = DynamicImage::ImageRgb8(rgb_img);

    // Convert the DynamicImage to grayscale (Luma8)
    let gray_img = dynamic_img.to_luma8();

    // Get raw bytes of the grayscale image
    let gray_bytes = gray_img.into_raw();

    gray_bytes
}
