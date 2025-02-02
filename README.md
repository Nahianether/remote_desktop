# 📌 remote_desktop
🚀 🎥 **remote_desktop** is a high-performance **remote desktop streaming** application built in **Rust**, inspired by **TeamViewer**. It allows a **server** to capture a client's screen and stream it to other connected clients in real-time.

## 🔥 Features
- ✅ **Multi-Client Support**: A single **server** can stream to multiple **clients** simultaneously.
- ✅ **Real-Time Video Capture**: Uses  crate to capture screen efficiently.
- ✅ **Async Data Streaming**: Built on top of **Tokio** for high-speed async communication.
- ✅ **Efficient Networking**: Uses **TCP/UDP sockets** for seamless transmission.

## 🛠 How It Works
1. **Server** listens for incoming connections from clients.
2. **Client** captures its screen using the  crate.
3. **Captured frames** are encoded and streamed to the server using **Tokio**.
4. **Server** receives the video feed and broadcasts it to all connected clients in real-time.
5. **Clients** can view the live feed from the server.

## 📦 Tech Stack
| Component           | Library/Framework |
|--------------------|-----------------|
| **Screen Capture** |          |
| **Networking**    |  (async TCP/UDP) |
| **Concurrency**   | , Rust async tasks |
| **Serialization** | ,   |
| **Transport**     | TCP/UDP sockets |

## 🚀 Getting Started
### **🔧 Prerequisites**
Ensure you have Rust installed on your system.
```sh
# Install Rust if not already installed
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### **🔹 Running the Server**
```sh
cargo run --bin server
```

### **🔹 Running the Client**
```sh
cargo run --bin client
```

## 📂 Repository Structure
```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── src
    ├── main.rs
    ├── modules
    └── test.txt

3 directories, 5 files
```

## 📜 Recent Commit History
- e6c6773 Merge branch 'master' of https://github.com/Nahianether/remote_desktop (12 seconds ago) by Nahianether\n

## 🎯 Roadmap & Future Improvements
- ✅ Add **encryption** for secure video streaming.
- ✅ Implement **frame compression** to reduce bandwidth usage.
- ✅ Introduce **WebRTC/WebSocket** for browser-based streaming.
- ✅ Develop a **GUI client** for easier user experience.

## 🤝 Contributing
Contributions are welcome! Feel free to open issues or pull requests.

## 📜 License
MIT License. See [LICENSE](LICENSE) for more details.

🕒 Last updated: Sun Feb  2 03:56:17 UTC 2025

_🚀 This README is auto-generated by GitHub Actions._
