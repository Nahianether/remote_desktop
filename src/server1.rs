use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};

type Clients = Arc<Mutex<HashMap<String, mpsc::Sender<mpsc::Sender<Vec<u8>>>>>>;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");
    println!("Server is running on {}", address);

    // Use tokio::sync::Mutex instead of std::sync::Mutex
    let clients: Clients = Arc::new(Mutex::new(HashMap::new())); // Shared clients registry

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let clients_clone = clients.clone();

        // Spawn a task to handle the connection
        tokio::spawn(async move {
            handle_connection(socket, clients_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, clients: Clients) {
    println!("Client connected!");

    // Buffer to identify the type of client
    let mut buf = vec![0; 10];
    if socket.read_exact(&mut buf[..6]).await.is_err() {
        println!("Failed to identify client type.");
        return;
    }

    let client_type = String::from_utf8_lossy(&buf[..6]).trim().to_string();

    if client_type == "SOURCE" {
        handle_source_client(socket, clients).await; // Handle a source client
    } else if client_type == "VIEWER" {
        handle_viewer_client(socket, clients).await; // Handle a viewer client
    } else {
        println!("Unknown client type: {}", client_type);
    }
}

async fn handle_source_client(mut socket: TcpStream, clients: Clients) {
    // Source client sends its user ID
    let mut buf = vec![0; 64];
    if socket.read_exact(&mut buf[..64]).await.is_err() {
        println!("Failed to receive user ID from source client.");
        return;
    }
    let user_id = String::from_utf8_lossy(&buf[..64]).trim().to_string();
    println!("Source client connected with user ID: {}", user_id);

    // Create a channel for the source client
    let (tx, mut rx) = mpsc::channel::<mpsc::Sender<Vec<u8>>>(10);

    // Register the source client in the shared hashmap
    let mut clients_guard = clients.lock().await; // `lock()` does not return a `Result`
    clients_guard.insert(user_id.clone(), tx);

    // Handle incoming screen data from the source client
    let mut frame = vec![0; 1920 * 1080 * 4]; // Assuming 1920x1080 resolution
    while let Ok(_) = socket.read_exact(&mut frame).await {
        // If there's a viewer waiting for the source's screen, forward the frame
        if let Some(viewer_tx) = rx.recv().await {
            if viewer_tx.send(frame.clone()).await.is_err() {
                println!("Viewer disconnected while receiving screen data.");
            }
        }
    }

    println!("Source client disconnected: {}", user_id);

    // Remove source client from registry
    let mut clients_guard = clients.lock().await;
    clients_guard.remove(&user_id);
}

async fn handle_viewer_client(mut socket: TcpStream, clients: Clients) {
    // Viewer client sends the user ID of the source client they want to view
    let mut buf = vec![0; 64];
    if socket.read_exact(&mut buf[..64]).await.is_err() {
        println!("Failed to receive user ID from viewer client.");
        return;
    }
    let user_id = String::from_utf8_lossy(&buf[..64]).trim().to_string();
    println!("Viewer client requested user ID: {}", user_id);

    // Look up the source client in the registry
    let clients_guard = clients.lock().await; // `lock()` does not return a `Result`
    if let Some(source_tx) = clients_guard.get(&user_id) {
        println!("Found source client with user ID: {}", user_id);

        // Create a channel for the viewer
        let (viewer_tx, mut viewer_rx) = mpsc::channel::<Vec<u8>>(10);

        // Send the viewer's sender to the source client
        if source_tx.send(viewer_tx).await.is_err() {
            println!("Source client disconnected: {}", user_id);
            return;
        }

        // Forward frames from the source client to the viewer
        while let Some(frame) = viewer_rx.recv().await {
            if socket.write_all(&frame).await.is_err() {
                println!("Viewer disconnected.");
                break;
            }
        }
    } else {
        println!("No source client found with user ID: {}", user_id);
    }
}
