use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use minifb::{Window, WindowOptions};

const WIDTH: usize = 1920;  // Screen width
const HEIGHT: usize = 1080; // Screen height

pub async fn start_viewer_client(address: &str, user_id: &str) {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("Failed to connect to server");
    println!("Connected to server as viewer client.");

    // Send client type to the server
    stream.write_all(b"VIEWER\n").await.expect("Failed to send client type");

    // Send the user ID of the source client to the server
    let user_id_bytes = format!("{:<64}", user_id).into_bytes(); // Pad to 64 bytes
    stream.write_all(&user_id_bytes).await.expect("Failed to send user ID");

    // Create a window for displaying the screen
    let mut window = Window::new(
        "Remote Desktop Viewer",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS

    // Buffers for received data
    let mut buffer = vec![0u32; WIDTH * HEIGHT];
    let mut received_data = vec![0; WIDTH * HEIGHT * 4]; // Raw BGRA data

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        // Read screen data from the server
        match stream.read_exact(&mut received_data).await {
            Ok(_) => {
                // Convert BGRA to RGB and display it
                for (i, chunk) in received_data.chunks(4).enumerate() {
                    let b = chunk[0] as u32;
                    let g = chunk[1] as u32;
                    let r = chunk[2] as u32;
                    buffer[i] = (r << 16) | (g << 8) | b; // Convert BGRA to RGB
                }
                window
                    .update_with_buffer(&buffer, WIDTH, HEIGHT)
                    .expect("Failed to update buffer");
            }
            Err(e) => {
                println!("Failed to read frame: {}", e);
                break;
            }
        }
    }
}
