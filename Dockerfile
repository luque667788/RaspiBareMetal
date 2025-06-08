FROM ubuntu:22.04

# Avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install everything we need
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    gcc-aarch64-linux-gnu \
    qemu-system-aarch64 \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add the ARM64 target
RUN rustup target add aarch64-unknown-none

# Set working directory
WORKDIR /app

# Default command
CMD ["make", "all"]