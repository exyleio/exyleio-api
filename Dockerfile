FROM rust:1.65.0-alpine
# prevent cargo build error
RUN apk add --update alpine-sdk
VOLUME /app
WORKDIR /app
ENTRYPOINT ["cargo", "run"]
