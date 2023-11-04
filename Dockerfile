# Using full rust image. This creates a large image size but works
# with libssl
# TODO Investigate using a smaller base image
FROM rust:1.73.0

RUN apt-get update && apt-get install -y libssl-dev

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["/app/target/release/gitsite"]