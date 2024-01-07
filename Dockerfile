FROM rust:latest AS dev

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build
RUN cargo install cargo-watch
CMD ["./target/myapp"]

EXPOSE 3000

FROM rust:latest AS production

WORKDIR /usr/src/myapp

COPY . .

RUN cargo build --release

CMD ["./target/release/myapp"]

