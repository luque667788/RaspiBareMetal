FROM ubuntu:22.04

# Avoid interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies for building QEMU and other tools
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    gcc-aarch64-linux-gnu \
    git \
    python3 \
    python3-venv \
    python3-pip \
    python3-setuptools \
    ninja-build \
    pkg-config \
    libglib2.0-dev \
    libfdt-dev \
    libpixman-1-dev \
    zlib1g-dev \
    libslirp-dev \
    libcap-ng-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies needed for QEMU build
RUN pip3 install tomli

# Build and install QEMU from source
RUN git clone https://gitlab.com/qemu-project/qemu.git && \
    cd qemu && \
    git checkout v10.0.2 && \
    ./configure --target-list=aarch64-softmmu --enable-slirp && \
    make -j$(nproc) && \
    make install && \
    cd .. && \
    rm -rf qemu

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Add the ARM64 target
RUN rustup target add aarch64-unknown-none

# Set working directory
WORKDIR /app

# Default command
CMD ["make", "all"]