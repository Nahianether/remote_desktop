Remote Desktop Project (Rust + WebSocket)
This project is a remote desktop application similar to TeamViewer or AnyDesk, built using Rust and WebSocket technology. 
It consists of three main components: a server, a client, and an admin interface.

Overview
Server: The server is deployed on a domain or a real IP address using Docker. It acts as the central hub for communication between the client and the admin.
Client: The client captures the user's screen and sends the video data (in bytes format) to the server via WebSocket.
Admin: The admin interface receives the video data from the server and allows for real-time previewing of the client's screen.

Goal
The primary goal of this project is to deploy the server in a way that enables seamless communication between the client and the admin from anywhere over the internet. 
This ensures that the client's screen can be streamed and viewed remotely by the admin in real time.