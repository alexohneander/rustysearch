# 1. This tells docker to use the Rust official image
FROM rustlang/rust:nightly-bullseye as BUILDER

# Define WORKDIR
WORKDIR /build

# 2. Copy the files in your machine to the Docker image
COPY ./ ./

# Build your program for release
RUN cargo build --release

# Runner Step
FROM rustlang/rust:nightly-bullseye as RUNNER

# Define WORKDIR
WORKDIR /app

# Copy Binary and License
COPY --from=BUILDER /build/target/release/rustysearch /app/
COPY --from=BUILDER /build/LICENSE /app/

# Expose Port
EXPOSE 4000

# Run the binary
CMD ["/app/rustysearch"]
