# Stage 1: Build
FROM rust:1.94-slim-bookworm as builder

WORKDIR /app

# Install dependencies needed for compilation (if any)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy only Cargo files first to cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy the rest of the source code
COPY . .

# Touch main.rs to ensure it gets recompiled
RUN touch src/main.rs
RUN cargo build --release

# Stage 2: Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL is required by many Rust crates)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder
COPY --from=builder /app/target/release/psisoft-backend ./psisoft-backend

# Copy migrations for auto-migration on startup
COPY --from=builder /app/migrations ./migrations

# Expose the API port
EXPOSE 3000

# Run the backend
CMD ["./psisoft-backend"]
