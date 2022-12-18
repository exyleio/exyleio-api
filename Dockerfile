FROM jarredsumner/bun:edge
RUN apk add --update nodejs
VOLUME /app
WORKDIR /app
COPY package.json package.json
COPY bun.lockb bun.lockb
RUN bun install
COPY . .
ENTRYPOINT ["bun", "run", "nodemon"]
