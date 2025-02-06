use crate::{
    helpers::enums::Mode,
    modules::{client::client::run_client, server::ws::run_ws},
};

use anyhow::{bail, Result};
use std::env;

pub async fn run() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 2 {
        eprintln!(
            "Usage: {} <server|admin|client> [address] [user_id]",
            args[0]
        );
        return;
    }

    let mode = args.get(1).unwrap().as_str();
    println!("{:?}", mode);
    match user_mod(mode) {
        Ok(Mode::Server) => run_ws().await,
        Ok(Mode::Admin) => {}
        Ok(Mode::Client) => {
            if args.len() < 4 {
                eprintln!(
                    "Usage: {} <server|admin|client> [address] [user_id]",
                    args[0]
                );
                return;
            }
            let socket_addr = args.get(2).map(String::as_str).unwrap();
            let user_id = args.get(3).map(String::as_str).unwrap();
            match run_client(user_id, socket_addr).await {
                Ok(_) => println!("Client connection Closed"),
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }

    // let mode = &args[1];
    // let address = args.get(2).map(String::as_str).unwrap_or("127.0.0.1:8080");

    // match mode.as_str() {
    //     "server" => {
    //         // Start the server to handle source and viewer connections
    //         println!("Starting server on address: {}", address);
    //         start_server(address).await;
    //     }
    //     "source" => {
    //         // Start the source client to send screen data
    //         let user_id = args.get(3).map(String::as_str).unwrap_or("default_source");
    //         println!(
    //             "Starting source client with user ID: '{}' and connecting to server at {}",
    //             user_id, address
    //         );
    //         start_source_client(address, user_id).await;
    //     }
    //     "viewer" => {
    //         // Start the viewer client to view screen data
    //         if let Some(user_id) = args.get(3).map(String::as_str) {
    //             println!(
    //                 "Starting viewer client to view user ID: '{}' and connecting to server at {}",
    //                 user_id, address
    //             );
    //             start_viewer_client(address, user_id).await;
    //         } else {
    //             eprintln!("Error: Viewer must provide a user ID to view.");
    //             eprintln!("Usage: {} viewer [address] [user_id]", args[0]);
    //         }
    //     }
    //     _ => {
    //         eprintln!("Invalid mode. Use either 'server' or 'source', or 'viewer'.");
    //     }
    // }
}

fn user_mod(mode: &str) -> Result<Mode> {
    match mode {
        "server" => Ok(Mode::Server),
        "admin" => Ok(Mode::Admin),
        "client" => Ok(Mode::Client),
        _ => bail!(
            "Invalid mode. Use either 'server' or 'admin', or 'client'. You provided: `{}`",
            mode
        ),
    }
}
