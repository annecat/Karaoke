# ===============================
# 1️⃣ Build frontend with Trunk
# ===============================
FROM rust:1.92 AS frontend-builder

WORKDIR /app/frontend

# Install trunk
RUN cargo install trunk

# Copy frontend sources
COPY frontend/Cargo.toml frontend/Cargo.lock ./
COPY frontend/index.html ./
COPY frontend/static ./static
COPY frontend/src ./src

RUN rustup target add wasm32-unknown-unknown


# Build Yew frontend
RUN trunk build --release --public-url .

# ===============================
# 2️⃣ Build backend
# ===============================
FROM rust:1.92 AS backend-builder

WORKDIR /app/backend

# Copy backend sources
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy real backend code
COPY backend/res ./res
COPY backend/src ./src
COPY backend/migrations ./migrations 


RUN cargo build --release


#Runtime
FROM debian:bookworm-slim

WORKDIR /app

# Certificate HTTPS
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy backend binary
COPY --from=backend-builder /app/backend/target/release/karaoke ./karaoke

# Copy frontend public folder
COPY --from=frontend-builder /app/frontend/dist ./public

COPY --from=backend-builder /app/backend/res ./res 

# Cloud Run écoute sur 8080
EXPOSE 8080

CMD ["./karaoke"]
