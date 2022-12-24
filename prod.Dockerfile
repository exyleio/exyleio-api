FROM rust:1.65.0-alpine AS builder
# prevent cargo build error
RUN apk add --update alpine-sdk
COPY . .
RUN cargo build --release

FROM alpine:latest
COPY --from=builder ./target/release/exyleio-api exyleio-api
ENTRYPOINT exyleio-api
