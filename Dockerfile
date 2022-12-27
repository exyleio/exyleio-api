FROM rust:1-alpine
VOLUME /app
WORKDIR /app
ENTRYPOINT cargo run
