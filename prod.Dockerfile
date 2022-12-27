# Build API Binary
FROM rust:1-alpine AS builder
RUN apk add \
    # prevent "linking with `cc` failed" error
    alpine-sdk \
    # required to build pocketbase-sdk
    openssl openssl-dev
COPY . .
RUN cargo build --release

# Run API Binary
FROM alpine:latest
COPY --from=builder ./target/release/exyleio-api ./exyleio-api
COPY Rocket.toml Rocket.toml
ENTRYPOINT ./exyleio-api
