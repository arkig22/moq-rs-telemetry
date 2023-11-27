FROM rust:latest

# Install required utilities, ffmpeg, wget, and CA certificates
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    update-ca-certificates
    
# Create a build directory and copy over all of the files
WORKDIR /build
COPY . .

# Build the Rust project
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/build/target \
    cargo build --release && cp /build/target/release/moq-* /usr/local/cargo/bin

COPY ca-certificates/ /usr/local/share/ca-certificates/

# Set default command
RUN update-ca-certificates
CMD ["sleep", "infinity"]
