FROM rust:1-alpine
VOLUME /app
WORKDIR /app
RUN apk add \
    # prevent "linking with `cc` failed" error
    alpine-sdk \
    # required to build pocketbase-sdk
    openssl openssl-dev
RUN cargo install cargo-watch
ENTRYPOINT cargo watch -x run
