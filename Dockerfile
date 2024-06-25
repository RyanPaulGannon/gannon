# Stage 1: Build stage
FROM rust:1.79-slim-buster AS builder

WORKDIR /usr/src/gannon

# Copy only the dependency manifests to cache dependencies
COPY Cargo.lock Cargo.toml ./

# Create the source directory and dummy main.rs to satisfy cargo build
RUN mkdir src && \
    echo "fn main() { println!(\"Dummy main function\") }" > src/main.rs && \
    cargo build --release

# Copy the rest of the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Runtime stage
FROM debian:buster-slim

WORKDIR /usr/src/gannon

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/gannon/target/release/gannon /usr/src/gannon/gannon

EXPOSE 8080

ENTRYPOINT ["./gannon"]
# FROM rust:1.79-slim-buster AS builder
#
# WORKDIR /usr/src/gannon
#
# COPY Cargo.lock Cargo.toml ./
#
# RUN mkdir src && cargo build --release
#
# COPY . .
#
# RUN cargo build --release
#
# FROM debian:buster-slim
#
# WORKDIR /usr/src/
#
# COPY --from=builder /usr/src/myapp/target/release/gannon /usr/src/gannon
#
# EXPOSE 8080 
#
# ENTRYPOINT ["./gannon"]
#
