# Use the official Rust image as a base image
FROM rust:latest AS builder

# Install system dependencies for X11 and windowing
RUN apt-get update && apt-get install -y \
    libxcb-randr0-dev \
    libx11-dev \
    libxrandr-dev \
    libxi-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/src/remote-desktop

# Copy Cargo.toml and Cargo.lock for dependency caching
COPY Cargo.toml Cargo.lock ./

# Create dummy source files for dependency caching
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the actual source code
COPY . .

# Build the project
RUN cargo build --release

# Use Debian Bookworm (which has glibc 2.36)
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libxcb1 \
    libx11-6 \
    libxrandr2 \
    libxi6 \
    libxcb-shm0 \
    libxcb-randr0 \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/remote-desktop/target/release/remote_desktop /usr/local/bin/remote_desktop

# Ensure the binary is executable
RUN chmod +x /usr/local/bin/remote_desktop

# Expose the port your application uses
EXPOSE 80
EXPOSE 443

# Set the entry point and default command
ENTRYPOINT ["/usr/local/bin/remote_desktop"]
CMD ["server"]
