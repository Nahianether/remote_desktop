use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;
use crate::screen_capture::capture_screen;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address).await.expect("Failed to bind address");
    println!("Server is running on {}", address);

    while let Ok((mut socket, _)) = listener.accept().await {
        println!("Client connected!");

        tokio::spawn(async move {
            loop {
                let screen_data = capture_screen(); // Capture the screen
                if let Err(e) = socket.write_all(&screen_data).await {
                    eprintln!("Failed to send frame: {}", e);
                    break;
                }
            }
        });
    }
}
