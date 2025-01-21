mod screen_capture1;
mod server1;
mod client1;
mod viewer1;

use std::env;

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <server|source|viewer> [address] [user_id]", args[0]);
        return;
    }

    let mode = &args[1];
    let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:8080");

    match mode.as_str() {
        "server" => {
            // Start the server to handle source and viewer connections
            println!("Starting server on address: {}", address);
            server1::start_server(address).await;
        }
        "source" => {
            // Start the source client to send screen data
            let user_id = args.get(3).map(String::as_str).unwrap_or("default_source");
            println!(
                "Starting source client with user ID: '{}' and connecting to server at {}",
                user_id, address
            );
            client1::start_source_client(address, user_id).await;
        }
        "viewer" => {
            // Start the viewer client to view screen data
            if let Some(user_id) = args.get(3).map(String::as_str) {
                println!(
                    "Starting viewer client to view user ID: '{}' and connecting to server at {}",
                    user_id, address
                );
                viewer1::start_viewer_client(address, user_id).await;
            } else {
                eprintln!("Error: Viewer must provide a user ID to view.");
                eprintln!("Usage: {} viewer [address] [user_id]", args[0]);
            }
        }
        _ => {
            eprintln!("Invalid mode. Use 'server', 'source', or 'viewer'.");
        }
    }
}
