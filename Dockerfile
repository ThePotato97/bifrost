# Building Stage
ARG RUST_VERSION=1.85
FROM rust:${RUST_VERSION}-slim-bookworm AS build
WORKDIR /app

# Copy necessary files for building
COPY LICENSE LICENSE
COPY src/ src/
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

RUN --mount=type=bind,source=doc,target=doc \
    --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=crates,target=crates \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    <<EOF
set -e
apt-get -y update && apt-get install -y --no-install-recommends pkg-config libssl-dev
cargo build --locked --release
cp target/release/bifrost /bifrost
EOF

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
