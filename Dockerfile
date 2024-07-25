FROM rust:latest

WORKDIR /usr/src/justarticles

COPY . .

RUN cargo build

CMD cargo run