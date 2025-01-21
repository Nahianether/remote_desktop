use scrap::{Capturer, Display};
use std::thread;
use std::time::Duration;

pub fn capture_screen() -> Vec<u8> {
    // Find the primary display
    let display = Display::primary().expect("Failed to find primary display");
    let mut capturer = Capturer::new(display).expect("Failed to start screen capturing");

    loop {
        match capturer.frame() {
            Ok(buffer) => {
                // Successfully captured a screen frame, return it as a Vec<u8>
                return buffer.to_vec();
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Wait for the next frame if the buffer is not ready
                thread::sleep(Duration::from_millis(10));
            }
            Err(e) => {
                // Log unexpected errors and retry
                eprintln!("Failed to capture screen: {:?}", e);
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}
