FROM rust:latest AS builder
WORKDIR /usr/src/app
COPY . .
RUN apt-get update && apt-get install -y libclang-dev && apt-get install -y cmake
RUN cargo build --release

FROM debian:stable
WORKDIR /usr/src/app
COPY --from=builder /usr/src/app/target/release/smolurl .
CMD [ "./smolurl", "--host", "0.0.0.0" ]
