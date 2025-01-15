use tokio::net::TcpStream;
use tokio::io::AsyncReadExt; // Import this for async read operations
use minifb::{Window, WindowOptions};

const WIDTH: usize = 1920;  // Set the screen width
const HEIGHT: usize = 1080; // Set the screen height

pub async fn start_client(address: &str) {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("Failed to connect to the server");
    println!("Connected to server at {}", address);

    // Create a minifb window
    let mut window = Window::new(
        "Remote Desktop Viewer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    // Ensure the window stays open
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

    // Buffer to store the raw screen data (RGBA)
    let mut buffer = vec![0u32; WIDTH * HEIGHT]; // u32 buffer for BGRA
    let mut received_data = vec![0; WIDTH * HEIGHT * 4]; // Raw BGRA buffer

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Read frame data from the server
        match stream.read_exact(&mut received_data).await {
            Ok(_) => {
                // Convert the raw BGRA data into the u32 buffer for minifb
                for (i, chunk) in received_data.chunks(4).enumerate() {
                    let b = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let r = chunk[2] as u32;
                    buffer[i] = (r << 16) | (g << 8) | b; // Convert BGRA to RGB
                }

                // Update the window with the new frame
                window
                    .update_with_buffer(&buffer, WIDTH, HEIGHT)
                    .expect("Failed to update window buffer");
            }
            Err(e) => {
                eprintln!("Failed to read frame: {}", e);
                break;
            }
        }
    }
}
