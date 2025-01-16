// mod modules;

// use std::env;

// use modules::{client::client, server::server};

// #[tokio::main]
// async fn main() {
//     let args: Vec<String> = env::args().collect();

//     if args.len() < 2 {
//         eprintln!("Usage: {} <server|client> [address]", args[0]);
//         return;
//     }

//     let mode = &args[1];
//     let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:8080");

//     match mode.as_str() {
//         "server" => server::start_server(address).await,
//         "client" => client::start_client(address).await,
//         _ => eprintln!("Invalid mode. Use 'server' or 'client'."),
//     }
// }


mod screen_capture1;
mod server1;
mod client1;
mod viewer1;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <server|source|viewer> [address]", args[0]);
        return;
    }

    let mode = &args[1];
    let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:8080");

    match mode.as_str() {
        "server" => server1::start_server(address).await,
        "source" => client1::start_source_client(address).await,
        "viewer" => viewer1::start_viewer_client(address).await,
        _ => eprintln!("Invalid mode. Use 'server', 'source', or 'viewer'."),
    }
}
