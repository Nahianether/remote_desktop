// use tokio::io::AsyncWriteExt;
// use tokio::net::TcpListener;

// use crate::modules::screen_capture::screen_capture::capture_screen;

// pub async fn start_server(address: &str) {
//     let listener = TcpListener::bind(address)
//         .await
//         .expect("Failed to bind address");
//     println!("Server is running on {}", address);

//     while let Ok((mut socket, _)) = listener.accept().await {
//         println!("Client connected!");

//         tokio::spawn(async move {
//             loop {
//                 let screen_data = capture_screen();
//                 if let Err(e) = socket.write_all(&screen_data).await {
//                     eprintln!("Failed to send frame: {}", e);
//                     break;
//                 }
//             }
//         });
//     }
// }
