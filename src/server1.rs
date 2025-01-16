use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address).await.expect("Failed to bind address");
    println!("Server is running on {}", address);

    // Create a broadcast channel for relaying screen data
    let (tx, _) = broadcast::channel::<Vec<u8>>(10);

    loop {
        let (socket, _) = listener.accept().await.expect("Failed to accept connection");

        // Clone the broadcast sender to pass to the connection handler
        let tx_clone = tx.clone();

        // Spawn a task to handle the connection
        tokio::spawn(async move {
            handle_connection(socket, tx_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, tx: broadcast::Sender<Vec<u8>>) {
    println!("Client connected!");

    // Identify the type of client
    let mut buf = [0; 10];
    if socket.read_exact(&mut buf).await.is_err() {
        println!("Failed to identify client type.");
        return;
    }
    let client_type = String::from_utf8_lossy(&buf);

    if client_type.starts_with("SOURCE") {
        println!("Source client connected.");
        let mut frame = vec![0; 1920 * 1080 * 4]; // Assuming 1920x1080 resolution

        // Continuously read screen data from the source and broadcast it
        while let Ok(_) = socket.read_exact(&mut frame).await {
            // Log the size of the received frame
            println!("Received screen data from source: {} bytes", frame.len());

            if tx.send(frame.clone()).is_err() {
                println!("No viewers are connected.");
            }
        }
        println!("Source client disconnected.");
    } else if client_type.starts_with("VIEWER") {
        println!("Viewer client connected.");
        let mut rx = tx.subscribe(); // Subscribe to the broadcast channel

        // Continuously forward screen data to the viewer
        while let Ok(frame) = rx.recv().await {
            // Log the size of the frame being sent to the viewer
            println!("Sending screen data to viewer: {} bytes", frame.len());

            if socket.write_all(&frame).await.is_err() {
                println!("Viewer disconnected.");
                break;
            }
        }
    }
}

