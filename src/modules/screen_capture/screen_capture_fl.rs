use image::imageops::FilterType;
use image::{load_from_memory, DynamicImage, ImageBuffer, Luma, Rgb, Rgba, RgbaImage};
use scrap::{Capturer, Display};
use std::io::Cursor;
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

fn save_rgb_image_from_bytes(bytes: Vec<u8>, width: u32, height: u32) {
    // let received_data: Vec<u32> = bytes
    //     .chunks_exact(4)
    //     .map(|chunk| u32::from_ne_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
    //     .collect();

    // let mut buffer = vec![0u32; 1920 * 1080];

    // for i in 0..(1920 * 1080) {
    //     let pixel = received_data[i];
    //     let b = (pixel & 0xFF) as u32;
    //     let g = ((pixel >> 8) & 0xFF) as u32;
    //     let r = ((pixel >> 16) & 0xFF) as u32;
    //     buffer[i] = (r << 16) | (g << 8) | b;
    // }

    let rgb_image: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, bytes)
        .expect("Failed to create RGB image from raw bytes");

    rgb_image
        .save("output_rgb.png")
        .expect("Failed to save RGB image");
}

fn image_compress(bytes: Vec<u8>, width: u32, height: u32) -> Vec<u8> {
    print_image_size(bytes.clone());

    let rgb_img: ImageBuffer<image::Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, bytes)
            .expect("Failed to create image buffer from raw bytes");

    let mut dynamic_img = DynamicImage::ImageRgba8(rgb_img);
    dynamic_img = scale_to_fixed_height(&mut dynamic_img, 720);

    let r = dynamic_img.to_rgba8();
    print_image_size(r.clone().into_raw());
    r.into_raw()
}

fn scale_to_fixed_height(img: &mut DynamicImage, target_height: u32) -> DynamicImage {
    let (orig_width, orig_height) = (img.width() as u32, img.height() as u32);
    let aspect_ratio = orig_width as f32 / orig_height as f32;
    let new_width = (aspect_ratio * target_height as f32) as u32;
    println!("width: {}, height: {}", new_width, target_height);
    img.resize(new_width, target_height, FilterType::Nearest)
}

fn print_image_size(image_bytes: Vec<u8>) {
    let size_in_bytes = image_bytes.len();
    let size_in_kb = size_in_bytes as f64 / 1024.0;

    println!("Image size: {:.2} KB", size_in_kb);
    println!("Image size: {:.2} MB", size_in_kb / 1024.0);
}
