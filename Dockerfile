FROM rust:1-alpine
VOLUME /app
WORKDIR /app
# prevent "linking with `cc` failed" error
RUN apk add alpine-sdk
RUN cargo install cargo-watch
ENTRYPOINT cargo watch -x run
