FROM rust:1.85-slim AS builder
WORKDIR /app

# Copy only the relay crate (standalone, no workspace deps)
COPY Cargo.toml ./
COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/manda-relay /usr/local/bin/manda-relay
ENV RUST_LOG=manda_relay=info
EXPOSE 8080
CMD ["manda-relay"]
