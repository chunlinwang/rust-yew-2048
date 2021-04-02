FROM rust:1.50 AS base

ENV USER=root

RUN apt-get update && apt-get install -y libghc-postgresql-libpq-dev \
pkg-config libssl-dev argon2 clang llvm-dev libclang-dev \
libxcb-shape0-dev libxcb-xfixes0-dev

WORKDIR /app
RUN cargo init
COPY . /app/

RUN cargo fetch
RUN cargo install wasm-pack
RUN cargo install simple-http-server

CMD ["simple-http-server", "-p", "8001", "-i"]