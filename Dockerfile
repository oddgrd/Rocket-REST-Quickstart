FROM rust:latest AS builder

RUN USER=root cargo new --bin rocket_pg_template
WORKDIR /rocket_pg_template
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/rocket_pg_template*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=builder /rocket_pg_template/target/debug/rocket_pg_template /app/

ENV ROCKET_ADDRESS=0.0.0.0

CMD [ "/app/rocket_pg_template" ]