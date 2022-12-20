FROM rust:1.65.0-alpine
VOLUME /app
WORKDIR /app
ENTRYPOINT ["cargo", "run"]
