# Aether Compiler - Multi-Stage Docker Build
# Works on any cloud platform: AWS, GCP, Azure, DigitalOcean, etc.

# Stage 1: Build the Rust bootstrap compiler
FROM rust:1.75-slim AS builder

WORKDIR /build

# Install LLVM/Clang for LLVM IR compilation
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    && rm -rf /var/lib/apt/lists/*

# Copy bootstrap compiler source
COPY bootstrap-compiler/ ./bootstrap-compiler/

# Build the bootstrap compiler
WORKDIR /build/bootstrap-compiler
RUN cargo build --release

# Stage 2: Runtime image with Aether compiler
FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    clang \
    llvm \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /aether

# Copy the compiled bootstrap compiler
COPY --from=builder /build/bootstrap-compiler/target/release/aether_bootstrap /usr/local/bin/aether

# Copy Aether source files
COPY compiler/ ./compiler/
COPY stdlib/ ./stdlib/
COPY runtime/ ./runtime/
COPY bootstrap/ ./bootstrap/
COPY tests/ ./tests/

# Create output directory
RUN mkdir -p /aether/out

# Set entrypoint
ENTRYPOINT ["aether"]
CMD ["--help"]

# Usage:
#   docker build -t aether-compiler .
#   docker run aether-compiler hello.aether hello.ll
#   docker run -v $(pwd):/work aether-compiler /work/hello.aether /work/hello.ll
