FROM rust:1.90-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release --bin moviebox-api

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/moviebox-api /usr/local/bin/moviebox-api
ENV PORT=8080
EXPOSE 8080
CMD ["moviebox-api"]
