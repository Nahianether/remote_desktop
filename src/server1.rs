use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::{mpsc, Mutex};

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<Vec<u8>>>>>;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");
    println!("Server is running on {}", address);

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        let clients_clone = clients.clone();

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
        // Clone `clients` before passing it
        handle_source_client(socket, clients.clone()).await; // Handle a source client
    } else if client_type == "VIEWER" {
        // Clone `clients` before passing it
        handle_viewer_client(socket, clients.clone()).await; // Handle a viewer client
    } else {
        println!("Unknown client type: {}", client_type);
    }
}

async fn handle_source_client(mut socket: TcpStream, clients: Clients) {
    let mut buf = vec![0; 64];
    if socket.read_exact(&mut buf[..64]).await.is_err() {
        println!("Failed to receive user ID from source client.");
        return;
    }
    let user_id = String::from_utf8_lossy(&buf[..64]).trim().to_string();
    println!("Source client connected with user ID: {}", user_id);

    let (tx, _rx) = tokio::sync::broadcast::channel::<Vec<u8>>(10);

    // Register the source client in the shared hashmap
    let mut clients_guard = clients.lock().await;
    clients_guard.insert(user_id.clone(), tx);

    // Handle incoming screen data from the source client
    let mut frame = vec![0; 1920 * 1080 * 4]; // Assuming 1920x1080 resolution
    while let Ok(_) = socket.read_exact(&mut frame).await {
        println!("Received screen frame from source: {} bytes", frame.len());

        // Forward frames to viewers
        let clients_guard = clients.lock().await;
        for viewer_tx in clients_guard.values() {
            if viewer_tx.send(frame.clone()).is_err() {
                println!("Viewer disconnected while receiving screen data.");
            }
        }
    }

    println!("Source client disconnected: {}", user_id);

    // Remove the source client from the registry
    clients_guard.remove(&user_id);
}

async fn handle_viewer_client(mut socket: TcpStream, clients: Clients) {
    let mut buf = vec![0; 64];
    if socket.read_exact(&mut buf[..64]).await.is_err() {
        println!("Failed to receive user ID from viewer client.");
        return;
    }
    let user_id = String::from_utf8_lossy(&buf[..64]).trim().to_string();
    println!("Viewer client requested user ID: {}", user_id);

    let clients_guard = clients.lock().await;
    if let Some(source_tx) = clients_guard.get(&user_id) {
        println!("Found source client with user ID: {}", user_id);

        let mut source_rx = source_tx.subscribe();
        // Continuously forward frames from the source to the viewer
        while let Ok(frame) = source_rx.recv().await {
            println!("Forwarding frame to viewer: {} bytes", frame.len());
            if socket.write_all(&frame).await.is_err() {
                println!("Viewer disconnected.");
                break;
            }
        }
    } else {
        println!("No source client found with user ID: {}", user_id);
    }
}
