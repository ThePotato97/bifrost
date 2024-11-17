# Building Stage
ARG RUST_VERSION=1.80.1
FROM rust:${RUST_VERSION}-alpine AS build

# Install dependencies for building Rust applications with musl
RUN apk add --no-cache musl-dev

# Set the working directory to /app
WORKDIR /app

# Copy necessary files for building
COPY LICENSE LICENSE
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Build the project using musl
RUN cargo build --target x86_64-unknown-linux-musl --locked --release

# Copy the built binary to the root of the final image
RUN cp target/x86_64-unknown-linux-musl/release/bifrost /app/bifrost

# Final Stage
FROM alpine:latest

# Set /app as the working directory in the final image
WORKDIR /app

# Copy the binary from the build stage
COPY --from=build /app/bifrost /app/bifrost

# Set the binary as the entrypoint
ENTRYPOINT ["/app/bifrost"]
CMD ["/app/bifrost"]
