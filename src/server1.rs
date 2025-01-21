use std::collections::HashMap;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, Mutex};

type Clients = Arc<Mutex<HashMap<String, broadcast::Sender<Vec<u8>>>>>;

pub async fn start_server(address: &str) {
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind address");
    println!("Server is running on {}", address);

    // Shared registry of active users (user ID -> broadcast channel)
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    loop {
        let (socket, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");

        let clients_clone = clients.clone();

        // Spawn a task to handle each client connection
        tokio::spawn(async move {
            handle_connection(socket, clients_clone).await;
        });
    }
}

async fn handle_connection(mut socket: TcpStream, clients: Clients) {
    println!("Client connected!");

    // Identify the type of client (SOURCE or VIEWER)
    let mut buf = vec![0; 7]; // Expect exactly 7 bytes ("SOURCE\n" or "VIEWER\n")
    if socket.read_exact(&mut buf).await.is_err() {
        println!("Failed to identify client type.");
        return;
    }
    let client_type = String::from_utf8_lossy(&buf).trim().to_string();

    // Read the user ID (64 bytes)
    let mut buf = vec![0; 64];
    if socket.read_exact(&mut buf).await.is_err() {
        println!("Failed to receive user ID.");
        return;
    }
    let user_id = String::from_utf8_lossy(&buf).trim().to_string();

    if client_type == "SOURCE" {
        handle_source_client(socket, user_id, clients).await;
    } else if client_type == "VIEWER" {
        handle_viewer_client(socket, user_id, clients).await;
    } else {
        println!("Unknown client type: {}", client_type);
    }
}

async fn handle_source_client(
    mut socket: TcpStream,
    user_id: String,
    clients: Clients,
) {
    println!("Source client connected with user ID: {}", user_id);

    // Create a broadcast channel for this source
    let (tx, _) = broadcast::channel::<Vec<u8>>(10);

    // Register the source in the clients registry
    let mut clients_guard = clients.lock().await;
    clients_guard.insert(user_id.clone(), tx.clone());
    drop(clients_guard); // Release the lock

    // Continuously read frames from the source and broadcast them
    let mut frame = vec![0; 1920 * 1080 * 4]; // Assuming 1920x1080 resolution
    while let Ok(_) = socket.read_exact(&mut frame).await {
        println!("Received frame from source: {} bytes", frame.len());
        if tx.send(frame.clone()).is_err() {
            println!("No viewers are connected.");
        }
    }

    println!("Source client disconnected: {}", user_id);

    // Remove the source from the clients registry
    let mut clients_guard = clients.lock().await;
    clients_guard.remove(&user_id);
}

async fn handle_viewer_client(
    mut socket: TcpStream,
    target_user_id: String,
    clients: Clients,
) {
    println!("Viewer client requested to view user ID: {}", target_user_id);

    // Look up the target source in the registry
    let clients_guard = clients.lock().await;
    if let Some(tx) = clients_guard.get(&target_user_id) {
        println!("Found source for user ID: {}", target_user_id);

        // Subscribe to the broadcast channel
        let mut rx = tx.subscribe();

        // Continuously send frames to the viewer
        while let Ok(frame) = rx.recv().await {
            if socket.write_all(&frame).await.is_err() {
                println!("Viewer disconnected.");
                break;
            }
        }
    } else {
        println!("No source found for user ID: {}", target_user_id);
    }
}
