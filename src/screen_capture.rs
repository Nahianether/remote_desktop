use scrap::{Capturer, Display};
use std::thread;
use std::time::Duration;

pub fn capture_screen() -> Vec<u8> {
    // Get the primary display
    let display = Display::primary().expect("Failed to find primary display");
    let mut capturer = Capturer::new(display).expect("Failed to start screen capturing");

    // Capture the screen
    let frame = loop {
        match capturer.frame() {
            Ok(buffer) => break buffer.to_vec(),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // Retry capturing
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(e) => panic!("Failed to capture screen: {:?}", e),
        }
    };

    frame
}
