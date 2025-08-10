FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends gosu && rm -rf /var/lib/apt/lists/*
RUN addgroup --system --gid 1000 app && \
    adduser  --system --uid 1000 --gid 1000 app
RUN install -d -o 1000 -g 1000 /data

WORKDIR /app
ENV DATABASE_URL=sqlite:///data/sleep.db
COPY --from=builder /app/target/release/sleep-api /app/sleep-api
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh
ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
EXPOSE 8080
