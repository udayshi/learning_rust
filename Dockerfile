# --- Stage 1: Build Stage ---
FROM rust:1.89-alpine AS builder

WORKDIR /app

# Install musl development tools required for static compilation
RUN apk add --no-cache musl-dev

# # Copy dependency manifests first so dependency compilation remains cached when only source files change.
# COPY Cargo.toml Cargo.lock ./
# RUN mkdir src \
#     && printf 'fn main() {}\n' > src/main.rs \
#     && cargo build --release \
#     && rm -rf src

# # This layer invalidates only when application source files change.
# COPY src ./src
COPY  . .
RUN cargo build --release

# --- Stage 2: Runtime Stage (Alpine) ---
FROM alpine:latest

WORKDIR /app

# Copy the statically compiled binary from the builder stage
COPY --from=builder /app/target/release/rust_practise /app/rust_practise
EXPOSE 80
# Run the binary
CMD ["./rust_practise"]


#docker build -t learning_rust-alpine:latest .
