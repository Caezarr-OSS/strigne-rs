FROM rust:1.64.0-alpine

WORKDIR /app

COPY Cargo.toml .
COPY src/main.rs .

RUN cargo build --release

CMD ["./fichier_compresse"]
