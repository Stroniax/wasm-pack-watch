FROM rust:1.73.0-bullseye as build-watch
RUN cargo install wasm-pack
COPY . /src
RUN cargo install --path ./src

ENTRYPOINT [ "wasm-pack-watch", "/watch", "/build" ]