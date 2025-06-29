FROM rust:1.88 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/sleep-api /sleep-api
CMD ["/sleep-api"]
EXPOSE 8080
