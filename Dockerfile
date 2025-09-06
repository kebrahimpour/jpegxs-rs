# Development environment for JPEG XS
FROM rust:1.75-bookworm

# Install build dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    git \
    clang \
    llvm \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install pre-commit for hooks
RUN pip3 install --break-system-packages pre-commit

# Install Rust development tools
RUN rustup component add rustfmt clippy

# Install cargo tools
RUN cargo install cargo-watch cargo-edit cargo-audit

# Set working directory
WORKDIR /workspace

# Copy project files
COPY . .

CMD ["/bin/bash"]