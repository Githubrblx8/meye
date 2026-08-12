# Build stage
FROM rust:1.75-slim-bullseye as builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    cmake \
    clang \
    && rm -rf /var/lib/apt/lists/*

# Copy workspace files
COPY Cargo.toml .
COPY crates/ crates/

# Create dummy main.rs for each crate to cache dependencies
RUN mkdir -p crates/api/src crates/smtp/src crates/reputation/src crates/risk/src crates/worker/src crates/db/src crates/models/src
RUN echo "fn main() {}" > crates/api/src/main.rs
RUN echo "fn main() {}" > crates/smtp/src/main.rs
RUN for crate in reputation risk worker db models; do echo "[lib]\npath = \"src/lib.rs\"" >> crates/$crate/Cargo.toml && echo "pub fn dummy() {}" > crates/$crate/src/lib.rs; done

# Build dependencies only (cache layer)
RUN cargo build --release --workspace || true
RUN rm -rf target/release/*.d target/release/build/*/*.o

# Remove dummy files and rebuild with actual code
RUN rm -f crates/api/src/main.rs crates/smtp/src/main.rs
RUN for crate in reputation risk worker db models; do rm -f crates/$crate/src/lib.rs; done
RUN git checkout -- crates/reputation/Cargo.toml crates/risk/Cargo.toml crates/worker/Cargo.toml crates/db/Cargo.toml crates/models/Cargo.toml 2>/dev/null || true

# Copy actual source code
COPY crates/ crates/

# Final build
RUN cargo build --release --workspace

# Runtime stage
FROM debian:bullseye-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 meye

# Copy binary from builder
COPY --from=builder /app/target/release/meye-api /usr/local/bin/meye-api
COPY --from=builder /app/target/release/meye-smtp /usr/local/bin/meye-smtp
COPY --from=builder /app/target/release/meye-worker /usr/local/bin/meye-worker

# Copy startup script
COPY scripts/meye-start /usr/local/bin/meye-start
RUN chmod +x /usr/local/bin/meye-start

# Create directories
RUN mkdir -p /app/logs /app/config && chown -R meye:meye /app

USER meye

EXPOSE 3000 2525

CMD ["meye-start"]
