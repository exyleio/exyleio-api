FROM rust:1-alpine
VOLUME /app
WORKDIR /app

# required to build pocketbase-sdk
RUN apk add openssl openssl-dev

ENTRYPOINT cargo run --target x86_64-unknown-linux-musl
