FROM rust:1.73.0-slim-bookworm AS builder

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY /src /src
RUN cargo build --release

FROM debian:bookworm-slim

# RUN apt-get update && apt install -y pkg-config

# RUN apt install -y libssl3
RUN apt-get update && apt-get install -y pkg-config && apt-get install -y libssl3 && apt clean && rm -rf /var/lib/apt/lists/*

COPY --from=builder ./target/release/gitsite ./target/release/gitsite
CMD ["/target/release/gitsite"]
