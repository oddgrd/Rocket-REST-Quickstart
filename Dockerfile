FROM rust:latest AS build

RUN USER=root cargo new --bin rocket_pg_template
WORKDIR /rocket_pg_template
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
COPY ./tests ./tests
COPY ./migrations ./migrations
COPY ./diesel.toml ./diesel.toml
RUN rm ./target/debug/deps/rocket_pg_template*
RUN cargo build

FROM buildpack-deps:stretch

COPY --from=build /rocket_pg_template/target/debug/rocket_pg_template /app/

CMD [ "/app/rocket_pg_template" ]