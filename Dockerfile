FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/sleep-api /app/sleep-api
COPY --from=builder /app/migrations /app/migrations
ENV DATABASE_URL=/data/sleep.db
CMD ["/app/sleep-api"]
EXPOSE 8080
