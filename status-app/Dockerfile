# Stage 1: Builder
# We use nightly because your project seems to rely on it (based on previous config)
FROM rustlang/rust:nightly-bookworm AS builder

# 1. Install dependencies
# - npm: for tailwindcss (via trunk hooks)
# - pkg-config & libssl-dev: for common rust network crates
RUN apt-get update && apt-get install -y \
    npm \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# 2. Install Trunk (WASM bundler) & Add WASM target
# Use pre-built binary for speed
RUN cargo install --locked trunk
RUN rustup target add wasm32-unknown-unknown

# 3. Setup Workspace Schema
# We need to copy both the app and the sibling crate it depends on
WORKDIR /app
COPY hytale-health-checker /app/hytale-health-checker
COPY terraria-health-checker /app/terraria-health-checker
COPY status-monitor /app/status-monitor
COPY status-app /app/status-app

# 4. Build Frontend (Assets)
WORKDIR /app/status-app
# Install JS dependencies (TailwindCSS) since node_modules is ignored
RUN npm install
# This runs the tailwind hook and compiles src/lib.rs to dist/
RUN trunk build --release

# 5. Build Backend (Server)
# This compiles src/bin/server.rs to a native binary
RUN cargo build --release --bin server

# Stage 2: Runtime
# Minimal image to run the server
FROM debian:bookworm-slim AS runtime

WORKDIR /app

# Install SSL certs for making outbound requests (if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy artifacts from builder
COPY --from=builder /app/status-app/target/release/server /app/server
# Copy the compiled frontend assets to serve
COPY --from=builder /app/status-app/dist /app/dist

# Expose port (Internal container port)
EXPOSE 3000

# Set production environment
ENV RUST_LOG=info

# Run the server
CMD ["./server"]
