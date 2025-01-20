mod screen_capture1;
mod server1;
mod client1;
mod viewer1;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <server|source|viewer> [address] [user_id]", args[0]);
        return;
    }

    let mode = &args[1];
    let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:8080");

    match mode.as_str() {
        "server" => server1::start_server(address).await,
        "source" => {
            let user_id = args.get(3).map(String::as_str).unwrap_or("default_source");
            client1::start_source_client(address, user_id).await;
        }
        "viewer" => {
            let user_id = args.get(3).map(String::as_str).expect("Viewer must provide a user ID to view.");
            viewer1::start_viewer_client(address, user_id).await;
        }
        _ => eprintln!("Invalid mode. Use 'server', 'source', or 'viewer'."),
    }
}
