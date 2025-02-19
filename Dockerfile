
FROM rust:latest AS builder

WORKDIR /usr/src/app

# Install necessary build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libx11-dev \
    libxcb1-dev \
    libxcb-randr0-dev \
    libxcb-shape0-dev \
    libxcb-xfixes0-dev \
    libxcb-shm0-dev \
    libxrandr-dev \
    libxinerama-dev \
    libxcursor-dev \
    libxi-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the entire project
COPY . .

# Build the application in release mode
RUN cargo build --release

# === Runtime Stage ===
# Use a newer base image with updated glibc (Debian Bookworm)
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libx11-6 \
    libxcb1 \
    libxcb-randr0 \
    libxcb-shape0 \
    libxcb-xfixes0 \
    libxcb-shm0 \
    libxrandr2 \
    libxinerama1 \
    libxcursor1 \
    libxi6 \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/remote_desktop /app/server

# Expose the port used by your application
EXPOSE 8085

# Run the server binary (adjust the command-line arguments if needed)
CMD ["/app/server", "server"]
