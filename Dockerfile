FROM rust:latest

WORKDIR /usr/src/gannon

COPY Cargo.toml .
COPY src ./src

RUN cargo build --release

COPY templates ./templates

EXPOSE 8080

CMD ["./target/release/gannon"]

