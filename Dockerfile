FROM rust:1.76 as build

RUN USER=root cargo new --bin docker
WORKDIR /docker

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/docker*
RUN cargo build --release

FROM scratch

COPY --from=build /docker/target/release/docker .
CMD ["./docker"]

