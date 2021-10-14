FROM rust:1.50 AS base

ENV USER=root

RUN apt-get update && apt-get install -y libghc-postgresql-libpq-dev \
pkg-config libssl-dev argon2 clang llvm-dev libclang-dev \
libxcb-shape0-dev libxcb-xfixes0-dev

COPY . /app/
WORKDIR /app

RUN cargo fetch
RUN cargo install wasm-pack
RUN cargo install simple-http-server
RUN wasm-pack build --target web --out-name wasm --out-dir ./static/wasm

CMD ["simple-http-server", "-i", "./static/", "-p", "8001", "--nocache", "--try-file", "./static/index.html"]