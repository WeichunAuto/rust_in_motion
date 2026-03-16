# ---------- build stage ----------
FROM rust:1.90-slim-bookworm as builder

WORKDIR /app

# install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    perl \
    build-essential \
    ca-certificates \
    curl \
    nodejs \
    npm \
    && rm -rf /var/lib/apt/lists/*

# install wasm target
RUN rustup target add wasm32-unknown-unknown

# 安装 cargo-leptos
RUN cargo install cargo-leptos --locked

# copy project
COPY . .

# 安装前端依赖（解决 Tailwind watcher）
# RUN npm install
RUN npm install && npm install tw-animate-css

# build release
RUN cargo leptos build --release


# ---------- runtime ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# copy binary
COPY --from=builder /app/target/release/rust_in_motion /app/rust_in_motion

# copy leptos site
COPY --from=builder /app/target/site /app/site

# copy migrations (important)
COPY migrations /app/migrations

# copy config (important)
COPY config /app/config

EXPOSE 3000

CMD ["./rust_in_motion"]