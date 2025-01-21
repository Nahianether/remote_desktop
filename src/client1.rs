use crate::screen_capture1::capture_screen;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn start_source_client(address: &str, user_id: &str) {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("Failed to connect to server");
    println!("Connected to server as source client.");

    // Send client type and user ID to the server
    stream
        .write_all(b"SOURCE\n")
        .await
        .expect("Failed to send client type");
    let user_id_bytes = format!("{:<64}", user_id).into_bytes(); // Pad to 64 bytes
    stream
        .write_all(&user_id_bytes)
        .await
        .expect("Failed to send user ID");

    loop {
        let screen_data = capture_screen(); // Capture the screen
        if stream.write_all(&screen_data).await.is_err() {
            println!("Failed to send screen data to server.");
            break;
        }
    }
}
