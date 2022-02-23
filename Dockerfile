FROM rust:1.58

RUN USER=root cargo new --bin actix_demo
WORKDIR /actix_demo

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN touch src/main.rs
RUN cargo install --locked --path .

EXPOSE 8080

CMD ["actix_demo"]
