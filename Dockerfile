# Use a multistage build, leveraging Rust's statically linked binaries
# to build a small image

# Build step - generate a compiled binary
FROM rust:latest AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime step - run the binary
FROM debian:bullseye-slim AS runtime

WORKDIR /app

# Install ca-certificates - it is needed to verify TLS certificates
# when establishing HTTPS connections
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Library from postgres required by Diesel
    && apt-get install libpq5 -y \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/rocket_rest_quickstart rocket_rest_quickstart

# We need these to run Diesel migrations
COPY migrations migrations

ENV ROCKET_ADDRESS=0.0.0.0

ENTRYPOINT ["./rocket_rest_quickstart"]