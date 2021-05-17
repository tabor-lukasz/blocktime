FROM rust:1.52 as builder

WORKDIR /usr/src/blocktime

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm src/*.rs

RUN rm -f target/release/deps/blocktime*

EXPOSE 12345

COPY . .

RUN cargo build --release

CMD ["/usr/src/blocktime/target/release/blocktime"]