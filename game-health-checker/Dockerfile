FROM rust:1.84-slim-bookworm as builder

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Add wasm target
RUN rustup target add wasm32-unknown-unknown

# Install trunk
RUN cargo install --locked trunk

WORKDIR /app

# Copy the entire workspace
COPY . .

# Build frontend
# trunk expects to run where the index.html is, usually.
# frontend/index.html exists.
WORKDIR /app/frontend
RUN trunk build --release

# Build backend
WORKDIR /app
# We build from root so workspace resolution works perfectly
RUN cargo build --release --bin backend

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/backend ./sentinel

# Copy frontend assets
# The code expects "frontend/dist" or "../frontend/dist".
# We create "frontend/dist" in the working directory.
COPY --from=builder /app/frontend/dist ./frontend/dist

# Expose the port
EXPOSE 3000

# Set the command
CMD ["./sentinel"]
