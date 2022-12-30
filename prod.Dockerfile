# Build API Binary
FROM rust:1-alpine AS builder
# prevent "linking with `cc` failed" error
RUN apk add alpine-sdk
COPY . .
RUN cargo build --release

# Run API Binary
FROM alpine:latest
COPY --from=builder ./target/release/exyleio-api ./exyleio-api
COPY Rocket.toml Rocket.toml
CMD ["./exyleio-api"]
