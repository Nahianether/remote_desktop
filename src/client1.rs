use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::screen_capture1::capture_screen;

pub async fn start_source_client(address: &str) {
    let mut stream = TcpStream::connect(address)
        .await
        .expect("Failed to connect to server");
    println!("Connected to server as source client.");

    // Send client type to the server
    stream.write_all(b"SOURCE\n").await.expect("Failed to send client type");

    loop {
        let screen_data = capture_screen(); // Capture the screen

        // Log the size of the screen data
        println!("Captured screen data of size: {}", screen_data.len());

        if stream.write_all(&screen_data).await.is_err() {
            println!("Failed to send screen data to server.");
            break;
        }
    }
}

