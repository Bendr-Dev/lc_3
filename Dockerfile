FROM rust:latest

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

ENTRYPOINT ["./target/release/lc_3"]
