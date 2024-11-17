# Building Stage
ARG RUST_VERSION=1.80.1
FROM rust:${RUST_VERSION}-alpine AS build

# Install dependencies for building Rust applications with musl
RUN apk add --no-cache musl-dev

WORKDIR /app

# Copy necessary files for building
COPY LICENSE LICENSE
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build the project using musl (no need for the `--mount` flag in a Dockerfile)
RUN cargo build --target x86_64-unknown-linux-musl --locked --release
RUN cp target/x86_64-unknown-linux-musl/release/bifrost /bifrost

# Final Stage
FROM alpine:latest

# Copy the binary from the build stage
COPY --from=build /bifrost /bifrost

# Set the binary as the entrypoint
ENTRYPOINT ["/bifrost"]

CMD ["/bifrost"]
