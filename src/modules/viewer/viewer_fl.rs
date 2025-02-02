use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub async fn start_viewer_client(address: &str, target_user_id: &str) {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("Failed to connect to server");
    println!("Connected to server as viewer client.");

    // Send client type and target user ID to the server
    stream
        .write_all(b"VIEWER\n")
        .await
        .expect("Failed to send client type");
    let target_user_id_bytes = format!("{:<64}", target_user_id).into_bytes(); // Pad to 64 bytes
    stream
        .write_all(&target_user_id_bytes)
        .await
        .expect("Failed to send target user ID");

    // Create a minifb (minifb crates for view) window to display the screen
    let mut window = minifb::Window::new(
        "Remote Desktop Viewer",
        1920,
        1080,
        minifb::WindowOptions::default(),
    )
    .expect("Failed to create window");

    window.limit_update_rate(Some(std::time::Duration::from_micros(16600))); // ~60 FPS limit (error occured or buffer drop if decrease from 60)

    let mut buffer = vec![0u32; 1920 * 1080];
    let mut received_data = vec![0; 1920 * 1080 * 4];

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if let Ok(_) = stream.read_exact(&mut received_data).await {
            // Convert BGRA to RGB and display it
            for (i, chunk) in received_data.chunks(4).enumerate() {
                let b = chunk[0] as u32;
                let g = chunk[1] as u32;
                let r = chunk[2] as u32;
                buffer[i] = (r << 16) | (g << 8) | b;
            }
            window
                .update_with_buffer(&buffer, 1920, 1080)
                .expect("Failed to update window buffer");
        } else {
            println!("Failed to receive frame.");
            break;
        }
    }
}
