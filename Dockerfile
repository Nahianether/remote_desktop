# Use the official Rust image as a builder
FROM rust:latest AS builder
WORKDIR /usr/src/app

# Copy and build the application
COPY . .
RUN cargo build --release

# Use a minimal base image for production
FROM debian:bullseye-slim
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/rust-websocket-server /app/server

# Expose WebSocket port
EXPOSE 8080

# Run the WebSocket server
CMD ["./server"]
