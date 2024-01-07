FROM rust:latest AS dev

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build
RUN cargo install cargo-watch
CMD ["./target/release/myapp"]

EXPOSE 3000
