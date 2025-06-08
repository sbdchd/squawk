# Builder stage
FROM rust:1-slim-bookworm AS builder

# Labels for the builder stage
LABEL stage=builder

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    libclang-dev \
    perl \
    pkg-config \
    make \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
WORKDIR /usr/src/squawk

# First, copy only the workspace configuration and toolchain files
COPY Cargo.toml Cargo.lock rust-toolchain.toml ./

# Copy the entire project
COPY . .

# Build the release version
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

# Metadata labels
LABEL org.opencontainers.image.title="Squawk"
LABEL org.opencontainers.image.description="Linter for PostgreSQL focused on database migrations"
LABEL org.opencontainers.image.licenses="Apache-2.0 OR MIT"
LABEL org.opencontainers.image.source="https://github.com/sbdchd/squawk"

# Create a non-root user to run the application
RUN groupadd -r squawk && useradd -r -g squawk squawk

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/squawk/target/release/squawk /usr/local/bin/squawk

# Set the ownership of the binary
RUN chown squawk:squawk /usr/local/bin/squawk

# Switch to non-root user
USER squawk

WORKDIR /data

# Command to run when the container starts
ENTRYPOINT ["squawk"]
CMD ["--help"]
